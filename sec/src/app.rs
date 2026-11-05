use crate::{
    execution::Execution,
    plugin::Plugins,
    resources::{Resources, SharedResources},
    stage::{IntoProcessors, Stage, StageDefinitions, StageLabel, Stages},
};
use std::sync::Arc;

/// Registration container. It does not own a loop, listener, or protocol lifecycle.
#[derive(Default)]
pub struct App {
    definitions: Arc<StageDefinitions>,
    shared: Arc<SharedResources>,
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
    pub fn execution(&self) -> Execution {
        Execution {
            resources: Resources::with_shared(Arc::clone(&self.shared)),
            stages: Stages::new(Arc::clone(&self.definitions)),
        }
    }
}
