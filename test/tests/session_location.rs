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
#[ignore = "requires COMPALLM_TEST_DATABASE_URL; uses a disposable database"]
async fn dropping_saved_location_preserves_session_recognition() -> anyhow::Result<()> {
    use sea_orm::{
        ConnectionTrait, Database,
        sea_query::{Alias, ColumnDef, Query, Table},
    };
    use sea_orm_migration::{MigratorTrait, SchemaManager};
    let url = std::env::var("COMPALLM_TEST_DATABASE_URL")?;
    let name = format!(
        "location_drop_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos()
    );
    let admin = Database::connect(&url).await?;
    admin
        .execute_unprepared(&format!("CREATE DATABASE {name}"))
        .await?;
    let result = async {
        let (base, parameters) = url.split_once('?').unwrap_or((&url, ""));
        let (server, _) = base
            .rsplit_once('/')
            .ok_or_else(|| anyhow::anyhow!("database URL"))?;
        let db = Database::connect(format!("{server}/{name}?{parameters}")).await?;
        let manager = SchemaManager::new(&db);
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("user_sessions"))
                    .col(ColumnDef::new(Alias::new("id")).big_integer().primary_key())
                    .to_owned(),
            )
            .await?;
        let migrations = compallm_postgres::Migrator::migrations();
        let metadata = migrations
            .iter()
            .find(|m| m.name().ends_with("000044_session_metadata"))
            .unwrap();
        let drop_location = migrations
            .iter()
            .find(|m| m.name().ends_with("000045_drop_session_location"))
            .unwrap();
        metadata.up(&manager).await?;
        db.execute(
            &Query::insert()
                .into_table(Alias::new("user_sessions"))
                .columns(["id", "ip_address", "user_agent", "location"].map(Alias::new))
                .values_panic([
                    1_i64.into(),
                    "203.0.113.42".into(),
                    "Test browser".into(),
                    "Old city".into(),
                ])
                .to_owned(),
        )
        .await?;
        drop_location.up(&manager).await?;
        assert!(!manager.has_column("user_sessions", "location").await?);
        let row = db
            .query_one(
                &Query::select()
                    .columns(["id", "ip_address", "user_agent"].map(Alias::new))
                    .from(Alias::new("user_sessions"))
                    .to_owned(),
            )
            .await?
            .unwrap();
        assert_eq!(row.try_get::<i64>("", "id")?, 1);
        assert_eq!(row.try_get::<String>("", "ip_address")?, "203.0.113.42");
        assert_eq!(row.try_get::<String>("", "user_agent")?, "Test browser");
        db.close().await?;
        Ok::<_, anyhow::Error>(())
    }
    .await;
    admin
        .execute_unprepared(&format!("DROP DATABASE {name} WITH (FORCE)"))
        .await?;
    admin.close().await?;
    result
}

#[tokio::test]
async fn public_location_lookup_accepts_a_json_body_without_credentials() {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use std::sync::Arc;
    use tower::ServiceExt;
    let database = Arc::new(
        CityDatabase::open(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/GeoIP2-City-Test.mmdb"
        )))
        .unwrap(),
    );
    let app = compallm_geolocation::router(database);
    let preflight = app
        .clone()
        .oneshot(
            Request::builder()
                .method("OPTIONS")
                .uri("/v1/locations")
                .header("origin", "https://portal.example.com")
                .header("access-control-request-method", "POST")
                .header("access-control-request-headers", "content-type")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(preflight.headers()["access-control-allow-origin"], "*");
    for (ip, expected) in [
        ("81.2.69.160", Some("London, United Kingdom")),
        ("127.0.0.1", None),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/locations")
                    .header("origin", "https://portal.example.com")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::json!({"ip":ip}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["cache-control"], "no-store");
        let body = axum::body::to_bytes(response.into_body(), 4096)
            .await
            .unwrap();
        let location: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(location["location"].as_str(), expected);
    }
}
