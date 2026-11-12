use compallm_api::http::listener::bind_tcp_listener;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::test]
async fn restart_reuses_the_port_after_serving_ipv4_and_ipv6_connections() {
    let mut listener = bind_tcp_listener(32000).await.unwrap();
    let port = listener.local_addr().unwrap().port();
    for host in ["127.0.0.1", "::1"] {
        let mut client = TcpStream::connect((host, port)).await.unwrap();
        let (mut server, _) = listener.accept().await.unwrap();
        server.write_all(b"response").await.unwrap();
        server.shutdown().await.unwrap();
        drop(server);
        let mut response = Vec::new();
        client.read_to_end(&mut response).await.unwrap();
        drop(client);
        drop(listener);
        listener = bind_tcp_listener(port).await.unwrap();
        assert_eq!(listener.local_addr().unwrap().port(), port);
    }
}

#[tokio::test]
async fn occupied_ports_are_skipped_without_sharing_the_active_listener() {
    let first = bind_tcp_listener(32100).await.unwrap();
    let first_port = first.local_addr().unwrap().port();
    let second = bind_tcp_listener(first_port).await.unwrap();
    assert!(second.local_addr().unwrap().port() > first_port);
    let client = TcpStream::connect(("127.0.0.1", first_port)).await.unwrap();
    let (accepted, _) = first.accept().await.unwrap();
    assert_eq!(accepted.local_addr().unwrap().port(), first_port);
    drop(client);
}
