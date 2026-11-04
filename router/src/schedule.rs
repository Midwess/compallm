use crate::{
    resources::Resources,
    system::{IntoSystem, System},
};
use std::{any::TypeId, collections::HashMap, io};

/// Schedule identity is its Rust type. Registration does not execute it.
pub trait ScheduleLabel: Send + Sync + 'static {}
pub struct HeaderReceived;
impl ScheduleLabel for HeaderReceived {}

/// Runs after HeaderReceived succeeds, before consuming any body bytes.
/// This transition also occurs for requests with an empty body.
pub struct BodyStarted;
impl ScheduleLabel for BodyStarted {}

/// Ordered systems with independent parameter and local state.
#[derive(Default)]
pub struct Schedule {
    systems: Vec<Box<dyn System>>,
}
impl Schedule {
    pub fn add_systems<M>(&mut self, systems: impl IntoSystems<M>) {
        self.systems.extend(systems.into_systems());
    }
    pub fn run(&mut self, resources: &mut Resources) -> io::Result<()> {
        for system in &mut self.systems {
            if let Err(error) = system.run(resources) {
                resources.discard_pending();
                return Err(error);
            }
            resources.apply_pending();
        }
        Ok(())
    }
}

pub trait IntoSystems<Marker>: Sized {
    fn into_systems(self) -> Vec<Box<dyn System>>;
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

impl<F, M> IntoSystems<Single<M>> for F
where
    F: IntoSystem<M>,
{
    fn into_systems(self) -> Vec<Box<dyn System>> {
        vec![Box::new(self.into_system())]
    }
}
impl<T, M> IntoSystems<Chained<M>> for Chain<T>
where
    T: IntoSystems<M>,
{
    fn into_systems(self) -> Vec<Box<dyn System>> {
        self.0.into_systems()
    }
}
macro_rules! system_tuple {
    ($($item:ident : $marker:ident : $index:tt),+) => {
        impl<$($item, $marker),+> IntoSystems<Tuple<($($marker,)+)>> for ($($item,)+)
        where $($item: IntoSystems<$marker>),+
        {
            fn into_systems(self) -> Vec<Box<dyn System>> {
                let mut systems = Vec::new();
                $(systems.extend(self.$index.into_systems());)+
                systems
            }
        }
    };
}
system_tuple!(A: MA: 0, B: MB: 1);
system_tuple!(A: MA: 0, B: MB: 1, C: MC: 2);
system_tuple!(A: MA: 0, B: MB: 1, C: MC: 2, D: MD: 3);

type Configure = dyn Fn(&mut Schedule) + Send + Sync;
#[derive(Default)]
pub(crate) struct ScheduleDefinitions {
    definitions: HashMap<TypeId, Vec<Box<Configure>>>,
}
impl ScheduleDefinitions {
    pub(crate) fn add<L: ScheduleLabel, M, S>(&mut self, label: L, systems: S)
    where
        S: IntoSystems<M> + Clone + Send + Sync + 'static,
    {
        self.configure(label, move |schedule| schedule.add_systems(systems.clone()));
    }
    pub(crate) fn configure<L: ScheduleLabel>(
        &mut self,
        _: L,
        configure: impl Fn(&mut Schedule) + Send + Sync + 'static,
    ) {
        self.definitions
            .entry(TypeId::of::<L>())
            .or_default()
            .push(Box::new(configure));
    }
    pub(crate) fn build(&self) -> Schedules {
        let mut schedules = HashMap::new();
        for (&label, configurations) in &self.definitions {
            let mut schedule = Schedule::default();
            for configure in configurations {
                configure(&mut schedule);
            }
            schedules.insert(label, schedule);
        }
        Schedules { schedules }
    }
}

pub(crate) struct Schedules {
    schedules: HashMap<TypeId, Schedule>,
}
impl Schedules {
    pub(crate) fn run<L: ScheduleLabel>(
        &mut self,
        _: L,
        resources: &mut Resources,
    ) -> io::Result<()> {
        if let Some(schedule) = self.schedules.get_mut(&TypeId::of::<L>()) {
            schedule.run(resources)?;
        }
        Ok(())
    }
}
