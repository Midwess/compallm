use crate::{
    execution::Execution,
    plugin::Plugins,
    resources::{Resources, SharedResources},
    stage::{IntoProcessors, Stage, StageDefinitions, StageLabel, Stages},
};
use std::{num::NonZeroUsize, sync::Arc};

/// Registration container. It does not own a loop, listener, or protocol lifecycle.
pub struct App {
    definitions: Arc<StageDefinitions>,
    shared: Arc<SharedResources>,
    max_parallelism: NonZeroUsize,
}
impl Default for App {
    fn default() -> Self {
        Self {
            definitions: Arc::default(),
            shared: Arc::default(),
            max_parallelism: std::thread::available_parallelism()
                .unwrap_or(NonZeroUsize::new(1).unwrap()),
        }
    }
}
impl App {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_processors<M, C>(&mut self, label: impl StageLabel, processors: C)
    where
        C: IntoProcessors<M> + Clone + Send + Sync + 'static,
    {
        Arc::make_mut(&mut self.definitions).add(label, processors);
    }
    pub fn configure_stage(
        &mut self,
        label: impl StageLabel,
        configure: impl Fn(&mut Stage) + Send + Sync + 'static,
    ) {
        Arc::make_mut(&mut self.definitions).configure(label, configure);
    }
    pub fn insert_shared_resource<T: Send + Sync + 'static>(&mut self, resource: Arc<T>) {
        Arc::make_mut(&mut self.shared).insert(resource);
    }
    pub fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) {
        plugins.build(self);
    }
    pub fn set_max_parallelism(&mut self, limit: NonZeroUsize) {
        self.max_parallelism = limit;
    }
    pub fn execution(&self) -> Execution {
        Execution {
            max_parallelism: self.max_parallelism,
            resources: Resources::with_shared(Arc::clone(&self.shared)),
            stages: Stages::new(Arc::clone(&self.definitions)),
        }
    }
}
