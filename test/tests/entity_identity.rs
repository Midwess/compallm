use compallm_core::{
    auth::Session,
    organization::{Group, Organization, Role},
    user::User,
};

#[tokio::test]
async fn entity_creation_owns_ids_and_default_policy_relationships() {
    compallm_postgres::init_id_generator("entity-identity-test").unwrap();
    let user = User::from_email("james@example.com").await.unwrap();
    let organization = Organization::new("Research").await.unwrap();
    let roles = Role::defaults(organization.id).await.unwrap();
    assert!(roles[0].org_owner());
    assert!(!roles[1].org_owner());
    assert!(!roles[2].org_owner());
    let owner = roles[0].clone();
    let custom = Role::new(organization.id, "Reviewer", "", " ")
        .await
        .unwrap();
    let group = Group::new(organization.id, "Operations", "", vec![owner.clone()])
        .await
        .unwrap();
    let session = Session::new(user.id, organization.id, organization.name.clone())
        .await
        .unwrap();
    let ids = [
        user.id,
        organization.id,
        roles[0].id,
        roles[1].id,
        roles[2].id,
        custom.id,
        group.id,
        session.id,
    ];
    assert_eq!(
        ids.into_iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        ids.len()
    );
    assert_eq!(session.user_id, user.id);
    assert_eq!(session.organization_id, organization.id);
    assert_eq!(group.roles, vec![owner]);
    for role in &roles {
        assert_eq!(role.organization_id, organization.id);
        assert!(
            role.cedar_source
                .contains(&format!("{}:{}", organization.id, role.id))
        );
    }
    assert!(
        roles[1]
            .cedar_source
            .contains(&format!("{}:{}", organization.id, roles[0].id))
    );
    let mut model = compallm_core::model::Model {
        id: 999,
        organization_id: organization.id,
        name: "Logical model".into(),
        service_kind: compallm_core::provider::ServiceKind::Llm,
        enabled: false,
        capabilities: vec![compallm_core::provider::Capability::Text],
        limits: Default::default(),
        thinking: None,
        pricing: serde_json::from_value(serde_json::json!({"request":{"usdPerRequest":"1"}}))
            .unwrap(),
    };
    let mut expected = model.clone();
    model.initialize_id().await.unwrap();
    assert_ne!(model.id, expected.id);
    expected.id = model.id;
    assert_eq!(model, expected);
}

#[tokio::test]
async fn role_constructor_validates_and_normalizes_iam_length() {
    use compallm_core::organization::role::RoleCreationError;

    compallm_postgres::init_id_generator("role-name-test").unwrap();
    for invalid in ["  ".to_string(), "界".repeat(65)] {
        assert!(matches!(
            Role::new(1, invalid, "", "").await,
            Err(RoleCreationError::InvalidName(_))
        ));
    }
    let name = "界".repeat(64);
    let role = Role::new(1, format!("  {name}  "), "", "").await.unwrap();
    assert_eq!(role.name, name);
    assert_ne!(role.id, 0);
}

#[tokio::test]
async fn group_constructor_limits_assigned_roles() {
    use compallm_core::organization::group::GroupCreationError;

    compallm_postgres::init_id_generator("group-role-limit-test").unwrap();
    let role = Role::new(1, "Role", "", "").await.unwrap();
    let roles: Vec<_> = (0..Group::MAX_ROLES)
        .map(|index| Role {
            id: index as u64 + 1,
            ..role.clone()
        })
        .collect();
    assert_eq!(
        Group::new(1, "Group", "", roles.clone())
            .await
            .unwrap()
            .roles
            .len(),
        32
    );
    let mut too_many = roles;
    too_many.push(too_many[0].clone());
    assert!(matches!(
        Group::new(1, "Group", "", too_many).await,
        Err(GroupCreationError::TooManyRoles)
    ));
}

#[tokio::test]
async fn group_constructor_checks_name_and_role_scope() {
    use compallm_core::organization::group::GroupCreationError;
    compallm_postgres::init_id_generator("group-validation-test").unwrap();
    let role = Role::new(1, "Role", "", "").await.unwrap();
    assert!(matches!(
        Group::new(1, "  ", "", vec![]).await,
        Err(GroupCreationError::InvalidName(_))
    ));
    assert!(matches!(
        Group::new(2, "Group", "", vec![role.clone()]).await,
        Err(GroupCreationError::ForeignRole)
    ));
    assert!(matches!(
        Group::new(1, "Group", "", vec![role.clone(), role]).await,
        Err(GroupCreationError::DuplicateRole)
    ));
    let group = Group::new(1, "  Research  ", "", vec![]).await.unwrap();
    assert_eq!(group.name, "Research");
}

#[tokio::test]
async fn role_constructor_checks_cedar_and_preserves_source() {
    use compallm_core::organization::role::RoleCreationError;
    compallm_postgres::init_id_generator("role-policy-validation-test").unwrap();
    for source in [
        "invalid policy",
        "permit(principal, action == Compallm::Action::\"unknown\", resource);",
        "permit(principal == ?principal, action, resource);",
    ] {
        assert!(matches!(
            Role::new(1, "Role", "", source).await,
            Err(RoleCreationError::InvalidPolicy(_))
        ));
    }
    let source = "// kept\npermit(principal, action == Compallm::Action::\"model.list\", resource is Compallm::Organization);\n";
    let role = Role::new(1, "Role", "", source).await.unwrap();
    assert_eq!(role.cedar_source, source);
    assert!(role.ensure_deletable().is_ok());
    let mut owner = role;
    owner.org_owner = true;
    owner.name = "Renamed".into();
    assert!(owner.ensure_deletable().is_err());
}
