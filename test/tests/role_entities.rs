#[path = "support/roles.rs"]
mod role_fixture;
use role_fixture::default_roles;

use cedar_policy::{
    Authorizer, Context, Decision, Entities, Entity, EntityUid, PolicySet, Request,
    RestrictedExpression, ValidationMode, Validator,
};
use compallm_core::{
    authorization::all_policies,
    organization::{Group, OrganizationMember, Role},
};
use std::{collections::HashMap, str::FromStr};

#[tokio::test]
async fn role_color_is_stable_and_warm() {
    let mut role = default_roles(42, [901, 502, 73]).await.remove(0);
    let color = role.color();
    assert_eq!(color.len(), 7);
    assert!(color.starts_with('#'));
    let rgb = u32::from_str_radix(&color[1..], 16).unwrap();
    assert!(rgb >> 16 > (rgb >> 8) & 255);
    assert!(rgb >> 16 > rgb & 255);
    assert_eq!(role.color(), color);
    role.name = "Different role".into();
    assert_ne!(role.color(), color);
}
async fn member(index: usize) -> OrganizationMember {
    OrganizationMember {
        organization_id: 42,
        user_id: 100,
        roles: vec![default_roles(42, [901, 502, 73]).await.remove(index)],
        groups: vec![],
    }
}
fn decision(
    member: &OrganizationMember,
    action: &str,
    kind: &str,
    org: u64,
    id: u64,
    enabled: bool,
) -> Decision {
    let mut entities = Entities::try_from(member).unwrap();
    let org_uid: EntityUid = format!("Compallm::Organization::\"{org}\"")
        .parse()
        .unwrap();
    let uid: EntityUid = if kind == "Organization" {
        org_uid.clone()
    } else {
        format!("Compallm::{kind}::\"{org}:{id}\"").parse().unwrap()
    };
    let mut attrs = HashMap::new();
    if kind != "Organization" {
        attrs.insert(
            "organization".into(),
            RestrictedExpression::new_entity_uid(org_uid.clone()),
        );
    }
    if kind == "Model" {
        attrs.insert("enabled".into(), RestrictedExpression::new_bool(enabled));
    }
    if entities.get(&uid).is_none() {
        entities = entities
            .add_entities(
                [Entity::new(
                    uid.clone(),
                    attrs,
                    if kind == "Organization" {
                        Default::default()
                    } else {
                        [org_uid].into()
                    },
                )
                .unwrap()],
                Some(all_policies()),
            )
            .unwrap();
    }
    let mut policies = PolicySet::new();
    for role in member.effective_roles() {
        for policy in PolicySet::try_from(role).unwrap().policies() {
            policies.add(policy.clone()).unwrap();
        }
    }
    let req = Request::new(
        EntityUid::from_str("Compallm::User::\"100\"").unwrap(),
        format!("Compallm::Action::\"{action}\"").parse().unwrap(),
        uid,
        Context::empty(),
        Some(all_policies()),
    )
    .unwrap();
    let response = Authorizer::new().is_authorized(&req, &policies, &entities);
    assert_eq!(response.diagnostics().errors().count(), 0);
    response.decision()
}
#[tokio::test]
async fn all_defaults_have_explicit_valid_cedar_source() {
    let fixture_roles_0 = default_roles(42, [901, 502, 73]).await;

    let validator = Validator::new(all_policies().clone());
    for role in fixture_roles_0.clone() {
        assert!(!role.cedar_source.is_empty());
        let set = PolicySet::try_from(&role).unwrap();
        let result = validator.validate(&set, ValidationMode::Strict);
        assert!(
            result.validation_passed(),
            "{}: {:?}",
            role.name,
            result.validation_errors().collect::<Vec<_>>()
        );
        assert!(
            role.cedar_source
                .contains(&format!("Compallm::Role::\"42:{}\"", role.id))
        );
    }
}
#[tokio::test]
async fn default_grants_differentiate_owner_admin_and_member() {
    for (index, delete, roles, use_model) in [
        (0, Decision::Allow, Decision::Allow, Decision::Allow),
        (1, Decision::Deny, Decision::Allow, Decision::Allow),
        (2, Decision::Deny, Decision::Deny, Decision::Allow),
    ] {
        let member = member(index).await;
        assert_eq!(
            decision(&member, "organization.delete", "Organization", 42, 0, true),
            delete
        );
        assert_eq!(
            decision(&member, "role.create", "Organization", 42, 0, true),
            roles
        );
        assert_eq!(
            decision(&member, "model.use", "Model", 42, 99, true),
            use_model
        );
        assert_eq!(
            decision(&member, "model.use", "Model", 42, 99, false),
            Decision::Deny
        );
        assert_eq!(
            decision(&member, "model.use", "Model", 43, 99, true),
            Decision::Deny
        );
    }
}
#[tokio::test]
async fn group_roles_keep_provenance_and_inherit_without_copying_to_direct_roles() {
    let fixture_roles_0 = default_roles(42, [901, 502, 73]).await;

    let mut member = member(2).await;
    member.roles.clear();
    member.groups.push(Group {
        id: 9,
        organization_id: 42,
        name: "Operations".into(),
        description: String::new(),
        roles: vec![fixture_roles_0.clone().remove(1)],
    });
    assert!(member.roles.is_empty());
    assert_eq!(
        decision(&member, "role.read", "Organization", 42, 0, true),
        Decision::Allow
    );
    let mut foreign = member.clone();
    foreign.groups[0].roles[0].organization_id = 43;
    assert!(Entities::try_from(&foreign).is_err());
}

