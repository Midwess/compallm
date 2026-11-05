use crate::{
    resources::Resources,
    stage::{StageLabel, Stages},
};
use std::io;

/// Independent state with lazy stages, manually invoked by the consumer's driver.
pub struct Execution {
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
}
