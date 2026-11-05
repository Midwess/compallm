use super::{bitset::BitSet, compiled::CompiledStage};

/// Reusable run state, following Bevy's ready/running/completed sets and dependency counters.
#[derive(Default)]
pub(super) struct ExecutorState {
    pub ready: BitSet,
    pub running: BitSet,
    pub completed: BitSet,
    pub remaining: Vec<usize>,
    pub num_running: usize,
    pub num_completed: usize,
    pub task_failed: bool,
}
impl ExecutorState {
    pub fn reset(&mut self, compiled: &CompiledStage) {
        self.ready.copy_from(&compiled.starting);
        self.running.reset(compiled.metadata.len());
        self.completed.reset(compiled.metadata.len());
        self.remaining.clone_from(&compiled.dependency_counts);
        self.num_running = 0;
        self.num_completed = 0;
        self.task_failed = false;
    }
    #[inline]
    pub fn can_run(&self, index: usize, compiled: &CompiledStage) -> bool {
        let meta = &compiled.metadata[index];
        meta.processor_conflicts.is_disjoint(&self.running)
            && meta.condition_conflicts.is_disjoint(&self.running)
    }
    pub fn start(&mut self, index: usize) {
        self.ready.remove(index);
        self.running.insert(index);
        self.num_running += 1;
    }
    pub fn finish_running(&mut self, index: usize) {
        self.running.remove(index);
        self.num_running -= 1;
    }
    pub fn complete(&mut self, index: usize, compiled: &CompiledStage) {
        self.completed.insert(index);
        self.num_completed += 1;
        for &dependent in &compiled.dependents[index] {
            self.remaining[dependent] -= 1;
            if self.remaining[dependent] == 0 && !self.completed.contains(dependent) {
                self.ready.insert(dependent);
            }
        }
    }
}
