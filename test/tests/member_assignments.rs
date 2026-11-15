use compallm_core::organization::{Group, OrganizationMember, Role, group::GroupCreationError};
fn role(id: u64, org: u64, owner: bool) -> Role {
    Role {
        id,
        organization_id: org,
        name: format!("Role {id}"),
        description: String::new(),
        cedar_source: String::new(),
        org_owner: owner,
    }
}
#[test]
fn replacing_direct_role_preserves_multiple_groups_and_rejects_foreign_role() {
    let group = Group::from_parts(
        20,
        10,
        "One".into(),
        String::new(),
        vec![role(3, 10, false)],
    )
    .unwrap();
    let mut member = OrganizationMember {
        organization_id: 10,
        user_id: 9,
        role: role(1, 10, false),
        groups: vec![group],
    };
    member
        .assign_group(Group::from_parts(21, 10, "Two".into(), String::new(), vec![]).unwrap())
        .unwrap();
    member.assign_role(role(2, 10, false)).unwrap();
    assert_eq!(member.role.id, 2);
    assert_eq!(member.groups.len(), 2);
    assert!(member.assign_role(role(4, 11, false)).is_err());
    assert_eq!(member.role.id, 2);
    assert_eq!(
        member.effective_roles().map(|r| r.id).collect::<Vec<_>>(),
        vec![2, 3]
    );
}
#[tokio::test]
async fn owner_is_rejected_by_group_creation_rehydration_and_role_replacement() {
    assert!(matches!(
        Group::new(10, "Group", "", vec![role(1, 10, true)]).await,
        Err(GroupCreationError::OwnerRole)
    ));
    assert!(matches!(
        Group::from_parts(
            20,
            10,
            "Group".into(),
            String::new(),
            vec![role(1, 10, true)]
        ),
        Err(GroupCreationError::OwnerRole)
    ));
    let mut group = Group::from_parts(
        20,
        10,
        "Group".into(),
        String::new(),
        vec![role(2, 10, false)],
    )
    .unwrap();
    assert!(matches!(
        group.replace_roles(vec![role(1, 10, true)]),
        Err(GroupCreationError::OwnerRole)
    ));
    assert_eq!(group.roles()[0].id, 2);
}
