use crate::{
    controllers::auth::delivery::TokenDeliverySettings,
    di::{AuthSettings, Di},
    error::ApiError,
    middleware::authorization::{Authorization, RequireToken},
};
use axum::{
    Router,
    extract::{Path, Request},
    http::{
        StatusCode,
        header::{AUTHORIZATION, COOKIE, ORIGIN},
    },
    middleware::Next,
    routing::{get, post},
};
use axum_test::TestServer;
use compallm_core::{
    auth::{
        AccessToken, EmailOtpAccessToken, EmailOtpChallenge, RefreshToken, Session,
        UserAccessToken, UserTokens,
    },
    organization::OrganizationRole,
    persistent::DbContext,
};
use compallm_postgres::{
    OrganizationMemberSchema, OrganizationRoleSchema, UserSchema, UserSessionSchema,
    UserStatusSchema,
    postgres_connection::PostgresConnection,
    sea_orm::{DatabaseConnection, DbBackend, DbErr, IntoMockRow, MockDatabase},
};
use std::{collections::BTreeMap, sync::Arc, time::Duration};

const SECRET: &str = "test-signing-secret-at-least-32-bytes";

#[derive(Clone, Copy)]
enum RefreshState {
    Valid,
    Missing,
    Revoked,
    Changed,
    Disabled,
    NoMembership,
    DatabaseFailure,
    Member,
}

struct Fixture {
    di: &'static Di,
    db: Arc<DatabaseConnection>,
    session: Session,
}

impl Fixture {
    fn new(state: RefreshState) -> Self {
        Self::with_security(state, false)
    }

    fn with_security(state: RefreshState, secure: bool) -> Self {
        let session = Session::new(42, 7, 84);
        let at = sea_orm::prelude::DateTimeUtc::from_timestamp(session.created_at as i64, 0)
            .unwrap()
            .fixed_offset();
        let mut stored = UserSessionSchema {
            id: 7,
            user_id: 42,
            organization_id: 84,
            refresh_token_id: session.refresh_token_id.clone(),
            created_at: at,
            revoked_at: None,
        };
        if matches!(state, RefreshState::Revoked) {
            stored.revoked_at = Some(at);
        }
        if matches!(state, RefreshState::Changed) {
            stored.refresh_token_id = "B".repeat(43);
        }
        let user = UserSchema {
            id: 42,
            email: "james@midwess.com".into(),
            display_name: "james".into(),
            status: UserStatusSchema::Active,
            email_verified_at: at,
            last_login_at: None,
            created_at: at,
            updated_at: at,
        };
        let mut user: BTreeMap<_, _> = user.into_mock_row().into_column_value_tuples().collect();
        user.insert(
            "status".into(),
            if matches!(state, RefreshState::Disabled) {
                "disabled"
            } else {
                "active"
            }
            .into(),
        );
        let member = OrganizationMemberSchema {
            organization_id: 84,
            user_id: 42,
            role: OrganizationRoleSchema::Owner,
            created_at: at,
        };
        let mut member: BTreeMap<_, _> =
            member.into_mock_row().into_column_value_tuples().collect();
        member.insert(
            "role".into(),
            if matches!(state, RefreshState::Member) {
                "member"
            } else {
                "owner"
            }
            .into(),
        );
        let db = MockDatabase::new(DbBackend::Postgres);
        let db = if matches!(state, RefreshState::DatabaseFailure) {
            db.append_query_errors([DbErr::Custom("private database details".into())])
        } else {
            db.append_query_results([if matches!(state, RefreshState::Missing) {
                vec![]
            } else {
                vec![stored]
            }])
            .append_query_results([vec![user]])
            .append_query_results([if matches!(state, RefreshState::NoMembership) {
                vec![]
            } else {
                vec![member]
            }])
        };
        let db = Arc::new(db.into_connection());
        let di = Box::leak(Box::new(Di::new(
            DbContext::new(Arc::new(PostgresConnection::new(db.clone()))),
            AuthSettings {
                jwt_secret: SECRET.into(),
                user_token_ttl_seconds: 900,
                refresh_token_ttl_seconds: 2_592_000,
                cookie_secure: secure,
                public_origin: "https://app.example.com".into(),
            },
            None,
        )));
        di.initialize_services().unwrap();
        Self { di, db, session }
    }

