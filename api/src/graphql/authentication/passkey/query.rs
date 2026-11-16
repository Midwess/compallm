use super::PasskeySummary;
use crate::{
    graphql::{
        base_type::{Collection, Payload},
        error::Error,
    },
    http::authentication::{AuthenticationContext, UserContext},
};
use compallm_core::{
    authentication::passkey::PasskeyService, persistent::passkey::PasskeyRepository,
};
pub async fn methods(email: &str, service: &PasskeyService) -> Result<Payload<bool>, Error> {
    Ok(service.methods(email).await?.into())
}
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
