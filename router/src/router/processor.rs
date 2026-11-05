use crate::resources::{ContextWriter, Local, Res, ResMut, Resources};
use std::{io, marker::PhantomData};

/// Fetch a typed argument and keep any per-processor state between runs.
pub trait ProcessorParam: 'static {
    type State: Default + Send + 'static;
    type Item<'a>;
    fn fetch<'a>(
        resources: &'a Resources,
        state: &'a mut Self::State,
    ) -> io::Result<Self::Item<'a>>;
}

impl ProcessorParam for ContextWriter<'static> {
    type State = ();
    type Item<'a> = ContextWriter<'a>;
    fn fetch<'a>(resources: &'a Resources, _: &'a mut ()) -> io::Result<Self::Item<'a>> {
        Ok(ContextWriter(resources))
    }
}

impl<T: Send + Sync + 'static> ProcessorParam for Res<'static, T> {
    type State = ();
    type Item<'a> = Res<'a, T>;
    fn fetch<'a>(resources: &'a Resources, _: &'a mut ()) -> io::Result<Self::Item<'a>> {
        resources.get()
    }
}
impl<T: Send + Sync + 'static> ProcessorParam for ResMut<'static, T> {
    type State = ();
    type Item<'a> = ResMut<'a, T>;
    fn fetch<'a>(resources: &'a Resources, _: &'a mut ()) -> io::Result<Self::Item<'a>> {
        resources.get_mut()
    }
}
impl<T: Default + Send + 'static> ProcessorParam for Local<'static, T> {
    type State = T;
    type Item<'a> = Local<'a, T>;
    fn fetch<'a>(_: &'a Resources, state: &'a mut T) -> io::Result<Self::Item<'a>> {
        Ok(Local(state))
    }
}

pub trait Processor: Send {
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
    fn into_result(self) -> io::Result<()> {
        Ok(())
    }
}
impl ProcessorOutput for io::Result<()> {
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
    F: FnMut() -> O + Send,
    O: ProcessorOutput,
{
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
            F: FnMut($($param),+) -> O + Send,
            F: for<'a> FnMut($($param::Item<'a>),+) -> O,
            O: ProcessorOutput,
        {
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
