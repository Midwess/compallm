use super::{IntoProcessors, Stage, StageLabel};
use crate::resources::Resources;
use std::{any::TypeId, collections::HashMap, io, num::NonZeroUsize, sync::Arc};

type Configure = dyn Fn(&mut Stage) + Send + Sync;
#[derive(Clone, Default)]
pub(crate) struct StageDefinitions {
    definitions: HashMap<TypeId, Vec<Arc<Configure>>>,
}
impl StageDefinitions {
    pub(crate) fn add<M, C>(&mut self, label: impl StageLabel, processors: C)
    where
        C: IntoProcessors<M> + Clone + Send + Sync + 'static,
    {
        self.configure(label, move |stage| stage.add_processors(processors.clone()));
    }
    pub(crate) fn configure<L: StageLabel>(
        &mut self,
        _: L,
        configure: impl Fn(&mut Stage) + Send + Sync + 'static,
    ) {
        self.definitions
            .entry(TypeId::of::<L>())
            .or_default()
            .push(Arc::new(configure));
    }
    fn build(&self, label: TypeId) -> Option<Stage> {
        let configurations = self.definitions.get(&label)?;
        let mut stage = Stage::default();
        for configure in configurations {
            configure(&mut stage);
        }
        Some(stage)
    }
}

pub(crate) struct Stages {
    definitions: Arc<StageDefinitions>,
    stages: HashMap<TypeId, Stage>,
}
impl Stages {
    pub(crate) fn new(definitions: Arc<StageDefinitions>) -> Self {
        Self {
            definitions,
            stages: HashMap::new(),
        }
    }
    fn get_or_build<L: StageLabel>(&mut self, _: L) -> Option<&mut Stage> {
        use std::collections::hash_map::Entry;
        Some(match self.stages.entry(TypeId::of::<L>()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let stage = self.definitions.build(*entry.key())?;
                entry.insert(stage)
            }
        })
    }
    pub(crate) fn run<L: StageLabel>(
        &mut self,
        label: L,
        resources: &mut Resources,
    ) -> io::Result<()> {
        if let Some(stage) = self.get_or_build(label) {
            stage.run(resources)?;
        }
        Ok(())
    }
    pub(crate) async fn run_async<L: StageLabel>(
        &mut self,
        label: L,
        resources: &mut Resources,
        limit: NonZeroUsize,
    ) -> io::Result<()> {
        if let Some(stage) = self.get_or_build(label) {
            stage.run_async(resources, limit).await?;
        }
        Ok(())
    }
}
