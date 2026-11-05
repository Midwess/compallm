mod executor;

use crate::{
    access::Access,
    processor::{Condition, IntoCondition, IntoProcessor, Processor},
    resources::Resources,
};
use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    io,
    marker::PhantomData,
    num::NonZeroUsize,
    sync::Arc,
};

/// A stage is selected by its label type; registration never runs it automatically.
pub trait StageLabel: Send + Sync + 'static {}

pub struct ProcessorConfig {
    processor: Box<dyn Processor>,
    access_override: Option<Access>,
    before: Vec<TypeId>,
    after: Vec<TypeId>,
    predecessors: Vec<usize>,
    conditions: Vec<Box<dyn Condition>>,
}

/// A cached dependency plan with per-execution state. Async cancellation invalidates it.
#[derive(Default)]
pub struct Stage {
    processors: Vec<Option<ProcessorConfig>>,
    plan: Option<Plan>,
    interrupted: bool,
}
struct Plan {
    order: Vec<usize>,
    predecessors: Vec<Vec<usize>>,
}
impl ProcessorConfig {
    fn access(&self) -> Access {
        let mut access = self
            .access_override
            .clone()
            .unwrap_or_else(|| self.processor.access());
        for condition in &self.conditions {
            access.merge(condition.access());
        }
        access
    }
    fn allowed(&mut self, resources: &Resources) -> io::Result<bool> {
        for condition in &mut self.conditions {
            if !condition.evaluate(resources)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
impl Stage {
    pub fn add_processors<M>(&mut self, processors: impl IntoProcessors<M>) {
        let mut configs = processors.into_processors();
        for config in &mut configs {
            for predecessor in &mut config.predecessors {
                *predecessor += self.processors.len();
            }
        }
        self.processors.extend(configs.into_iter().map(Some));
        self.plan = None;
    }
    fn initialize(&mut self) -> io::Result<()> {
        if self.interrupted {
            return Err(io::Error::other(
                "stage was cancelled or panicked; drop its execution",
            ));
        }
        if self.plan.is_none() {
            self.plan = Some(self.prepare()?);
        }
        Ok(())
    }
    pub fn run(&mut self, resources: &mut Resources) -> io::Result<()> {
        self.initialize()?;
        if self
            .processors
            .iter()
            .flatten()
            .any(|c| c.processor.is_async())
        {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "async stage requires run_stage_async",
            ));
        }
        for &index in &self.plan.as_ref().expect("prepared stage").order {
            let config = self.processors[index]
                .as_mut()
                .expect("available processor");
            let result = (|| {
                if !config.allowed(resources)? {
                    return Ok(false);
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

    fn prepare(&self) -> io::Result<Plan> {
        let count = self.processors.len();
        let mut edges = HashSet::new();
        for (index, config) in self.processors.iter().enumerate() {
            let config = config.as_ref().expect("available processor");
            for &predecessor in &config.predecessors {
                edges.insert((predecessor, index));
            }
            for (targets, before) in [(&config.before, true), (&config.after, false)] {
                for target in targets {
                    let mut found = false;
                    for (other, candidate) in self.processors.iter().enumerate() {
                        if candidate
                            .as_ref()
                            .expect("available processor")
                            .processor
                            .processor_type()
                            == *target
                        {
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
        let mut predecessors = vec![Vec::new(); count];
        for (from, to) in edges {
            remaining[to] += 1;
            dependents[from].push(to);
            predecessors[to].push(from);
        }
        let mut order = Vec::with_capacity(count);
        let mut done = vec![false; count];
        while order.len() < count {
            let Some(next) = (0..count).find(|&i| !done[i] && remaining[i] == 0) else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "cycle in stage dependencies",
                ));
            };
            done[next] = true;
            order.push(next);
            for &dependent in &dependents[next] {
                remaining[dependent] -= 1;
            }
        }
        Ok(Plan {
            order,
            predecessors,
        })
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
    fn with_access(self, access: Access) -> AccessConfigured<Self> {
        AccessConfigured {
            processors: self,
            access,
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
pub struct WithAccess<M>(PhantomData<M>);
#[derive(Clone)]
pub struct AccessConfigured<T> {
    processors: T,
    access: Access,
}
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
            access_override: None,
            before: Vec::new(),
            after: Vec::new(),
            predecessors: Vec::new(),
            conditions: Vec::new(),
        }]
    }
}
impl<T, M> IntoProcessors<WithAccess<M>> for AccessConfigured<T>
where
    T: IntoProcessors<M>,
{
    fn into_processors(self) -> Vec<ProcessorConfig> {
        let mut configs = self.processors.into_processors();
        for config in &mut configs {
            config.access_override = Some(self.access.clone());
        }
        configs
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
