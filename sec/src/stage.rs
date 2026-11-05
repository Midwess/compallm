use crate::{
    processor::{Condition, IntoCondition, IntoProcessor, Processor},
    resources::Resources,
};
use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    io,
    marker::PhantomData,
    sync::Arc,
};

/// A stage is selected by its label type; registration never runs it automatically.
pub trait StageLabel: Send + Sync + 'static {}

pub struct ProcessorConfig {
    processor: Box<dyn Processor>,
    before: Vec<TypeId>,
    after: Vec<TypeId>,
    predecessors: Vec<usize>,
    conditions: Vec<Box<dyn Condition>>,
}

/// A lazily prepared dependency plan and its execution-local processor instances.
#[derive(Default)]
pub struct Stage {
    processors: Vec<ProcessorConfig>,
    order: Option<Vec<usize>>,
}
impl Stage {
    pub fn add_processors<M>(&mut self, processors: impl IntoProcessors<M>) {
        append(&mut self.processors, processors.into_processors());
        self.order = None;
    }

    pub fn run(&mut self, resources: &mut Resources) -> io::Result<()> {
        if self.order.is_none() {
            self.order = Some(self.prepare()?);
        }
        for &index in self.order.as_ref().expect("prepared stage") {
            let config = &mut self.processors[index];
            let result = (|| {
                for condition in &mut config.conditions {
                    if !condition.evaluate(resources)? {
                        return Ok(false);
                    }
                }
                config.processor.run(resources)?;
                Ok(true)
            })();
            match result {
                Ok(true) => resources.apply_pending(),
                Ok(false) => resources.discard_pending(),
                Err(error) => {
                    resources.discard_pending();
                    return Err(error);
                }
            }
        }
        Ok(())
    }

    fn prepare(&self) -> io::Result<Vec<usize>> {
        let count = self.processors.len();
        let mut edges = HashSet::new();
        for (index, config) in self.processors.iter().enumerate() {
            for &predecessor in &config.predecessors {
                edges.insert((predecessor, index));
            }
            for (targets, before) in [(&config.before, true), (&config.after, false)] {
                for target in targets {
                    let mut found = false;
                    for (other, candidate) in self.processors.iter().enumerate() {
                        if candidate.processor.processor_type() == *target {
                            edges.insert(if before {
                                (index, other)
                            } else {
                                (other, index)
                            });
                            found = true;
                        }
                    }
                    if !found {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "stage dependency target is not registered",
                        ));
                    }
                }
            }
        }
        let mut remaining = vec![0; count];
        let mut dependents = vec![Vec::new(); count];
        for (from, to) in edges {
            remaining[to] += 1;
            dependents[from].push(to);
        }
        let mut order = Vec::with_capacity(count);
        let mut completed = vec![false; count];
        while order.len() < count {
            let Some(next) = (0..count).find(|&i| !completed[i] && remaining[i] == 0) else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "cycle in stage dependencies",
                ));
            };
            completed[next] = true;
            order.push(next);
            for &dependent in &dependents[next] {
                remaining[dependent] -= 1;
            }
        }
        Ok(order)
    }
}

fn append(target: &mut Vec<ProcessorConfig>, mut configs: Vec<ProcessorConfig>) {
    let offset = target.len();
    for config in &mut configs {
        for predecessor in &mut config.predecessors {
            *predecessor += offset;
        }
    }
    target.extend(configs);
}

pub trait IntoProcessors<Marker>: Sized {
    fn into_processors(self) -> Vec<ProcessorConfig>;
    fn chain(self) -> Chain<Self> {
        Chain(self)
    }
    fn before<T: 'static>(self, _: T) -> Ordered<Self> {
        Ordered {
            processors: self,
            before: vec![TypeId::of::<T>()],
            after: Vec::new(),
        }
    }
    fn after<T: 'static>(self, _: T) -> Ordered<Self> {
        Ordered {
            processors: self,
            before: Vec::new(),
            after: vec![TypeId::of::<T>()],
        }
    }
    fn run_if<M, C: IntoCondition<M>>(self, condition: C) -> Conditional<Self, C, M> {
        Conditional {
            processors: self,
            condition,
            marker: PhantomData,
        }
    }
}

pub struct Single<M>(PhantomData<M>);
pub struct Tuple<M>(PhantomData<M>);
pub struct Chained<M>(PhantomData<M>);
pub struct WithOrder<M>(PhantomData<M>);
pub struct WithCondition<P, C>(PhantomData<(P, C)>);
#[derive(Clone)]
pub struct Chain<T>(T);
#[derive(Clone)]
pub struct Ordered<T> {
    processors: T,
    before: Vec<TypeId>,
    after: Vec<TypeId>,
}
pub struct Conditional<P, C, M> {
    processors: P,
    condition: C,
    marker: PhantomData<fn() -> M>,
}
impl<P: Clone, C: Clone, M> Clone for Conditional<P, C, M> {
    fn clone(&self) -> Self {
        Self {
            processors: self.processors.clone(),
            condition: self.condition.clone(),
            marker: PhantomData,
        }
    }
}

impl<F, M> IntoProcessors<Single<M>> for F
where
    F: IntoProcessor<M>,
{
    fn into_processors(self) -> Vec<ProcessorConfig> {
        vec![ProcessorConfig {
            processor: Box::new(self.into_processor()),
            before: Vec::new(),
            after: Vec::new(),
            predecessors: Vec::new(),
            conditions: Vec::new(),
        }]
    }
}
impl<T, M> IntoProcessors<Chained<M>> for Chain<T>
where
    T: IntoProcessors<M>,
{
    fn into_processors(self) -> Vec<ProcessorConfig> {
        let mut configs = self.0.into_processors();
        for (index, config) in configs.iter_mut().enumerate().skip(1) {
            config.predecessors.push(index - 1);
        }
        configs
    }
}
impl<T, M> IntoProcessors<WithOrder<M>> for Ordered<T>
where
    T: IntoProcessors<M>,
{
    fn into_processors(self) -> Vec<ProcessorConfig> {
        let mut configs = self.processors.into_processors();
        for config in &mut configs {
            config.before.extend(&self.before);
            config.after.extend(&self.after);
        }
        configs
    }
}
impl<P, PM, C, CM> IntoProcessors<WithCondition<PM, CM>> for Conditional<P, C, CM>
where
    P: IntoProcessors<PM>,
    C: IntoCondition<CM> + Clone,
{
    fn into_processors(self) -> Vec<ProcessorConfig> {
        let mut configs = self.processors.into_processors();
        for config in &mut configs {
            config
                .conditions
                .push(Box::new(self.condition.clone().into_condition()));
        }
        configs
    }
}
macro_rules! processor_tuple {
    ($($item:ident : $marker:ident : $index:tt),+) => {
        impl<$($item, $marker),+> IntoProcessors<Tuple<($($marker,)+)>> for ($($item,)+)
        where $($item: IntoProcessors<$marker>),+
        {
            fn into_processors(self) -> Vec<ProcessorConfig> {
                let mut processors = Vec::new();
                $(append(&mut processors, self.$index.into_processors());)+
                processors
            }
        }
    };
}
processor_tuple!(A: MA: 0, B: MB: 1);
processor_tuple!(A: MA: 0, B: MB: 1, C: MC: 2);
processor_tuple!(A: MA: 0, B: MB: 1, C: MC: 2, D: MD: 3);

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
