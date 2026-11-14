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
    let member = OrganizationMember {
        organization_id: 7,
        user_id: 11,
        roles: vec![role(
            1,
            r#"permit(principal in Compallm::Role::"7:1", action == Compallm::Action::"provider.create", resource == Compallm::Organization::"7");"#,
        )],
        groups: vec![Group {
            id: 4,
            organization_id: 7,
            name: String::new(),
            description: String::new(),
            roles: vec![role(
                2,
                r#"permit(principal in Compallm::Role::"7:2", action == Compallm::Action::"provider.read", resource == Compallm::Organization::"7");"#,
            )],
        }],
    };
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
        roles: vec![role(1, permit)],
        groups: vec![Group {
            id: 4,
            organization_id: 7,
            name: String::new(),
            description: String::new(),
            roles: vec![role(2, forbid)],
        }],
    };
    let permission = MemberPermission::try_from(access(7, &member)).unwrap();
    assert!(matches!(
        Action::ProviderRead.check_permission(&permission),
        Err(MemberPermissionError::Denied)
    ));
    // A valid policy can still fail at evaluation, such as arithmetic overflow.
    member.groups[0].roles[0].cedar_source = r#"forbid(principal, action == Compallm::Action::"provider.read", resource == Compallm::Organization::"7") when { 9223372036854775807 + 1 > 0 };"#.into();
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
        roles: vec![owner.clone()],
        groups: vec![],
    };
    let permission = MemberPermission::try_from(access(7, &actor)).unwrap();
    let group = Group {
        id: 23,
        organization_id: 7,
        name: "Owners".into(),
        description: String::new(),
        roles: vec![owner],
    };
    let target = OrganizationMember {
        organization_id: 7,
        user_id: 1,
        roles: vec![],
        groups: vec![],
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
        roles: vec![role(11, source)],
        groups: vec![],
    };
    let permission = MemberPermission::try_from(access(7, &actor)).unwrap();
    let group = Group {
        id: 23,
        organization_id: 7,
        name: "Restricted".into(),
        description: String::new(),
        roles: vec![],
    };
    let target = OrganizationMember {
        organization_id: 7,
        user_id: 1,
        roles: vec![role(99, "")],
        groups: vec![],
    };
    assert!(matches!(
        (&group, &target)
            .for_action(Action::GroupMemberAdd)
            .check_permission(&permission),
        Err(MemberPermissionError::Denied)
    ));
}
