use compallm_core::{
    authentication::{Session, UserAccessToken},
    authorization::{MemberPermission, PolicySnapshot, policy::Action},
    organization::{OrganizationMember, Role},
};
use std::time::Duration;

pub fn permission(organization_id: u64, actions: &[Action]) -> MemberPermission {
    let actions = actions
        .iter()
        .map(|action| format!("Compallm::Action::\"{}\"", action.name()))
        .collect::<Vec<_>>()
        .join(",");
    let source = if actions.is_empty() {
        String::new()
    } else {
        format!(
            "permit(principal, action in [{actions}], resource in Compallm::Organization::\"{organization_id}\");"
        )
    };
    from_source(organization_id, &source)
}

pub fn from_source(organization_id: u64, source: &str) -> MemberPermission {
    let member = OrganizationMember {
        organization_id,
        user_id: 100,
        roles: vec![Role {
            id: 1,
            organization_id,
            name: String::new(),
            description: String::new(),
            cedar_source: source.to_owned(),
            org_owner: false,
        }],
        groups: Vec::new(),
    };
    MemberPermission::try_from(
        UserAccessToken::new(
            Session {
                id: 1,
                user_id: 100,
                organization_id,
                authorization_version: 1,
                revoked_at: None,
            },
            PolicySnapshot::new(&member).unwrap(),
            Duration::from_secs(60),
        )
        .unwrap(),
    )
    .unwrap()
}
