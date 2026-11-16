use super::{PasskeyCeremony, PasskeySummary, input::*};
use crate::{
    graphql::{
        authentication::{SessionCredentials, input::CredentialDelivery},
        base_type::Payload,
        error::Error,
    },
    http::authentication::{CookieDelivery, UserContext},
};
use compallm_core::authentication::passkey::PasskeyService;
use juniper::ID;
use validify::Validify;
pub async fn registration_start(
    user: &UserContext,
    service: &PasskeyService,
) -> Result<Payload<PasskeyCeremony>, Error> {
    Ok(PasskeyCeremony::try_from(
        service
            .registration_start(user.require().await?.access())
            .await?,
    )?
    .into())
}
pub async fn registration_finish(
    mut input: PasskeyFinishInput,
    mut name: PasskeyRegistrationInput,
    user: &UserContext,
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
                &input.token,
                response,
                &name.name,
            )
            .await?,
    )
    .into())
}
pub async fn authentication_start(
    service: &PasskeyService,
) -> Result<Payload<PasskeyCeremony>, Error> {
    Ok(PasskeyCeremony::try_from(service.authentication_start().await?)?.into())
}
pub async fn authentication_finish(
    mut input: PasskeyFinishInput,
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
    let tokens = service
        .authentication_finish(&input.token, response)
        .await?;
    Ok(SessionCredentials::try_from((tokens, mode, cookies))?.into())
}
pub async fn reauthentication_start(
    user: &UserContext,
    service: &PasskeyService,
) -> Result<Payload<PasskeyCeremony>, Error> {
    Ok(PasskeyCeremony::try_from(
        service
            .reauthentication_start(user.require().await?.access())
            .await?,
    )?
    .into())
}
pub async fn reauthentication_finish(
    mut input: PasskeyFinishInput,
    user: &UserContext,
    service: &PasskeyService,
) -> Result<Payload<bool>, Error> {
    input.validify()?;
    let response = serde_json::from_value(serde_json::Value::from(&input))
        .map_err(|_| Error::invalid_input())?;
    service
        .reauthentication_finish(user.require().await?.access(), &input.token, response)
        .await?;
    Ok(true.into())
}
pub async fn remove(
    id: ID,
    user: &UserContext,
    service: &PasskeyService,
) -> Result<Payload<ID>, Error> {
    let id = id.parse::<u64>().map_err(|_| Error::invalid_input())?;
    Ok(ID::new(
        service
            .remove(user.require().await?.access(), id)
            .await?
            .to_string(),
    )
    .into())
}
pub async fn email_request(
    user: &UserContext,
    service: &PasskeyService,
) -> Result<Payload<PasskeyCeremony>, Error> {
    Ok(PasskeyCeremony::try_from(
        service
            .email_request(user.require().await?.access())
            .await?,
    )?
    .into())
}
pub async fn email_confirm(
    mut input: EmailReauthenticationInput,
    user: &UserContext,
    service: &PasskeyService,
) -> Result<Payload<bool>, Error> {
    input.validify()?;
    service
        .email_confirm(user.require().await?.access(), &input.token, &input.code)
        .await?;
    Ok(true.into())
}

pub async fn update(
    id: ID,
    mut input: PasskeyRegistrationInput,
    user: &UserContext,
    service: &PasskeyService,
) -> Result<Payload<PasskeySummary>, Error> {
    input.validify()?;
    let id = id.parse::<u64>().map_err(|_| Error::invalid_input())?;
    Ok(PasskeySummary::from(
        service
            .rename(user.require().await?.access(), id, &input.name)
            .await?,
    )
    .into())
}