    fn server(&self) -> TestServer {
        let mut router = Router::new();
        self.di.inject(&mut router);
        TestServer::new(router).unwrap()
    }

    fn tokens(&self) -> UserTokens {
        UserTokens::new(
            &self.session,
            OrganizationRole::Owner,
            SECRET,
            Duration::from_secs(900),
            Duration::from_secs(2_592_000),
        )
        .unwrap()
    }

    fn expired_access(&self) -> String {
        let mut claims = UserAccessToken::new(
            self.session.clone(),
            OrganizationRole::Owner,
            Duration::from_secs(900),
        )
        .unwrap();
        claims.validity.issued_at -= 901;
        claims.validity.expires_at -= 901;
        hero::token::jwt::create_jwt_token(
            AccessToken::User(claims),
            SECRET,
            Duration::from_secs(3600),
        )
        .unwrap()
    }

    fn log(&self) -> String {
        format!("{:?}", self.db.as_ref().clone().into_transaction_log())
    }

    fn guards_server(&self) -> TestServer {
        let settings: &'static TokenDeliverySettings = Box::leak(Box::new(TokenDeliverySettings {
            secure: false,
            public_origin: "https://app.example.com".into(),
            access_max_age_seconds: 900,
            refresh_max_age_seconds: 2_592_000,
        }));
        let auth = self.di.auth_service();
        let db = self.di.db_context();
        let router = Router::new().route("/admin/{organization}", post(Self::guarded))
            .route_layer(axum::middleware::from_fn(move |request: Request, next: Next| async move {
                Authorization::authenticate(auth, settings, request, next).await
            }))
            .layer(axum::middleware::from_fn(move |request: Request, next: Next| async move { db.scope(next.run(request)).await }));
        TestServer::new(router).unwrap()
    }

    fn typed_server(&self) -> TestServer {
        let settings: &'static TokenDeliverySettings = Box::leak(Box::new(TokenDeliverySettings {
            secure: false,
            public_origin: "https://app.example.com".into(),
            access_max_age_seconds: 900,
            refresh_max_age_seconds: 2_592_000,
        }));
        let auth = self.di.auth_service();
        let db = self.di.db_context();
        let router = Router::new()
            .route("/user", get(Self::requires_user))
            .route("/refresh", get(Self::requires_refresh))
            .route("/otp", get(Self::requires_otp))
            .route("/both", get(Self::requires_both))
            .route("/raw", get(Self::raw_extensions))
            .route_layer(axum::middleware::from_fn(move |request: Request, next: Next| async move {
                Authorization::authenticate(auth, settings, request, next).await
            }))
            .layer(axum::middleware::from_fn(move |request: Request, next: Next| async move {
                db.scope(next.run(request)).await
            }));
        TestServer::new(router).unwrap()
    }

    async fn requires_user(_: RequireToken<UserAccessToken>) -> StatusCode {
        StatusCode::NO_CONTENT
    }
    async fn requires_refresh(_: RequireToken<RefreshToken>) -> StatusCode {
        StatusCode::NO_CONTENT
    }
    async fn requires_otp(_: RequireToken<EmailOtpAccessToken>) -> StatusCode {
        StatusCode::NO_CONTENT
    }
    async fn requires_both(
        _: RequireToken<UserAccessToken>,
        _: RequireToken<RefreshToken>,
    ) -> StatusCode {
        StatusCode::NO_CONTENT
    }
    async fn raw_extensions(request: Request) -> axum::Json<serde_json::Value> {
        axum::Json(serde_json::json!({
            "user":request.extensions().get::<UserAccessToken>().is_some(),
            "refresh":request.extensions().get::<RefreshToken>().is_some(),
            "otp":request.extensions().get::<EmailOtpAccessToken>().is_some(),
        }))
    }

    async fn guarded(
        user: RequireToken<UserAccessToken>,
        Path(organization): Path<u64>,
    ) -> Result<StatusCode, ApiError> {
        user.require_organization(organization)?;
        user.require_roles(&[OrganizationRole::Owner, OrganizationRole::Admin])?;
        Ok(StatusCode::NO_CONTENT)
    }
}

