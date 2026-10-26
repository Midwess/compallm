use crate::app::App;
use axum::{Router, routing::get};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::oneshot,
    time::{Duration, timeout},
};

#[tokio::test]
async fn busy_starting_port_is_skipped_and_the_reserved_listener_serves_ipv4_http() {
    // Exercise the listener hook without booting Loco or touching the database.
    let occupied = App::tcp_listener(43000).await.unwrap();
    let preferred = occupied.local_addr().unwrap().port();
    let listener = App::tcp_listener(i32::from(preferred)).await.unwrap();
    let chosen = listener.local_addr().unwrap().port();
    assert!(chosen > preferred);
    assert!(listener.local_addr().unwrap().ip().is_unspecified());
    let (stop, stopped) = oneshot::channel();
    let app = Router::new().route("/api/health", get(|| async { "ready" }));
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let mut stream = TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, chosen))
        .await
        .unwrap();
    stream
        .write_all(b"GET /api/health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();
    let mut response = String::new();
    timeout(Duration::from_secs(5), stream.read_to_string(&mut response))
        .await
        .unwrap()
        .unwrap();
    assert!(response.starts_with("HTTP/1.1 200"));
    assert!(response.ends_with("ready"));
    stop.send(()).unwrap();
    timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn invalid_starting_ports_fail_before_listener_discovery() {
    for port in [-1, 0, 50101, 65535, 65536, i32::MAX] {
        let error = App::tcp_listener(port).await.unwrap_err();
        assert!(error.to_string().contains("between 1 and 50100"));
    }
}

#[tokio::test]
async fn api_routes_and_missing_api_paths_stay_in_axum_ahead_of_the_frontend_fallback() {
    let router = App::with_api_fallback(
        Router::new()
            .route(
                "/api/health",
                get(|| async { axum::Json(serde_json::json!({"ready":true})) }),
            )
            .fallback(|| async { axum::response::Html("<!doctype html><p>frontend</p>") }),
    );
    let server = axum_test::TestServer::new(router).unwrap();
    let response = server.get("/api/health").await;
    response.assert_status_ok();
    assert_eq!(response.json::<serde_json::Value>()["ready"], true);
    for path in ["/api", "/api/not-found"] {
        let response = server.get(path).await;
        response.assert_status_not_found();
        assert_eq!(
            response.json::<serde_json::Value>()["message"],
            "API endpoint not found"
        );
    }
    let response = server.get("/client-page").await;
    response.assert_status_ok();
    assert!(response.text().contains("frontend"));
}
