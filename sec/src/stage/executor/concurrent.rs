//! Completion-driven executor adapted from Bevy's readiness/dependency algorithm.
//! Safe hecs ownership and Tokio tasks replace Bevy's UnsafeWorldCell/scoped tasks.
use crate::{
    context_writer::InsertResource,
    resources::Resources,
    stage::{ProcessorConfig, compiled::CompiledStage, graph::StageGraph, state::ExecutorState},
};
use std::{collections::BTreeMap, io, num::NonZeroUsize};

type Pending = Vec<Box<dyn InsertResource>>;
type Outcome = (usize, ProcessorConfig, io::Result<bool>, Pending);

pub(in crate::stage) async fn run(
    graph: &mut StageGraph,
    compiled: &CompiledStage,
    state: &mut ExecutorState,
    resources: &mut Resources,
    limit: NonZeroUsize,
) -> io::Result<()> {
    state.reset(compiled);
    let mut tasks = tokio::task::JoinSet::<Outcome>::new();
    let mut unapplied = BTreeMap::<usize, Pending>::new();
    let mut failure = None;
    loop {
        // Structural hecs changes require quiescence, like Bevy's ApplyDeferred.
        if tasks.is_empty() && !unapplied.is_empty() {
            for (index, pending) in std::mem::take(&mut unapplied) {
                resources.apply(pending);
                state.complete(index, compiled);
            }
        }
        if failure.is_none() && unapplied.is_empty() {
            while state.num_running < limit.get() {
                let next = state
                    .ready
                    .ones()
                    .find(|&index| state.can_run(index, compiled));
                let Some(index) = next else {
                    break;
                };
                state.start(index);
                let mut config = graph.nodes[index].take().expect("available processor");
                let mut invocation = resources.invocation();
                if compiled.metadata[index].is_async {
                    tasks.spawn(async move {
                        let result = async {
                            if !config.allowed(&invocation)? {
                                return Ok(false);
                            }
                            config.processor.run_async(&invocation).await?;
                            Ok(true)
                        }
                        .await;
                        let pending = invocation.take_pending();
                        (index, config, result, pending)
                    });
                } else {
                    tasks.spawn_blocking(move || {
                        let result = (|| {
                            if !config.allowed(&invocation)? {
                                return Ok(false);
                            }
                            config.processor.run(&invocation)?;
                            Ok(true)
                        })();
                        let pending = invocation.take_pending();
                        (index, config, result, pending)
                    });
                }
            }
        }
        if tasks.is_empty() {
            if let Some(error) = failure {
                return Err(error);
            }
            if state.num_completed == compiled.metadata.len() {
                return Ok(());
            }
            return Err(io::Error::other("stage executor has no runnable work"));
        }
        match tasks.join_next().await.expect("running task") {
            Ok((index, config, result, pending)) => {
                graph.nodes[index] = Some(config);
                state.finish_running(index);
                match result {
                    Ok(true) if !pending.is_empty() => {
                        unapplied.insert(index, pending);
                    }
                    Ok(_) => state.complete(index, compiled),
                    Err(error) => {
                        if failure.is_none() {
                            failure = Some(error);
                        }
                    }
                }
            }
            Err(_) => {
                state.num_running -= 1;
                state.task_failed = true;
                if failure.is_none() {
                    failure = Some(io::Error::other(
                        "SEC processor task failed; drop its execution",
                    ));
                }
            }
        }
    }
}
