use sec::prelude::*;
use std::{
    io,
    num::NonZeroUsize,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{sync::Barrier, time::timeout};

struct Received;
impl StageLabel for Received {}
#[derive(Clone)]
struct Input(String);
struct Output(String);
struct Calls(AtomicUsize);

#[tokio::test]
async fn async_owned_inputs_and_deferred_output_feed_dependent_processors() {
    async fn map(input: Owned<Input>, mut writer: ContextWriter) -> io::Result<()> {
        tokio::task::yield_now().await;
        writer.insert_resource(Output(input.0.0.to_uppercase()));
        Ok(())
    }
    let mut app = App::new();
    app.add_processors(
        Received,
        (|output: Res<Output>| assert_eq!(output.0, "HELLO")).after(map),
    );
    // Function calls need parentheses for configuration syntax.
    app.configure_stage(Received, |stage| {
        stage.add_processors(async_processor(map));
    });
    let mut execution = app.execution();
    execution.resources_mut().insert(Input("hello".into()));
    assert_eq!(
        execution.run_stage(Received).unwrap_err().kind(),
        io::ErrorKind::Unsupported
    );
    execution.run_stage_async(Received).await.unwrap();
    assert_eq!(execution.resources().get::<Input>().unwrap().0, "hello");
    assert_eq!(execution.resources().get::<Output>().unwrap().0, "HELLO");
}

#[tokio::test]
async fn independent_async_processors_overlap_with_bounded_parallelism() {
    let barrier = Arc::new(Barrier::new(2));
    let mut app = App::new();
    app.set_max_parallelism(NonZeroUsize::new(2).unwrap());
    app.insert_shared_resource(barrier);
    app.insert_shared_resource(Arc::new(Calls(AtomicUsize::new(0))));
    for _ in 0..2 {
        app.add_processors(
            Received,
            async_processor(
                |barrier: Shared<Barrier>, calls: Shared<Calls>| async move {
                    barrier.wait().await;
                    calls.0.fetch_add(1, Ordering::SeqCst);
                },
            ),
        );
    }
    let mut execution = app.execution();
    timeout(Duration::from_secs(3), execution.run_stage_async(Received))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        execution
            .resources()
            .get_shared::<Calls>()
            .unwrap()
            .0
            .load(Ordering::SeqCst),
        2
    );
}

struct CpuProbe {
    active: Arc<AtomicUsize>,
    peak: Arc<AtomicUsize>,
    threads: Arc<Mutex<Vec<std::thread::ThreadId>>>,
}
impl Processor for CpuProbe {
    fn access(&self) -> Access {
        Access::default().read::<u32>()
    }
    fn run(&mut self, resources: &Resources) -> io::Result<()> {
        let value = resources.get::<u32>()?;
        assert_eq!(*value, 1);
        self.threads
            .lock()
            .unwrap()
            .push(std::thread::current().id());
        let n = self.active.fetch_add(1, Ordering::SeqCst) + 1;
        self.peak.fetch_max(n, Ordering::SeqCst);
        let deadline = std::time::Instant::now() + Duration::from_secs(2);
        while self.peak.load(Ordering::SeqCst) < 2 && std::time::Instant::now() < deadline {
            std::thread::yield_now();
        }
        self.active.fetch_sub(1, Ordering::SeqCst);
        Ok(())
    }
}
#[tokio::test]
async fn compatible_synchronous_processors_use_parallel_blocking_workers() {
    let active = Arc::new(AtomicUsize::new(0));
    let peak = Arc::new(AtomicUsize::new(0));
    let threads = Arc::new(Mutex::new(Vec::new()));
    let mut app = App::new();
    app.set_max_parallelism(NonZeroUsize::new(2).unwrap());
    app.configure_stage(Received, {
        let active = active.clone();
        let peak = peak.clone();
        let threads = threads.clone();
        move |stage| {
            for _ in 0..2 {
                stage.add_processors(CpuProbe {
                    active: active.clone(),
                    peak: peak.clone(),
                    threads: threads.clone(),
                });
            }
        }
    });
    let mut execution = app.execution();
    execution.resources_mut().insert(1_u32);
    execution.run_stage_async(Received).await.unwrap();
    assert_eq!(peak.load(Ordering::SeqCst), 2);
    let threads = threads.lock().unwrap();
    assert_ne!(threads[0], threads[1]);
}

