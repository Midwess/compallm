use router::processor::IntoProcessor;
use router::{prelude::*, processor::Processor};
use std::io;

fn stage_with<M>(processor: impl IntoProcessors<M>) -> Stage {
    let mut stage = Stage::default();
    stage.add_processors(processor);
    stage
}

#[test]
fn conflicting_parameters_panic_without_running_the_function() {
    let mut resources = Resources::default();
    resources.insert(5_u32);
    let called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let observed = called.clone();
    let mut stage = stage_with(move |_: Res<u32>, _: ResMut<u32>| {
        observed.store(true, std::sync::atomic::Ordering::SeqCst);
    });
    let panic =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| stage.run(&mut resources)));
    assert!(panic.is_err());
    assert!(!called.load(std::sync::atomic::Ordering::SeqCst));
    // Failed parameter fetching must release any earlier borrow.
    assert_eq!(*resources.get_mut::<u32>().unwrap(), 5);
}

#[test]
fn missing_resource_returns_error_before_execution() {
    let mut resources = Resources::default();
    let mut stage = stage_with(|_: Res<u32>| -> () { panic!("missing parameter") });
    assert_eq!(
        stage.run(&mut resources).unwrap_err().kind(),
        io::ErrorKind::NotFound
    );
}

#[test]
fn local_state_persists_per_processor_and_typed_writes_reach_later_processors() {
    let mut resources = Resources::default();
    resources.insert(0_u32);
    let mut stage = Stage::default();
    stage.add_processors(|mut count: Local<u32>, mut total: ResMut<u32>| {
        *count += 1;
        *total = *count;
    });
    stage.run(&mut resources).unwrap();
    assert_eq!(*resources.get::<u32>().unwrap(), 1);
    stage.run(&mut resources).unwrap();
    assert_eq!(*resources.get::<u32>().unwrap(), 2);
}

#[test]
fn function_conversion_can_execute_without_a_stage() {
    let mut resources = Resources::default();
    resources.insert(String::from("header"));
    let mut processor = (|mut value: ResMut<String>| value.push_str(" received")).into_processor();
    processor.run(&resources).unwrap();
    assert_eq!(&*resources.get::<String>().unwrap(), "header received");
}

#[test]
fn writer_insert_after_processor_borrows_end_and_are_visible_to_next_processor() {
    let mut resources = Resources::default();
    resources.insert(1_u32);
    let mut stage = Stage::default();
    stage.add_processors(|value: Res<u32>, mut writer: ContextWriter| {
        writer.insert_resource(*value + 1);
        assert_eq!(*value, 1);
    });
    stage.add_processors(|value: Res<u32>| assert_eq!(*value, 2));
    stage.run(&mut resources).unwrap();
    assert_eq!(*resources.get::<u32>().unwrap(), 2);
}

#[test]
fn writer_from_failed_processor_are_discarded() {
    let mut resources = Resources::default();
    let mut stage = Stage::default();
    stage.add_processors(|mut writer: ContextWriter| -> io::Result<()> {
        writer.insert_resource(42_u32);
        Err(io::Error::new(
            io::ErrorKind::NotConnected,
            "no available provider",
        ))
    });
    stage.add_processors(|| -> () { panic!("must not run after selection error") });
    assert_eq!(
        stage.run(&mut resources).unwrap_err().kind(),
        io::ErrorKind::NotConnected
    );
    assert_eq!(
        resources.get::<u32>().err().unwrap().kind(),
        io::ErrorKind::NotFound
    );
    // No queued insert should leak into a subsequent successful stage.
    Stage::default().run(&mut resources).unwrap();
    assert!(resources.get::<u32>().is_err());
}

#[test]
fn struct_processor_and_function_share_resources_and_preserve_instance_state() {
    struct Counter {
        runs: u32,
    }
    impl Processor for Counter {
        fn run(&mut self, resources: &Resources) -> io::Result<()> {
            self.runs += 1;
            *resources.get_mut::<u32>()? = self.runs;
            Ok(())
        }
    }
    let mut stage = Stage::default();
    stage.add_processors(
        (Counter { runs: 0 }, |mut value: ResMut<u32>| {
            *value *= 10;
        })
            .chain(),
    );
    for expected in [10, 20] {
        let mut resources = Resources::default();
        resources.insert(0_u32);
        stage.run(&mut resources).unwrap();
        assert_eq!(*resources.get::<u32>().unwrap(), expected);
    }
}
