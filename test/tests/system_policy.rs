use std::str::FromStr;

use cedar_policy::{
    Authorizer, Context, Decision, Entities, EntityUid, PolicySet, Request, Schema, ValidationMode,
    Validator,
};
use compallm_core::{
    authorization::{SystemPolicy, all_policies},
    model::Model,
    organization::{Group, Organization, OrganizationMember, Role},
    provider::{Provider, ProviderModel},
    user::User,
};
use serde_json::json;

fn uid(kind: &str, id: &str) -> EntityUid {
    EntityUid::from_str(&format!("Compallm::{kind}::\"{id}\"")).unwrap()
}

fn request(action: &str, kind: &str, context: Context) -> anyhow::Result<Request> {
    Request::new(
        uid("User", "100"),
        uid("Action", action),
        uid(kind, "42:200"),
        context,
        Some(all_policies()),
    )
    .map_err(Into::into)
}

#[test]
fn all_entity_policies_assemble_into_one_cached_schema() {
    let assembled = Schema::from_schema_fragments([
        User::policy(),
        Role::policy(),
        Group::policy(),
        Organization::policy(),
        OrganizationMember::policy(),
        Provider::policy(),
        ProviderModel::policy(),
        Model::policy(),
    ])
    .unwrap();
    assert_eq!(
        assembled.action_entities().unwrap().iter().count(),
        compallm_core::authorization::policy::Action::ALL.len()
    );
    assert!(std::ptr::eq(all_policies(), all_policies()));
    let names = all_policies()
        .action_entities()
        .unwrap()
        .iter()
        .map(|entity| entity.uid().to_string())
        .collect::<Vec<_>>();
    assert!(
        names.iter().all(|id| !id.contains(".manage")
            && !id.contains(".configure")
            && !id.contains(".own"))
    );
    assert!(
        !names
            .iter()
            .any(|id| id.contains("credential.read") || id.contains("credential.export"))
    );
}

#[test]
fn actions_have_explicit_resource_types_and_do_not_imply_grants() {
    for (prefix, operations, kind) in [
        ("organization", "delete", "Organization"),
        ("organization.settings", "read update", "Organization"),
        ("member", "read invite", "Organization"),
        ("member", "remove", "Member"),
        ("provider", "read create", "Organization"),
        ("provider", "delete", "Provider"),
        ("provider.configuration", "update", "Provider"),
        ("provider.credential", "replace", "Provider"),
        ("provider.model", "read", "Organization"),
        ("provider.model", "create", "Provider"),
        ("provider.model", "delete", "ProviderModel"),
        ("provider.model.configuration", "update", "ProviderModel"),
        ("provider.model.pricing", "update", "ProviderModel"),
        ("model", "read create", "Organization"),
        ("model", "delete use", "Model"),
        ("model.configuration", "update", "Model"),
        ("model.pricing", "update", "Model"),
    ] {
        for operation in operations.split_whitespace() {
            let action = format!("{prefix}.{operation}");
            let req = request(&action, kind, Context::empty()).unwrap();
            let other_kind = if kind == "User" { "Provider" } else { "User" };
            assert!(
                request(&action, other_kind, Context::empty()).is_err(),
                "{action}"
            );
            let decision =
                Authorizer::new().is_authorized(&req, &PolicySet::new(), &Entities::empty());
            assert_eq!(
                decision.decision(),
                Decision::Deny,
                "{action} must not grant itself"
            );
        }
    }
}

#[test]
fn role_assignment_requires_typed_role_context() {
    for action in ["member.role.assign", "member.role.unassign"] {
        assert!(request(action, "Member", Context::empty()).is_err());
        let context = || {
            Context::from_json_value(
                json!({"role": {"__entity": {"type":"Compallm::Role", "id":"42:7"}}}),
                None,
            )
            .unwrap()
        };
        request(action, "Member", context()).unwrap();
        assert!(request(action, "Provider", context()).is_err());
        let wrong = Context::from_json_value(json!({"role":"42:7"}), None).unwrap();
        assert!(request(action, "Member", wrong).is_err());
    }
}

