use super::PasskeySummary;
use crate::{
    graphql::{base_type::Collection, error::Error},
    http::authentication::{AuthenticationContext, UserContext},
};
use compallm_core::persistent::passkey::PasskeyRepository;
pub async fn list(
    user: &UserContext,
    identity: &AuthenticationContext,
    repository: &dyn PasskeyRepository,
) -> Result<Collection<PasskeySummary>, Error> {
    let actor = user.require().await?.access();
    let records = identity
        .execute(repository.list(actor.session.user_id))
        .await?;
    Collection::complete(
        records.into_iter().map(PasskeySummary::from).collect(),
        &format!("passkeys:{}", actor.session.user_id),
        |p| p.cursor_position(),
    )
    .map_err(|_| Error::internal())
}