#[tokio::test]
async fn bearer_and_cookie_access_populate_typed_identity_without_database_reads() {
    let fixture = Fixture::new(RefreshState::Valid);
    let server = fixture.server();
    let tokens = fixture.tokens();
    for (name, value) in [
        (AUTHORIZATION, format!("bEaReR {}", tokens.access_token)),
        (COOKIE, format!("compallm-access={}", tokens.access_token)),
    ] {
        let response = server.get("/api/auth/me").add_header(name, value).await;
        response.assert_status_ok();
        assert_eq!(
            response.json::<serde_json::Value>(),
            serde_json::json!({"sessionId":7,"userId":42,"organizationId":84,"organizationRole":"owner"})
        );
        assert_eq!(response.header("cache-control"), "no-store");
        assert!(response.headers().get("set-cookie").is_none());
        assert!(!response.text().contains("refresh_token_id"));
    }
    assert_eq!(fixture.log(), "[]");
}

#[tokio::test]
async fn public_routes_stay_public_but_missing_credentials_are_rejected() {
    let fixture = Fixture::new(RefreshState::Valid);
    let server = fixture.server();
    let response = server.get("/api/auth/me").await;
    response.assert_status_unauthorized();
    assert_eq!(response.header("www-authenticate"), "Bearer");
    server
        .post("/api/auth/signin/email")
        .add_header(AUTHORIZATION, "Bearer stale")
        .json(&serde_json::json!({"delivery":"token","email":"james@example.com"}))
        .await
        .assert_status_service_unavailable();
    assert_eq!(fixture.log(), "[]");
}

#[tokio::test]
async fn explicit_bearer_never_falls_back_to_valid_cookie_credentials() {
    let fixture = Fixture::new(RefreshState::Valid);
    let server = fixture.server();
    let tokens = fixture.tokens();
    let cookies = format!(
        "compallm-access={}; compallm-refresh={}",
        tokens.access_token, tokens.refresh_token
    );
    for value in [
        "Basic abc".to_owned(),
        "Bearer".to_owned(),
        "Bearer invalid".to_owned(),
        format!("Bearer {} extra", tokens.access_token),
        format!("Bearer {}", fixture.expired_access()),
        format!("Bearer {}", tokens.refresh_token),
    ] {
        let response = server
            .get("/api/auth/me")
            .add_header(AUTHORIZATION, value)
            .add_header(COOKIE, &cookies)
            .await;
        response.assert_status_unauthorized();
        assert!(response.headers().get("set-cookie").is_none());
    }
    let response = server
        .get("/api/auth/me")
        .add_header(AUTHORIZATION, format!("Bearer {}", tokens.access_token))
        .add_header(AUTHORIZATION, "Bearer invalid")
        .await;
    response.assert_status_unauthorized();
    assert_eq!(fixture.log(), "[]");
}

#[tokio::test]
async fn bearer_identity_wins_when_cookie_belongs_to_another_user() {
    let fixture = Fixture::new(RefreshState::Valid);
    let other = UserTokens::new(
        &Session::new(99, 9, 90),
        OrganizationRole::Member,
        SECRET,
        Duration::from_secs(900),
        Duration::from_secs(3600),
    )
    .unwrap();
    let response = fixture
        .server()
        .get("/api/auth/me")
        .add_header(
            AUTHORIZATION,
            format!("Bearer {}", fixture.tokens().access_token),
        )
        .add_header(COOKIE, format!("compallm-access={}", other.access_token))
        .await;
    response.assert_status_ok();
    assert_eq!(response.json::<serde_json::Value>()["userId"], 42);
    assert_eq!(fixture.log(), "[]");
}

