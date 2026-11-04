use compallm_core::{
    model::Model,
    provider::{ModelLimits, Pricing, RequestPricing, ServiceKind},
};
use router::prelude::*;
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    time::timeout,
};

fn model() -> Model {
    Model {
        id: 1,
        organization_id: 10,
        name: "general-chat".into(),
        service_kind: ServiceKind::Llm,
        enabled: true,
        capabilities: vec![],
        limits: ModelLimits::default(),
        thinking: None,
        pricing: Pricing::Request(RequestPricing {
            usd_per_request: "0".parse().unwrap(),
        }),
    }
}

#[tokio::test]
async fn header_schedule_waits_for_complete_headers_and_runs_in_order() {
    timeout(Duration::from_secs(5), async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let mut client = TcpStream::connect(listener.local_addr().unwrap())
            .await
            .unwrap();
        let (stream, _) = listener.accept().await.unwrap();
        let called = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let first = called.clone();
        let second = called.clone();
        let mut router = Router::new(model());
        router.configure_schedule(HeaderReceived, move |schedule| {
            let first = first.clone();
            let second = second.clone();
            schedule.add_systems(
                (
                    move |mut header: ResMut<RequestHeader>, model: Res<RouterModel>| {
                        assert_eq!(model.0.id, 1);
                        assert_eq!(header.method, "POST");
                        assert_eq!(header.target, "/original");
                        assert_eq!(header.fields.len(), 2);
                        header.target = "/mapped".into();
                        first.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    },
                    move |header: Res<RequestHeader>| {
                        assert_eq!(second.load(std::sync::atomic::Ordering::SeqCst), 1);
                        assert_eq!(header.target, "/mapped");
                        second.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    },
                )
                    .chain(),
            );
        });
        router.on_connection(stream).unwrap();
        client
            .write_all(b"POST /original HTTP/1.1\r\nHost: localhost\r\nX-Test: yes\r\n")
            .await
            .unwrap();
        assert!(
            timeout(Duration::from_millis(50), router.join_next())
                .await
                .is_err()
        );
        assert_eq!(called.load(std::sync::atomic::Ordering::SeqCst), 0);
        // No body needs to arrive before the header schedule executes.
        client.write_all(b"\r\n").await.unwrap();
        router.join_next().await.unwrap().unwrap().unwrap();
        assert_eq!(called.load(std::sync::atomic::Ordering::SeqCst), 2);
        assert_eq!(router.pending_connections(), 0);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn invalid_or_oversized_headers_do_not_run_systems() {
    timeout(Duration::from_secs(5), async {
        let oversized = format!(
            "GET / HTTP/1.1\r\nX-Large: {}\r\n\r\n",
            "x".repeat(33 * 1024)
        );
        for payload in [b"INVALID\r\n\r\n".as_slice(), oversized.as_bytes()] {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let mut client = TcpStream::connect(listener.local_addr().unwrap())
                .await
                .unwrap();
            let (stream, _) = listener.accept().await.unwrap();
            let mut router = Router::new(model());
            router.add_systems(HeaderReceived, || -> () { panic!("must not execute") });
            router.on_connection(stream).unwrap();
            let _ = client.write_all(payload).await;
            let error = router.join_next().await.unwrap().unwrap().unwrap_err();
            assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
        }
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn dropping_router_aborts_active_connection() {
    timeout(Duration::from_secs(5), async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let mut client = TcpStream::connect(listener.local_addr().unwrap())
            .await
            .unwrap();
        let (stream, _) = listener.accept().await.unwrap();
        let mut router = Router::new(model());
        router.on_connection(stream).unwrap();
        drop(router);
        let mut byte = [0];
        assert_eq!(client.read(&mut byte).await.unwrap(), 0);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn system_local_state_is_isolated_between_connections() {
    timeout(Duration::from_secs(5), async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let called = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let observed = called.clone();
        let mut router = Router::new(model());
        router.add_systems(HeaderReceived, move |mut runs: Local<usize>| {
            assert_eq!(*runs, 0);
            *runs += 1;
            observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });
        for _ in 0..2 {
            let mut client = TcpStream::connect(listener.local_addr().unwrap())
                .await
                .unwrap();
            let (stream, _) = listener.accept().await.unwrap();
            router.on_connection(stream).unwrap();
            client
                .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
                .await
                .unwrap();
            router.join_next().await.unwrap().unwrap().unwrap();
        }
        assert_eq!(called.load(std::sync::atomic::Ordering::SeqCst), 2);
    })
    .await
    .unwrap();
}

struct FutureMoment;
impl ScheduleLabel for FutureMoment {}

#[tokio::test]
async fn custom_schedule_registration_does_not_run_at_header_received() {
    timeout(Duration::from_secs(5), async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let mut client = TcpStream::connect(listener.local_addr().unwrap())
            .await
            .unwrap();
        let (stream, _) = listener.accept().await.unwrap();
        let mut router = Router::new(model());
        router.add_systems(FutureMoment, || -> () {
            panic!("untriggered schedule must not run")
        });
        router.configure_schedule(FutureMoment, |schedule| {
            schedule
                .add_systems(|| -> () { panic!("untriggered configured schedule must not run") });
        });
        router.on_connection(stream).unwrap();
        client
            .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .await
            .unwrap();
        router.join_next().await.unwrap().unwrap().unwrap();
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn header_system_failure_stops_the_schedule_and_returns_to_caller() {
    timeout(Duration::from_secs(5), async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let mut client = TcpStream::connect(listener.local_addr().unwrap())
            .await
            .unwrap();
        let (stream, _) = listener.accept().await.unwrap();
        let mut router = Router::new(model());
        router.add_systems(
            HeaderReceived,
            |header: Res<RequestHeader>| -> std::io::Result<()> {
                assert_eq!(header.method, "GET");
                Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "rejected",
                ))
            },
        );
        router.add_systems(HeaderReceived, || -> () {
            panic!("must stop after rejection")
        });
        router.on_connection(stream).unwrap();
        client
            .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .await
            .unwrap();
        let error = router.join_next().await.unwrap().unwrap().unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::PermissionDenied);
    })
    .await
    .unwrap();
}
