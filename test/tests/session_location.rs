use compallm_geolocation::CityDatabase;
use std::path::Path;

#[test]
fn sign_in_location_is_approximate_and_local_addresses_stay_unknown() {
    let geoip = CityDatabase::open(Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/GeoIP2-City-Test.mmdb"
    )))
    .unwrap();
    assert_eq!(
        geoip.location("81.2.69.160".parse().unwrap()).as_deref(),
        Some("London, United Kingdom")
    );
    for ip in [
        "127.0.0.1",
        "10.0.0.1",
        "100.64.0.1",
        "::1",
        "fc00::1",
        "2001:db8::1",
        "::ffff:127.0.0.1",
    ] {
        assert_eq!(geoip.location(ip.parse().unwrap()), None);
    }
}

#[tokio::test]
#[ignore = "requires a migrated COMPALLM_TEST_DATABASE_URL"]
async fn migrated_sessions_expose_nullable_recognition_columns() -> anyhow::Result<()> {
    let db = sea_orm::Database::connect(std::env::var("COMPALLM_TEST_DATABASE_URL")?).await?;
    let manager = sea_orm_migration::SchemaManager::new(&db);
    for column in ["ip_address", "user_agent", "location"] {
        assert!(manager.has_column("user_sessions", column).await?);
    }
    db.close().await?;
    Ok(())
}

#[tokio::test]
async fn api_location_enrichment_reads_city_and_ignores_server_failure() {
    use compallm_api::geolocation::LocationClient;
    use std::sync::Arc;
    let database = Arc::new(
        CityDatabase::open(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/GeoIP2-City-Test.mmdb"
        )))
        .unwrap(),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, compallm_geolocation::router(database))
            .await
            .unwrap();
    });
    let client = LocationClient::new(&format!("http://{address}")).unwrap();
    assert_eq!(
        client
            .location("81.2.69.160".parse().unwrap())
            .await
            .as_deref(),
        Some("London, United Kingdom")
    );
    assert_eq!(client.location("127.0.0.1".parse().unwrap()).await, None);
    server.abort();
    let _ = server.await;
    assert_eq!(client.location("81.2.69.160".parse().unwrap()).await, None);
}

#[tokio::test]
async fn slow_location_responses_are_bounded_and_optional() {
    use compallm_api::geolocation::LocationClient;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let app = axum::Router::new().route(
            "/v1/locations/{ip}",
            axum::routing::get(|| async {
                std::future::pending::<()>().await;
                "unreachable"
            }),
        );
        axum::serve(listener, app).await.unwrap();
    });
    let client = LocationClient::new(&format!("http://{address}")).unwrap();
    let start = std::time::Instant::now();
    assert_eq!(client.location("81.2.69.160".parse().unwrap()).await, None);
    assert!(start.elapsed() < std::time::Duration::from_secs(2));
    server.abort();
}