#[test]
fn schema_rejects_unknown_actions_and_credential_exposure() {
    let validator = Validator::new(all_policies().clone());
    for action in [
        "user.update",
        "user.delete",
        "organization.update",
        "member.list",
        "provider.list",
        "provider.update",
        "provider.model.list",
        "provider.model.update",
        "model.list",
        "model.update",
        "session.read",
        "provider.view",
        "provider.manage",
        "provider.credentials.replace",
        "provider.credential.read",
        "conversation.read",
    ] {
        assert!(request(action, "Provider", Context::empty()).is_err());
        let policies = PolicySet::from_str(&format!(
            "permit(principal, action == Compallm::Action::\"{action}\", resource);"
        ))
        .unwrap();
        assert!(
            !validator
                .validate(&policies, ValidationMode::Strict)
                .validation_passed()
        );
    }
    let secret = PolicySet::from_str("permit(principal, action == Compallm::Action::\"provider.configuration.update\", resource) when { resource.apiKey == \"secret\" };").unwrap();
    assert!(
        !validator
            .validate(&secret, ValidationMode::Strict)
            .validation_passed()
    );
}

fn entities() -> Entities {
    Entities::from_json_value(json!([
        {"uid":{"type":"Compallm::User","id":"100"},"attrs":{},"parents":[{"type":"Compallm::Group","id":"42:9"}]},
        {"uid":{"type":"Compallm::Group","id":"42:9"},"attrs":{"organization":{"__entity":{"type":"Compallm::Organization","id":"42"}}},"parents":[{"type":"Compallm::Role","id":"42:7"}]},
        {"uid":{"type":"Compallm::Role","id":"42:7"},"attrs":{"organization":{"__entity":{"type":"Compallm::Organization","id":"42"}}},"parents":[]},
        {"uid":{"type":"Compallm::Organization","id":"42"},"attrs":{},"parents":[]},
        {"uid":{"type":"Compallm::Organization","id":"43"},"attrs":{},"parents":[]},
        {"uid":{"type":"Compallm::Provider","id":"42:200"},"attrs":{"organization":{"__entity":{"type":"Compallm::Organization","id":"42"}}},"parents":[{"type":"Compallm::Organization","id":"42"}]},
        {"uid":{"type":"Compallm::Provider","id":"43:200"},"attrs":{"organization":{"__entity":{"type":"Compallm::Organization","id":"43"}}},"parents":[{"type":"Compallm::Organization","id":"43"}]}
    ]), Some(all_policies())).unwrap()
}

#[test]
fn role_inheritance_preserves_organization_scope_and_forbid_precedence() {
    let permit = r#"permit (
        principal in Compallm::Role::"42:7",
        action == Compallm::Action::"provider.read",
        resource == Compallm::Organization::"42"
    );"#;
    let mut policies = PolicySet::from_str(permit).unwrap();
    let validator = Validator::new(all_policies().clone());
    assert!(
        validator
            .validate(&policies, ValidationMode::Strict)
            .validation_passed()
    );
    let authorizer = Authorizer::new();
    let local = Request::new(
        uid("User", "100"),
        uid("Action", "provider.read"),
        uid("Organization", "42"),
        Context::empty(),
        Some(all_policies()),
    )
    .unwrap();
    assert_eq!(
        authorizer
            .is_authorized(&local, &policies, &entities())
            .decision(),
        Decision::Allow
    );
    let foreign = Request::new(
        uid("User", "100"),
        uid("Action", "provider.read"),
        uid("Organization", "43"),
        Context::empty(),
        Some(all_policies()),
    )
    .unwrap();
    assert_eq!(
        authorizer
            .is_authorized(&foreign, &policies, &entities())
            .decision(),
        Decision::Deny
    );
    policies = PolicySet::from_str(&format!("{permit}\nforbid(principal, action == Compallm::Action::\"provider.read\", resource is Compallm::Organization);" )).unwrap();
    assert!(
        validator
            .validate(&policies, ValidationMode::Strict)
            .validation_passed()
    );
    assert_eq!(
        authorizer
            .is_authorized(&local, &policies, &entities())
            .decision(),
        Decision::Deny
    );
}

#[test]
fn resource_relationships_exclude_nested_groups_and_untyped_ownership() {
    let set = PolicySet::from_str(
        "permit(principal, action == Compallm::Action::\"provider.model.read\", resource) when { resource == Compallm::Organization::\"42\" };",
    ).unwrap();
    assert!(
        Validator::new(all_policies().clone())
            .validate(&set, ValidationMode::Strict)
            .validation_passed()
    );
    let nested = json!([{"uid":{"type":"Compallm::Group","id":"42:9"},"attrs":{"organization":{"__entity":{"type":"Compallm::Organization","id":"42"}}},"parents":[{"type":"Compallm::Group","id":"42:10"}]}]);
    assert!(Entities::from_json_value(nested, Some(all_policies())).is_err());
}