#[tokio::test]
async fn missing_or_expired_access_is_renewed_and_current_request_succeeds() {
    for expired in [false, true] {
        let fixture = Fixture::new(RefreshState::Valid);
        let tokens = fixture.tokens();
        let cookies = if expired {
            format!(
                "compallm-access={}; compallm-refresh={}",
                fixture.expired_access(),
                tokens.refresh_token
            )
        } else {
            format!("compallm-refresh={}", tokens.refresh_token)
        };
        let response = fixture
            .server()
            .get("/api/auth/me")
            .add_header(COOKIE, cookies)
            .add_header(ORIGIN, "https://app.example.com")
            .await;
        response.assert_status_ok();
        assert_eq!(response.json::<serde_json::Value>()["userId"], 42);
        assert_eq!(response.headers().get_all("set-cookie").iter().count(), 2);
        assert!(matches!(
            AccessToken::decode(response.cookie("compallm-access").value(), SECRET).unwrap(),
            AccessToken::User(_)
        ));
        assert!(matches!(
            AccessToken::decode(response.cookie("compallm-refresh").value(), SECRET).unwrap(),
            AccessToken::Refresh(_)
        ));
        let log = fixture.log();
        assert!(log.contains("SELECT"));
        assert!(!log.contains("INSERT") && !log.contains("UPDATE") && !log.contains("BEGIN"));
    }
}

#[tokio::test]
async fn browser_same_origin_get_without_origin_can_refresh_but_cross_site_cannot() {
    let fixture = Fixture::new(RefreshState::Valid);
    let response = fixture
        .server()
        .get("/api/auth/me")
        .add_header(
            COOKIE,
            format!("compallm-refresh={}", fixture.tokens().refresh_token),
        )
        .add_header("sec-fetch-site", "same-origin")
        .await;
    response.assert_status_ok();
    for site in [None, Some("cross-site"), Some("same-site")] {
        let fixture = Fixture::new(RefreshState::Valid);
        let server = fixture.server();
        let mut request = server.get("/api/auth/me").add_header(
            COOKIE,
            format!("compallm-refresh={}", fixture.tokens().refresh_token),
        );
        if let Some(site) = site {
            request = request.add_header("sec-fetch-site", site);
        }
        let response = request.await;
        response.assert_status_forbidden();
        assert_eq!(
            response
                .cookie("compallm-refresh")
                .max_age()
                .unwrap()
                .whole_seconds(),
            0
        );
        assert_eq!(fixture.log(), "[]");
    }
}

#[tokio::test]
async fn expired_outer_jwt_envelope_also_triggers_cookie_renewal() {
    let fixture = Fixture::new(RefreshState::Valid);
    let claims = UserAccessToken::new(
        fixture.session.clone(),
        OrganizationRole::Owner,
        Duration::from_secs(900),
    )
    .unwrap();
    let now = compallm_utils::time::unix_timestamp();
    let jwt = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &serde_json::json!({"exp":now-120,"iat":now-1000,"data":AccessToken::User(claims)}),
        &jsonwebtoken::EncodingKey::from_secret(SECRET.as_bytes()),
    )
    .unwrap();
    let response = fixture
        .server()
        .get("/api/auth/me")
        .add_header(
            COOKIE,
            format!(
                "compallm-access={jwt}; compallm-refresh={}",
                fixture.tokens().refresh_token
            ),
        )
        .add_header(ORIGIN, "https://app.example.com")
        .await;
    response.assert_status_ok();
    assert_eq!(response.headers().get_all("set-cookie").iter().count(), 2);
}

#[tokio::test]
async fn invalid_refresh_state_clears_cookies_without_rewriting_or_revoking_sessions() {
    for state in [
        RefreshState::Missing,
        RefreshState::Revoked,
        RefreshState::Changed,
        RefreshState::Disabled,
        RefreshState::NoMembership,
    ] {
        let fixture = Fixture::new(state);
        let response = fixture
            .server()
            .get("/api/auth/me")
            .add_header(
                COOKIE,
                format!("compallm-refresh={}", fixture.tokens().refresh_token),
            )
            .add_header(ORIGIN, "https://app.example.com")
            .await;
        assert!(matches!(
            response.status_code(),
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN
        ));
        for name in ["compallm-access", "compallm-refresh"] {
            let cookie = response.cookie(name);
            assert_eq!(cookie.value(), "");
            assert_eq!(cookie.max_age().unwrap().whole_seconds(), 0);
        }
        let log = fixture.log();
        assert!(!log.contains("UPDATE") && !log.contains("INSERT"));
    }
}

