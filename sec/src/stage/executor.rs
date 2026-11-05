use super::Stage;
use crate::{access::Access, resources::Resources};
use std::{io, num::NonZeroUsize};

impl Stage {
    /// Bounded wave execution. Only compatible, dependency-ready work runs together.
    pub async fn run_async(
        &mut self,
        resources: &mut Resources,
        limit: NonZeroUsize,
    ) -> io::Result<()> {
        self.initialize()?;
        tokio::runtime::Handle::try_current()
            .map_err(|_| io::Error::other("async SEC execution requires Tokio"))?;
        self.interrupted = true;
        let mut completed = vec![false; self.processors.len()];
        while completed.iter().any(|done| !done) {
            let plan = self.plan.as_ref().expect("prepared stage");
            let mut wave = Vec::new();
            let mut access = Access::default();
            for &index in &plan.order {
                if completed[index] || !plan.predecessors[index].iter().all(|&p| completed[p]) {
                    continue;
                }
                let requested = self.processors[index]
                    .as_ref()
                    .expect("available processor")
                    .access();
                if wave.is_empty() || access.compatible(&requested) {
                    access.merge(requested);
                    wave.push(index);
                    if wave.len() == limit.get() {
                        break;
                    }
                }
            }
            let mut tasks = tokio::task::JoinSet::new();
            for index in wave {
                let mut config = self.processors[index].take().expect("available processor");
                let mut invocation = resources.invocation();
                if config.processor.is_async() {
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
            let mut outcomes = Vec::new();
            let mut task_failure = false;
            while let Some(result) = tasks.join_next().await {
                match result {
                    Ok(outcome) => outcomes.push(outcome),
                    Err(_) => task_failure = true,
                }
            }
            // All invocation storage borrows have ended before structural insertion.
            outcomes.sort_by_key(|outcome| outcome.0);
            let mut error = None;
            for (index, config, result, pending) in outcomes {
                self.processors[index] = Some(config);
                completed[index] = true;
                match result {
                    Ok(true) => resources.apply(pending),
                    Ok(false) => {}
                    Err(failure) => {
                        if error.is_none() {
                            error = Some(failure);
                        }
                    }
                }
            }
            if task_failure {
                return Err(io::Error::other(
                    "SEC processor task failed; drop its execution",
                ));
            }
            if let Some(error) = error {
                self.interrupted = false;
                return Err(error);
            }
        }
        self.interrupted = false;
        Ok(())
    }
}
