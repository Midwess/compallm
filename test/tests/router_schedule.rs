use router::system::IntoSystem;
use router::{prelude::*, system::System};
use std::io;

fn schedule_with<M>(system: impl IntoSystems<M>) -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems(system);
    schedule
}

#[test]
fn conflicting_parameters_return_error_without_running_the_function() {
    let mut resources = Resources::default();
    resources.insert(5_u32);
    let mut schedule = schedule_with(|_: Res<u32>, _: ResMut<u32>| -> () {
        panic!("conflicting parameters must not be passed to the system")
    });
    assert_eq!(
        schedule.run(&resources).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    // Failed parameter fetching must release any earlier borrow.
    assert_eq!(*resources.get_mut::<u32>().unwrap(), 5);
}

#[test]
fn missing_resource_returns_error_before_execution() {
    let resources = Resources::default();
    let mut schedule = schedule_with(|_: Res<u32>| -> () { panic!("missing parameter") });
    assert_eq!(
        schedule.run(&resources).unwrap_err().kind(),
        io::ErrorKind::NotFound
    );
}

#[test]
fn local_state_persists_per_system_and_typed_writes_reach_later_systems() {
    let mut resources = Resources::default();
    resources.insert(0_u32);
    let mut schedule = Schedule::default();
    schedule.add_systems(|mut count: Local<u32>, mut total: ResMut<u32>| {
        *count += 1;
        *total = *count;
    });
    schedule.run(&resources).unwrap();
    assert_eq!(*resources.get::<u32>().unwrap(), 1);
    schedule.run(&resources).unwrap();
    assert_eq!(*resources.get::<u32>().unwrap(), 2);
}

#[test]
fn function_conversion_can_execute_without_a_schedule() {
    let mut resources = Resources::default();
    resources.insert(String::from("header"));
    let mut system = (|mut value: ResMut<String>| value.push_str(" received")).into_system();
    system.run(&resources).unwrap();
    assert_eq!(&*resources.get::<String>().unwrap(), "header received");
}