#[tokio::test]
async fn database_failure_clears_browser_credentials_and_hides_internal_details() {
    let fixture = Fixture::new(RefreshState::DatabaseFailure);
    let response = fixture
        .server()
        .get("/api/auth/me")
        .add_header(
            COOKIE,
            format!("compallm-refresh={}", fixture.tokens().refresh_token),
        )
        .add_header(ORIGIN, "https://app.example.com")
        .await;
    response.assert_status_internal_server_error();
    assert_eq!(
        response
            .cookie("compallm-access")
            .max_age()
            .unwrap()
            .whole_seconds(),
        0
    );
    assert_eq!(
        response
            .cookie("compallm-refresh")
            .max_age()
            .unwrap()
            .whole_seconds(),
        0
    );
    assert!(!response.text().contains("private database details"));
}

#[tokio::test]
async fn wrong_token_variants_and_duplicate_cookies_cannot_authenticate() {
    let fixture = Fixture::new(RefreshState::Valid);
    let server = fixture.server();
    let tokens = fixture.tokens();
    let otp = EmailOtpChallenge::new("james@example.com", SECRET).unwrap();
    for value in [
        tokens.refresh_token.clone(),
        otp.access_token,
        "invalid-signature".into(),
    ] {
        let response = server
            .get("/api/auth/me")
            .add_header(
                COOKIE,
                format!(
                    "compallm-access={value}; compallm-refresh={}",
                    tokens.refresh_token
                ),
            )
            .await;
        response.assert_status_unauthorized();
    }
    server
        .get("/api/auth/me")
        .add_header(
            COOKIE,
            format!(
                "compallm-access={}; compallm-access={}",
                tokens.access_token, tokens.access_token
            ),
        )
        .await
        .assert_status_unauthorized();
    assert_eq!(fixture.log(), "[]");
}

#[tokio::test]
async fn mutation_origin_and_role_and_organization_guards_are_enforced() {
    let fixture = Fixture::new(RefreshState::Valid);
    let server = fixture.guards_server();
    let tokens = fixture.tokens();
    for origin in [None, Some("https://evil.example")] {
        let mut request = server
            .post("/admin/84")
            .add_header(COOKIE, format!("compallm-access={}", tokens.access_token));
        if let Some(origin) = origin {
            request = request.add_header(ORIGIN, origin);
        }
        request.await.assert_status_forbidden();
    }
    server
        .post("/admin/84")
        .add_header(COOKIE, format!("compallm-access={}", tokens.access_token))
        .add_header(ORIGIN, "https://app.example.com")
        .await
        .assert_status(StatusCode::NO_CONTENT);
    server
        .post("/admin/84")
        .add_header(AUTHORIZATION, format!("Bearer {}", tokens.access_token))
        .await
        .assert_status(StatusCode::NO_CONTENT);
    server
        .post("/admin/99")
        .add_header(AUTHORIZATION, format!("Bearer {}", tokens.access_token))
        .await
        .assert_status_forbidden();
    let member = UserTokens::new(
        &fixture.session,
        OrganizationRole::Member,
        SECRET,
        Duration::from_secs(900),
        Duration::from_secs(3600),
    )
    .unwrap();
    server
        .post("/admin/84")
        .add_header(AUTHORIZATION, format!("Bearer {}", member.access_token))
        .await
        .assert_status_forbidden();
    assert_eq!(fixture.log(), "[]");
}

#[tokio::test]
async fn required_token_extractor_returns_401_when_middleware_has_not_run() {
    let server =
        TestServer::new(Router::new().route("/admin/{organization}", post(Fixture::guarded)))
            .unwrap();
    server.post("/admin/84").await.assert_status_unauthorized();
}

#[tokio::test]
async fn secure_cookie_authentication_uses_host_prefixed_names_and_refreshes_securely() {
    let fixture = Fixture::with_security(RefreshState::Valid, true);
    let server = fixture.server();
    let response = server
        .get("/api/auth/me")
        .add_header(
            COOKIE,
            format!("__Host-access={}", fixture.tokens().access_token),
        )
        .await;
    response.assert_status_ok();
    let response = server
        .get("/api/auth/me")
        .add_header(
            COOKIE,
            format!("__Host-refresh={}", fixture.tokens().refresh_token),
        )
        .add_header(ORIGIN, "https://app.example.com")
        .await;
    response.assert_status_ok();
    for name in ["__Host-access", "__Host-refresh"] {
        let cookie = response.cookie(name);
        assert_eq!(cookie.secure(), Some(true));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.domain(), None);
    }
}

