#[path = "support/graphql.rs"]
mod graphql;
use compallm_api::di::{AuthSettings, Di, GraphqlSettings};
use compallm_core::{
    analytics::UserProfileValue,
    authentication::{EmailOtpAccessToken, token::InvitationToken},
    authorization::MemberPermission,
    onboarding::{Flow, OnboardingError, Settings, Step},
    persistent::{
        DbContext, analytics::AnalyticsRepository as _, onboarding::OnboardingRepository as _,
        organization::OrganizationRepository as _, preference::user::UserPreferenceRepository as _,
        user::UserRepository as _,
    },
    user::ProfilePatch,
};
use compallm_postgres::{
    postgres_connection::PostgresConnection,
    repository::{
        analytics::AnalyticsRepository, onboarding::OnboardingRepository,
        organization::OrganizationRepository, preference::user::UserPreferenceRepository,
        user::UserRepository,
    },
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter,
    Set,
};
use std::sync::Arc;

#[tokio::test]
#[ignore = "requires COMPALLM_TEST_DATABASE_URL with migrations applied"]
async fn signup_invitation_snapshots_requirements_and_analytics_are_scoped() -> anyhow::Result<()> {
    let pool = Arc::new(Database::connect(std::env::var("COMPALLM_TEST_DATABASE_URL")?).await?);
    let root = DbContext::new(Arc::new(PostgresConnection::new(pool.clone())));
    DbContext::initialize_default(root.postgres().scoped())?;
    compallm_postgres::init_id_generator("onboarding-integration")?;
    let di = Box::leak(Box::new(Di::new(
        root.clone(),
        AuthSettings {
            jwt_secret: "onboarding-integration-secret-at-least-32".into(),
            user_token_ttl_seconds: 1800,
            refresh_token_ttl_seconds: 86400,
        },
        GraphqlSettings {
            public_origin: "http://localhost".into(),
            cookie_secure: false,
            ..Default::default()
        },
        None,
    )));
    di.initialize_services()?;
    let suffix = hero::distributed_id::try_gen_id().await?;
    let otp = EmailOtpAccessToken::new(format!("onboard-owner-{suffix}@example.com"))?;
    let credentials = di.auth_service().signup(&otp, &otp.code).await?;
    let owner = di
        .auth_service()
        .authenticate_access(&credentials.access_token)
        .await?;
    let org = owner.session.organization_id;
    let router = graphql::browser_router(di);
    let result=graphql::execute(&router,&credentials.access_token,"{ onboardingContext { node { required flow step shouldPresent settings { requireAvatar } } } }").await?;
    assert!(result.get("errors").is_none(), "{result}");
    assert_eq!(
        result["data"]["onboardingContext"]["node"]["required"],
        true
    );
    let analytics=graphql::execute(&router,&credentials.access_token,r#"mutation { analyticUserProfilePut(input:{key:"purpose",value:"Learning"}) { node {key value} } }"#).await?;
    assert!(analytics.get("errors").is_none(), "{analytics}");
    let analytics = graphql::execute(
        &router,
        &credentials.access_token,
        r#"{ analyticUserProfile(key:"purpose") { node {value} } }"#,
    )
    .await?;
    assert_eq!(
        analytics["data"]["analyticUserProfile"]["node"]["value"],
        "Learning"
    );

    let owner_permission = MemberPermission::try_from(owner.clone())?;
    let own = root
        .scope(OnboardingRepository.find(org, owner.session.user_id, false))
        .await?
        .unwrap();
    assert!(own.required);
    assert_eq!(own.progress.flow, Flow::Personal);
    assert_eq!(own.progress.step, Step::Profile);
    assert!(
        root.scope(UserPreferenceRepository.find(owner.session.user_id))
            .await?
            .is_some()
    );
    type Members = <compallm_postgres::OrganizationMemberSchema as ModelTrait>::Entity;
    type MemberColumn = <Members as EntityTrait>::Column;
    assert_eq!(
        Members::find()
            .filter(MemberColumn::UserId.eq(owner.session.user_id as i64))
            .count(pool.as_ref())
            .await?,
        1
    );
    let service = di.onboarding_service();
    let settings = Settings {
        welcome: "Welcome aboard".into(),
        require_avatar: true,
        require_full_name: true,
        ..Default::default()
    };
    root.scope(service.update_settings(&owner_permission, settings))
        .await?;
    let email = format!("onboard-invited-{suffix}@example.com");
    let sent = di
        .invitation_service()
        .send_invitation(
            &owner_permission,
            &email,
            "http://localhost/api/invitations/open?token=",
        )
        .await?;
    let invitation =
        InvitationToken::try_from(di.auth_service().authenticate_token(&sent.token).await?)?;
    let mut otp = EmailOtpAccessToken::new(email)?;
    otp.invitation_id = Some(invitation.invitation_id);
    let credentials = di
        .invitation_service()
        .invitation_confirm(&invitation, &otp, &otp.code, true)
        .await?;
    let actor = di
        .auth_service()
        .authenticate_access(&credentials.access_token)
        .await?;
    assert_eq!(actor.session.organization_id, org);
    let user_id = actor.session.user_id;
    assert_eq!(
        Members::find()
            .filter(MemberColumn::UserId.eq(user_id as i64))
            .count(pool.as_ref())
            .await?,
        1
    );
    assert!(
        root.scope(UserPreferenceRepository.find(user_id))
            .await?
            .is_some()
    );
    let permission = Arc::new(MemberPermission::try_from(actor.clone())?);
    let personal = root
        .fresh()?
        .restrict(async move { Ok::<_, std::convert::Infallible>(permission) })?;
    let before = personal
        .scope(OnboardingRepository.find(org, user_id, false))
        .await?
        .unwrap();
    assert_eq!(before.progress.flow, Flow::InvitedNew);
    assert!(before.progress.settings.require_avatar);
    assert!(
        personal
            .scope(OnboardingRepository.settings(org))
            .await?
            .is_none()
    );
    assert!(
        personal
            .scope(OnboardingRepository.find(org, owner.session.user_id, false))
            .await?
            .is_none()
    );
    assert!(
        personal
            .scope(service.update_settings(
                &MemberPermission::try_from(actor.clone())?,
                Settings::default()
            ))
            .await
            .is_err()
    );
    root.scope(service.update_settings(&owner_permission, Settings::default()))
        .await?;
    assert!(
        personal
            .scope(OnboardingRepository.find(org, user_id, false))
            .await?
            .unwrap()
            .progress
            .settings
            .require_full_name
    );
    assert!(matches!(
        personal.scope(service.advance(&actor, false)).await,
        Err(OnboardingError::Requirements)
    ));
    service.present(&actor, false).await?;
    let presented = personal
        .scope(OnboardingRepository.find(org, user_id, false))
        .await?
        .unwrap();
    assert!(
        presented
            .progress
            .presented_session_ids
            .contains(&actor.session.id)
    );
    assert!(presented.required);
    let signed_user = root
        .scope(UserRepository.find_by_id(user_id))
        .await?
        .unwrap();
    let another_otp = EmailOtpAccessToken::new(signed_user.email)?;
    let another_login = di
        .auth_service()
        .signin(&another_otp, &another_otp.code)
        .await?;
    let another = di
        .auth_service()
        .authenticate_access(&another_login.access_token)
        .await?;
    service.present(&another, false).await?;
    let reminders = root
        .scope(OnboardingRepository.find(org, user_id, false))
        .await?
        .unwrap()
        .progress
        .presented_session_ids;
    assert!(reminders.contains(&actor.session.id) && reminders.contains(&another.session.id));

    personal
        .scope(di.profile_service().update_profile(
            &actor,
            ProfilePatch {
                display_name: Some("Invited Person".into()),
                ..Default::default()
            },
        ))
        .await?;
    assert!(matches!(
        personal.scope(service.advance(&actor, false)).await,
        Err(OnboardingError::Requirements)
    ));
    type Users = <compallm_postgres::UserSchema as ModelTrait>::Entity;
    type UserModel = <Users as EntityTrait>::ActiveModel;
    UserModel {
        id: Set(user_id as i64),
        avatar_path: Set(Some(format!("/avatars/{user_id}"))),
        ..Default::default()
    }
    .update(pool.as_ref())
    .await?;
    for expected in [
        Step::Work,
        Step::Preferences,
        Step::Introduction,
        Step::Ready,
    ] {
        assert_eq!(
            personal
                .scope(service.advance(&actor, false))
                .await?
                .progress
                .step,
            expected
        );
    }
    let (left, right) = tokio::join!(service.complete(&actor), service.complete(&actor));
    assert!(!left?.required && !right?.required);
    let saved = personal
        .scope(OnboardingRepository.find(org, user_id, false))
        .await?
        .unwrap();
    assert!(!saved.required);
    assert_eq!(saved.progress.step, Step::Ready);
    root.scope(OnboardingRepository.initialize(org, user_id, saved.progress.clone()))
        .await?;
    assert!(
        !personal
            .scope(OnboardingRepository.find(org, user_id, false))
            .await?
            .unwrap()
            .required
    );
    type Onboarding = <compallm_postgres::OrganizationMemberOnboardingSchema as ModelTrait>::Entity;
    type OnboardingColumn = <Onboarding as EntityTrait>::Column;
    let restricted_db =
        compallm_postgres::postgres_connection::PostgresConnection::from_context(&personal)?;
    assert!(
        Onboarding::update_many()
            .col_expr(
                OnboardingColumn::Completed,
                sea_orm::sea_query::Expr::value(false)
            )
            .filter(OnboardingColumn::OrganizationId.eq(org as i64))
            .filter(OnboardingColumn::UserId.eq(user_id as i64))
            .exec(restricted_db)
            .await
            .is_err()
    );

    personal.scope(service.advance(&actor, true)).await?;
    assert!(
        !personal
            .scope(OnboardingRepository.find(org, user_id, false))
            .await?
            .unwrap()
            .required
    );
    let analytics = di.analytics_service();
    personal
        .scope(analytics.put(
            &actor,
            UserProfileValue {
                key: "work".into(),
                value: "Engineering".into(),
            },
        ))
        .await?;
    personal
        .scope(analytics.put(
            &actor,
            UserProfileValue {
                key: "work".into(),
                value: "Research".into(),
            },
        ))
        .await?;
    assert_eq!(
        personal
            .scope(AnalyticsRepository.find(user_id, "work"))
            .await?
            .unwrap()
            .value,
        "Research"
    );
    assert!(
        personal
            .scope(AnalyticsRepository.find(owner.session.user_id, "work"))
            .await?
            .is_none()
    );
    assert!(
        personal
            .scope(AnalyticsRepository.put(
                owner.session.user_id,
                UserProfileValue {
                    key: "work".into(),
                    value: "foreign".into()
                }
            ))
            .await
            .is_err()
    );
    let other_user = root
        .scope(UserRepository.find_by_id(7_000_000_000_000_000_102))
        .await?
        .unwrap();
    root.scope(service.update_settings(
        &owner_permission,
        Settings {
            require_full_name: true,
            require_avatar: true,
            ..Default::default()
        },
    ))
    .await?;
    let sent = di
        .invitation_service()
        .send_invitation(
            &owner_permission,
            &other_user.email,
            "http://localhost/api/invitations/open?token=",
        )
        .await?;
    let invitation =
        InvitationToken::try_from(di.auth_service().authenticate_token(&sent.token).await?)?;
    let mut otp = EmailOtpAccessToken::new(other_user.email)?;
    otp.invitation_id = Some(invitation.invitation_id);
    let credentials = di
        .invitation_service()
        .invitation_confirm(&invitation, &otp, &otp.code, false)
        .await?;
    let existing = di
        .auth_service()
        .authenticate_access(&credentials.access_token)
        .await?;
    let joined = root
        .scope(OnboardingRepository.find(org, existing.session.user_id, false))
        .await?
        .unwrap();
    assert_eq!(joined.progress.flow, Flow::InvitedExisting);
    assert_eq!(joined.progress.step, Step::Introduction);
    assert_eq!(
        root.scope(service.advance(&existing, false))
            .await?
            .progress
            .step,
        Step::Ready
    );
    assert!(!service.complete(&existing).await?.required);
    assert!(
        root.scope(
            OrganizationRepository
                .find_membership(7_000_000_000_000_000_000, existing.session.user_id)
        )
        .await?
        .is_some()
    );
    Ok(())
}
