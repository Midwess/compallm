use super::ProcessorConfig;
use crate::processor::{IntoCondition, IntoProcessor};
use std::{any::TypeId, marker::PhantomData};

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
