use crate::{
    processor::{IntoProcessor, Processor},
    resources::Resources,
};
use std::{any::TypeId, collections::HashMap, io, sync::Arc};

/// Stage identity is its Rust type. Registration does not execute it.
pub trait StageLabel: Send + Sync + 'static {}
pub struct HeaderReceived;
impl StageLabel for HeaderReceived {}

/// Runs after HeaderReceived succeeds, before consuming any body bytes.
/// This transition also occurs for requests with an empty body.
pub struct BodyStarted;
impl StageLabel for BodyStarted {}

/// Ordered processor instances whose parameter and local state persist between runs.
#[derive(Default)]
pub struct Stage {
    processors: Vec<Box<dyn Processor>>,
}
impl Stage {
    pub fn add_processors<M>(&mut self, processors: impl IntoProcessors<M>) {
        self.processors.extend(processors.into_processors());
    }
    pub fn run(&mut self, resources: &mut Resources) -> io::Result<()> {
        for processor in &mut self.processors {
            if let Err(error) = processor.run(resources) {
                resources.discard_pending();
                return Err(error);
            }
            resources.apply_pending();
        }
        Ok(())
    }
}

pub trait IntoProcessors<Marker>: Sized {
    fn into_processors(self) -> Vec<Box<dyn Processor>>;
    /// Explicitly group an ordered sequence; all execution is sequential for now.
    fn chain(self) -> Chain<Self> {
        Chain(self)
    }
}
pub struct Single<M>(std::marker::PhantomData<M>);
pub struct Tuple<M>(std::marker::PhantomData<M>);
pub struct Chained<M>(std::marker::PhantomData<M>);
#[derive(Clone)]
pub struct Chain<T>(T);

impl<F, M> IntoProcessors<Single<M>> for F
where
    F: IntoProcessor<M>,
{
    fn into_processors(self) -> Vec<Box<dyn Processor>> {
        vec![Box::new(self.into_processor())]
    }
}
impl<T, M> IntoProcessors<Chained<M>> for Chain<T>
where
    T: IntoProcessors<M>,
{
    fn into_processors(self) -> Vec<Box<dyn Processor>> {
        self.0.into_processors()
    }
}
macro_rules! processor_tuple {
    ($($item:ident : $marker:ident : $index:tt),+) => {
        impl<$($item, $marker),+> IntoProcessors<Tuple<($($marker,)+)>> for ($($item,)+)
        where $($item: IntoProcessors<$marker>),+
        {
            fn into_processors(self) -> Vec<Box<dyn Processor>> {
                let mut processors = Vec::new();
                $(processors.extend(self.$index.into_processors());)+
                processors
            }
        }
    };
}
processor_tuple!(A: MA: 0, B: MB: 1);
processor_tuple!(A: MA: 0, B: MB: 1, C: MC: 2);
processor_tuple!(A: MA: 0, B: MB: 1, C: MC: 2, D: MD: 3);

type Configure = dyn Fn(&mut Stage) + Send + Sync;

/// Shared, immutable definitions; executable processor state is connection-local.
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

/// Connection-local stage cache, initialized only for moments that are reached.
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

    pub(crate) fn run<L: StageLabel>(&mut self, _: L, resources: &mut Resources) -> io::Result<()> {
        use std::collections::hash_map::Entry;
        let stage = match self.stages.entry(TypeId::of::<L>()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let Some(stage) = self.definitions.build(*entry.key()) else {
                    return Ok(());
                };
                entry.insert(stage)
            }
        };
        stage.run(resources)
    }
}
