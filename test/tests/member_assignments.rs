use compallm_core::organization::{Group, OrganizationMember, Role};

#[test]
fn assignment_is_scoped_and_idempotent() {
    let role = Role {
        id: 1,
        organization_id: 10,
        name: "Role".into(),
        description: String::new(),
        cedar_source: String::new(),
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
