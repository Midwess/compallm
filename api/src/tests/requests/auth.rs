use axum::http::header::AUTHORIZATION;
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
            cookie_secure: false,
            public_origin: String::new(),
            user_token_ttl_seconds: 1800,
            refresh_token_ttl_seconds: 30 * 24 * 60 * 60,
        }
    }

    fn di(mailbox: Option<Box<dyn EmailService>>) -> &'static Di {
        Self::di_with_signup_checks(mailbox, false, Self::settings(), 1)
    }

    fn di_for(
        mailbox: Option<Box<dyn EmailService>>,
        signin: bool,
        settings: AuthSettings,
    ) -> &'static Di {
        Self::di_with_signup_checks(mailbox, signin, settings, 0)
    }

    fn di_with_signup_checks(
        mailbox: Option<Box<dyn EmailService>>,
        signin: bool,
        settings: AuthSettings,
        signup_checks: usize,
    ) -> &'static Di {
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
        let checks: Vec<Vec<std::collections::BTreeMap<String, sea_orm::Value>>> =
            vec![vec![]; signup_checks];
        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results(checks)
            .append_exec_results([MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .append_query_results(if signin {
                vec![vec![row], vec![member]]
            } else {
                vec![
                    vec![],
                    vec![row],
                    vec![organization],
                    vec![member.clone()],
                    vec![member],
                ]
            })
            .into_connection();
        let di: &'static Di = Box::leak(Box::new(Di::new(
            DbContext::new(Arc::new(PostgresConnection::new(Arc::new(db)))),
            settings,
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
        .json(&serde_json::json!({"delivery":"token","email":"james@midwess.com"}))
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
        assert_eq!(settings["auth"]["user_token_ttl_seconds"], 1800);
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
        .json(&serde_json::json!({"delivery":"token","email": " James@MIDWESS.COM "}))
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
    let wrong_code = if claims.code == "000000" {
        "000001"
    } else {
        "000000"
    };
    server
        .post("/api/auth/signup/email/confirmation/otp")
        .add_header(AUTHORIZATION, format!("Bearer {}", token))
        .json(&serde_json::json!({ "code": wrong_code, "delivery":"token"}))
        .await
        .assert_status_unauthorized();
    let response = server
        .post("/api/auth/signup/email/confirmation/otp")
        .add_header(AUTHORIZATION, format!("Bearer {}", token))
        .json(&serde_json::json!({ "code": format!(" {} ", claims.code), "delivery":"token"}))
        .await;
    response.assert_status(axum::http::StatusCode::CREATED);
    let tokens: serde_json::Value = response.json();
    assert_eq!(response.headers().get_all("set-cookie").iter().count(), 0);
    let access = compallm_core::auth::UserAccessToken::try_from(
        AccessToken::decode(tokens["accessToken"].as_str().unwrap(), SECRET).unwrap(),
    )
    .unwrap();
    assert_eq!(access.validity.expires_at - access.validity.issued_at, 1800);
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
        .json(&serde_json::json!({"delivery":"token","email":"not-an-email"}))
        .await
        .assert_status_bad_request();
    server
        .post("/api/auth/signup/email")
        .json(&serde_json::json!({}))
        .await
        .assert_status_bad_request();
    let response = server
        .post("/api/auth/signup/email")
        .json(&serde_json::json!({"delivery":"token","email":"james@midwess.com"}))
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
        .json(&serde_json::json!({"delivery":"token","email":"reader@example.com"}))
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
    let server = Fixture::server(Fixture::di_with_signup_checks(
        Some(Box::new(mailbox)),
        false,
        Fixture::settings(),
        3,
    ));
    for email in [
        "james@gmail.com",
        "james@example.org",
        "james@sub.midwess.com",
    ] {
        let response = server
            .post("/api/auth/signup/email")
            .json(&serde_json::json!({"delivery":"token","email":email}))
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
fn openapi_exposes_signup_and_signin_delivery() {
    let document = serde_json::to_value(controllers::openapi()).unwrap();
    let paths = document["paths"].as_object().unwrap();
    let confirmation = &document["components"]["schemas"]["ConfirmSignupOtpRequest"];
    assert!(confirmation["properties"].get("displayName").is_none());
    assert_eq!(confirmation["required"].as_array().unwrap().len(), 1);
    assert!(confirmation["properties"].get("token").is_none());
    assert_eq!(paths.len(), 5);
    assert!(paths["/api/auth/signup/email"].get("post").is_some());
    assert!(
        paths["/api/auth/signup/email/confirmation/otp"]
            .get("post")
            .is_some()
    );
}

#[tokio::test]
async fn cookie_confirmation_sets_both_cookies_without_exposing_credentials() {
    compallm_postgres::init_id_generator("api-tests").unwrap();
    for signin in [false, true] {
        for secure in [false, true] {
            let mut settings = Fixture::settings();
            settings.cookie_secure = secure;
            settings.public_origin = if secure {
                "https://app.example.com".into()
            } else {
                String::new()
            };
            let server = Fixture::server(Fixture::di_for(None, signin, settings));
            let challenge =
                compallm_core::auth::EmailOtpChallenge::new("james@midwess.com", SECRET).unwrap();
            let path = if signin {
                "/api/auth/signin/email/confirmation/otp"
            } else {
                "/api/auth/signup/email/confirmation/otp"
            };
            let origin = if secure {
                "https://app.example.com"
            } else {
                "http://127.0.0.1:3456"
            };
            let response = server
                .post(path)
                .add_header(axum::http::header::ORIGIN, origin)
                .add_header(axum::http::header::HOST, "127.0.0.1:3456")
                .add_header(AUTHORIZATION, format!("Bearer {}", challenge.access_token))
                .json(&serde_json::json!({"code":challenge.code}))
                .await;
            response.assert_status(if signin {
                axum::http::StatusCode::OK
            } else {
                axum::http::StatusCode::CREATED
            });
            assert_eq!(response.json::<serde_json::Value>(), serde_json::json!({}));
            assert_eq!(response.header("cache-control"), "no-store");
            assert_eq!(response.headers().get_all("set-cookie").iter().count(), 2);
            let names = if secure {
                ["__Host-access", "__Host-refresh"]
            } else {
                ["compallm-access", "compallm-refresh"]
            };
            for (index, name) in names.iter().enumerate() {
                let cookie = response.cookie(name);
                assert_eq!(cookie.http_only(), Some(true));
                assert_eq!(cookie.secure().unwrap_or(false), secure);
                assert_eq!(cookie.same_site(), Some(cookie::SameSite::Lax));
                assert_eq!(cookie.path(), Some("/"));
                assert_eq!(cookie.domain(), None);
                assert_eq!(
                    cookie.max_age().unwrap().whole_seconds(),
                    if index == 0 { 1800 } else { 2_592_000 }
                );
                let claims = AccessToken::decode(cookie.value(), SECRET).unwrap();
                assert!(matches!(
                    (&claims, index),
                    (AccessToken::User(_), 0) | (AccessToken::Refresh(_), 1)
                ));
                let json = serde_json::to_value(claims).unwrap();
                assert!(!json.to_string().contains("delivery"));
            }
        }
    }
}

#[tokio::test]
async fn signin_delivers_email_and_returns_json_tokens_only_when_requested() {
    compallm_postgres::init_id_generator("api-tests").unwrap();
    let mailbox = Mailbox::default();
    let messages = mailbox.0.clone();
    let server = Fixture::server(Fixture::di_for(
        Some(Box::new(mailbox)),
        true,
        Fixture::settings(),
    ));
    let challenge = server
        .post("/api/auth/signin/email")
        .json(&serde_json::json!({"delivery":"token","email":"james@midwess.com"}))
        .await;
    challenge.assert_status_ok();
    let body: serde_json::Value = challenge.json();
    let token = body["token"].as_str().unwrap();
    let otp = EmailOtpAccessToken::try_from(AccessToken::decode(token, SECRET).unwrap()).unwrap();
    assert!(
        messages.lock().unwrap()[0]
            .text
            .plain_text()
            .contains(&otp.code)
    );
    let response = server
        .post("/api/auth/signin/email/confirmation/otp")
        .add_header(AUTHORIZATION, format!("Bearer {}", token))
        .json(&serde_json::json!({"code":format!(" {} ", otp.code),"delivery":"token"}))
        .await;
    response.assert_status_ok();
    assert!(response.headers().get("set-cookie").is_none());
    let body: serde_json::Value = response.json();
    assert!(matches!(
        AccessToken::decode(body["accessToken"].as_str().unwrap(), SECRET).unwrap(),
        AccessToken::User(_)
    ));
    assert!(matches!(
        AccessToken::decode(body["refreshToken"].as_str().unwrap(), SECRET).unwrap(),
        AccessToken::Refresh(_)
    ));
}

#[tokio::test]
async fn invalid_delivery_or_cookie_origin_never_sets_credentials() {
    let server = Fixture::server(Fixture::di(None));
    for path in [
        "/api/auth/signup/email/confirmation/otp",
        "/api/auth/signin/email/confirmation/otp",
    ] {
        for origin in [
            None,
            Some("null"),
            Some("https://evil.example"),
            Some("http://localhost:3456"),
        ] {
            let mut request = server
                .post(path)
                .add_header(axum::http::header::HOST, "127.0.0.1:3456");
            if let Some(origin) = origin {
                request = request.add_header(axum::http::header::ORIGIN, origin);
            }
            let response = request
                .add_header(
                    AUTHORIZATION,
                    format!(
                        "Bearer {}",
                        compallm_core::auth::EmailOtpChallenge::new("james@midwess.com", SECRET)
                            .unwrap()
                            .access_token
                    ),
                )
                .json(&serde_json::json!({"code":"123456","delivery":"cookie"}))
                .await;
            response.assert_status_forbidden();
            assert!(response.headers().get("set-cookie").is_none());
            assert!(
                response
                    .json::<serde_json::Value>()
                    .get("accessToken")
                    .is_none()
            );
        }
        server
            .post(path)
            .add_header(
                AUTHORIZATION,
                format!(
                    "Bearer {}",
                    compallm_core::auth::EmailOtpChallenge::new("james@midwess.com", SECRET)
                        .unwrap()
                        .access_token
                ),
            )
            .json(&serde_json::json!({"code":"123456","delivery":"both"}))
            .await
            .assert_status_bad_request();
    }
}

#[tokio::test]
async fn failed_confirmation_never_sets_cookies_or_returns_tokens() {
    let server = Fixture::server(Fixture::di(None));
    for path in [
        "/api/auth/signup/email/confirmation/otp",
        "/api/auth/signin/email/confirmation/otp",
    ] {
        let response = server
            .post(path)
            .add_header(axum::http::header::ORIGIN, "http://localhost:3000")
            .add_header(axum::http::header::HOST, "localhost:3000")
            .add_header(AUTHORIZATION, format!("Bearer {}", "invalid"))
            .json(&serde_json::json!({"code":"123456"}))
            .await;
        response.assert_status_unauthorized();
        assert!(response.headers().get("set-cookie").is_none());
        let body: serde_json::Value = response.json();
        assert!(body.get("accessToken").is_none());
        assert!(body.get("refreshToken").is_none());
    }
}

#[tokio::test]
async fn verified_signin_without_account_returns_not_found_without_credentials() {
    let server = Fixture::server(Fixture::di(None));
    let challenge =
        compallm_core::auth::EmailOtpChallenge::new("james@midwess.com", SECRET).unwrap();
    let response = server
        .post("/api/auth/signin/email/confirmation/otp")
        .add_header(AUTHORIZATION, format!("Bearer {}", challenge.access_token))
        .json(&serde_json::json!({"code":challenge.code,"delivery":"token"}))
        .await;
    response.assert_status_not_found();
    assert!(response.headers().get("set-cookie").is_none());
    assert!(
        response
            .json::<serde_json::Value>()
            .get("accessToken")
            .is_none()
    );
}

#[tokio::test]
async fn secure_cookie_mode_requires_the_configured_origin() {
    let mut settings = Fixture::settings();
    settings.cookie_secure = true;
    settings.public_origin = "https://app.example.com".into();
    let server = Fixture::server(Fixture::di_for(None, true, settings));
    for origin in [
        "https://evil.example",
        "http://app.example.com",
        "https://app.example.com.evil.example",
    ] {
        let response = server
            .post("/api/auth/signin/email/confirmation/otp")
            .add_header(axum::http::header::ORIGIN, origin)
            .add_header(
                AUTHORIZATION,
                format!(
                    "Bearer {}",
                    compallm_core::auth::EmailOtpChallenge::new("james@midwess.com", SECRET)
                        .unwrap()
                        .access_token
                ),
            )
            .json(&serde_json::json!({"code":"123456"}))
            .await;
        response.assert_status_forbidden();
        assert!(response.headers().get("set-cookie").is_none());
    }
    let mut settings = Fixture::settings();
    settings.cookie_secure = true;
    let server = Fixture::server(Fixture::di_for(None, true, settings));
    server
        .post("/api/auth/signin/email/confirmation/otp")
        .add_header(axum::http::header::ORIGIN, "http://localhost:3000")
        .add_header(axum::http::header::HOST, "localhost:3000")
        .add_header(
            AUTHORIZATION,
            format!(
                "Bearer {}",
                compallm_core::auth::EmailOtpChallenge::new("james@midwess.com", SECRET)
                    .unwrap()
                    .access_token
            ),
        )
        .json(&serde_json::json!({"code":"123456"}))
        .await
        .assert_status_forbidden();
}

#[tokio::test]
async fn dto_validation_rejects_bad_formats_without_echoing_credentials() {
    let mailbox = Mailbox::default();
    let messages = mailbox.0.clone();
    let server = Fixture::server(Fixture::di(Some(Box::new(mailbox))));
    for path in ["/api/auth/signup/email", "/api/auth/signin/email"] {
        for email in [" ", "a@@example.com", "sensitive-invalid-email"] {
            let response = server
                .post(path)
                .json(&serde_json::json!({"delivery":"token","email":email}))
                .await;
            response.assert_status_bad_request();
            assert_eq!(
                response.json::<serde_json::Value>(),
                serde_json::json!({"message":"Invalid request fields"})
            );
        }
        server
            .post(path)
            .json(&serde_json::json!({"delivery":"token","email":42}))
            .await
            .assert_status_bad_request();
        server
            .post(path)
            .json(&serde_json::json!({}))
            .await
            .assert_status_bad_request();
    }
    assert!(messages.lock().unwrap().is_empty());
    for path in [
        "/api/auth/signup/email/confirmation/otp",
        "/api/auth/signin/email/confirmation/otp",
    ] {
        for code in ["", "12345", "1234567", "abcdef", "１２３４５６"] {
            let response = server
                .post(path)
                .add_header(
                    AUTHORIZATION,
                    format!(
                        "Bearer {}",
                        compallm_core::auth::EmailOtpChallenge::new("james@midwess.com", SECRET)
                            .unwrap()
                            .access_token
                    ),
                )
                .json(&serde_json::json!({"code":code,"delivery":"token"}))
                .await;
            response.assert_status_bad_request();
            assert_eq!(
                response.json::<serde_json::Value>(),
                serde_json::json!({"message":"Invalid request fields"})
            );
            assert!(response.headers().get("set-cookie").is_none());
        }
        for token in [String::new(), "a".repeat(16385)] {
            let response = server
                .post(path)
                .add_header(AUTHORIZATION, format!("Bearer {token}"))
                .json(&serde_json::json!({"code":"123456","delivery":"token"}))
                .await;
            response.assert_status_unauthorized();
            assert!(response.headers().get("set-cookie").is_none());
        }
    }
}

#[tokio::test]
async fn both_email_request_dtos_normalize_before_delivery() {
    let mailbox = Mailbox::default();
    let messages = mailbox.0.clone();
    let server = Fixture::server(Fixture::di(Some(Box::new(mailbox))));
    for path in ["/api/auth/signup/email", "/api/auth/signin/email"] {
        let response = server
            .post(path)
            .json(&serde_json::json!({"delivery":"token","email":"  James@MIDWESS.COM  "}))
            .await;
        response.assert_status_ok();
        let body: serde_json::Value = response.json();
        let otp = EmailOtpAccessToken::try_from(
            AccessToken::decode(body["token"].as_str().unwrap(), SECRET).unwrap(),
        )
        .unwrap();
        assert_eq!(otp.email, "james@midwess.com");
    }
    assert_eq!(messages.lock().unwrap().len(), 2);
    assert!(
        messages
            .lock()
            .unwrap()
            .iter()
            .all(|message| message.to == "james@midwess.com")
    );
}

#[test]
fn dto_email_normalization_keeps_ascii_case_policy() {
    use validify::Validify;
    for input in [
        "  İ@EXAMPLE.COM  ",
        " James@EXÄMPLE.COM ",
        " James@MIDWESS.COM ",
    ] {
        let mut dto = controllers::auth::SignupEmailRequest {
            email: input.into(),
            delivery: Default::default(),
        };
        let expected = input.trim().to_ascii_lowercase();
        let result = dto.validify();
        assert_eq!(dto.email, expected);
        assert_eq!(
            result.is_ok(),
            compallm_core::user::User::validate_email(&expected).is_ok()
        );
    }
}

#[tokio::test]
async fn otp_confirmation_requires_typed_bearer_and_rejects_legacy_json_tokens() {
    let server = Fixture::server(Fixture::di(None));
    let challenge =
        compallm_core::auth::EmailOtpChallenge::new("james@midwess.com", SECRET).unwrap();
    let tokens = compallm_core::auth::UserTokens::new(
        &compallm_core::auth::Session::new(42, 7, 84),
        compallm_core::organization::OrganizationRole::Owner,
        SECRET,
        std::time::Duration::from_secs(1800),
        std::time::Duration::from_secs(3600),
    )
    .unwrap();
    for path in [
        "/api/auth/signup/email/confirmation/otp",
        "/api/auth/signin/email/confirmation/otp",
    ] {
        let response = server.post(path).json(&serde_json::json!({"token":challenge.access_token,"code":challenge.code,"delivery":"token"})).await;
        response.assert_status_unauthorized();
        assert!(
            response
                .json::<serde_json::Value>()
                .get("accessToken")
                .is_none()
        );
        for token in [&tokens.access_token, &tokens.refresh_token, "invalid"] {
            server
                .post(path)
                .add_header(AUTHORIZATION, format!("Bearer {token}"))
                .json(&serde_json::json!({"code":challenge.code,"delivery":"token"}))
                .await
                .assert_status_unauthorized();
        }
        server.post(path).add_header(AUTHORIZATION, format!("Bearer {}", challenge.access_token))
            .json(&serde_json::json!({"token":challenge.access_token,"code":challenge.code,"delivery":"token"})).await.assert_status_bad_request();
    }
}

#[tokio::test]
async fn confirmation_rejects_expired_or_tampered_otp_bearer_before_registration() {
    use compallm_core::auth::{EmailOtpAccessToken, EncryptedAccessToken};
    let server = Fixture::server(Fixture::di(None));
    let mut claims = EmailOtpAccessToken::new("james@midwess.com".into()).unwrap();
    let code = claims.code.clone();
    claims.validity.issued_at -= 601;
    claims.validity.expires_at -= 601;
    let encrypted = EncryptedAccessToken::new(SECRET)
        .unwrap()
        .encrypt(&AccessToken::EmailOtp(claims))
        .unwrap();
    let expired = AccessToken::Encrypted(encrypted)
        .encode(SECRET, std::time::Duration::from_secs(3600))
        .unwrap();
    let challenge =
        compallm_core::auth::EmailOtpChallenge::new("james@midwess.com", SECRET).unwrap();
    let mut parts: Vec<_> = challenge
        .access_token
        .split('.')
        .map(str::to_owned)
        .collect();
    parts[2] = "invalid-signature".into();
    let tampered = parts.join(".");
    for path in [
        "/api/auth/signup/email/confirmation/otp",
        "/api/auth/signin/email/confirmation/otp",
    ] {
        for token in [&expired, &tampered] {
            let response = server
                .post(path)
                .add_header(AUTHORIZATION, format!("Bearer {token}"))
                .json(&serde_json::json!({"code":code,"delivery":"token"}))
                .await;
            response.assert_status_unauthorized();
            assert!(response.headers().get("set-cookie").is_none());
            assert!(
                response
                    .json::<serde_json::Value>()
                    .get("accessToken")
                    .is_none()
            );
        }
    }
}

#[tokio::test]
async fn cookie_otp_flow_works_without_a_refresh_cookie_or_exposing_the_challenge() {
    compallm_postgres::init_id_generator("api-tests").unwrap();
    for signin in [false, true] {
        for secure in [false, true] {
            let mailbox = Mailbox::default();
            let messages = mailbox.0.clone();
            let mut settings = Fixture::settings();
            settings.cookie_secure = secure;
            settings.public_origin = "https://app.example.com".into();
            let server = Fixture::server(Fixture::di_with_signup_checks(
                Some(Box::new(mailbox)),
                signin,
                settings,
                usize::from(!signin),
            ));
            let path = if signin {
                "/api/auth/signin/email"
            } else {
                "/api/auth/signup/email"
            };
            let response = server
                .post(path)
                .add_header(axum::http::header::ORIGIN, "https://app.example.com")
                .json(&serde_json::json!({"email":" James@MIDWESS.COM "}))
                .await;
            response.assert_status_ok();
            assert_eq!(response.json::<serde_json::Value>(), serde_json::json!({}));
            let names = if secure {
                ["__Host-access", "__Host-refresh"]
            } else {
                ["compallm-access", "compallm-refresh"]
            };
            let access = response.cookie(names[0]);
            assert_eq!(access.http_only(), Some(true));
            assert_eq!(access.secure().unwrap_or(false), secure);
            assert_eq!(access.path(), Some("/"));
            assert_eq!(access.max_age().unwrap().whole_seconds(), 600);
            assert_eq!(
                response.cookie(names[1]).max_age().unwrap().whole_seconds(),
                0
            );
            let claims =
                EmailOtpAccessToken::try_from(AccessToken::decode(access.value(), SECRET).unwrap())
                    .unwrap();
            assert!(
                messages.lock().unwrap()[0]
                    .text
                    .plain_text()
                    .contains(&claims.code)
            );
            // Only the access cookie exists; no Bearer header or refresh token is needed.
            server
                .get("/api/auth/me")
                .add_header(
                    axum::http::header::COOKIE,
                    format!("{}={}", names[0], access.value()),
                )
                .await
                .assert_status_unauthorized();
            let confirmed = server
                .post(&format!("{path}/confirmation/otp"))
                .add_header(
                    axum::http::header::COOKIE,
                    format!("{}={}", names[0], access.value()),
                )
                .add_header(axum::http::header::ORIGIN, "https://app.example.com")
                .json(&serde_json::json!({"code":claims.code}))
                .await;
            confirmed.assert_status(if signin {
                axum::http::StatusCode::OK
            } else {
                axum::http::StatusCode::CREATED
            });
            assert_eq!(confirmed.json::<serde_json::Value>(), serde_json::json!({}));
            assert!(matches!(
                AccessToken::decode(confirmed.cookie(names[0]).value(), SECRET).unwrap(),
                AccessToken::User(_)
            ));
            assert!(matches!(
                AccessToken::decode(confirmed.cookie(names[1]).value(), SECRET).unwrap(),
                AccessToken::Refresh(_)
            ));
        }
    }
}

#[tokio::test]
async fn otp_cookie_delivery_requires_origin_and_rejects_misspelled_delivery() {
    let mailbox = Mailbox::default();
    let messages = mailbox.0.clone();
    let server = Fixture::server(Fixture::di(Some(Box::new(mailbox))));
    for path in ["/api/auth/signup/email", "/api/auth/signin/email"] {
        let response = server
            .post(path)
            .json(&serde_json::json!({"email":"james@example.com","delivery":"cookie"}))
            .await;
        response.assert_status_forbidden();
        assert!(response.headers().get("set-cookie").is_none());
        assert!(response.json::<serde_json::Value>().get("token").is_none());
        server
            .post(path)
            .json(&serde_json::json!({"email":"james@example.com","deliver":"token"}))
            .await
            .assert_status_bad_request();
    }
    assert!(messages.lock().unwrap().is_empty());
}

#[tokio::test]
async fn failed_cookie_code_request_keeps_the_existing_challenge() {
    let server = Fixture::server(Fixture::di(Some(Box::new(ThrottledEmail))));
    for path in ["/api/auth/signup/email", "/api/auth/signin/email"] {
        let response = server
            .post(path)
            .add_header(axum::http::header::ORIGIN, "http://localhost:3000")
            .add_header(axum::http::header::HOST, "localhost:3000")
            .json(&serde_json::json!({"email":"james@example.com","delivery":"cookie"}))
            .await;
        response.assert_status(axum::http::StatusCode::TOO_MANY_REQUESTS);
        assert!(response.headers().get("set-cookie").is_none());
        assert!(response.json::<serde_json::Value>().get("token").is_none());
    }
}

#[tokio::test]
async fn registered_email_is_rejected_before_sending_or_issuing_signup_credentials() {
    for delivery in ["cookie", "token"] {
        let mailbox = Mailbox::default();
        let messages = mailbox.0.clone();
        // The existing-user result must be consumed by the signup preflight lookup.
        let server = Fixture::server(Fixture::di_for(
            Some(Box::new(mailbox)),
            true,
            Fixture::settings(),
        ));
        let response = server
            .post("/api/auth/signup/email")
            .add_header(axum::http::header::ORIGIN, "http://127.0.0.1:3000")
            .add_header(axum::http::header::HOST, "127.0.0.1:3000")
            .json(&serde_json::json!({"email":" James@MIDWESS.COM ","delivery":delivery}))
            .await;
        response.assert_status(axum::http::StatusCode::CONFLICT);
        assert_eq!(
            response.json::<serde_json::Value>(),
            serde_json::json!({"message":"Email is already registered. Please sign in instead."})
        );
        assert!(response.headers().get("set-cookie").is_none());
        assert!(messages.lock().unwrap().is_empty());
    }
}

#[tokio::test]
async fn failed_signup_lookup_does_not_send_mail_or_issue_credentials() {
    let mailbox = Mailbox::default();
    let messages = mailbox.0.clone();
    let db = MockDatabase::new(DbBackend::Postgres)
        .append_query_errors([sea_orm::DbErr::Custom("private lookup failure".into())])
        .into_connection();
    let di: &'static Di = Box::leak(Box::new(Di::new(
        DbContext::new(Arc::new(PostgresConnection::new(Arc::new(db)))),
        Fixture::settings(),
        Some(Box::new(mailbox)),
    )));
    di.initialize_services().unwrap();
    let response = Fixture::server(di)
        .post("/api/auth/signup/email")
        .json(&serde_json::json!({"email":"james@example.com","delivery":"token"}))
        .await;
    response.assert_status_internal_server_error();
    assert!(response.headers().get("set-cookie").is_none());
    assert!(response.json::<serde_json::Value>().get("token").is_none());
    assert!(!response.text().contains("private lookup failure"));
    assert!(messages.lock().unwrap().is_empty());
}
