use compallm_api::graphql::organization::member::input::{MemberFilterInput, MemberMatchInput};
use compallm_core::{
    authentication::{Session, UserAccessToken},
    authorization::{MemberPermission, PolicySnapshot},
    organization::Group,
    persistent::{DbContext, organization::OrganizationRepository as _},
};
use compallm_postgres::{
    postgres_connection::PostgresConnection, repository::organization::OrganizationRepository,
};
use compallm_query::{
    Cursor, Pagination,
    filter::{IDFilter, StringFilter},
};
use sea_orm::{Database, EntityTrait, ModelTrait, Set};
use std::{sync::Arc, time::Duration};
const ORG: u64 = 7_000_000_000_000_000_000;
fn filter(role: u64, email: Option<&str>) -> MemberFilterInput {
    MemberFilterInput {
        role_id: Some(IDFilter {
            eq: Some(juniper::ID::new(role.to_string())),
        }),
        or: email.map(|email| {
            vec![MemberMatchInput {
                email: Some(StringFilter {
                    ilike: Some(email.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        }),
        ..Default::default()
    }
}
#[tokio::test]
#[ignore = "requires COMPALLM_TEST_DATABASE_URL with migrated debug seed"]
async fn role_filter_includes_inherited_members_before_paging_and_binds_cursors()
-> anyhow::Result<()> {
    let pool = Arc::new(Database::connect(std::env::var("COMPALLM_TEST_DATABASE_URL")?).await?);
    let root = DbContext::new(Arc::new(PostgresConnection::new(pool.clone())));
    compallm_postgres::init_id_generator("member-role-filter")?;
    let owner = root
        .scope(OrganizationRepository.find_membership(ORG, ORG + 100))
        .await?
        .unwrap()
        .0;
    let member_role = root
        .scope(OrganizationRepository.find_role(ORG, ORG + 3))
        .await?
        .unwrap();
    let group = Group::create(
        ORG,
        format!("Role filter {}", compallm_utils::time::unix_timestamp()),
        "",
        vec![member_role],
    )
    .await?;
    let group = root
        .scope(OrganizationRepository.create_group(group))
        .await?;
    type Links = <compallm_postgres::OrganizationGroupMemberSchema as ModelTrait>::Entity;
    type Link = <Links as EntityTrait>::ActiveModel;
    Links::insert(Link {
        organization_id: Set(ORG as i64),
        group_id: Set(group.id as i64),
        user_id: Set((ORG + 101) as i64),
    })
    .exec(pool.as_ref())
    .await?;
    let second_group = Group::create(
        ORG,
        format!("Second group {}", compallm_utils::time::unix_timestamp()),
        "",
        vec![],
    )
    .await?;
    let second_group = root
        .scope(OrganizationRepository.create_group(second_group))
        .await?;
    Links::insert(Link {
        organization_id: Set(ORG as i64),
        group_id: Set(second_group.id as i64),
        user_id: Set((ORG + 101) as i64),
    })
    .exec(pool.as_ref())
    .await?;
    let permission = Arc::new(MemberPermission::try_from(UserAccessToken::new(
        Session {
            id: 1,
            user_id: ORG + 100,
            organization_id: ORG,
            authorization_version: 1,
            revoked_at: None,
            metadata: Default::default(),
        },
        PolicySnapshot::new(&owner)?,
        Duration::from_secs(600),
    )?)?);
    let restricted = root
        .fresh()?
        .restrict(async move { Ok::<_, std::convert::Infallible>(permission) })?;
    let first = restricted
        .scope(OrganizationRepository.list_members(
            Box::new(filter(ORG + 3, None)),
            Pagination {
                first: Some(1),
                ..Default::default()
            },
            vec![],
            "role-test",
            None,
        ))
        .await?;
    assert_eq!(first.edges.len(), 1);
    assert!(first.page_info.has_next_page);
    assert!(first.edges[0].node.effective_role_ids.contains(&(ORG + 3)));
    let all = restricted
        .scope(OrganizationRepository.list_members(
            Box::new(filter(ORG + 3, None)),
            Pagination {
                first: Some(100),
                ..Default::default()
            },
            vec![],
            "role-test",
            None,
        ))
        .await?;
    assert!(all.edges.iter().any(|edge| edge.node.id == ORG + 101));
    assert!(all.edges.iter().any(|edge| edge.node.id == ORG + 102));
    assert!(!all.edges.iter().any(|edge| edge.node.id == ORG + 100));
    let inherited_only = all
        .edges
        .iter()
        .find(|edge| edge.node.id == ORG + 101)
        .unwrap();
    assert_eq!(inherited_only.node.role.iter().len(), 1);
    assert_eq!(inherited_only.node.role.as_ref().unwrap().id, ORG + 2);
    assert!(
        inherited_only
            .node
            .groups
            .iter()
            .any(|info| info.id == group.id && info.name == group.name)
    );
    assert!(
        inherited_only
            .node
            .groups
            .iter()
            .any(|info| info.id == second_group.id)
    );
    let combined = restricted
        .scope(OrganizationRepository.list_members(
            Box::new(filter(ORG + 3, Some("admin@compallm.com"))),
            Pagination {
                first: Some(20),
                ..Default::default()
            },
            vec![],
            "role-test",
            None,
        ))
        .await?;
    assert_eq!(combined.edges.len(), 1);
    assert_eq!(combined.edges[0].node.id, ORG + 101);
    let mismatched = restricted
        .scope(OrganizationRepository.list_members(
            Box::new(filter(ORG + 1, None)),
            Pagination {
                first: Some(1),
                after: first.page_info.end_cursor.map(Cursor),
                ..Default::default()
            },
            vec![],
            "role-test",
            None,
        ))
        .await;
    assert!(mismatched.is_err());
    let prioritized = restricted
        .scope(OrganizationRepository.list_members(
            Box::new(filter(ORG + 3, None)),
            Pagination {
                first: Some(1),
                ..Default::default()
            },
            vec![compallm_query::OrderBy {
                field: "display_name".into(),
                direction: compallm_query::OrderByDirection::DescNullsLast,
            }],
            "role-test",
            Some(ORG + 102),
        ))
        .await?;
    assert_eq!(prioritized.edges[0].node.id, ORG + 102);
    let next = restricted
        .scope(OrganizationRepository.list_members(
            Box::new(filter(ORG + 3, None)),
            Pagination {
                first: Some(20),
                after: prioritized.page_info.end_cursor.map(Cursor),
                ..Default::default()
            },
            vec![compallm_query::OrderBy {
                field: "display_name".into(),
                direction: compallm_query::OrderByDirection::DescNullsLast,
            }],
            "role-test",
            Some(ORG + 102),
        ))
        .await?;
    assert!(!next.edges.iter().any(|edge| edge.node.id == ORG + 102));
    let excluded = restricted
        .scope(OrganizationRepository.list_members(
            Box::new(filter(ORG + 3, None)),
            Pagination {
                first: Some(100),
                ..Default::default()
            },
            vec![],
            "role-test",
            Some(ORG + 100),
        ))
        .await?;
    assert!(!excluded.edges.iter().any(|edge| edge.node.id == ORG + 100));
    let foreign = restricted
        .scope(OrganizationRepository.list_members(
            Box::new(filter(ORG + 999, None)),
            Pagination {
                first: Some(20),
                ..Default::default()
            },
            vec![],
            "role-test",
            None,
        ))
        .await?;
    assert!(foreign.edges.is_empty());
    let expected_joined = hero::distributed_id::id_to_unix_timestamp(ORG + 101) / 1000;
    assert_eq!(inherited_only.node.joined_at(), expected_joined);
    let foreign_user = compallm_core::user::User::from_email(&format!(
        "directory-foreign-{}@example.com",
        group.id
    ))
    .await?;
    use compallm_core::persistent::user::UserRepository as _;
    root.scope(compallm_postgres::repository::user::UserRepository.create(foreign_user.clone()))
        .await?;
    let other_org = compallm_core::organization::Organization::new("Directory isolation").await?;
    root.scope(OrganizationRepository.create(other_org.clone()))
        .await?;
    let other_role =
        compallm_core::organization::Role::new(other_org.id, "Foreign", "", "").await?;
    root.scope(OrganizationRepository.create_roles(vec![other_role.clone()]))
        .await?;
    root.scope(OrganizationRepository.add_member(
        compallm_core::organization::OrganizationMember {
            organization_id: other_org.id,
            user_id: foreign_user.id,
            role: other_role,
            groups: vec![],
        },
    ))
    .await?;
    let scoped_all = restricted
        .scope(OrganizationRepository.list_members(
            Box::new(MemberFilterInput::default()),
            Pagination {
                first: Some(100),
                ..Default::default()
            },
            vec![],
            "isolation",
            Some(foreign_user.id),
        ))
        .await?;
    assert!(
        !scoped_all
            .edges
            .iter()
            .any(|edge| edge.node.id == foreign_user.id)
    );
    root.scope(OrganizationRepository.delete_group(ORG, second_group.id))
        .await?;
    root.scope(OrganizationRepository.delete_group(ORG, group.id))
        .await?;
    Ok(())
}
