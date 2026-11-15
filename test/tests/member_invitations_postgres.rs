use compallm_api::di::{AuthSettings, Di, GraphqlSettings};
use compallm_core::{
    authentication::{
        AuthError, EmailOtpAccessToken, Session, UserAccessToken, token::InvitationToken,
    },
    authorization::{MemberPermission, PolicySnapshot},
    persistent::{
        DbContext, invitation::InvitationRepository as _,
        organization::OrganizationRepository as _, user::UserRepository as _,
    },
};
use compallm_postgres::{
    postgres_connection::PostgresConnection,
    repository::{
        invitation::InvitationRepository, organization::OrganizationRepository,
        user::UserRepository,
    },
};
use sea_orm::{Database, EntityTrait, ModelTrait, PaginatorTrait};
use std::{sync::Arc, time::Duration};

const ORG: u64 = 7_000_000_000_000_000_000;
const OWNER: u64 = ORG + 100;
fn member_permission(member: &compallm_core::organization::OrganizationMember) -> MemberPermission {
    MemberPermission::try_from(
        UserAccessToken::new(
            Session {
                id: 1,
                user_id: member.user_id,
                organization_id: member.organization_id,
                authorization_version: 1,
                revoked_at: None,
            },
            PolicySnapshot::new(member).unwrap(),
            Duration::from_secs(1800),
        )
        .unwrap(),
    )
    .unwrap()
}
fn otp(claims: &InvitationToken) -> EmailOtpAccessToken {
    let mut otp = EmailOtpAccessToken::new(claims.email.clone()).unwrap();
    otp.invitation_id = Some(claims.invitation_id);
    otp
}