#[tokio::test]
async fn refreshed_request_uses_current_membership_role() {
    let fixture = Fixture::new(RefreshState::Member);
    let response = fixture
        .server()
        .get("/api/auth/me")
        .add_header(
            COOKIE,
            format!("compallm-refresh={}", fixture.tokens().refresh_token),
        )
        .add_header(ORIGIN, "https://app.example.com")
        .await;
    response.assert_status_ok();
    assert_eq!(
        response.json::<serde_json::Value>()["organizationRole"],
        "member"
    );
}

#[tokio::test]
async fn expired_invalid_or_wrong_variant_refresh_cookie_does_not_authenticate() {
    let fixture = Fixture::new(RefreshState::Valid);
    let mut expired =
        compallm_core::auth::RefreshToken::from_session(&fixture.session, Duration::from_secs(60))
            .unwrap();
    expired.validity.issued_at -= 61;
    expired.validity.expires_at -= 61;
    let expired = hero::token::jwt::create_jwt_token(
        AccessToken::Refresh(expired),
        SECRET,
        Duration::from_secs(3600),
    )
    .unwrap();
    for refresh in [expired, "invalid".into(), fixture.tokens().access_token] {
        let response = fixture
            .server()
            .get("/api/auth/me")
            .add_header(COOKIE, format!("compallm-refresh={refresh}"))
            .add_header(ORIGIN, "https://app.example.com")
            .await;
        response.assert_status_unauthorized();
        assert_eq!(
            response
                .cookie("compallm-refresh")
                .max_age()
                .unwrap()
                .whole_seconds(),
            0
        );
    }
    assert_eq!(fixture.log(), "[]");
}

#[tokio::test]
async fn bearer_inserts_only_its_verified_concrete_token_type() {
    let fixture = Fixture::new(RefreshState::Valid);
    let server = fixture.typed_server();
    let tokens = fixture.tokens();
    let otp = EmailOtpChallenge::new("james@example.com", SECRET).unwrap();
    for (token, expected) in [
        (tokens.access_token, [true, false, false]),
        (tokens.refresh_token, [false, true, false]),
        (otp.access_token, [false, false, true]),
    ] {
        for (index, path) in ["/user", "/refresh", "/otp"].iter().enumerate() {
            let response = server
                .get(path)
                .add_header(AUTHORIZATION, format!("Bearer {token}"))
                .await;
            response.assert_status(if expected[index] {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::UNAUTHORIZED
            });
        }
        let response = server
            .get("/raw")
            .add_header(AUTHORIZATION, format!("Bearer {token}"))
            .await;
        response.assert_status_ok();
        assert_eq!(
            response.json::<serde_json::Value>(),
            serde_json::json!({"user":expected[0],"refresh":expected[1],"otp":expected[2]})
        );
    }
    assert_eq!(fixture.log(), "[]");
}

#[tokio::test]
async fn cookie_extensions_include_only_matching_valid_refresh_claims() {
    let fixture = Fixture::new(RefreshState::Valid);
    let server = fixture.typed_server();
    let tokens = fixture.tokens();
    let other = UserTokens::new(
        &Session::new(99, 9, 90),
        OrganizationRole::Owner,
        SECRET,
        Duration::from_secs(900),
        Duration::from_secs(3600),
    )
    .unwrap();
    let replaced = UserTokens::new(
        &Session::new(42, 7, 84),
        OrganizationRole::Owner,
        SECRET,
        Duration::from_secs(900),
        Duration::from_secs(3600),
    )
    .unwrap();
    for (refresh, present) in [
        (tokens.refresh_token.clone(), true),
        (other.refresh_token, false),
        (replaced.refresh_token, false),
        ("invalid".into(), false),
        (tokens.access_token.clone(), false),
    ] {
        let cookies = format!(
            "compallm-access={}; compallm-refresh={refresh}",
            tokens.access_token
        );
        let response = server.get("/raw").add_header(COOKIE, &cookies).await;
        response.assert_status_ok();
        assert_eq!(
            response.json::<serde_json::Value>(),
            serde_json::json!({"user":true,"refresh":present,"otp":false})
        );
        server
            .get("/both")
            .add_header(COOKIE, cookies)
            .await
            .assert_status(if present {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::UNAUTHORIZED
            });
    }
    assert_eq!(fixture.log(), "[]");
}

