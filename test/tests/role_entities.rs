use cedar_policy::{
    Authorizer, Context, Decision, Entities, Entity, EntityUid, PolicySet, Request,
    RestrictedExpression, ValidationMode, Validator,
};
use compallm_core::{
    authorization::all_policies,
    organization::{Group, OrganizationMember, Role},
};
use std::{collections::HashMap, str::FromStr};
fn member(index: usize) -> OrganizationMember {
    OrganizationMember {
        organization_id: 42,
        user_id: 100,
        roles: vec![Role::defaults(42, [901, 502, 73]).remove(index)],
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
#[test]
fn all_defaults_have_explicit_valid_cedar_source() {
    let validator = Validator::new(all_policies().clone());
    for role in Role::defaults(42, [901, 502, 73]) {
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
#[test]
fn default_grants_differentiate_owner_admin_and_member() {
    for (index, delete, roles, use_model) in [
        (0, Decision::Allow, Decision::Allow, Decision::Allow),
        (1, Decision::Deny, Decision::Allow, Decision::Allow),
        (2, Decision::Deny, Decision::Deny, Decision::Allow),
    ] {
        let member = member(index);
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
#[test]
fn group_roles_keep_provenance_and_inherit_without_copying_to_direct_roles() {
    let mut member = member(2);
    member.roles.clear();
    member.groups.push(Group {
        id: 9,
        organization_id: 42,
        name: "Operations".into(),
        description: String::new(),
        roles: vec![Role::defaults(42, [901, 502, 73]).remove(1)],
    });
    assert!(member.roles.is_empty());
    assert_eq!(
        decision(&member, "role.list", "Organization", 42, 0, true),
        Decision::Allow
    );
    let mut foreign = member.clone();
    foreign.groups[0].roles[0].organization_id = 43;
    assert!(Entities::try_from(&foreign).is_err());
}

#[test]
fn statement_identity_survives_formatting_and_reordering() {
    let mut role = Role::defaults(42, [901, 502, 73]).remove(0);
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

#[test]
fn empty_groups_remain_inside_the_organization_resource_scope() {
    let member = member(0);
    let group = Group {
        id: 77,
        organization_id: 42,
        name: "Empty team".into(),
        description: String::new(),
        roles: vec![],
    };
    let entity = Entity::try_from(&group).unwrap();
    let target = entity.uid();
    let data = Entities::try_from(&member)
        .unwrap()
        .add_entities([entity], Some(all_policies()))
        .unwrap();
    let policies = PolicySet::try_from(&member.roles[0]).unwrap();
    let request = Request::new(
        "Compallm::User::\"100\"".parse().unwrap(),
        "Compallm::Action::\"group.metadata.read\"".parse().unwrap(),
        target,
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

#[test]
fn default_permissions_and_owner_restrictions_use_supplied_record_ids() {
    let roles = Role::defaults(42, [901, 502, 73]);
    let admin = OrganizationMember {
        organization_id: 42,
        user_id: 100,
        roles: vec![roles[1].clone()],
        groups: vec![],
    };
    assert_eq!(
        decision(&admin, "role.assign", "Role", 42, 901, true),
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
