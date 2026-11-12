#[path = "support/permission.rs"]
mod permission_fixture;
#[path = "support/roles.rs"]
mod role_fixture;

use compallm_core::{
    authorization::{
        MemberPermissionError,
        condition::{CheckPermission, ForAction},
        policy::Action,
    },
    model::Model,
    organization::OrganizationMember,
    provider::ServiceKind,
};

fn model(organization_id: u64, id: u64, enabled: bool) -> Model {
    Model {
        id,
        organization_id,
        name: String::new(),
        service_kind: ServiceKind::Llm,
        enabled,
        capabilities: Vec::new(),
        limits: Default::default(),
        thinking: None,
        pricing: serde_json::from_value(serde_json::json!({"request": {"usdPerRequest": "0"}}))
            .unwrap(),
    }
}

#[test]
fn resource_checks_preserve_identity_scope_availability_and_forbid_precedence() {
    let permission = permission_fixture::from_source(
        42,
        r#"
        permit(principal, action in [Compallm::Action::"model.configuration.update", Compallm::Action::"model.use"], resource is Compallm::Model in Compallm::Organization::"42");
        forbid(principal, action == Compallm::Action::"model.configuration.update", resource == Compallm::Model::"42:18");
        forbid(principal, action == Compallm::Action::"model.use", resource is Compallm::Model) unless { resource.enabled };
    "#,
    );
    let enabled = model(42, 17, true);
    enabled
        .for_action(Action::ModelConfigurationUpdate)
        .and(enabled.for_action(Action::ModelUse))
        .check_permission(&permission)
        .unwrap();
    assert!(matches!(
        model(42, 18, true)
            .for_action(Action::ModelConfigurationUpdate)
            .check_permission(&permission),
        Err(MemberPermissionError::Denied)
    ));
    assert!(matches!(
        model(42, 19, false)
            .for_action(Action::ModelUse)
            .check_permission(&permission),
        Err(MemberPermissionError::Denied)
    ));
    let broad = permission_fixture::from_source(42, "permit(principal, action, resource);");
    assert!(matches!(
        model(43, 17, true)
            .for_action(Action::ModelConfigurationUpdate)
            .check_permission(&broad),
        Err(MemberPermissionError::Denied)
    ));
    assert!(matches!(
        enabled
            .for_action(Action::ProviderDelete)
            .check_permission(&broad),
        Err(MemberPermissionError::Evaluation)
    ));
    assert!(matches!(
        Action::ModelUse.check_permission(&broad),
        Err(MemberPermissionError::Evaluation)
    ));
}

#[tokio::test]
async fn contextual_assignment_requires_independent_delegation_and_supports_nested_and() {
    let roles = role_fixture::default_roles(42, [7, 8, 9]).await;
    // The fixture actor uses role 1; rewrite the principal scope without changing the grants.
    let admin_source = roles[1]
        .cedar_source
        .replace("Compallm::Role::\"42:8\"", "Compallm::Role::\"42:1\"");
    let admin = permission_fixture::from_source(42, &admin_source);
    let target = OrganizationMember {
        organization_id: 42,
        user_id: 200,
        roles: Vec::new(),
        groups: Vec::new(),
    };
    let assignment = (&target, &roles[2]).for_action(Action::MemberRoleAssign);
    Action::GroupRead
        .and(assignment.and(roles[2].for_action(Action::RoleAssign)))
        .check_permission(&admin)
        .unwrap();
    assert!(matches!(
        (&target, &roles[0])
            .for_action(Action::MemberRoleAssign)
            .and(roles[0].for_action(Action::RoleAssign))
            .check_permission(&admin),
        Err(MemberPermissionError::Denied)
    ));
    let without_delegation = permission_fixture::permission(42, &[Action::MemberRoleAssign]);
    (&target, &roles[2])
        .for_action(Action::MemberRoleAssign)
        .check_permission(&without_delegation)
        .unwrap();
    assert!(matches!(
        (&target, &roles[2])
            .for_action(Action::MemberRoleAssign)
            .and(roles[2].for_action(Action::RoleAssign))
            .check_permission(&without_delegation),
        Err(MemberPermissionError::Denied)
    ));
    assert!(matches!(
        target
            .for_action(Action::MemberRoleAssign)
            .check_permission(&admin),
        Err(MemberPermissionError::Evaluation)
    ));
    let foreign_role = compallm_core::organization::Role {
        organization_id: 43,
        ..roles[2].clone()
    };
    assert!(matches!(
        (&target, &foreign_role)
            .for_action(Action::MemberRoleAssign)
            .check_permission(&admin),
        Err(MemberPermissionError::Denied)
    ));
}
