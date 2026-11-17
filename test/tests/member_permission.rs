use std::time::Duration;

use compallm_core::{
    authentication::{Session, UserAccessToken},
    authorization::{
        MemberPermission, MemberPermissionError, PolicySnapshot, condition::CheckPermission,
        policy::Action,
    },
    organization::{Group, OrganizationMember, Role},
};

fn access(organization_id: u64, member: &OrganizationMember) -> UserAccessToken {
    UserAccessToken::new(
        Session {
            id: 1,
            user_id: member.user_id,
            organization_id,
            authorization_version: 1,
            revoked_at: None,
            metadata: Default::default(),
        },
        PolicySnapshot::new(member).unwrap(),
        Duration::from_secs(60),
    )
    .unwrap()
}

fn role(id: u64, source: &str) -> Role {
    Role {
        id,
        organization_id: 7,
        name: String::new(),
        description: String::new(),
        cedar_source: source.into(),
        org_owner: false,
    }
}

#[test]
fn direct_and_group_permissions_apply_to_the_members_organization() {
    let member = OrganizationMember { organization_id: 7, user_id: 11, groups: vec![Group::new(4, 7, String::new(), String::new(), vec![role(
                2,
                r#"permit(principal in Compallm::Role::"7:2", action == Compallm::Action::"provider.read", resource == Compallm::Organization::"7");"#,
            )]).unwrap()], role: role(
            1,
            r#"permit(principal in Compallm::Role::"7:1", action == Compallm::Action::"provider.create", resource == Compallm::Organization::"7");"#,
        ) };
    let permission = MemberPermission::try_from(access(7, &member)).unwrap();
    assert_eq!(permission.user_id(), 11);
    assert_eq!(permission.organization_id(), 7);
    assert!(Action::ProviderCreate.check_permission(&permission).is_ok());
    assert!(Action::ProviderRead.check_permission(&permission).is_ok());
    assert!(matches!(
        Action::ModelRead.check_permission(&permission),
        Err(MemberPermissionError::Denied)
    ));
    // The same policy source cannot grant access in another organization.
    let foreign = MemberPermission::try_from(access(8, &member)).unwrap();
    assert!(matches!(
        Action::ProviderCreate.check_permission(&foreign),
        Err(MemberPermissionError::Denied)
    ));
    assert!(matches!(
        Action::ProviderRead.check_permission(&foreign),
        Err(MemberPermissionError::Denied)
    ));
}

#[test]
fn forbid_and_evaluation_errors_cannot_be_overridden_by_a_permit() {
    let permit = r#"permit(principal, action == Compallm::Action::"provider.read", resource == Compallm::Organization::"7");"#;
    let forbid = r#"forbid(principal, action == Compallm::Action::"provider.read", resource == Compallm::Organization::"7");"#;
    let mut member = OrganizationMember {
        organization_id: 7,
        user_id: 11,
        groups: vec![
            Group::new(4, 7, String::new(), String::new(), vec![role(2, forbid)]).unwrap(),
        ],
        role: role(1, permit),
    };
    let permission = MemberPermission::try_from(access(7, &member)).unwrap();
    assert!(matches!(
        Action::ProviderRead.check_permission(&permission),
        Err(MemberPermissionError::Denied)
    ));
    // A valid policy can still fail at evaluation, such as arithmetic overflow.
    let mut changed = member.groups[0].roles().to_vec();
    changed[0].cedar_source = r#"forbid(principal, action == Compallm::Action::"provider.read", resource == Compallm::Organization::"7") when { 9223372036854775807 + 1 > 0 };"#.into();
    member.groups[0].replace_roles(changed).unwrap();
    let permission = MemberPermission::try_from(access(7, &member)).unwrap();
    assert!(matches!(
        Action::ProviderRead.check_permission(&permission),
        Err(MemberPermissionError::Evaluation)
    ));
}