#[tokio::test]
#[ignore = "requires COMPALLM_TEST_DATABASE_URL with all migrations and debug seed applied"]
async fn invitations_accept_once_without_personal_org_preserve_grants_and_enforce_rls()
-> anyhow::Result<()> {
    let pool = Arc::new(Database::connect(std::env::var("COMPALLM_TEST_DATABASE_URL")?).await?);
    let root = DbContext::new(Arc::new(PostgresConnection::new(pool.clone())));
    DbContext::initialize_default(root.postgres().scoped())?;
    compallm_postgres::init_id_generator("invitation-integration")?;
    let di = Box::leak(Box::new(Di::new(
        root.clone(),
        AuthSettings {
            jwt_secret: "invitation-integration-secret-at-least-32".into(),
            user_token_ttl_seconds: 1800,
            refresh_token_ttl_seconds: 86400,
        },
        GraphqlSettings {
            cookie_secure: false,
            public_origin: "http://localhost".into(),
            ..Default::default()
        },
        Some(Box::new(TestMail)),
    )));
    di.initialize_services()?;
    let auth = di.auth_service();
    let invitations = di.invitation_service();
    let owner = root
        .scope(OrganizationRepository.find_membership(ORG, OWNER))
        .await?
        .unwrap()
        .0;
    let permission = member_permission(&owner);
    let suffix = compallm_utils::time::unix_timestamp();
    let email = format!("new-{suffix}@example.com");
    let mut issued = invitations
        .invitation_create(&permission, &format!("  {}  ", email.to_uppercase()), None)
        .await?;
    assert_eq!(issued.invitation.email, email);
    invitations
        .invitation_send(
            &mut issued,
            "http://localhost/api/invitations/open?token=copyable",
        )
        .await?;
    assert!(!issued.email_admitted);
    assert!(matches!(
        invitations
            .invitation_create(&permission, &email, None)
            .await,
        Err(AuthError::InvitationConflict)
    ));
    let claims = InvitationToken::try_from(auth.authenticate_token(&issued.token).await?)?;
    assert!(auth.authenticate_access(&issued.token).await.is_err());
    assert!(invitations.invitation_context(&claims).await?.signup);
    let mut tampered = issued.token.clone().into_bytes();
    tampered[20] ^= 1;
    assert!(
        auth.authenticate_token(std::str::from_utf8(&tampered)?)
            .await
            .is_err()
    );
    assert!(
        invitations
            .invitation_email(&claims, "wrong@example.com", true)
            .await
            .is_err()
    );
    let challenge = otp(&claims);
    assert!(matches!(
        invitations
            .invitation_confirm(&claims, &challenge, "wrong", true)
            .await,
        Err(AuthError::IncorrectCode)
    ));
    assert!(auth.signup(&challenge, &challenge.code).await.is_err());
    type Orgs = <compallm_postgres::OrganizationSchema as ModelTrait>::Entity;
    let org_count = Orgs::find().count(pool.as_ref()).await?;
    let (a, b) = tokio::join!(
        invitations.invitation_confirm(&claims, &challenge, &challenge.code, true),
        invitations.invitation_confirm(&claims, &challenge, &challenge.code, true)
    );
    assert_eq!(usize::from(a.is_ok()) + usize::from(b.is_ok()), 1);
    let tokens = a.or(b)?;
    let user_claims = auth.authenticate_access(&tokens.access_token).await?;
    assert_eq!(user_claims.session.organization_id, ORG);
    assert_eq!(Orgs::find().count(pool.as_ref()).await?, org_count);
    let user = root
        .scope(UserRepository.find_by_email(&email))
        .await?
        .unwrap();
    let membership = root
        .scope(OrganizationRepository.find_membership(ORG, user.id))
        .await?
        .unwrap()
        .0;
    assert_eq!(membership.effective_roles().take(1).count(), 1);
    let directory_permission = Arc::new(permission.clone());
    let directory = root
        .fresh()?
        .restrict(async move { Ok::<_, std::convert::Infallible>(directory_permission) })?;
    let info = directory
        .scope(OrganizationRepository.list_members(
            Box::new(compallm_query::Query::new()),
            compallm_query::Pagination {
                first: Some(100),
                ..Default::default()
            },
            vec![],
            "join-date",
            Some(user.id),
        ))
        .await?;
    assert_eq!(
        info.edges
            .iter()
            .find(|edge| edge.node.id == user.id)
            .unwrap()
            .node
            .joined_at(),
        hero::distributed_id::id_to_unix_timestamp(user.id) / 1000
    );
    assert_eq!(membership.role.name, "Member");
    use compallm_core::persistent::preference::user::UserPreferenceRepository as _;
    assert!(
        root.scope(
            compallm_postgres::repository::preference::user::UserPreferenceRepository.find(user.id)
        )
        .await?
        .is_some()
    );
    assert!(invitations.invitation_context(&claims).await.is_err());
    let accepted = root
        .scope(InvitationRepository.find(ORG, claims.invitation_id, false))
        .await?
        .unwrap();
    assert_eq!(accepted.accepted_by, Some(user.id));

    let other = root
        .scope(
            di.organization_creation_service()
                .create("Other-workspace", &user),
        )
        .await?;
    let other_membership = root
        .scope(OrganizationRepository.find_membership(other.id, user.id))
        .await?
        .unwrap()
        .0;
    let existing = invitations
        .invitation_create(
            &member_permission(&other_membership),
            "owner@compallm.com",
            None,
        )
        .await?;
    let existing_claims =
        InvitationToken::try_from(auth.authenticate_token(&existing.token).await?)?;
    let existing_otp = otp(&existing_claims);
    let tokens = invitations
        .invitation_confirm(&existing_claims, &existing_otp, &existing_otp.code, false)
        .await?;
    assert_eq!(
        auth.authenticate_access(&tokens.access_token)
            .await?
            .session
            .organization_id,
        other.id
    );
    let after = root
        .scope(OrganizationRepository.find_membership(ORG, OWNER))
        .await?
        .unwrap()
        .0;
    assert_eq!(after.role, owner.role);

    let rotate = invitations
        .invitation_create(&permission, &format!("rotate-{suffix}@example.com"), None)
        .await?;
    let old = InvitationToken::try_from(auth.authenticate_token(&rotate.token).await?)?;
    let fresh = invitations
        .invitation_resend(&permission, old.invitation_id)
        .await?;
    assert_ne!(fresh.invitation.id, old.invitation_id);
    assert!(invitations.invitation_context(&old).await.is_err());
    let fresh_claims = InvitationToken::try_from(auth.authenticate_token(&fresh.token).await?)?;
    invitations
        .invitation_revoke(&permission, fresh.invitation.id)
        .await?;
    assert!(invitations.invitation_context(&fresh_claims).await.is_err());

    let preserved = invitations
        .invitation_create(&permission, &format!("preserve-{suffix}@example.com"), None)
        .await?;
    let preserved_claims =
        InvitationToken::try_from(auth.authenticate_token(&preserved.token).await?)?;
    let preserved_user = root
        .scope(
            di.registration_service()
                .register_invited(&preserved_claims.email),
        )
        .await?;
    let mut with_grants = owner.clone();
    with_grants.user_id = preserved_user.id;
    root.scope(OrganizationRepository.add_member(with_grants.clone()))
        .await?;
    let preserved_otp = otp(&preserved_claims);
    invitations
        .invitation_confirm(
            &preserved_claims,
            &preserved_otp,
            &preserved_otp.code,
            false,
        )
        .await?;
    assert_eq!(
        root.scope(OrganizationRepository.find_membership(ORG, preserved_user.id))
            .await?
            .unwrap()
            .0
            .role,
        with_grants.role
    );

    let lost_permission = invitations
        .invitation_create(&permission, &format!("lost-{suffix}@example.com"), None)
        .await?;
    let lost_claims =
        InvitationToken::try_from(auth.authenticate_token(&lost_permission.token).await?)?;
    type Links = <compallm_postgres::OrganizationMemberSchema as ModelTrait>::Entity;
    type LinkColumn = <Links as EntityTrait>::Column;
    use sea_orm::{ColumnTrait, QueryFilter};
    Links::update_many()
        .col_expr(
            LinkColumn::RoleId,
            sea_orm::sea_query::Expr::value((ORG + 3) as i64),
        )
        .filter(LinkColumn::OrganizationId.eq(ORG as i64))
        .filter(LinkColumn::UserId.eq(OWNER as i64))
        .exec(pool.as_ref())
        .await?;
    let lost_otp = otp(&lost_claims);
    invitations
        .invitation_confirm(&lost_claims, &lost_otp, &lost_otp.code, true)
        .await?;
    let current_owner = root
        .scope(OrganizationRepository.find_membership(ORG, OWNER))
        .await?
        .unwrap()
        .0;
    root.scope(OrganizationRepository.save_member(&owner, &current_owner))
        .await?;

    let expired = invitations
        .invitation_create(&permission, &format!("expired-{suffix}@example.com"), None)
        .await?;
    let mut expired_claims =
        InvitationToken::try_from(auth.authenticate_token(&expired.token).await?)?;
    expired_claims.validity.expires_at = compallm_utils::time::unix_timestamp();
    expired_claims.validity.issued_at =
        expired_claims.validity.expires_at - InvitationToken::TTL_SECONDS;
    assert!(matches!(
        invitations.invitation_context(&expired_claims).await,
        Err(AuthError::ExpiredToken)
    ));
    let expired_otp = otp(&expired_claims);
    assert!(
        invitations
            .invitation_confirm(&expired_claims, &expired_otp, &expired_otp.code, true)
            .await
            .is_err()
    );

    let disabled = invitations
        .invitation_create(&permission, &format!("disabled-{suffix}@example.com"), None)
        .await?;
    let disabled_claims =
        InvitationToken::try_from(auth.authenticate_token(&disabled.token).await?)?;
    let disabled_user = root
        .scope(
            di.registration_service()
                .register_invited(&disabled_claims.email),
        )
        .await?;
    type Users = <compallm_postgres::UserSchema as ModelTrait>::Entity;
    type UserColumn = <Users as EntityTrait>::Column;
    Users::update_many()
        .col_expr(
            UserColumn::Status,
            sea_orm::sea_query::Expr::value(compallm_postgres::UserStatusSchema::Disabled),
        )
        .filter(UserColumn::Id.eq(disabled_user.id as i64))
        .exec(pool.as_ref())
        .await?;
    assert!(matches!(
        invitations.invitation_context(&disabled_claims).await,
        Err(AuthError::UserDisabled)
    ));
    let disabled_otp = otp(&disabled_claims);
    assert!(
        invitations
            .invitation_confirm(&disabled_claims, &disabled_otp, &disabled_otp.code, false)
            .await
            .is_err()
    );

    let disabled_org = invitations
        .invitation_create(
            &member_permission(&other_membership),
            &format!("disabled-org-{suffix}@example.com"),
            None,
        )
        .await?;
    let disabled_org_claims =
        InvitationToken::try_from(auth.authenticate_token(&disabled_org.token).await?)?;
    type OrgColumn = <Orgs as EntityTrait>::Column;
    Orgs::update_many()
        .col_expr(
            OrgColumn::Status,
            sea_orm::sea_query::Expr::value(compallm_postgres::OrganizationStatusSchema::Disabled),
        )
        .filter(OrgColumn::Id.eq(other.id as i64))
        .exec(pool.as_ref())
        .await?;
    assert!(
        invitations
            .invitation_context(&disabled_org_claims)
            .await
            .is_err()
    );
    let disabled_org_otp = otp(&disabled_org_claims);
    assert!(
        invitations
            .invitation_confirm(
                &disabled_org_claims,
                &disabled_org_otp,
                &disabled_org_otp.code,
                true
            )
            .await
            .is_err()
    );
    Orgs::update_many()
        .col_expr(
            OrgColumn::Status,
            sea_orm::sea_query::Expr::value(compallm_postgres::OrganizationStatusSchema::Active),
        )
        .filter(OrgColumn::Id.eq(other.id as i64))
        .exec(pool.as_ref())
        .await?;

    verify_browser_flow(
        di,
        &member_permission(&owner),
        &format!("browser-{suffix}@example.com"),
    )
    .await?;
    let batch_emails = vec![
        format!("batch-a-{suffix}@example.com"),
        format!("batch-b-{suffix}@example.com"),
    ];
    let batch = invitations
        .invitations_create(&member_permission(&owner), &batch_emails)
        .await?;
    assert_eq!(batch.len(), 2);
    let rollback_email = format!("batch-rollback-{suffix}@example.com");
    assert!(
        invitations
            .invitations_create(
                &member_permission(&owner),
                &[rollback_email.clone(), "owner@compallm.com".into()]
            )
            .await
            .is_err()
    );
    assert!(
        !root
            .scope(InvitationRepository.has_pending(
                ORG,
                &rollback_email,
                compallm_utils::time::unix_timestamp()
            ))
            .await?
    );
    let restricted = root.fresh()?.restrict({
        let permission = Arc::new(permission);
        async move { Ok::<_, std::convert::Infallible>(permission) }
    })?;
    let page = restricted
        .scope(InvitationRepository.list(
            Box::new(compallm_query::Query::new()),
            compallm_query::Pagination {
                first: Some(100),
                ..Default::default()
            },
            vec![],
            "owner",
        ))
        .await?;
    assert!(!page.edges.is_empty());
    assert!(
        page.edges
            .iter()
            .all(|edge| edge.node.organization_id == ORG)
    );
    assert!(
        restricted
            .scope(InvitationRepository.find(other.id, existing.invitation.id, false))
            .await?
            .is_none()
    );
    let denied = member_permission(&membership);
    assert!(
        invitations
            .invitation_create(&denied, "denied@example.com", None)
            .await
            .is_err()
    );
    let restricted = root.fresh()?.restrict({
        let denied = Arc::new(denied);
        async move { Ok::<_, std::convert::Infallible>(denied) }
    })?;
    assert!(
        restricted
            .scope(InvitationRepository.list(
                Box::new(compallm_query::Query::new()),
                compallm_query::Pagination {
                    first: Some(20),
                    ..Default::default()
                },
                vec![],
                "denied"
            ))
            .await?
            .edges
            .is_empty()
    );
    let forbidden = compallm_core::organization::invitation::Invitation {
        id: hero::distributed_id::try_gen_id().await?,
        organization_id: ORG,
        email: format!("rls-write-{suffix}@example.com"),
        inviter_id: user.id,
        status: compallm_core::organization::invitation::InvitationStatus::Pending,
        expires_at: compallm_utils::time::unix_timestamp() + 604800,
        accepted_at: None,
        accepted_by: None,
        revoked_at: None,
    };
    assert!(
        restricted
            .scope(InvitationRepository.create(forbidden))
            .await
            .is_err()
    );
    Ok(())
}

