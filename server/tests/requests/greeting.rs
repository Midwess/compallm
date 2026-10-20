use cosin_server::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_default_greeting() {
    request::<App, _, _>(|request, _ctx| async move {
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
