use super::PolicyAction;
use crate::{
    graphql::{base_type::Collection, error::Error},
    http::authentication::UserContext,
};
use compallm_core::authorization::policy::Action;

pub(crate) async fn list(user: &UserContext) -> Result<Collection<PolicyAction>, Error> {
    let session = &user.require().await?.access().session;
    let mut nodes: Vec<_> = Action::ALL
        .iter()
        .copied()
        .map(PolicyAction::from)
        .collect();
    nodes.sort_by(|a, b| a.name.cmp(&b.name));
    let scope = format!(
        "organization-policy-actions:{}:{}",
        session.organization_id, session.user_id
    );
    Collection::complete(nodes, &scope, |node| vec![Some(node.name.clone())])
        .map_err(|_| Error::internal())
}
