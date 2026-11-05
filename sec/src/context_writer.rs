use crate::resources::Resources;
use std::sync::{Arc, Mutex};

pub(crate) trait InsertResource: Send {
    fn apply(self: Box<Self>, resources: &mut Resources);
}
struct Insert<T>(T);
impl<T: Send + Sync + 'static> InsertResource for Insert<T> {
    fn apply(self: Box<Self>, resources: &mut Resources) {
        resources.insert(self.0);
    }
}

/// This mutex protects deferred queue operations, never component values or awaits.
#[derive(Clone, Default)]
pub(crate) struct PendingInsertions(Arc<Mutex<Vec<Box<dyn InsertResource>>>>);
impl PendingInsertions {
    pub(crate) fn writer(&self) -> ContextWriter {
        ContextWriter(self.clone())
    }
    fn push(&self, insertion: Box<dyn InsertResource>) {
        self.0
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .push(insertion);
    }
    pub(crate) fn take(&self) -> Vec<Box<dyn InsertResource>> {
        std::mem::take(&mut *self.0.lock().unwrap_or_else(|error| error.into_inner()))
    }
}

/// Processor-invocation-local insertions, applied only after successful processing.
#[derive(Clone)]
pub struct ContextWriter(PendingInsertions);
impl ContextWriter {
    pub fn insert_resource<T: Send + Sync + 'static>(&mut self, value: T) {
        self.0.push(Box::new(Insert(value)));
    }
}
