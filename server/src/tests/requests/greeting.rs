use crate::app::App;
use compallm_postgres::{Connection as PostgresConnection, from_bigint, next_id, to_bigint};
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_default_greeting() {
    request::<App, _, _>(|request, ctx| async move {
        let postgres = PostgresConnection::from_context(&ctx);
        postgres
            .ping()
            .await
            .expect("PostgreSQL should be reachable");
        let id = next_id().await.expect("Snowflake ID should generate");
        let database_id = to_bigint(id).expect("generated ID should fit PostgreSQL BIGINT");
        assert_eq!(from_bigint(database_id), Ok(id));

        let response = request.get("/api/greeting").await;

        assert_eq!(response.status_code(), 200);
        response.assert_json(&serde_json::json!({
            "message": "Hello, Vite!",
            "servedBy": "Loco + Axum",
            "requestCount": 1
        }));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_create_personalized_greeting() {
    request::<App, _, _>(|request, _ctx| async move {
        let response = request
            .post("/api/greeting")
            .json(&serde_json::json!({ "name": "Ada" }))
            .await;

        assert_eq!(response.status_code(), 200);
        response.assert_json(&serde_json::json!({
            "message": "Hello, Ada!",
            "servedBy": "Loco + Axum",
            "requestCount": 1
        }));
    })
    .await;
}
