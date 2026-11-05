use super::bitset::BitSet;
use crate::access::Access;

/// Immutable executable metadata, prepared once after graph changes (Bevy SystemSchedule).
pub(super) struct CompiledStage {
    pub order: Vec<usize>,
    pub dependency_counts: Vec<usize>,
    pub dependents: Vec<Vec<usize>>,
    pub metadata: Vec<TaskMetadata>,
    pub starting: BitSet,
    pub has_async: bool,
}
pub(super) struct TaskMetadata {
    pub processor_conflicts: BitSet,
    pub condition_conflicts: BitSet,
    pub is_async: bool,
}
impl CompiledStage {
    pub fn new(
        order: Vec<usize>,
        dependency_counts: Vec<usize>,
        dependents: Vec<Vec<usize>>,
        accesses: Vec<(Access, Access, bool)>,
    ) -> Self {
        let count = accesses.len();
        let mut metadata = Vec::with_capacity(count);
        for (processor, condition, is_async) in &accesses {
            let mut processor_conflicts = BitSet::new(count);
            let mut condition_conflicts = BitSet::new(count);
            for (other, (other_processor, other_condition, _)) in accesses.iter().enumerate() {
                if !processor.compatible(other_processor) || !processor.compatible(other_condition)
                {
                    processor_conflicts.insert(other);
                }
                if !condition.compatible(other_processor) || !condition.compatible(other_condition)
                {
                    condition_conflicts.insert(other);
                }
            }
            metadata.push(TaskMetadata {
                processor_conflicts,
                condition_conflicts,
                is_async: *is_async,
            });
        }
        let mut starting = BitSet::new(count);
        for (node, &dependencies) in dependency_counts.iter().enumerate() {
            if dependencies == 0 {
                starting.insert(node);
            }
        }
        let has_async = accesses.iter().any(|(_, _, is_async)| *is_async);
        Self {
            order,
            dependency_counts,
            dependents,
            metadata,
            starting,
            has_async,
        }
    }
}