#[test]
fn self_group_assignment_preserves_the_verified_principal() {
    use compallm_core::authorization::condition::ForAction;
    let owner = role(
        11,
        r#"permit(principal in Compallm::Role::"7:11", action == Compallm::Action::"group.member.add", resource == Compallm::Group::"7:23") when { context.member.user == principal };"#,
    );
    let actor = OrganizationMember {
        organization_id: 7,
        user_id: 1,
        groups: vec![],
        role: owner.clone(),
    };
    let permission = MemberPermission::try_from(access(7, &actor)).unwrap();
    let group = Group::new(
        23,
        7,
        "Owners".into(),
        String::new(),
        vec![Role {
            org_owner: false,
            ..owner
        }],
    )
    .unwrap();
    let target = OrganizationMember {
        organization_id: 7,
        user_id: 1,
        groups: vec![],
        role: compallm_core::organization::Role {
            id: 3,
            organization_id: 7,
            name: "Member".into(),
            description: String::new(),
            cedar_source: String::new(),
            org_owner: false,
        },
    };
    (&group, &target)
        .for_action(Action::GroupMemberAdd)
        .check_permission(&permission)
        .unwrap();
}

#[test]
fn target_membership_cannot_add_authority_to_the_verified_principal() {
    use compallm_core::authorization::condition::ForAction;
    let source = r#"permit(principal in Compallm::Role::"7:99", action == Compallm::Action::"group.member.add", resource == Compallm::Group::"7:23");"#;
    let actor = OrganizationMember {
        organization_id: 7,
        user_id: 1,
        groups: vec![],
        role: role(11, source),
    };
    let permission = MemberPermission::try_from(access(7, &actor)).unwrap();
    let group = Group::new(23, 7, "Restricted".into(), String::new(), vec![]).unwrap();
    let target = OrganizationMember {
        organization_id: 7,
        user_id: 1,
        groups: vec![],
        role: role(99, ""),
    };
    assert!(matches!(
        (&group, &target)
            .for_action(Action::GroupMemberAdd)
            .check_permission(&permission),
        Err(MemberPermissionError::Denied)
    ));
}

#[test]
fn contextual_delegation_requires_both_checks_and_combines_forbids() {
    use compallm_core::authorization::condition::ForAction;
    let grant = role(
        1,
        r#"permit(principal, action == Compallm::Action::"group.role.assign", resource == Compallm::Group::"7:23") when { context.role == Compallm::Role::"7:9" }; permit(principal, action == Compallm::Action::"role.assign", resource == Compallm::Role::"7:9");"#,
    );
    let restriction = role(
        2,
        r#"forbid(principal, action == Compallm::Action::"role.assign", resource == Compallm::Role::"7:9");"#,
    );
    let mut member = OrganizationMember {
        organization_id: 7,
        user_id: 11,
        groups: vec![],
        role: grant,
    };
    let group = Group::new(23, 7, String::new(), String::new(), vec![]).unwrap();
    let target = role(9, "");
    let permission = MemberPermission::try_from(access(7, &member)).unwrap();
    (&group, &target)
        .for_action(Action::GroupRoleAssign)
        .and((&target).for_action(Action::RoleAssign))
        .check_permission(&permission)
        .unwrap();
    let other = role(10, "");
    assert!(matches!(
        (&group, &other)
            .for_action(Action::GroupRoleAssign)
            .check_permission(&permission),
        Err(MemberPermissionError::Denied)
    ));
    member
        .groups
        .push(Group::new(3, 7, String::new(), String::new(), vec![restriction]).unwrap());
    let permission = MemberPermission::try_from(access(7, &member)).unwrap();
    assert!(matches!(
        (&group, &target)
            .for_action(Action::GroupRoleAssign)
            .and((&target).for_action(Action::RoleAssign))
            .check_permission(&permission),
        Err(MemberPermissionError::Denied)
    ));
    let mut foreign = group;
    foreign.organization_id = 8;
    assert!(matches!(
        (&foreign)
            .for_action(Action::GroupMetadataUpdate)
            .check_permission(&permission),
        Err(MemberPermissionError::Denied)
    ));
}