#[test]
fn route_actions_identify_the_selected_upstream_model_with_typed_context() {
    for action in [
        "model.route.add",
        "model.route.update",
        "model.route.remove",
    ] {
        assert!(request(action, "Model", Context::empty()).is_err());
        let context = Context::from_json_value(
            json!({"providerModel": {
                "__entity":{"type":"Compallm::ProviderModel","id":"42:300"}
            }}),
            None,
        )
        .unwrap();
        request(action, "Model", context).unwrap();
        let wrong = Context::from_json_value(
            json!({"providerModel": {
                "__entity":{"type":"Compallm::Provider","id":"42:300"}
            }}),
            None,
        )
        .unwrap();
        assert!(request(action, "Model", wrong).is_err());
    }
}

#[test]
fn public_profiles_have_no_rbac_action_or_broad_read_permit() {
    let validator = Validator::new(all_policies().clone());
    for action in [
        "user.read",
        "user.profile.read",
        "organization.read",
        "organization.profile.read",
    ] {
        let kind = if action.starts_with("user.") {
            "User"
        } else {
            "Organization"
        };
        assert!(
            request(action, kind, Context::empty()).is_err(),
            "{action} is outside RBAC"
        );
        let set = PolicySet::from_str(&format!(
            "permit(principal, action == Compallm::Action::\"{action}\", resource);"
        ))
        .unwrap();
        assert!(
            !validator
                .validate(&set, ValidationMode::Strict)
                .validation_passed()
        );
    }
}

#[test]
fn viewing_a_disabled_model_does_not_authorize_invocation() {
    let policies = PolicySet::from_str(
        r#"
        permit(principal in Compallm::Role::"42:7", action == Compallm::Action::"model.read",
            resource == Compallm::Organization::"42");
        permit(principal in Compallm::Role::"42:7", action == Compallm::Action::"model.use",
            resource is Compallm::Model in Compallm::Organization::"42") when { resource.enabled };
    "#,
    )
    .unwrap();
    assert!(
        Validator::new(all_policies().clone())
            .validate(&policies, ValidationMode::Strict)
            .validation_passed()
    );
    for enabled in [false, true] {
        // Existing role membership fixtures are extended with an actual Model.
        let model = json!({"uid":{"type":"Compallm::Model","id":"42:200"},"attrs":{"organization":{"__entity":{"type":"Compallm::Organization","id":"42"}},"enabled":enabled},"parents":[{"type":"Compallm::Organization","id":"42"}]});
        let model_entities =
            Entities::from_json_value(json!([model]), Some(all_policies())).unwrap();
        let combined = entities()
            .add_entities(model_entities.iter().cloned(), Some(all_policies()))
            .unwrap();
        for action in ["model.read", "model.use"] {
            let req = if action == "model.read" {
                Request::new(
                    uid("User", "100"),
                    uid("Action", action),
                    uid("Organization", "42"),
                    Context::empty(),
                    Some(all_policies()),
                )
                .unwrap()
            } else {
                request(action, "Model", Context::empty()).unwrap()
            };
            let result = Authorizer::new().is_authorized(&req, &policies, &combined);
            assert_eq!(
                result.decision(),
                if action == "model.read" || enabled {
                    Decision::Allow
                } else {
                    Decision::Deny
                }
            );
            assert_eq!(result.diagnostics().errors().count(), 0);
        }
    }
}

#[test]
fn configuration_update_does_not_imply_secret_replacement_or_read_access() {
    let policies = PolicySet::from_str(
        r#"
        permit(principal in Compallm::Role::"42:7",
            action == Compallm::Action::"provider.configuration.update",
            resource is Compallm::Provider in Compallm::Organization::"42");
    "#,
    )
    .unwrap();
    assert!(
        Validator::new(all_policies().clone())
            .validate(&policies, ValidationMode::Strict)
            .validation_passed()
    );
    for (action, expected) in [
        ("provider.configuration.update", Decision::Allow),
        ("provider.credential.replace", Decision::Deny),
    ] {
        let response = Authorizer::new().is_authorized(
            &request(action, "Provider", Context::empty()).unwrap(),
            &policies,
            &entities(),
        );
        assert_eq!(response.decision(), expected, "{action}");
        assert_eq!(response.diagnostics().errors().count(), 0);
    }
}
