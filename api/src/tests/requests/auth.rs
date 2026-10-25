use std::sync::{Arc, Mutex};

use axum_test::TestServer;
use compallm_core::{
    auth::{AccessToken, EmailOtpAccessToken},
    mail::{EmailError, EmailRequest, EmailService},
    persistent::DbContext,
};
use compallm_postgres::{UserSchema, UserStatusSchema, postgres_connection::PostgresConnection};
use sea_orm::{ActiveEnum, DbBackend, IntoMockRow, MockDatabase};

use crate::{
    controllers,
    di::{AuthSettings, Di},
};

const SECRET: &str = "test-signing-secret-at-least-32-bytes";

#[derive(Default)]
struct Mailbox(Mutex<Vec<EmailRequest>>);

#[async_trait::async_trait]
impl EmailService for Mailbox {
    async fn send(&self, request: EmailRequest) -> Result<(), EmailError> {
        self.0.lock().unwrap().push(request);
        Ok(())
    }
}

struct Fixture;

impl Fixture {
    fn settings() -> AuthSettings {
        AuthSettings {
            jwt_secret: SECRET.into(),
            allowed_email_domain: "midwess.com".into(),
            user_token_ttl_seconds: 900,
            refresh_token_ttl_seconds: 30 * 24 * 60 * 60,
        }
    }

    fn di(mailbox: Option<Arc<dyn EmailService>>) -> Di {
        let now = "2026-10-25T00:00:00Z"
            .parse::<sea_orm::prelude::DateTimeWithTimeZone>()
            .unwrap();
        let row = UserSchema {
            id: 42,
            email: "james@midwess.com".into(),
            display_name: "James".into(),
            status: UserStatusSchema::Active,
            email_verified_at: now,
            last_login_at: None,
            created_at: now,
            updated_at: now,
        };
        let status = row.status.to_value().value.to_string();
        let mut row: std::collections::BTreeMap<_, _> =
            row.into_mock_row().into_column_value_tuples().collect();
        row.insert("status".to_string(), status.into());
        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([vec![], vec![row], vec![]])
            .into_connection();
        Di::new(
            DbContext::new(Arc::new(PostgresConnection::new(Arc::new(db)))),
            Self::settings(),
            mailbox,
        )
        .unwrap()
    }

    fn server(di: Di) -> TestServer {
        let db = di.db_context().clone();
        let (router, _) = controllers::api_router().split_for_parts();
        let router = router.with_state(di).layer(axum::middleware::from_fn(
            move |request: axum::extract::Request, next: axum::middleware::Next| {
                let db = db.clone();
                async move { db.scope(next.run(request)).await }
            },
        ));
        TestServer::new(router).unwrap()
    }
}

struct FailingEmail;

#[async_trait::async_trait]
impl EmailService for FailingEmail {
    async fn send(&self, _: EmailRequest) -> Result<(), EmailError> {
        Err(EmailError::Delivery(anyhow::anyhow!(
            "transport unavailable"
        )))
    }
}

#[tokio::test]
async fn failed_email_delivery_does_not_return_a_challenge() {
    let server = Fixture::server(Fixture::di(Some(Arc::new(FailingEmail))));
    let response = server
        .post("/api/auth/signup/email")
        .json(&serde_json::json!({"email":"james@midwess.com"}))
        .await;
    response.assert_status_service_unavailable();
    assert!(response.json::<serde_json::Value>().get("token").is_none());
    assert!(!response.text().contains("transport unavailable"));
}

#[tokio::test]
async fn test_and_development_auth_configuration_load_without_connecting() {
    use loco_rs::{app::Hooks, environment::Environment};
    for environment in [Environment::Test, Environment::Development] {
        let config = crate::app::App::load_config(&environment).await.unwrap();
        let settings = config.settings.unwrap();
        assert!(settings["auth"]["jwt_secret"].as_str().unwrap().len() >= 32);
        assert_eq!(settings["auth"]["refresh_token_ttl_seconds"], 2_592_000);
    }
}

#[tokio::test]
async fn signup_delivers_code_and_confirmation_creates_user() {
    compallm_postgres::init_id_generator("api-tests").unwrap();
    let mailbox = Arc::new(Mailbox::default());
    let di = Fixture::di(Some(mailbox.clone()));
    assert!(Arc::ptr_eq(&Di::user_repository(), &Di::user_repository()));
    let server = Fixture::server(di);
    let response = server
        .post("/api/auth/signup/email")
        .json(&serde_json::json!({"email": " James@MIDWESS.COM "}))
        .await;
    response.assert_status_ok();
    assert_eq!(response.header("cache-control"), "no-store");
    let body: serde_json::Value = response.json();
    assert_eq!(body.as_object().unwrap().len(), 1);
    let token = body["token"].as_str().unwrap();
    let claims =
        EmailOtpAccessToken::try_from(AccessToken::decode(token, SECRET).unwrap()).unwrap();
    {
        let messages = mailbox.0.lock().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].to, "james@midwess.com");
        assert!(messages[0].text.to_string().contains(&claims.code));
        assert!(messages[0].text.plain_text().contains(&claims.code));
    }
    server
        .post("/api/auth/signup/email/confirmation/otp")
        .json(&serde_json::json!({"token": token, "code": "wrong", "displayName": "James"}))
        .await
        .assert_status_unauthorized();
    let response = server
        .post("/api/auth/signup/email/confirmation/otp")
        .json(&serde_json::json!({"token": token, "code": claims.code, "displayName": "James"}))
        .await;
    response.assert_status(axum::http::StatusCode::CREATED);
    let tokens: serde_json::Value = response.json();
    assert!(matches!(
        AccessToken::decode(tokens["accessToken"].as_str().unwrap(), SECRET).unwrap(),
        AccessToken::User(_)
    ));
    assert!(matches!(
        AccessToken::decode(tokens["refreshToken"].as_str().unwrap(), SECRET).unwrap(),
        AccessToken::Refresh(_)
    ));
    server.get("/api/greeting").await.assert_status_not_found();
}

#[tokio::test]
async fn email_endpoint_rejects_bad_inputs_and_unconfigured_delivery() {
    let server = Fixture::server(Fixture::di(None));
    server
        .post("/api/auth/signup/email")
        .json(&serde_json::json!({"email":"a@elsewhere.com"}))
        .await
        .assert_status_bad_request();
    server
        .post("/api/auth/signup/email")
        .json(&serde_json::json!({}))
        .await
        .assert_status_bad_request();
    let response = server
        .post("/api/auth/signup/email")
        .json(&serde_json::json!({"email":"james@midwess.com"}))
        .await;
    response.assert_status(axum::http::StatusCode::SERVICE_UNAVAILABLE);
    assert!(response.json::<serde_json::Value>().get("token").is_none());
}

#[test]
fn openapi_exposes_only_signup_routes() {
    let document = serde_json::to_value(controllers::openapi()).unwrap();
    let paths = document["paths"].as_object().unwrap();
    assert_eq!(paths.len(), 2);
    assert!(paths["/api/auth/signup/email"].get("post").is_some());
    assert!(
        paths["/api/auth/signup/email/confirmation/otp"]
            .get("post")
            .is_some()
    );
}
