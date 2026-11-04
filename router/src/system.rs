use crate::resources::{Local, Res, ResMut, Resources};
use std::{io, marker::PhantomData};

/// Fetch a typed argument and keep any per-system state between runs.
pub trait SystemParam: 'static {
    type State: Default + Send + 'static;
    type Item<'a>;
    fn fetch<'a>(
        resources: &'a Resources,
        state: &'a mut Self::State,
    ) -> io::Result<Self::Item<'a>>;
}

impl<T: Send + Sync + 'static> SystemParam for Res<'static, T> {
    type State = ();
    type Item<'a> = Res<'a, T>;
    fn fetch<'a>(resources: &'a Resources, _: &'a mut ()) -> io::Result<Self::Item<'a>> {
        resources.get()
    }
}
impl<T: Send + Sync + 'static> SystemParam for ResMut<'static, T> {
    type State = ();
    type Item<'a> = ResMut<'a, T>;
    fn fetch<'a>(resources: &'a Resources, _: &'a mut ()) -> io::Result<Self::Item<'a>> {
        resources.get_mut()
    }
}
impl<T: Default + Send + 'static> SystemParam for Local<'static, T> {
    type State = T;
    type Item<'a> = Local<'a, T>;
    fn fetch<'a>(_: &'a Resources, state: &'a mut T) -> io::Result<Self::Item<'a>> {
        Ok(Local(state))
    }
}

pub trait System: Send {
    fn run(&mut self, resources: &Resources) -> io::Result<()>;
}

pub trait IntoSystem<Marker> {
    type System: System + 'static;
    fn into_system(self) -> Self::System;
}

pub trait SystemOutput {
    fn into_result(self) -> io::Result<()>;
}
impl SystemOutput for () {
    fn into_result(self) -> io::Result<()> {
        Ok(())
    }
}
impl SystemOutput for io::Result<()> {
    fn into_result(self) -> io::Result<()> {
        self
    }
}

/// A function together with its typed parameter state.
pub struct FunctionSystem<F, State, Marker> {
    function: F,
    state: State,
    marker: PhantomData<fn() -> Marker>,
}

impl<F, O> IntoSystem<fn() -> O> for F
where
    F: FnMut() -> O + Send + 'static,
    O: SystemOutput + 'static,
{
    type System = FunctionSystem<F, (), fn() -> O>;
    fn into_system(self) -> Self::System {
        FunctionSystem {
            function: self,
            state: (),
            marker: PhantomData,
        }
    }
}
impl<F, O> System for FunctionSystem<F, (), fn() -> O>
where
    F: FnMut() -> O + Send,
    O: SystemOutput,
{
    fn run(&mut self, _: &Resources) -> io::Result<()> {
        (self.function)().into_result()
    }
}

macro_rules! function_system {
    ($($param:ident : $index:tt),+) => {
        impl<F, O, $($param: SystemParam),+> IntoSystem<fn($($param),+) -> O> for F
        where
            F: FnMut($($param),+) -> O + Send + 'static,
            F: for<'a> FnMut($($param::Item<'a>),+) -> O,
            O: SystemOutput + 'static,
        {
            type System = FunctionSystem<F, ($($param::State,)+), fn($($param),+) -> O>;
            fn into_system(self) -> Self::System {
                FunctionSystem { function: self, state: Default::default(), marker: PhantomData }
            }
        }
        impl<F, O, $($param: SystemParam),+> System
            for FunctionSystem<F, ($($param::State,)+), fn($($param),+) -> O>
        where
            F: FnMut($($param),+) -> O + Send,
            F: for<'a> FnMut($($param::Item<'a>),+) -> O,
            O: SystemOutput,
        {
            fn run(&mut self, resources: &Resources) -> io::Result<()> {
                (self.function)($($param::fetch(resources, &mut self.state.$index)?),+).into_result()
            }
        }
    };
}
function_system!(P0: 0);
function_system!(P0: 0, P1: 1);
function_system!(P0: 0, P1: 1, P2: 2);
function_system!(P0: 0, P1: 1, P2: 2, P3: 3);
