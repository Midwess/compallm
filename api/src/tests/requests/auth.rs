use std::sync::{Arc, Mutex};

use axum_test::TestServer;
use compallm_core::{
    auth::{AccessToken, EmailOtpAccessToken},
    mail::{EmailError, EmailRequest, EmailService},
    persistent::DbContext,
};
use compallm_postgres::{
    OrganizationMemberSchema, OrganizationRoleSchema, OrganizationSchema, OrganizationStatusSchema,
    UserSchema, UserStatusSchema, postgres_connection::PostgresConnection,
};
use sea_orm::{ActiveEnum, DbBackend, IntoMockRow, MockDatabase, MockExecResult};

use crate::{
    controllers,
    di::{AuthSettings, Di},
};

const SECRET: &str = "test-signing-secret-at-least-32-bytes";

#[derive(Default)]
struct Mailbox(Arc<Mutex<Vec<EmailRequest>>>);

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
            user_token_ttl_seconds: 900,
            refresh_token_ttl_seconds: 30 * 24 * 60 * 60,
        }
    }

    fn di(mailbox: Option<Box<dyn EmailService>>) -> &'static Di {
        let now = "2026-10-25T00:00:00Z"
            .parse::<sea_orm::prelude::DateTimeWithTimeZone>()
            .unwrap();
        let row = UserSchema {
            id: 42,
            email: "james@midwess.com".into(),
            display_name: "james".into(),
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
        let organization = OrganizationSchema {
            id: 84,
            name: "james's organization".into(),
            status: OrganizationStatusSchema::Active,
            created_by: 42,
            created_at: now,
            updated_at: now,
        };
        let mut organization: std::collections::BTreeMap<_, _> = organization
            .into_mock_row()
            .into_column_value_tuples()
            .collect();
        organization.insert("status".to_string(), "active".into());
        let member = OrganizationMemberSchema {
            organization_id: 84,
            user_id: 42,
            role: OrganizationRoleSchema::Owner,
            created_at: now,
        };
        let mut member: std::collections::BTreeMap<_, _> =
            member.into_mock_row().into_column_value_tuples().collect();
        member.insert("role".to_string(), "owner".into());
        let db = MockDatabase::new(DbBackend::Postgres)
            .append_exec_results([MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .append_query_results([
                vec![],
                vec![row],
                vec![organization],
                vec![member.clone()],
                vec![member],
            ])
            .into_connection();
        let di: &'static Di = Box::leak(Box::new(Di::new(
            DbContext::new(Arc::new(PostgresConnection::new(Arc::new(db)))),
            Self::settings(),
            mailbox,
        )));
        di.initialize_services().unwrap();
        di
    }

    fn server(di: &'static Di) -> TestServer {
        let mut router = axum::Router::new();
        di.inject(&mut router);
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
    let server = Fixture::server(Fixture::di(Some(Box::new(FailingEmail))));
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
        let smtp: compallm_mailing::SmtpSettings =
            serde_json::from_value(settings["smtp"].clone()).unwrap();
        assert!(
            smtp.password == std::env::var("SMTP_PASSWORD").unwrap_or_default(),
            "SMTP password must preserve its literal contents"
        );
        assert!(
            smtp.username == std::env::var("SMTP_USERNAME").unwrap_or_default(),
            "SMTP username must preserve its literal contents"
        );
        if environment == Environment::Test {
            assert!(!smtp.enabled);
        } else {
            assert_eq!(smtp.port, 1025);
            assert!(!smtp.host.is_empty());
        }
    }
}

#[tokio::test]
async fn signup_delivers_code_and_confirmation_creates_user() {
    compallm_postgres::init_id_generator("api-tests").unwrap();
    let mailbox = Mailbox::default();
    let messages = mailbox.0.clone();
    let di = Fixture::di(Some(Box::new(mailbox)));
    assert!(std::ptr::eq(di.user_repository(), di.user_repository()));
    assert!(std::ptr::eq(di.auth_service(), di.auth_service()));
    let service = di.auth_service();
    di.initialize_services().unwrap();
    assert!(std::ptr::eq(service, di.auth_service()));
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
        let messages = messages.lock().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].to, "james@midwess.com");
        assert!(messages[0].text.to_string().contains(&claims.code));
        assert!(messages[0].text.plain_text().contains(&claims.code));
    }
    server
        .post("/api/auth/signup/email/confirmation/otp")
        .json(&serde_json::json!({"token": token, "code": "wrong"}))
        .await
        .assert_status_unauthorized();
    let response = server
        .post("/api/auth/signup/email/confirmation/otp")
        .json(&serde_json::json!({"token": token, "code": claims.code}))
        .await;
    response.assert_status(axum::http::StatusCode::CREATED);
    let tokens: serde_json::Value = response.json();
    let access = compallm_core::auth::UserAccessToken::try_from(
        AccessToken::decode(tokens["accessToken"].as_str().unwrap(), SECRET).unwrap(),
    )
    .unwrap();
    assert_eq!(access.session.organization_id, 84);
    assert_eq!(
        access.organization_role,
        compallm_core::organization::OrganizationRole::Owner
    );
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
        .json(&serde_json::json!({"email":"not-an-email"}))
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

struct ThrottledEmail;
#[async_trait::async_trait]
impl EmailService for ThrottledEmail {
    async fn send(&self, _: EmailRequest) -> Result<(), EmailError> {
        Err(EmailError::Throttled)
    }
}

#[tokio::test]
async fn throttled_recipient_returns_429_without_a_replacement_challenge() {
    let server = Fixture::server(Fixture::di(Some(Box::new(ThrottledEmail))));
    let response = server
        .post("/api/auth/signup/email")
        .json(&serde_json::json!({"email":"reader@example.com"}))
        .await;
    response.assert_status(axum::http::StatusCode::TOO_MANY_REQUESTS);
    let body: serde_json::Value = response.json();
    assert!(body.get("token").is_none());
    assert!(
        body["message"]
            .as_str()
            .unwrap()
            .contains("existing challenge")
    );
}

#[tokio::test]
async fn signup_email_accepts_addresses_from_any_domain() {
    let mailbox = Mailbox::default();
    let messages = mailbox.0.clone();
    let server = Fixture::server(Fixture::di(Some(Box::new(mailbox))));
    for email in [
        "james@gmail.com",
        "james@example.org",
        "james@sub.midwess.com",
    ] {
        let response = server
            .post("/api/auth/signup/email")
            .json(&serde_json::json!({"email":email}))
            .await;
        response.assert_status_ok();
        let body: serde_json::Value = response.json();
        let claims = EmailOtpAccessToken::try_from(
            AccessToken::decode(body["token"].as_str().unwrap(), SECRET).unwrap(),
        )
        .unwrap();
        assert_eq!(claims.email, email);
    }
    assert_eq!(messages.lock().unwrap().len(), 3);
}

#[test]
fn openapi_exposes_only_signup_routes() {
    let document = serde_json::to_value(controllers::openapi()).unwrap();
    let paths = document["paths"].as_object().unwrap();
    let confirmation = &document["components"]["schemas"]["ConfirmSignupOtpRequest"];
    assert!(confirmation["properties"].get("displayName").is_none());
    assert_eq!(confirmation["required"].as_array().unwrap().len(), 2);
    assert_eq!(paths.len(), 2);
    assert!(paths["/api/auth/signup/email"].get("post").is_some());
    assert!(
        paths["/api/auth/signup/email/confirmation/otp"]
            .get("post")
            .is_some()
    );
}