#[tokio::test]
async fn statement_identity_survives_formatting_and_reordering() {
    let fixture_roles_0 = default_roles(42, [901, 502, 73]).await;

    let mut role = fixture_roles_0.clone().remove(0);
    let original = PolicySet::try_from(&role).unwrap();
    let ids = original
        .policies()
        .map(|p| p.id().to_string())
        .collect::<std::collections::BTreeSet<_>>();
    role.cedar_source = original
        .policies()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join("\n\n");
    assert_eq!(
        PolicySet::try_from(&role)
            .unwrap()
            .policies()
            .map(|p| p.id().to_string())
            .collect::<std::collections::BTreeSet<_>>(),
        ids
    );
}

#[tokio::test]
async fn empty_groups_remain_inside_the_organization_resource_scope() {
    let member = member(0).await;
    let group = Group {
        id: 77,
        organization_id: 42,
        name: "Empty team".into(),
        description: String::new(),
        roles: vec![],
    };
    let entity = Entity::try_from(&group).unwrap();
    let data = Entities::try_from(&member)
        .unwrap()
        .add_entities([entity], Some(all_policies()))
        .unwrap();
    let policies = PolicySet::try_from(&member.roles[0]).unwrap();
    let request = Request::new(
        "Compallm::User::\"100\"".parse().unwrap(),
        "Compallm::Action::\"group.read\"".parse().unwrap(),
        "Compallm::Organization::\"42\"".parse().unwrap(),
        Context::empty(),
        Some(all_policies()),
    )
    .unwrap();
    assert_eq!(
        Authorizer::new()
            .is_authorized(&request, &policies, &data)
            .decision(),
        Decision::Allow
    );
}

#[tokio::test]
async fn default_permissions_and_owner_restrictions_use_generated_record_ids() {
    compallm_postgres::init_id_generator("generated-role-test").unwrap();
    let fixture_roles_0 = Role::defaults(42).await.unwrap();

    let roles = fixture_roles_0.clone();
    let admin = OrganizationMember {
        organization_id: 42,
        user_id: 100,
        roles: vec![roles[1].clone()],
        groups: vec![],
    };
    assert_eq!(
        decision(&admin, "role.assign", "Role", 42, roles[0].id, true),
        Decision::Deny
    );
    assert_eq!(
        decision(&admin, "role.assign", "Role", 42, 700, true),
        Decision::Allow
    );
    let owner = OrganizationMember {
        organization_id: 42,
        user_id: 100,
        roles: vec![roles[0].clone()],
        groups: vec![],
    };
    assert_eq!(
        decision(&owner, "organization.delete", "Organization", 42, 0, true),
        Decision::Allow
    );
    for role in roles {
        assert!(!role.cedar_source.contains("__"));
        assert!(role.cedar_source.contains(&format!("42:{}", role.id)));
    }
}
