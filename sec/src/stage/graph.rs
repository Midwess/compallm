use super::{ProcessorConfig, compiled::CompiledStage};
use crate::access::Access;
use std::{
    any::TypeId,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    io,
};

#[derive(Default)]
pub(super) struct StageGraph {
    pub nodes: Vec<Option<ProcessorConfig>>,
    pub changed: bool,
}
impl StageGraph {
    pub fn add(&mut self, mut configs: Vec<ProcessorConfig>) {
        let offset = self.nodes.len();
        for config in &mut configs {
            for predecessor in &mut config.predecessors {
                *predecessor += offset;
            }
        }
        self.nodes.extend(configs.into_iter().map(Some));
        self.changed = true;
    }
    pub fn compile(&self) -> io::Result<CompiledStage> {
        let count = self.nodes.len();
        let mut lookup = HashMap::<TypeId, Vec<usize>>::new();
        let mut accesses = Vec::with_capacity(count);
        for (index, node) in self.nodes.iter().enumerate() {
            let node = node.as_ref().expect("available processor");
            node.processor.validate()?;
            let processor = node
                .access_override
                .clone()
                .unwrap_or_else(|| node.processor.access());
            let mut conditions = Access::default();
            for condition in &node.conditions {
                condition.validate()?;
                conditions.merge(condition.access());
            }
            accesses.push((processor, conditions, node.processor.is_async()));
            lookup
                .entry(node.processor.processor_type())
                .or_default()
                .push(index);
        }
        let mut edges = HashSet::new();
        for (index, node) in self.nodes.iter().enumerate() {
            let node = node.as_ref().expect("available processor");
            for &predecessor in &node.predecessors {
                edges.insert((predecessor, index));
            }
            for (targets, before) in [(&node.before, true), (&node.after, false)] {
                for target in targets {
                    let matches = lookup.get(target).ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "stage dependency target is not registered",
                        )
                    })?;
                    for &other in matches {
                        edges.insert(if before {
                            (index, other)
                        } else {
                            (other, index)
                        });
                    }
                }
            }
        }
        let mut counts = vec![0; count];
        let mut dependents = vec![Vec::new(); count];
        for (from, to) in edges {
            counts[to] += 1;
            dependents[from].push(to);
        }
        for dependents in &mut dependents {
            dependents.sort_unstable();
        }
        let mut remaining = counts.clone();
        let mut ready = BinaryHeap::new();
        for (index, &deps) in counts.iter().enumerate() {
            if deps == 0 {
                ready.push(Reverse(index));
            }
        }
        let mut order = Vec::with_capacity(count);
        while let Some(Reverse(index)) = ready.pop() {
            order.push(index);
            for &dependent in &dependents[index] {
                remaining[dependent] -= 1;
                if remaining[dependent] == 0 {
                    ready.push(Reverse(dependent));
                }
            }
        }
        if order.len() != count {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cycle in stage dependencies",
            ));
        }
        Ok(CompiledStage::new(order, counts, dependents, accesses))
    }
}