#[tokio::test]
async fn conflicting_writes_are_serialized_and_state_is_reused() {
    let mut app = App::new();
    app.set_max_parallelism(NonZeroUsize::new(4).unwrap());
    app.add_processors(Received, |mut value: ResMut<u32>, mut local: Local<u32>| {
        *local += 1;
        *value += *local;
    });
    app.add_processors(Received, |mut value: ResMut<u32>| {
        *value *= 10;
    });
    let mut execution = app.execution();
    execution.resources_mut().insert(0_u32);
    execution.run_stage_async(Received).await.unwrap();
    assert_eq!(*execution.resources().get::<u32>().unwrap(), 10);
    execution.run_stage_async(Received).await.unwrap();
    assert_eq!(*execution.resources().get::<u32>().unwrap(), 120);
}

#[tokio::test]
async fn failure_stops_dependents_and_keeps_successful_sibling_writes() {
    async fn fail(mut writer: ContextWriter) -> io::Result<()> {
        writer.insert_resource(10_u32);
        Err(io::Error::new(io::ErrorKind::PermissionDenied, "denied"))
    }
    async fn succeed(mut writer: ContextWriter) {
        writer.insert_resource(20_u64);
    }
    let mut app = App::new();
    app.set_max_parallelism(NonZeroUsize::new(2).unwrap());
    app.add_processors(Received, async_processor(fail));
    app.add_processors(Received, async_processor(succeed));
    app.add_processors(
        Received,
        (|| -> () { panic!("dependent must not start") }).after(fail),
    );
    let mut execution = app.execution();
    assert_eq!(
        execution
            .run_stage_async(Received)
            .await
            .unwrap_err()
            .kind(),
        io::ErrorKind::PermissionDenied
    );
    assert!(execution.resources().get::<u32>().is_err());
    assert_eq!(*execution.resources().get::<u64>().unwrap(), 20);
}

#[tokio::test]
async fn conditions_contribute_resource_access_and_skip_async_work() {
    let mut app = App::new();
    app.add_processors(
        Received,
        async_processor(|| async {
            assert!(std::hint::black_box(false), "condition false");
        })
        .run_if(|value: Res<u32>| *value == 1),
    );
    let mut execution = app.execution();
    execution.resources_mut().insert(0_u32);
    execution.run_stage_async(Received).await.unwrap();
}

#[tokio::test]
async fn cancelling_async_stage_invalidates_it_and_discards_pending_insertions() {
    let mut app = App::new();
    app.add_processors(
        Received,
        async_processor(|mut writer: ContextWriter| async move {
            writer.insert_resource(42_u32);
            std::future::pending::<()>().await;
        }),
    );
    let mut execution = app.execution();
    assert!(
        timeout(
            Duration::from_millis(30),
            execution.run_stage_async(Received)
        )
        .await
        .is_err()
    );
    assert!(
        execution
            .run_stage_async(Received)
            .await
            .unwrap_err()
            .to_string()
            .contains("cancelled")
    );
    assert!(execution.resources().get::<u32>().is_err());
}

