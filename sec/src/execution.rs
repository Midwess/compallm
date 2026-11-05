use crate::{
    resources::Resources,
    stage::{StageLabel, Stages},
};
use std::{io, num::NonZeroUsize};

/// Independent state with lazy stages, manually invoked by the consumer's driver.
pub struct Execution {
    pub(crate) max_parallelism: NonZeroUsize,
    pub(crate) resources: Resources,
    pub(crate) stages: Stages,
}
impl Execution {
    pub fn resources(&self) -> &Resources {
        &self.resources
    }
    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }
    /// Repeated calls reuse this execution's processor state. No automatic rewind occurs.
    pub fn run_stage(&mut self, label: impl StageLabel) -> io::Result<()> {
        self.stages.run(label, &mut self.resources)
    }
    /// Run compatible processors concurrently. Requires Tokio; cancellation invalidates the stage.
    pub async fn run_stage_async(&mut self, label: impl StageLabel) -> io::Result<()> {
        self.stages
            .run_async(label, &mut self.resources, self.max_parallelism)
            .await
    }
}