#[tokio::test]
async fn renewal_inserts_both_new_token_types_for_the_current_request() {
    let fixture = Fixture::new(RefreshState::Valid);
    let response = fixture
        .typed_server()
        .get("/both")
        .add_header(
            COOKIE,
            format!("compallm-refresh={}", fixture.tokens().refresh_token),
        )
        .add_header(ORIGIN, "https://app.example.com")
        .await;
    response.assert_status(StatusCode::NO_CONTENT);
    assert_eq!(response.headers().get_all("set-cookie").iter().count(), 2);
}

#[tokio::test]
async fn explicit_bearer_does_not_mix_with_cookie_refresh_claims() {
    let fixture = Fixture::new(RefreshState::Valid);
    let tokens = fixture.tokens();
    let response = fixture
        .typed_server()
        .get("/raw")
        .add_header(AUTHORIZATION, format!("Bearer {}", tokens.access_token))
        .add_header(COOKIE, format!("compallm-refresh={}", tokens.refresh_token))
        .await;
    response.assert_status_ok();
    assert_eq!(
        response.json::<serde_json::Value>(),
        serde_json::json!({"user":true,"refresh":false,"otp":false})
    );
    assert_eq!(fixture.log(), "[]");
}

#[tokio::test]
async fn selected_credentials_cannot_inherit_token_extensions_from_another_layer() {
    let fixture = Fixture::new(RefreshState::Valid);
    let auth = fixture.di.auth_service();
    let settings: &'static TokenDeliverySettings = Box::leak(Box::new(TokenDeliverySettings {
        secure: false,
        public_origin: "https://app.example.com".into(),
        access_max_age_seconds: 900,
        refresh_max_age_seconds: 2_592_000,
    }));
    let inherited = UserAccessToken::new(
        fixture.session.clone(),
        OrganizationRole::Owner,
        Duration::from_secs(900),
    )
    .unwrap();
    let router = Router::new()
        .route("/user", get(Fixture::requires_user))
        .route_layer(axum::middleware::from_fn(
            move |request: Request, next: Next| async move {
                Authorization::authenticate(auth, settings, request, next).await
            },
        ))
        .layer(axum::Extension(inherited));
    let server = TestServer::new(router).unwrap();
    server
        .get("/user")
        .add_header(
            AUTHORIZATION,
            format!("Bearer {}", fixture.tokens().refresh_token),
        )
        .await
        .assert_status_unauthorized();
}

#[tokio::test]
async fn otp_access_cookie_populates_only_the_otp_type_without_refresh_or_database_work() {
    let fixture = Fixture::new(RefreshState::Valid);
    let server = fixture.typed_server();
    let challenge = EmailOtpChallenge::new("james@example.com", SECRET).unwrap();
    let cookies = format!("compallm-access={}", challenge.access_token);
    server
        .get("/otp")
        .add_header(COOKIE, &cookies)
        .await
        .assert_status(StatusCode::NO_CONTENT);
    server
        .get("/user")
        .add_header(COOKIE, &cookies)
        .await
        .assert_status_unauthorized();
    server
        .get("/refresh")
        .add_header(COOKIE, &cookies)
        .await
        .assert_status_unauthorized();
    let response = server.get("/raw").add_header(COOKIE, &cookies).await;
    response.assert_status_ok();
    assert_eq!(
        response.json::<serde_json::Value>(),
        serde_json::json!({"user":false,"refresh":false,"otp":true})
    );
    assert!(response.headers().get("set-cookie").is_none());
    assert_eq!(fixture.log(), "[]");
}
