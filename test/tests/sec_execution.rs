use sec::prelude::*;
use std::{
    io,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

struct Received;
impl StageLabel for Received {}
struct NeverReached;
impl StageLabel for NeverReached {}
struct Input(String);
struct Output(String);
struct Settings {
    prefix: String,
}
struct Visits(usize);

fn validate(input: Res<Input>) -> io::Result<()> {
    if input.0.is_empty() {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "empty input"))
    } else {
        Ok(())
    }
}
fn map(input: Res<Input>, settings: Shared<Settings>, mut writer: ContextWriter) {
    writer.insert_resource(Output(format!("{}{}", settings.prefix, input.0)));
}
fn record(output: Res<Output>, mut visits: ResMut<Visits>) {
    assert!(output.0.starts_with("mapped:"));
    visits.0 += 1;
}

struct MappingPlugin;
impl Plugin for MappingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_shared_resource(Arc::new(Settings {
            prefix: "mapped:".into(),
        }));
        // Reverse registration demonstrates dependency ordering rather than insertion order.
        app.add_processors(Received, (record.after(map), map.after(validate), validate));
    }
}
struct CountingPlugin;
impl Plugin for CountingPlugin {
    fn build(&self, app: &mut App) {
        app.add_processors(
            Received,
            (|mut local: Local<usize>, mut visits: ResMut<Visits>| {
                *local += 1;
                visits.0 += *local;
            })
            .after(record),
        );
    }
}

#[test]
fn plugins_build_ordered_stages_and_execution_state_is_isolated() {
    let mut app = App::new();
    app.add_plugins((MappingPlugin, CountingPlugin));
    let mut first = app.execution();
    first.resources_mut().insert(Input("first".into()));
    first.resources_mut().insert(Visits(0));
    first.run_stage(Received).unwrap();
    assert_eq!(first.resources().get::<Output>().unwrap().0, "mapped:first");
    assert_eq!(first.resources().get::<Visits>().unwrap().0, 2);
    first.run_stage(Received).unwrap();
    assert_eq!(first.resources().get::<Visits>().unwrap().0, 5);
    let mut second = app.execution();
    second.resources_mut().insert(Input("second".into()));
    second.resources_mut().insert(Visits(0));
    second.run_stage(Received).unwrap();
    assert_eq!(
        second.resources().get::<Output>().unwrap().0,
        "mapped:second"
    );
    assert_eq!(second.resources().get::<Visits>().unwrap().0, 2);
}

#[test]
fn stage_factories_are_lazy_and_reused_for_manual_replay() {
    let built = Arc::new(AtomicUsize::new(0));
    let count = built.clone();
    let mut app = App::new();
    app.configure_stage(Received, move |stage| {
        count.fetch_add(1, Ordering::SeqCst);
        stage.add_processors(|mut visits: ResMut<Visits>| {
            visits.0 += 1;
        });
    });
    app.configure_stage(NeverReached, |_| panic!("unreached stage must not build"));
    let mut execution = app.execution();
    execution.resources_mut().insert(Visits(0));
    assert_eq!(built.load(Ordering::SeqCst), 0);
    execution.run_stage(Received).unwrap();
    execution.run_stage(Received).unwrap();
    assert_eq!(built.load(Ordering::SeqCst), 1);
    assert_eq!(execution.resources().get::<Visits>().unwrap().0, 2);
}

#[test]
fn shared_resources_are_shared_by_identity_and_snapshotted_at_creation() {
    struct Value(usize);
    let value = Arc::new(Value(1));
    let mut app = App::new();
    app.insert_shared_resource(value.clone());
    let first = app.execution();
    let second = app.execution();
    let a = first.resources().get_shared::<Value>().unwrap();
    let b = second.resources().get_shared::<Value>().unwrap();
    assert!(std::ptr::eq(&*a, &*b));
    assert!(std::ptr::eq(&*a, &*value));
    app.insert_shared_resource(Arc::new(Value(2)));
    assert_eq!(first.resources().get_shared::<Value>().unwrap().0, 1);
    assert_eq!(
        app.execution().resources().get_shared::<Value>().unwrap().0,
        2
    );
}

#[test]
fn conditions_skip_or_error_without_committing_deferred_writes() {
    struct Allowed(bool);
    let mut app = App::new();
    app.add_processors(
        Received,
        (|mut visits: ResMut<Visits>| {
            visits.0 += 1;
        })
        .run_if(
            |allowed: Res<Allowed>, mut local: Local<usize>| -> io::Result<bool> {
                *local += 1;
                if *local == 3 {
                    return Err(io::Error::new(io::ErrorKind::PermissionDenied, "expired"));
                }
                Ok(allowed.0)
            },
        ),
    );
    let mut execution = app.execution();
    execution.resources_mut().insert(Allowed(false));
    execution.resources_mut().insert(Visits(0));
    execution.run_stage(Received).unwrap();
    assert_eq!(execution.resources().get::<Visits>().unwrap().0, 0);
    execution.resources_mut().insert(Allowed(true));
    execution.run_stage(Received).unwrap();
    assert_eq!(execution.resources().get::<Visits>().unwrap().0, 1);
    assert_eq!(
        execution.run_stage(Received).unwrap_err().kind(),
        io::ErrorKind::PermissionDenied
    );
    assert_eq!(execution.resources().get::<Visits>().unwrap().0, 1);
}

