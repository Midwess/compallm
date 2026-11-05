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

    pub(crate) fn build(&self) -> Stages {
        let mut stages = HashMap::new();
        for (&label, configurations) in &self.definitions {
            let mut stage = Stage::default();
            for configure in configurations {
                configure(&mut stage);
            }
            stages.insert(label, stage);
        }
        Stages { stages }
    }
}

/// Executable stages owned exclusively by a single connection.
pub(crate) struct Stages {
    stages: HashMap<TypeId, Stage>,
}
impl Stages {
    pub(crate) fn run<L: StageLabel>(&mut self, _: L, resources: &mut Resources) -> io::Result<()> {
        if let Some(stage) = self.stages.get_mut(&TypeId::of::<L>()) {
            stage.run(resources)?;
        }
        Ok(())
    }
}
