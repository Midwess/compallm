use compallm_api::di::{AuthSettings, Di, GraphqlSettings};
use compallm_core::{
    authentication::{EmailOtpAccessToken, Session, UserAccessToken},
    authorization::{MemberPermission, PolicySnapshot},
    organization::{Group, Role},
    persistent::{
        DbContext, organization::OrganizationRepository as _, session::SessionRepository as _,
    },
};
use compallm_postgres::{
    postgres_connection::PostgresConnection,
    repository::{organization::OrganizationRepository, session::SessionRepository},
};
use sea_orm::Database;
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

#[tokio::test]
#[ignore = "requires COMPALLM_TEST_DATABASE_URL with migrated debug seed"]
async fn role_assignment_replaces_direct_grants_preserves_groups_and_invalidates_sessions()
-> anyhow::Result<()> {
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;
    let pool = Arc::new(Database::connect(std::env::var("COMPALLM_TEST_DATABASE_URL")?).await?);
    let root = DbContext::new(Arc::new(PostgresConnection::new(pool.clone())));
    DbContext::initialize_default(root.postgres().scoped())?;
    compallm_postgres::init_id_generator("member-role-assignment")?;
    let di = Box::leak(Box::new(Di::new(
        root.clone(),
        AuthSettings {
            jwt_secret: "role-assignment-integration-secret-at-least-32".into(),
            user_token_ttl_seconds: 1800,
            refresh_token_ttl_seconds: 86400,
        },
        GraphqlSettings {
            cookie_secure: false,
            public_origin: "http://localhost".into(),
            ..Default::default()
        },
        None,
    )));
    di.initialize_services()?;
    let repo = OrganizationRepository;
    use compallm_core::persistent::user::UserRepository as _;
    let user = root
        .scope(compallm_postgres::repository::user::UserRepository.find_by_id(OWNER))
        .await?
        .unwrap();
    let workspace = root
        .scope(di.organization_creation_service().create(
            &format!("Ownership-{}", hero::distributed_id::try_gen_id().await?),
            &user,
        ))
        .await?;
    let org = workspace.id;
    let defaults = root.scope(repo.list_role_info()).await?;
    let member_role = defaults
        .iter()
        .find(|r| r.organization_id == org && r.name == "Member")
        .unwrap()
        .id;
    let admin_role = defaults
        .iter()
        .find(|r| r.organization_id == org && r.name == "Admin")
        .unwrap()
        .id;
    let owner_role = defaults
        .iter()
        .find(|r| r.organization_id == org && r.org_owner())
        .unwrap()
        .id;
    let target = ORG + 102;
    root.scope(
        repo.add_member(compallm_core::organization::OrganizationMember {
            organization_id: org,
            user_id: target,
            groups: vec![],
            role: defaults
                .iter()
                .find(|r| r.id == member_role)
                .unwrap()
                .clone(),
        }),
    )
    .await?;
    let owner = root
        .scope(repo.find_membership(org, OWNER))
        .await?
        .unwrap()
        .0;
    let permission = member_permission(&owner);
    let previous = root
        .scope(repo.find_membership(org, target))
        .await?
        .unwrap()
        .0;
    let group = Group::create(
        org,
        format!("Assignment {}", hero::distributed_id::try_gen_id().await?),
        "",
        vec![],
    )
    .await?;
    let group = root.scope(repo.create_group(group)).await?;
    root.scope(di.rbac_service().update_group(
        &permission,
        group.id,
        compallm_core::organization::group::GroupChanges {
            add_member_ids: vec![target],
            ..Default::default()
        },
    ))
    .await?;
    let role = Role::new(
        org,
        format!("Assignment {}", hero::distributed_id::try_gen_id().await?),
        "",
        "permit(principal, action == Compallm::Action::\"member.read\", resource);",
    )
    .await?;
    root.scope(repo.create_roles(vec![role.clone()])).await?;
    root.scope(di.rbac_service().update_group(
        &permission,
        group.id,
        compallm_core::organization::group::GroupChanges {
            add_role_ids: vec![role.id],
            ..Default::default()
        },
    ))
    .await?;
    let session = Session {
        id: hero::distributed_id::try_gen_id().await?,
        user_id: target,
        organization_id: org,
        authorization_version: 1,
        revoked_at: None,
    };
    root.scope(SessionRepository.create(session.clone()))
        .await?;
    let service = di.rbac_service();
    root.scope(service.assign_role(&permission, role.id, target))
        .await?;
    let assigned = root
        .scope(repo.find_membership(org, target))
        .await?
        .unwrap()
        .0;
    assert_eq!(
        std::iter::once(&assigned.role)
            .map(|r| r.id)
            .collect::<Vec<_>>(),
        vec![role.id]
    );
    assert!(assigned.groups.iter().any(|g| g.id == group.id));
    assert_eq!(
        root.scope(SessionRepository.authorization_version(&session))
            .await?,
        Some(2)
    );
    root.scope(service.assign_role(&permission, role.id, target))
        .await?;
    assert_eq!(
        root.scope(SessionRepository.authorization_version(&session))
            .await?,
        Some(2)
    );
    let denied = member_permission(&assigned);
    assert!(
        root.scope(service.assign_role(&denied, admin_role, target))
            .await
            .is_err()
    );
    assert!(
        root.scope(service.assign_role(&permission, role.id, OWNER))
            .await
            .is_err()
    );
    assert!(
        root.scope(service.assign_role(&permission, role.id, 42))
            .await
            .is_err()
    );
    assert!(
        root.scope(service.assign_role(&permission, 42, target))
            .await
            .is_err()
    );
    let otp = EmailOtpAccessToken::new("owner@compallm.com".into())?;
    let login = root
        .scope(di.auth_service().signin(&otp, &otp.code))
        .await?;
    let bearer = di
        .auth_service()
        .switch_organization(&login.refresh_token, org)
        .await?
        .access_token;
    let app = browser_router(di);
    let request = |query: String| {
        Request::builder()
            .method("POST")
            .uri("/api/graphql")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {bearer}"))
            .body(Body::from(serde_json::json!({"query": query}).to_string()))
            .unwrap()
    };
    let owner_claims = di.auth_service().authenticate_access(&bearer).await?;
    let owner_version = root
        .scope(SessionRepository.authorization_version(&owner_claims.session))
        .await?;
    let self_change = app.clone().oneshot(request(format!(
        "mutation {{ roleAssign(id: \"{admin_role}\", memberId: \"{OWNER}\") {{ node {{ id }} }} }}"
    ))).await?;
    let self_error: serde_json::Value =
        serde_json::from_slice(&axum::body::to_bytes(self_change.into_body(), 65536).await?)?;
    assert_eq!(
        self_error["errors"][0]["extensions"]["code"],
        "SELF_ROLE_CHANGE"
    );
    assert_eq!(
        root.scope(SessionRepository.authorization_version(&owner_claims.session))
            .await?,
        owner_version
    );
    assert_eq!(
        root.scope(repo.find_membership(org, OWNER))
            .await?
            .unwrap()
            .0
            .role
            .id,
        owner_role
    );
    let response = app
        .clone()
        .oneshot(request(format!(
            "mutation {{ roleAssign(id: \"{}\", memberId: \"{}\") {{ node {{ id name }} }} }}",
            role.id, target
        )))
        .await?;
    let json: serde_json::Value =
        serde_json::from_slice(&axum::body::to_bytes(response.into_body(), 65536).await?)?;
    assert!(json.get("errors").is_none(), "{json}");
    assert_eq!(
        json["data"]["roleAssign"]["node"]["id"],
        role.id.to_string()
    );
    root.scope(service.assign_role(&permission, owner_role, target))
        .await?;
    let promoted = root
        .scope(repo.find_membership(org, target))
        .await?
        .unwrap()
        .0;
    assert!(std::iter::once(&promoted.role).any(|r| r.org_owner()));
    assert!(promoted.is_owner());
    assert!(
        root.scope(service.create_group(&permission, "Owner group denied", "", vec![owner_role]))
            .await
            .is_err()
    );
    assert!(
        root.scope(service.update_group(
            &permission,
            group.id,
            compallm_core::organization::group::GroupChanges {
                add_role_ids: vec![owner_role],
                ..Default::default()
            }
        ))
        .await
        .is_err()
    );
    let promoted_permission = member_permission(&promoted);
    let (first, second) = tokio::join!(
        root.scope(service.assign_role(&permission, member_role, target)),
        root.scope(service.assign_role(&promoted_permission, member_role, OWNER)),
    );
    assert_eq!(usize::from(first.is_ok()) + usize::from(second.is_ok()), 1);
    let survivors = root.scope(repo.active_owner_ids(org)).await?;
    assert_eq!(survivors.len(), 1);
    let surviving = root
        .scope(repo.find_membership(org, survivors[0]))
        .await?
        .unwrap()
        .0;
    let surviving_permission = member_permission(&surviving);
    root.scope(service.assign_role(&surviving_permission, owner_role, OWNER))
        .await?;
    let fresh_owner = root
        .scope(repo.find_membership(org, OWNER))
        .await?
        .unwrap()
        .0;
    let permission = member_permission(&fresh_owner);
    root.scope(service.assign_role(&permission, role.id, target))
        .await?;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
    root.scope(service.assign_role(&permission, admin_role, target))
        .await?;
    let target_user = root
        .scope(compallm_postgres::repository::user::UserRepository.find_by_id(target))
        .await?
        .unwrap();
    let target_otp = EmailOtpAccessToken::new(target_user.email)?;
    let target_login = root
        .scope(di.auth_service().signin(&target_otp, &target_otp.code))
        .await?;
    let target_bearer = di
        .auth_service()
        .switch_organization(&target_login.refresh_token, org)
        .await?
        .access_token;
    let fresh_otp = EmailOtpAccessToken::new("owner@compallm.com".into())?;
    let fresh_login = root
        .scope(di.auth_service().signin(&fresh_otp, &fresh_otp.code))
        .await?;
    let bearer = di
        .auth_service()
        .switch_organization(&fresh_login.refresh_token, org)
        .await?
        .access_token;
    for (credential, include_unseen, expected) in [
        (&bearer, false, true),
        (&target_bearer, false, false),
        (&target_bearer, true, true),
    ] {
        let query = format!(
            "query {{ roles(includeUnseen: {include_unseen}) {{ edges {{ node {{ id owner }} }} }} role(id: \"{owner_role}\", includeUnseen: {include_unseen}) {{ node {{ id }} }} }}"
        );
        let req = Request::builder()
            .method("POST")
            .uri("/api/graphql")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {credential}"))
            .body(Body::from(serde_json::json!({"query":query}).to_string()))?;
        let response = app.clone().oneshot(req).await?;
        let json: serde_json::Value =
            serde_json::from_slice(&axum::body::to_bytes(response.into_body(), 65536).await?)?;
        assert!(json.get("errors").is_none(), "{json}");
        assert_eq!(
            json["data"]["roles"]["edges"]
                .as_array()
                .unwrap()
                .iter()
                .any(|edge| edge["node"]["owner"] == true),
            expected
        );
        assert_eq!(!json["data"]["role"].is_null(), expected);
    }
    let admin_member = root
        .scope(repo.find_membership(org, target))
        .await?
        .unwrap()
        .0;
    assert!(
        root.scope(service.assign_role(&member_permission(&admin_member), owner_role, target))
            .await
            .is_err()
    );
    let concurrent_role = Role::new(
        org,
        format!("Concurrent {}", hero::distributed_id::try_gen_id().await?),
        "",
        "",
    )
    .await?;
    root.scope(repo.create_roles(vec![concurrent_role.clone()]))
        .await?;
    let (left, right) = tokio::join!(
        root.scope(service.assign_role(&permission, member_role, target)),
        root.scope(service.assign_role(&permission, concurrent_role.id, target)),
    );
    assert!(left.is_ok() || right.is_ok());
    let after_race = root
        .scope(repo.find_membership(org, target))
        .await?
        .unwrap()
        .0;
    assert_eq!(after_race.effective_roles().take(1).count(), 1);
    assert!(after_race.groups.iter().any(|g| g.id == group.id));
    type Links = <compallm_postgres::OrganizationMemberSchema as sea_orm::ModelTrait>::Entity;
    type LinkModel = <Links as sea_orm::EntityTrait>::ActiveModel;
    type LinkColumn = <Links as sea_orm::EntityTrait>::Column;
    use sea_orm::Set;
    let duplicate = Links::insert(LinkModel {
        organization_id: Set(org as i64),
        user_id: Set(target as i64),
        role_id: Set(owner_role as i64),
        status: Set(compallm_postgres::UserStatusSchema::Active),
    })
    .exec(pool.as_ref())
    .await;
    assert!(duplicate.is_err());
    let missing_role = Links::update_many()
        .col_expr(LinkColumn::RoleId, sea_orm::sea_query::Expr::value(0i64))
        .filter(LinkColumn::OrganizationId.eq(org as i64))
        .filter(LinkColumn::UserId.eq(target as i64))
        .exec(pool.as_ref())
        .await;
    assert!(missing_role.is_err());
    type GroupLinks =
        <compallm_postgres::OrganizationGroupRoleSchema as sea_orm::ModelTrait>::Entity;
    type GroupLinkModel = <GroupLinks as sea_orm::EntityTrait>::ActiveModel;
    for owner_marker in [false, true] {
        assert!(
            GroupLinks::insert(GroupLinkModel {
                organization_id: Set(org as i64),
                group_id: Set(group.id as i64),
                role_id: Set(owner_role as i64),
                owner_role: Set(owner_marker),
            })
            .exec(pool.as_ref())
            .await
            .is_err()
        );
    }
    let current = root
        .scope(repo.find_membership(org, target))
        .await?
        .unwrap()
        .0;
    root.scope(repo.save_member(&previous, &current)).await?;
    root.scope(service.update_group(
        &permission,
        group.id,
        compallm_core::organization::group::GroupChanges {
            remove_member_ids: vec![target],
            ..Default::default()
        },
    ))
    .await?;
    root.scope(service.delete_group(&permission, group.id))
        .await?;
    root.scope(service.delete_role(&permission, role.id))
        .await?;
    root.scope(service.delete_role(&permission, concurrent_role.id))
        .await?;
    Ok(())
}
fn browser_router(di: &'static Di) -> axum::Router {
    use axum::{
        Extension, Json, Router,
        extract::State,
        middleware::{from_fn, from_fn_with_state},
        routing::post,
    };
    use compallm_api::{
        graphql::{ApplicationSchema, GraphqlContext, endpoint},
        http::{
            authentication::{AuthenticationContext, CookieDelivery, UserContext},
            authorization::AuthorizationContext,
            middleware::{AuthenticationMiddleware, AuthorizationMiddleware},
        },
    };
    Router::new().route(
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
                endpoint::handle(
                    schema,
                    GraphqlContext::new(di, user, authentication, delivery, authorization),
                    request,
                )
                .await
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
}