#[test]
fn ordering_does_not_install_or_require_missing_optional_processors() {
    fn optional() {
        panic!("must not be installed by an ordering reference");
    }
    let mut app = App::new();
    app.add_processors(
        Received,
        (|mut visits: ResMut<Visits>| {
            visits.0 += 1;
        })
        .after(optional)
        .before(optional),
    );
    let mut execution = app.execution();
    execution.resources_mut().insert(Visits(0));
    execution.run_stage(Received).unwrap();
    assert_eq!(execution.resources().get::<Visits>().unwrap().0, 1);
}

#[test]
fn cycles_fail_before_any_processor_executes() {
    fn a() {}
    fn b() {}
    let called = Arc::new(AtomicUsize::new(0));
    let observed = called.clone();
    let mut app = App::new();
    app.add_processors(
        Received,
        (
            move || {
                observed.fetch_add(1, Ordering::SeqCst);
            },
            a.after(b),
            b.after(a),
        ),
    );
    assert_eq!(
        app.execution().run_stage(Received).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    assert_eq!(called.load(Ordering::SeqCst), 0);
}

#[test]
fn before_constraints_and_chains_keep_insertions_visible_to_dependents() {
    struct Value(usize);
    fn produce(mut writer: ContextWriter) {
        writer.insert_resource(Value(10));
    }
    fn increment(mut value: ResMut<Value>) {
        value.0 += 1;
    }
    fn check(value: Res<Value>) {
        assert_eq!(value.0, 11);
    }
    let mut app = App::new();
    app.add_processors(Received, (increment, check).chain());
    app.add_processors(Received, produce.before(increment));
    let mut execution = app.execution();
    execution.run_stage(Received).unwrap();
    assert_eq!(execution.resources().get::<Value>().unwrap().0, 11);
}

#[test]
fn condition_false_does_not_implicitly_skip_dependents() {
    let mut app = App::new();
    fn skipped() {
        panic!("false condition");
    }
    app.add_processors(Received, skipped.run_if(|| false));
    app.add_processors(
        Received,
        (|mut visits: ResMut<Visits>| {
            visits.0 += 1;
        })
        .after(skipped),
    );
    let mut execution = app.execution();
    execution.resources_mut().insert(Visits(0));
    execution.run_stage(Received).unwrap();
    assert_eq!(execution.resources().get::<Visits>().unwrap().0, 1);
}

#[test]
fn application_registration_changes_do_not_mutate_existing_executions() {
    let mut app = App::new();
    let mut existing = app.execution();
    app.add_processors(Received, |mut visits: ResMut<Visits>| {
        visits.0 += 1;
    });
    existing.resources_mut().insert(Visits(0));
    existing.run_stage(Received).unwrap();
    assert_eq!(existing.resources().get::<Visits>().unwrap().0, 0);
    let mut next = app.execution();
    next.resources_mut().insert(Visits(0));
    next.run_stage(Received).unwrap();
    assert_eq!(next.resources().get::<Visits>().unwrap().0, 1);
}

#[test]
fn nested_chain_indices_remain_correct_after_other_registered_processors() {
    struct Log(Vec<usize>);
    fn first(mut log: ResMut<Log>) {
        log.0.push(1);
    }
    fn second(mut log: ResMut<Log>) {
        log.0.push(2);
    }
    fn third(mut log: ResMut<Log>) {
        log.0.push(3);
    }
    let mut app = App::new();
    app.add_processors(Received, |mut log: ResMut<Log>| {
        log.0.push(0);
    });
    app.add_processors(Received, (first, (second, third).chain()).chain());
    let mut execution = app.execution();
    execution.resources_mut().insert(Log(Vec::new()));
    execution.run_stage(Received).unwrap();
    assert_eq!(
        execution.resources().get::<Log>().unwrap().0,
        vec![0, 1, 2, 3]
    );
}

#[test]
fn skipped_condition_discards_queued_insertions() {
    let mut app = App::new();
    app.add_processors(
        Received,
        (|| -> () {
            panic!("skipped");
        })
        .run_if(|mut writer: ContextWriter| {
            writer.insert_resource(42_u32);
            false
        }),
    );
    let mut execution = app.execution();
    execution.run_stage(Received).unwrap();
    assert!(execution.resources().get::<u32>().is_err());
}

#[test]
fn conflicting_condition_parameters_fail_before_processor_effects() {
    let mut app = App::new();
    app.add_processors(
        Received,
        (|| -> () {
            panic!("invalid parameters");
        })
        .run_if(|_: Res<u32>, _: ResMut<u32>| true),
    );
    let mut execution = app.execution();
    execution.resources_mut().insert(1_u32);
    assert_eq!(
        execution.run_stage(Received).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    assert_eq!(*execution.resources().get::<u32>().unwrap(), 1);
}

#[test]
fn adding_processors_rebuilds_the_plan_without_resetting_existing_local_state() {
    let mut stage = Stage::default();
    let mut resources = Resources::default();
    resources.insert(0_u32);
    fn count(mut local: Local<u32>, mut value: ResMut<u32>) {
        *local += 1;
        *value += *local;
    }
    stage.add_processors(count);
    stage.run(&mut resources).unwrap();
    stage.add_processors(
        (|mut value: ResMut<u32>| {
            *value *= 10;
        })
        .after(count),
    );
    stage.run(&mut resources).unwrap();
    assert_eq!(*resources.get::<u32>().unwrap(), 30);
}