struct TestMail;
#[async_trait::async_trait]
impl compallm_core::mail::EmailService for TestMail {
    async fn send(
        &self,
        request: compallm_core::mail::EmailRequest,
    ) -> Result<(), compallm_core::mail::EmailError> {
        if matches!(
            request.text,
            compallm_core::mail::EmailTemplate::Invitation(_)
        ) {
            return Err(compallm_core::mail::EmailError::Unavailable);
        }
        Ok(())
    }
}

fn browser_router(di: &'static Di) -> axum::Router {
    use axum::{
        Extension, Json, Router,
        extract::State,
        middleware::{from_fn, from_fn_with_state},
        routing::{get, post},
    };
    use compallm_api::{
        graphql::{ApplicationSchema, GraphqlContext, endpoint},
        http::{
            authentication::{AuthenticationContext, CookieDelivery, UserContext},
            authorization::AuthorizationContext,
            middleware::{AuthenticationMiddleware, AuthorizationMiddleware},
        },
    };
    Router::new()
        .route(
            "/api/invitations/open",
            get(compallm_api::http::invitation::open).with_state(di),
        )
        .route(
            "/api/graphql",
            post(
                |State(schema): State<ApplicationSchema>,
                 Extension(di): Extension<&'static Di>,
                 Extension(user): Extension<UserContext>,
                 Extension(authentication): Extension<Arc<AuthenticationContext>>,
                 Extension(delivery): Extension<CookieDelivery>,
                 Extension(authorization): Extension<AuthorizationContext>,
                 request: Result<
                    Json<endpoint::GraphQLRequest>,
                    axum::extract::rejection::JsonRejection,
                >| async move {
                    let mut response = endpoint::handle(
                        schema,
                        GraphqlContext::new(di, user, authentication, delivery, authorization),
                        request,
                    )
                    .await;
                    response
                        .headers_mut()
                        .append("set-cookie", "unrelated=retained; Path=/".parse().unwrap());
                    response
                },
            )
            .with_state(di.graphql_schema().clone())
            .layer(Extension(di))
            .route_layer(from_fn(AuthorizationMiddleware::handle))
            .route_layer(from_fn_with_state(
                di.authentication().clone(),
                AuthenticationMiddleware::handle,
            )),
        )
        .layer(from_fn(compallm_api::http::invitation::redact_query))
}
fn cookie_values(response: &axum::response::Response) -> Vec<String> {
    response
        .headers()
        .get_all("set-cookie")
        .iter()
        .map(|value| {
            value
                .to_str()
                .unwrap()
                .split(';')
                .next()
                .unwrap()
                .to_owned()
        })
        .collect()
}
async fn graph(
    router: &axum::Router,
    query: &str,
    cookies: &str,
    origin: bool,
) -> anyhow::Result<axum::response::Response> {
    use tower::ServiceExt;
    let mut request = axum::http::Request::builder()
        .method("POST")
        .uri("/api/graphql")
        .header("content-type", "application/json")
        .header("cookie", cookies);
    if origin {
        request = request.header("origin", "http://localhost");
    }
    Ok(router
        .clone()
        .oneshot(request.body(axum::body::Body::from(
            serde_json::json!({"query": query}).to_string(),
        ))?)
        .await?)
}
async fn verify_browser_flow(
    di: &'static Di,
    permission: &MemberPermission,
    email: &str,
) -> anyhow::Result<()> {
    use tower::ServiceExt;
    let auth = di.auth_service();
    let invitations = di.invitation_service();
    let issued = invitations
        .invitation_create(permission, email, None)
        .await?;
    let router = browser_router(di);
    let response = router
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .uri(format!("/api/invitations/open?token={}", issued.token))
                .header(
                    "cookie",
                    "compallm-access=old; compallm-refresh=old; compallm-invitation=old",
                )
                .body(axum::body::Body::empty())?,
        )
        .await?;
    assert_eq!(response.status(), axum::http::StatusCode::SEE_OTHER);
    assert_eq!(response.headers()["location"], "/invitation");
    assert_eq!(response.headers()["cache-control"], "no-store");
    assert_eq!(response.headers()["referrer-policy"], "no-referrer");
    let cookies = cookie_values(&response);
    assert!(
        cookies.contains(&"compallm-access=".into())
            && cookies.contains(&"compallm-refresh=".into())
    );
    let invitation_cookie = cookies
        .iter()
        .find(|value| value.starts_with("compallm-invitation="))
        .unwrap();
    let response = graph(
        &router,
        "{ invitationContext { node { email signup } } }",
        invitation_cookie,
        false,
    )
    .await?;
    let body: serde_json::Value =
        serde_json::from_slice(&axum::body::to_bytes(response.into_body(), 65536).await?)?;
    assert!(body.get("errors").is_some());
    let query = format!(
        "mutation {{ emailSignupRequest(input: {{ email: \"{email}\" }}) {{ node {{ lifetime {{ expiresInSeconds }} }} }} }}"
    );
    let response = graph(&router, &query, invitation_cookie, true).await?;
    let updated = cookie_values(&response);
    assert!(updated.contains(&"unrelated=retained".into()));
    assert!(
        !updated
            .iter()
            .any(|value| value.starts_with("compallm-invitation="))
    );
    let otp_cookie = updated
        .iter()
        .find(|value| value.starts_with("compallm-access="))
        .unwrap();
    let challenge = EmailOtpAccessToken::try_from(
        auth.authenticate_token(otp_cookie.strip_prefix("compallm-access=").unwrap())
            .await?,
    )?;
    let query = format!(
        "mutation {{ emailSignupConfirm(input: {{ code: \"{}\" }}) {{ node {{ accessLifetime {{ expiresInSeconds }} }} }} }}",
        challenge.code
    );
    let missing = graph(&router, &query, otp_cookie, true).await?;
    let body: serde_json::Value =
        serde_json::from_slice(&axum::body::to_bytes(missing.into_body(), 65536).await?)?;
    assert!(body.get("errors").is_some());
    let response = graph(
        &router,
        &query,
        &format!("{invitation_cookie}; {otp_cookie}"),
        true,
    )
    .await?;
    let updated = cookie_values(&response);
    assert!(updated.contains(&"unrelated=retained".into()));
    assert!(updated.contains(&"compallm-invitation=".into()));
    let access = updated
        .iter()
        .find(|value| value.starts_with("compallm-access="))
        .unwrap();
    let claims = auth
        .authenticate_access(access.strip_prefix("compallm-access=").unwrap())
        .await?;
    assert_eq!(claims.session.organization_id, permission.organization_id());
    assert!(
        updated
            .iter()
            .any(|value| value.starts_with("compallm-refresh=") && value.len() > 20)
    );
    Ok(())
}
