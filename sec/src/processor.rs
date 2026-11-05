use crate::{
    access::{Access, validate_parameters},
    resources::{ContextWriter, Local, Res, ResMut, Resources, Shared},
};
use std::{any::TypeId, future::Future, io, marker::PhantomData, pin::Pin};

pub type ProcessorFuture<'a> = Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'a>>;

/// Fetch a typed argument and keep any per-processor state between runs.
pub trait ProcessorParam: 'static {
    fn access() -> Access {
        Access::exclusive()
    }
    type State: Default + Send + 'static;
    type Item<'a>;
    fn fetch<'a>(
        resources: &'a Resources,
        state: &'a mut Self::State,
    ) -> io::Result<Self::Item<'a>>;
}

impl<T: Send + Sync + 'static> ProcessorParam for Shared<T> {
    fn access() -> Access {
        Access::default()
    }
    type State = ();
    type Item<'a> = Shared<T>;
    #[inline]
    fn fetch<'a>(resources: &'a Resources, _: &'a mut ()) -> io::Result<Self::Item<'a>> {
        resources.get_shared()
    }
}

impl ProcessorParam for ContextWriter {
    type State = ();
    type Item<'a> = ContextWriter;
    #[inline]
    fn fetch<'a>(resources: &'a Resources, _: &'a mut ()) -> io::Result<Self::Item<'a>> {
        Ok(resources.writer())
    }
}

impl<T: Send + Sync + 'static> ProcessorParam for Res<'static, T> {
    fn access() -> Access {
        Access::default().read::<T>()
    }
    type State = ();
    type Item<'a> = Res<'a, T>;
    #[inline]
    fn fetch<'a>(resources: &'a Resources, _: &'a mut ()) -> io::Result<Self::Item<'a>> {
        resources.get()
    }
}
impl<T: Send + Sync + 'static> ProcessorParam for ResMut<'static, T> {
    fn access() -> Access {
        Access::default().write::<T>()
    }
    type State = ();
    type Item<'a> = ResMut<'a, T>;
    #[inline]
    fn fetch<'a>(resources: &'a Resources, _: &'a mut ()) -> io::Result<Self::Item<'a>> {
        resources.get_mut()
    }
}
impl<T: Default + Send + 'static> ProcessorParam for Local<'static, T> {
    fn access() -> Access {
        Access::default()
    }
    type State = T;
    type Item<'a> = Local<'a, T>;
    fn fetch<'a>(_: &'a Resources, state: &'a mut T) -> io::Result<Self::Item<'a>> {
        Ok(Local(state))
    }
}

pub trait Processor: Send + 'static {
    fn validate(&self) -> io::Result<()> {
        Ok(())
    }
    fn access(&self) -> Access {
        Access::exclusive()
    }
    fn is_async(&self) -> bool {
        false
    }
    fn run_async<'a>(&'a mut self, resources: &'a Resources) -> ProcessorFuture<'a> {
        Box::pin(async move { self.run(resources) })
    }
    fn processor_type(&self) -> TypeId {
        TypeId::of::<Self>()
    }
    fn run(&mut self, resources: &Resources) -> io::Result<()>;
}

pub trait IntoProcessor<Marker> {
    type Processor: Processor + 'static;
    fn into_processor(self) -> Self::Processor;
}

/// Struct processors are already executable; registration retains the instance.
impl<P: Processor + 'static> IntoProcessor<()> for P {
    type Processor = P;

    fn into_processor(self) -> Self::Processor {
        self
    }
}

pub trait ProcessorOutput {
    fn into_result(self) -> io::Result<()>;
}
impl ProcessorOutput for () {
    #[inline]
    fn into_result(self) -> io::Result<()> {
        Ok(())
    }
}
impl ProcessorOutput for io::Result<()> {
    #[inline]
    fn into_result(self) -> io::Result<()> {
        self
    }
}

/// A function together with its typed parameter state.
pub struct FunctionProcessor<F, State, Marker> {
    function: F,
    state: State,
    marker: PhantomData<fn() -> Marker>,
}

impl<F, O> IntoProcessor<fn() -> O> for F
where
    F: FnMut() -> O + Send + 'static,
    O: ProcessorOutput + 'static,
{
    type Processor = FunctionProcessor<F, (), fn() -> O>;
    fn into_processor(self) -> Self::Processor {
        FunctionProcessor {
            function: self,
            state: (),
            marker: PhantomData,
        }
    }
}
impl<F, O> Processor for FunctionProcessor<F, (), fn() -> O>
where
    F: FnMut() -> O + Send + 'static,
    O: ProcessorOutput + 'static,
{
    fn processor_type(&self) -> TypeId {
        TypeId::of::<F>()
    }
    fn access(&self) -> Access {
        Access::default()
    }
    #[inline]
    fn run(&mut self, _: &Resources) -> io::Result<()> {
        (self.function)().into_result()
    }
}

