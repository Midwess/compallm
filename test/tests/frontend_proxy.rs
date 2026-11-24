use axum::{
    Router,
    body::{Body, Bytes},
    extract::State,
    http::{Request, StatusCode},
    response::Response,
    routing::post,
};
use compallm_api::http::frontend::{Frontend, FrontendSettings};
use futures_util::StreamExt;
use std::{convert::Infallible, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Notify,
};

#[tokio::test]
async fn portal_proxy_streams_both_directions_and_keeps_cookie_delivery() {
    let received = Arc::new(Notify::new());
    let finish = Arc::new(Notify::new());
    let upstream_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = upstream_listener.local_addr().unwrap().port();
    let state = (received.clone(), finish.clone());
    async fn upstream(
        State((received, finish)): State<(Arc<Notify>, Arc<Notify>)>,
        request: Request<Body>,
    ) -> Response {
        assert_eq!(request.headers()["x-forwarded-host"], "localhost:3000");
        assert_eq!(request.headers()["x-forwarded-proto"], "http");
        assert!(request.headers().get("x-remove-me").is_none());
        let mut body = request.into_body().into_data_stream();
        assert_eq!(body.next().await.unwrap().unwrap(), "first");
        received.notify_one();
        let mut remaining = Vec::new();
        while let Some(chunk) = body.next().await {
            remaining.extend_from_slice(&chunk.unwrap());
        }
        assert_eq!(remaining, b"second");
        let stream = futures_util::stream::unfold((0, finish), |(index, finish)| async move {
            match index {
                0 => Some((
                    Ok::<_, Infallible>(Bytes::from_static(b"early")),
                    (1, finish),
                )),
                1 => {
                    finish.notified().await;
                    Some((Ok(Bytes::from_static(b"late")), (2, finish)))
                }
                _ => None,
            }
        });
        let mut response = Response::new(Body::from_stream(stream));
        *response.status_mut() = StatusCode::ACCEPTED;
        response
            .headers_mut()
            .append("set-cookie", "first=1; HttpOnly; Path=/".parse().unwrap());
        response
            .headers_mut()
            .append("set-cookie", "second=2; HttpOnly; Path=/".parse().unwrap());
        response
    }
    let upstream = tokio::spawn(async move {
        axum::serve(
            upstream_listener,
            Router::new()
                .route("/page", post(upstream))
                .with_state(state),
        )
        .await
        .unwrap()
    });
    let frontend = Frontend::start(
        std::path::Path::new("."),
        3000,
        FrontendSettings {
            port,
            managed: false,
            ..Default::default()
        },
    )
    .await
    .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let proxy = tokio::spawn(axum::serve(listener, frontend.router()).into_future());
    let mut stream = tokio::net::TcpStream::connect(address).await.unwrap();
    stream.write_all(b"POST /page HTTP/1.1\r\nHost: localhost:3000\r\nTransfer-Encoding: chunked\r\nConnection: close, x-remove-me\r\nX-Remove-Me: private\r\nX-Forwarded-Host: attacker.invalid\r\n\r\n5\r\nfirst\r\n").await.unwrap();
    use hero::cancellation::{CancellationToken, FutureExtension};
    let deadline = CancellationToken::timeout(std::time::Duration::from_secs(10));
    received.notified().with_cancel(&deadline).await.unwrap();
    stream.write_all(b"6\r\nsecond\r\n0\r\n\r\n").await.unwrap();
    let mut response = Vec::new();
    let mut buffer = [0u8; 1024];
    loop {
        let count = stream
            .read(&mut buffer)
            .with_cancel(&deadline)
            .await
            .unwrap()
            .unwrap();
        assert_ne!(count, 0);
        response.extend_from_slice(&buffer[..count]);
        if response.windows(5).any(|window| window == b"early") {
            break;
        }
    }
    let headers = String::from_utf8_lossy(&response).to_lowercase();
    assert!(headers.starts_with("http/1.1 202"));
    assert_eq!(headers.matches("set-cookie:").count(), 2);
    assert!(!headers.contains("late"));
    finish.notify_one();
    stream
        .read_to_end(&mut response)
        .with_cancel(&deadline)
        .await
        .unwrap()
        .unwrap();
    assert!(response.windows(4).any(|window| window == b"late"));
    proxy.abort();
    upstream.abort();
    drop(frontend);
}