#[tokio::test]
async fn parallelism_limit_is_respected() {
    struct Monitor {
        active: AtomicUsize,
        peak: AtomicUsize,
    }
    let mut app = App::new();
    app.set_max_parallelism(NonZeroUsize::new(2).unwrap());
    let monitor = Arc::new(Monitor {
        active: AtomicUsize::new(0),
        peak: AtomicUsize::new(0),
    });
    app.insert_shared_resource(monitor.clone());
    for _ in 0..5 {
        app.add_processors(
            Received,
            async_processor(|monitor: Shared<Monitor>| async move {
                let active = monitor.active.fetch_add(1, Ordering::SeqCst) + 1;
                monitor.peak.fetch_max(active, Ordering::SeqCst);
                tokio::time::sleep(Duration::from_millis(10)).await;
                monitor.active.fetch_sub(1, Ordering::SeqCst);
            }),
        );
    }
    app.execution().run_stage_async(Received).await.unwrap();
    assert_eq!(monitor.peak.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn false_condition_and_exclusive_struct_are_safe_with_async_execution() {
    struct Custom;
    impl Processor for Custom {
        fn run(&mut self, resources: &Resources) -> io::Result<()> {
            *resources.get_mut::<u32>()? += 1;
            Ok(())
        }
    }
    let mut app = App::new();
    app.configure_stage(Received, |stage| stage.add_processors(Custom));
    app.add_processors(Received, |mut value: ResMut<u32>| {
        *value += 10;
    });
    let mut execution = app.execution();
    execution.resources_mut().insert(0_u32);
    execution.run_stage_async(Received).await.unwrap();
    assert_eq!(*execution.resources().get::<u32>().unwrap(), 11);
}

#[tokio::test]
async fn deferred_insertion_cannot_replace_a_resource_while_another_task_reads_it() {
    let mut app = App::new();
    app.set_max_parallelism(NonZeroUsize::new(2).unwrap());
    let active = Arc::new(AtomicUsize::new(0));
    let observed = active.clone();
    app.add_processors(Received, move |value: Res<u32>| {
        observed.store(1, Ordering::SeqCst);
        std::thread::sleep(Duration::from_millis(20));
        assert_eq!(*value, 1);
        observed.store(2, Ordering::SeqCst);
    });
    app.add_processors(Received, |mut writer: ContextWriter| {
        writer.insert_resource(2_u32);
    });
    let mut execution = app.execution();
    execution.resources_mut().insert(1_u32);
    execution.run_stage_async(Received).await.unwrap();
    assert_eq!(active.load(Ordering::SeqCst), 2);
    assert_eq!(*execution.resources().get::<u32>().unwrap(), 2);
}

#[tokio::test]
async fn hecs_borrow_conflict_fails_worker_for_bad_custom_access_declarations() {
    struct Writer {
        phase: Arc<AtomicUsize>,
    }
    impl Processor for Writer {
        // Deliberately incomplete declaration: actual borrowing must still be safe.
        fn access(&self) -> Access {
            Access::default()
        }
        fn run(&mut self, resources: &Resources) -> io::Result<()> {
            let mut value = resources.get_mut::<u32>()?;
            self.phase.store(1, Ordering::SeqCst);
            let deadline = std::time::Instant::now() + Duration::from_secs(2);
            while self.phase.load(Ordering::SeqCst) != 2 && std::time::Instant::now() < deadline {
                std::thread::yield_now();
            }
            assert_eq!(self.phase.load(Ordering::SeqCst), 2);
            *value = 2;
            Ok(())
        }
    }
    struct Reader {
        phase: Arc<AtomicUsize>,
    }
    impl Processor for Reader {
        fn access(&self) -> Access {
            Access::default()
        }
        fn run(&mut self, resources: &Resources) -> io::Result<()> {
            let deadline = std::time::Instant::now() + Duration::from_secs(2);
            while self.phase.load(Ordering::SeqCst) != 1 && std::time::Instant::now() < deadline {
                std::thread::yield_now();
            }
            let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                resources.get::<u32>().map(|_| ())
            }));
            self.phase.store(2, Ordering::SeqCst);
            match panic {
                Ok(result) => result,
                Err(payload) => std::panic::resume_unwind(payload),
            }
        }
    }
    let phase = Arc::new(AtomicUsize::new(0));
    let mut app = App::new();
    app.set_max_parallelism(NonZeroUsize::new(2).unwrap());
    app.configure_stage(Received, move |stage| {
        stage.add_processors((
            Writer {
                phase: phase.clone(),
            },
            Reader {
                phase: phase.clone(),
            },
        ))
    });
    let mut execution = app.execution();
    execution.resources_mut().insert(1_u32);
    let error = execution.run_stage_async(Received).await.unwrap_err();
    assert!(error.to_string().contains("processor task failed"));
    assert!(
        execution
            .run_stage_async(Received)
            .await
            .unwrap_err()
            .to_string()
            .contains("cancelled")
    );
    assert_eq!(*execution.resources().get::<u32>().unwrap(), 2);
}

#[tokio::test]
async fn dependent_starts_on_completion_without_waiting_for_unrelated_work() {
    struct Gate {
        dependent_finished: tokio::sync::Notify,
        calls: AtomicUsize,
    }
    async fn first() {}
    async fn unrelated(gate: Shared<Gate>) {
        gate.dependent_finished.notified().await;
    }
    async fn dependent(gate: Shared<Gate>) {
        gate.calls.fetch_add(1, Ordering::SeqCst);
        gate.dependent_finished.notify_one();
    }
    let mut app = App::new();
    app.set_max_parallelism(NonZeroUsize::new(2).unwrap());
    app.insert_shared_resource(Arc::new(Gate {
        dependent_finished: tokio::sync::Notify::new(),
        calls: AtomicUsize::new(0),
    }));
    app.add_processors(
        Received,
        (
            async_processor(first),
            async_processor(unrelated),
            async_processor(dependent).after(first),
        ),
    );
    let mut execution = app.execution();
    for _ in 0..2 {
        timeout(Duration::from_secs(3), execution.run_stage_async(Received))
            .await
            .unwrap()
            .unwrap();
    }
    assert_eq!(
        execution
            .resources()
            .get_shared::<Gate>()
            .unwrap()
            .calls
            .load(Ordering::SeqCst),
        2
    );
}

