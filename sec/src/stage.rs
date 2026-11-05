//! Compiled stage scheduling adapted from Bevy ECS; see sec/NOTICE.
mod bitset;
mod compiled;
mod config;
mod executor;
mod graph;
mod registry;
mod state;

pub use config::*;
pub(crate) use registry::{StageDefinitions, Stages};

use crate::{
    processor::{Condition, Processor},
    resources::Resources,
};
use compiled::CompiledStage;
use graph::StageGraph;
use state::ExecutorState;
use std::{any::TypeId, io, num::NonZeroUsize};

pub trait StageLabel: Send + Sync + 'static {}

pub struct ProcessorConfig {
    processor: Box<dyn Processor>,
    before: Vec<TypeId>,
    after: Vec<TypeId>,
    predecessors: Vec<usize>,
    conditions: Vec<Box<dyn Condition>>,
}
impl ProcessorConfig {
    fn allowed(&mut self, resources: &Resources) -> io::Result<bool> {
        for condition in &mut self.conditions {
            if !condition.evaluate(resources)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[derive(Default)]
pub struct Stage {
    graph: StageGraph,
    compiled: Option<CompiledStage>,
    state: ExecutorState,
    interrupted: bool,
}
impl Stage {
    pub fn add_processors<M>(&mut self, processors: impl IntoProcessors<M>) {
        self.graph.add(processors.into_processors());
    }
    fn initialize(&mut self) -> io::Result<()> {
        if self.interrupted {
            return Err(io::Error::other(
                "stage was cancelled or panicked; drop its execution",
            ));
        }
        if self.compiled.is_none() || self.graph.changed {
            self.compiled = Some(self.graph.compile()?);
            self.graph.changed = false;
        }
        Ok(())
    }
    pub fn run(&mut self, resources: &mut Resources) -> io::Result<()> {
        self.initialize()?;
        if self.compiled.as_ref().expect("compiled stage").has_async {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "async stage requires run_stage_async",
            ));
        }
        self.interrupted = true;
        let result = executor::sequential::run(
            &mut self.graph,
            self.compiled.as_ref().expect("compiled stage"),
            resources,
        );
        self.interrupted = false;
        result
    }
    pub async fn run_async(
        &mut self,
        resources: &mut Resources,
        limit: NonZeroUsize,
    ) -> io::Result<()> {
        self.initialize()?;
        tokio::runtime::Handle::try_current()
            .map_err(|_| io::Error::other("async SEC execution requires Tokio"))?;
        self.interrupted = true;
        let result = executor::concurrent::run(
            &mut self.graph,
            self.compiled.as_ref().expect("compiled stage"),
            &mut self.state,
            resources,
            limit,
        )
        .await;
        if !self.state.task_failed {
            self.interrupted = false;
        }
        result
    }
}
