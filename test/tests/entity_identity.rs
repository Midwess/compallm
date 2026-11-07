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
}