#[tokio::test]
async fn access_metadata_is_cached_and_executor_sets_cover_more_than_one_word() {
    struct Counts {
        access: AtomicUsize,
        runs: AtomicUsize,
    }
    #[derive(Clone)]
    struct Cached {
        counts: Arc<Counts>,
    }
    impl Processor for Cached {
        fn access(&self) -> Access {
            self.counts.access.fetch_add(1, Ordering::SeqCst);
            Access::default()
        }
        fn run(&mut self, _: &Resources) -> io::Result<()> {
            self.counts.runs.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }
    let counts = Arc::new(Counts {
        access: AtomicUsize::new(0),
        runs: AtomicUsize::new(0),
    });
    let mut app = App::new();
    app.set_max_parallelism(NonZeroUsize::new(4).unwrap());
    for _ in 0..70 {
        app.add_processors(
            Received,
            Cached {
                counts: counts.clone(),
            },
        );
    }
    let mut execution = app.execution();
    assert_eq!(counts.access.load(Ordering::SeqCst), 0);
    execution.run_stage_async(Received).await.unwrap();
    assert_eq!(counts.access.load(Ordering::SeqCst), 70);
    execution.run_stage_async(Received).await.unwrap();
    assert_eq!(counts.access.load(Ordering::SeqCst), 70);
    assert_eq!(counts.runs.load(Ordering::SeqCst), 140);
}

#[tokio::test]
async fn deferred_barrier_waits_for_readers_then_releases_dependent_work() {
    struct Gate {
        reader_started: AtomicUsize,
        reader_finished: AtomicUsize,
    }
    struct Output(u32);
    async fn produce(gate: Shared<Gate>, mut writer: ContextWriter) {
        while gate.reader_started.load(Ordering::SeqCst) == 0 {
            tokio::task::yield_now().await;
        }
        writer.insert_resource(Output(42));
    }
    fn read(value: Res<u32>, gate: Shared<Gate>) {
        gate.reader_started.store(1, Ordering::SeqCst);
        std::thread::sleep(Duration::from_millis(20));
        assert_eq!(*value, 1);
        gate.reader_finished.store(1, Ordering::SeqCst);
    }
    fn consume(output: Res<Output>, gate: Shared<Gate>) {
        assert_eq!(output.0, 42);
        assert_eq!(gate.reader_finished.load(Ordering::SeqCst), 1);
    }
    let mut app = App::new();
    app.set_max_parallelism(NonZeroUsize::new(2).unwrap());
    app.insert_shared_resource(Arc::new(Gate {
        reader_started: AtomicUsize::new(0),
        reader_finished: AtomicUsize::new(0),
    }));
    app.add_processors(
        Received,
        (async_processor(produce), read, consume.after(produce)),
    );
    let mut execution = app.execution();
    execution.resources_mut().insert(1_u32);
    timeout(Duration::from_secs(3), execution.run_stage_async(Received))
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn cached_condition_access_waits_for_conflicting_writer() {
    struct Observed(AtomicUsize);
    fn update(mut value: ResMut<u32>) {
        std::thread::sleep(Duration::from_millis(10));
        *value = 2;
    }
    fn observe(observed: Shared<Observed>) {
        observed.0.fetch_add(1, Ordering::SeqCst);
    }
    let mut app = App::new();
    app.set_max_parallelism(NonZeroUsize::new(2).unwrap());
    app.insert_shared_resource(Arc::new(Observed(AtomicUsize::new(0))));
    app.add_processors(
        Received,
        (update, observe.run_if(|value: Res<u32>| *value == 2)),
    );
    let mut execution = app.execution();
    execution.resources_mut().insert(1_u32);
    execution.run_stage_async(Received).await.unwrap();
    assert_eq!(
        execution
            .resources()
            .get_shared::<Observed>()
            .unwrap()
            .0
            .load(Ordering::SeqCst),
        1
    );
}
