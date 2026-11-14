use compallm_core::organization::{Group, OrganizationMember, Role};

#[test]
fn assignment_is_scoped_and_idempotent() {
    let role = Role {
        id: 1,
        organization_id: 10,
        name: "Role".into(),
        description: String::new(),
        cedar_source: String::new(),
        org_owner: false,
    };
    let group = Group {
        id: 2,
        organization_id: 10,
        name: "Group".into(),
        description: String::new(),
        roles: vec![role.clone()],
    };
    let mut member = OrganizationMember {
        organization_id: 10,
        user_id: 9,
        roles: vec![],
        groups: vec![],
    };
    member.assign_role(role.clone()).unwrap();
    member.assign_role(role.clone()).unwrap();
    member.assign_group(group.clone()).unwrap();
    member.assign_group(group.clone()).unwrap();
    assert_eq!(member.roles.len(), 1);
    assert_eq!(member.groups.len(), 1);
    assert!(
        member
            .assign_role(Role {
                organization_id: 11,
                ..role
            })
            .is_err()
    );
    assert!(
        member
            .assign_group(Group {
                organization_id: 11,
                ..group
            })
            .is_err()
    );
    assert_eq!(member.roles.len(), 1);
    assert_eq!(member.groups.len(), 1);
}

#[test]
fn unassignment_removes_only_the_requested_direct_relationship() {
    let role = Role {
        id: 1,
        organization_id: 10,
        name: "Role".into(),
        description: String::new(),
        cedar_source: String::new(),
        org_owner: false,
    };
    let group = Group {
        id: 2,
        organization_id: 10,
        name: "Group".into(),
        description: String::new(),
        roles: vec![role.clone()],
    };
    let mut member = OrganizationMember {
        organization_id: 10,
        user_id: 9,
        roles: vec![role],
        groups: vec![group],
    };
    member.unassign_role(999).unwrap();
    assert_eq!(member.roles.len(), 1);
    member.unassign_role(1).unwrap();
    member.unassign_role(1).unwrap();
    assert!(member.roles.is_empty());
    assert_eq!(member.effective_roles().count(), 1);
    member.unassign_group(999).unwrap();
    assert_eq!(member.groups.len(), 1);
    member.unassign_group(2).unwrap();
    member.unassign_group(2).unwrap();
    assert!(member.groups.is_empty());
    assert_eq!(member.effective_roles().count(), 0);
}

#[test]
fn organization_owner_role_cannot_be_unassigned() {
    use compallm_core::organization::member::MembershipAssignmentError;
    let owner = Role {
        id: 7,
        organization_id: 10,
        name: "renamed".into(),
        description: String::new(),
        cedar_source: String::new(),
        org_owner: true,
    };
    let mut member = OrganizationMember {
        organization_id: 10,
        user_id: 9,
        roles: vec![owner.clone()],
        groups: vec![],
    };
    assert!(matches!(
        member.unassign_role(owner.id),
        Err(MembershipAssignmentError::OwnerRole)
    ));
    assert_eq!(member.roles, vec![owner]);
}

#[test]
fn group_with_owner_role_can_be_unassigned() {
    let owner = Role {
        id: 7,
        organization_id: 10,
        name: "Owner".into(),
        description: String::new(),
        cedar_source: String::new(),
        org_owner: true,
    };
    let group = Group {
        id: 2,
        organization_id: 10,
        name: "Group".into(),
        description: String::new(),
        roles: vec![owner],
    };
    let mut member = OrganizationMember {
        organization_id: 10,
        user_id: 9,
        roles: vec![],
        groups: vec![group.clone()],
    };
    member.unassign_group(group.id).unwrap();
    assert!(member.groups.is_empty());
}
