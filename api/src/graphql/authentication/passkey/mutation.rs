use super::{PasskeyCeremony, PasskeySummary, input::*};
use crate::{
    graphql::{
        authentication::{
            SessionCredentials,
            input::{AuthenticationPurpose, CredentialDelivery},
        },
        base_type::Payload,
        error::Error,
    },
    http::authentication::{AuthenticationContext, CookieDelivery, UserContext},
};
use compallm_core::authentication::passkey::PasskeyService;
use juniper::ID;
use validify::Validify;
pub async fn registration_start(
    user: &UserContext,
    identity: &AuthenticationContext,
    service: &PasskeyService,
) -> Result<Payload<PasskeyCeremony>, Error> {
    Ok(PasskeyCeremony::try_from(
        service
            .registration_start(user.require().await?.access(), &identity.passkey_change()?)
            .await?,
    )?
    .into())
}
pub async fn registration_finish(
    mut input: PasskeyFinishInput,
    mut name: PasskeyRegistrationInput,
    user: &UserContext,
    identity: &AuthenticationContext,
    service: &PasskeyService,
) -> Result<Payload<PasskeySummary>, Error> {
    input.validify()?;
    name.validify()?;
    let response = serde_json::from_value(serde_json::Value::from(&input))
        .map_err(|_| Error::invalid_input())?;
    Ok(PasskeySummary::from(
        service
            .registration_finish(
                user.require().await?.access(),
                &identity.passkey_change()?,
                &input.token,
                response,
                &name.name,
            )
            .await?,
    )
    .into())
}
pub async fn authentication_start(
    purpose: AuthenticationPurpose,
    user: &UserContext,
    service: &PasskeyService,
) -> Result<Payload<PasskeyCeremony>, Error> {
    let ceremony = match purpose {
        AuthenticationPurpose::Login => service.authentication_start().await?,
        AuthenticationPurpose::PasskeyChange => {
            service
                .reauthentication_start(user.require().await?.access())
                .await?
        }
    };
    Ok(PasskeyCeremony::try_from(ceremony)?.into())
}
pub async fn authentication_finish(
    mut input: PasskeyFinishInput,
    user: &UserContext,
    cookies: &CookieDelivery,
    service: &PasskeyService,
) -> Result<Payload<SessionCredentials>, Error> {
    input.validify()?;
    let mode = input.delivery.ok_or_else(Error::invalid_input)?;
    if mode == CredentialDelivery::Cookie {
        cookies.validate_origin()?;
    }
    let response = serde_json::from_value(serde_json::Value::from(&input))
        .map_err(|_| Error::invalid_input())?;
    if input.purpose == Some(AuthenticationPurpose::PasskeyChange) {
        let grant = service
            .reauthentication_finish(user.require().await?.access(), &input.token, response)
            .await?;
        return Ok(SessionCredentials::try_from((grant, mode, cookies))?.into());
    }
    let tokens = service
        .authentication_finish(&input.token, response)
        .await?;
    Ok(SessionCredentials::try_from((tokens, mode, cookies))?.into())
}
pub async fn remove(
    id: ID,
    user: &UserContext,
    identity: &AuthenticationContext,
    service: &PasskeyService,
) -> Result<Payload<ID>, Error> {
    let id = id.parse::<u64>().map_err(|_| Error::invalid_input())?;
    Ok(ID::new(
        service
            .remove(
                user.require().await?.access(),
                &identity.passkey_change()?,
                id,
            )
            .await?
            .to_string(),
    )
    .into())
}
pub async fn update(
    id: ID,
    mut input: PasskeyRegistrationInput,
    user: &UserContext,
    identity: &AuthenticationContext,
    service: &PasskeyService,
) -> Result<Payload<PasskeySummary>, Error> {
    input.validify()?;
    let id = id.parse::<u64>().map_err(|_| Error::invalid_input())?;
    Ok(PasskeySummary::from(
        service
            .rename(
                user.require().await?.access(),
                &identity.passkey_change()?,
                id,
                &input.name,
            )
            .await?,
    )
    .into())
}