macro_rules! function_processor {
    ($($param:ident : $index:tt),+) => {
        impl<F, O, $($param: ProcessorParam),+> IntoProcessor<fn($($param),+) -> O> for F
        where
            F: FnMut($($param),+) -> O + Send + 'static,
            F: for<'a> FnMut($($param::Item<'a>),+) -> O,
            O: ProcessorOutput + 'static,
        {
            type Processor = FunctionProcessor<F, ($($param::State,)+), fn($($param),+) -> O>;
            fn into_processor(self) -> Self::Processor {
                FunctionProcessor { function: self, state: Default::default(), marker: PhantomData }
            }
        }
        impl<F, O, $($param: ProcessorParam),+> Processor
            for FunctionProcessor<F, ($($param::State,)+), fn($($param),+) -> O>
        where
            F: FnMut($($param),+) -> O + Send + 'static,
            F: for<'a> FnMut($($param::Item<'a>),+) -> O,
            O: ProcessorOutput + 'static,
        {
            fn validate(&self) -> io::Result<()> { validate_parameters(&[$($param::access()),+]) }
            fn access(&self) -> Access {
                let mut access = Access::default(); $(access.merge($param::access());)+ access
            }
            fn processor_type(&self) -> TypeId { TypeId::of::<F>() }
            #[inline]
            fn run(&mut self, resources: &Resources) -> io::Result<()> {
                (self.function)($($param::fetch(resources, &mut self.state.$index)?),+).into_result()
            }
        }
    };
}
function_processor!(P0: 0);
function_processor!(P0: 0, P1: 1);
function_processor!(P0: 0, P1: 1, P2: 2);
function_processor!(P0: 0, P1: 1, P2: 2, P3: 3);

pub trait Condition: Send {
    fn validate(&self) -> io::Result<()> {
        Ok(())
    }
    fn access(&self) -> Access {
        Access::exclusive()
    }
    fn evaluate(&mut self, resources: &Resources) -> io::Result<bool>;
}
pub trait IntoCondition<Marker> {
    type Condition: Condition + 'static;
    fn into_condition(self) -> Self::Condition;
}
pub trait ConditionOutput {
    fn into_result(self) -> io::Result<bool>;
}
impl ConditionOutput for bool {
    fn into_result(self) -> io::Result<bool> {
        Ok(self)
    }
}
impl ConditionOutput for io::Result<bool> {
    fn into_result(self) -> io::Result<bool> {
        self
    }
}
impl<F, O> IntoCondition<fn() -> O> for F
where
    F: FnMut() -> O + Send + 'static,
    O: ConditionOutput + 'static,
{
    type Condition = FunctionProcessor<F, (), fn() -> O>;
    fn into_condition(self) -> Self::Condition {
        FunctionProcessor {
            function: self,
            state: (),
            marker: PhantomData,
        }
    }
}
impl<F, O> Condition for FunctionProcessor<F, (), fn() -> O>
where
    F: FnMut() -> O + Send + 'static,
    O: ConditionOutput,
{
    fn access(&self) -> Access {
        Access::default()
    }
    fn evaluate(&mut self, _: &Resources) -> io::Result<bool> {
        (self.function)().into_result()
    }
}
macro_rules! function_condition {
    ($($param:ident : $index:tt),+) => {
        impl<F, O, $($param: ProcessorParam),+> IntoCondition<fn($($param),+) -> O> for F
        where F: FnMut($($param),+) -> O + Send + 'static,
              F: for<'a> FnMut($($param::Item<'a>),+) -> O, O: ConditionOutput + 'static,
        {
            type Condition = FunctionProcessor<F, ($($param::State,)+), fn($($param),+) -> O>;
            fn into_condition(self) -> Self::Condition {
                FunctionProcessor { function: self, state: Default::default(), marker: PhantomData }
            }
        }
        impl<F, O, $($param: ProcessorParam),+> Condition for FunctionProcessor<F, ($($param::State,)+), fn($($param),+) -> O>
        where F: FnMut($($param),+) -> O + Send + 'static,
              F: for<'a> FnMut($($param::Item<'a>),+) -> O, O: ConditionOutput,
        {
            fn validate(&self) -> io::Result<()> { validate_parameters(&[$($param::access()),+]) }
            fn access(&self) -> Access { let mut access = Access::default(); $(access.merge($param::access());)+ access }
            #[inline]
            fn evaluate(&mut self, resources: &Resources) -> io::Result<bool> {
                (self.function)($($param::fetch(resources, &mut self.state.$index)?),+).into_result()
            }
        }
    };
}
function_condition!(P0: 0);
function_condition!(P0: 0, P1: 1);
function_condition!(P0: 0, P1: 1, P2: 2);
function_condition!(P0: 0, P1: 1, P2: 2, P3: 3);
