use super::{
    EmailChallenge, SessionCredentials,
    input::{CredentialDelivery, EmailCodeInput, EmailInput},
};
use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authentication::{AuthenticationContext, CookieDelivery, UserContext},
};
use compallm_core::authentication::AuthService;
use validify::Validify;

/// Request signup email admission. Reject existing accounts before sending or issuing a challenge.
pub(crate) async fn signup_email_request(
    input: EmailInput,
    auth: &AuthService,
    identity: &AuthenticationContext,
    cookies: &CookieDelivery,
) -> Result<Payload<EmailChallenge>, Error> {
    let mut input = input;
    input.validify()?;
    let mode = input.delivery.unwrap_or_default();
    if mode == CredentialDelivery::Cookie {
        cookies.validate_origin()?;
    }
    (
        identity.execute(auth.signup_email(&input.email)).await?,
        mode,
        cookies,
    )
        .try_into()
        .map(|node: EmailChallenge| node.into())
}

/// Request signin email admission for an existing active account.
pub(crate) async fn signin_email_request(
    input: EmailInput,
    auth: &AuthService,
    identity: &AuthenticationContext,
    cookies: &CookieDelivery,
) -> Result<Payload<EmailChallenge>, Error> {
    let mut input = input;
    input.validify()?;
    let mode = input.delivery.unwrap_or_default();
    if mode == CredentialDelivery::Cookie {
        cookies.validate_origin()?;
    }
    (
        identity.execute(auth.signin_email(&input.email)).await?,
        mode,
        cookies,
    )
        .try_into()
        .map(|node: EmailChallenge| node.into())
}

/// Confirm the middleware-verified OTP challenge and atomically create the account and session.
pub(crate) async fn signup_email_confirm(
    input: EmailCodeInput,
    auth: &AuthService,
    identity: &AuthenticationContext,
    cookies: &CookieDelivery,
) -> Result<Payload<SessionCredentials>, Error> {
    let mut input = input;
    input.validify()?;
    let mode = input.delivery.unwrap_or_default();
    if mode == CredentialDelivery::Cookie {
        cookies.validate_origin()?;
    }
    (
        identity
            .execute(auth.signup(identity.otp().await?, &input.code))
            .await?,
        mode,
        cookies,
    )
        .try_into()
        .map(|node: SessionCredentials| node.into())
}

/// Confirm an existing user's middleware-verified OTP challenge and create a scoped session.
pub(crate) async fn signin_email_confirm(
    mut input: EmailCodeInput,
    auth: &AuthService,
    identity: &AuthenticationContext,
    cookies: &CookieDelivery,
) -> Result<Payload<SessionCredentials>, Error> {
    input.validify()?;
    let mode = input.delivery.unwrap_or_default();
    if mode == CredentialDelivery::Cookie {
        cookies.validate_origin()?;
    }
    (
        identity
            .execute(auth.signin(identity.otp().await?, &input.code))
            .await?,
        mode,
        cookies,
    )
        .try_into()
        .map(|node: SessionCredentials| node.into())
}

/// Explicit refresh Bearer only. Rebuild current authority; never set browser cookies.
pub(crate) async fn session_refresh(
    auth: &AuthService,
    identity: &AuthenticationContext,
    cookies: &CookieDelivery,
) -> Result<Payload<SessionCredentials>, Error> {
    (
        identity
            .execute(auth.refresh(identity.refresh().await?))
            .await?,
        CredentialDelivery::Token,
        cookies,
    )
        .try_into()
        .map(|node: SessionCredentials| node.into())
}

/// Revoke this verified login only. Clear cookie credentials after successful revocation.
pub(crate) async fn session_revoke(
    auth: &AuthService,
    identity: &AuthenticationContext,
    user: &UserContext,
    cookies: &CookieDelivery,
) -> Result<Payload<bool>, Error> {
    identity
        .execute(auth.signout(user.require().await?.access()))
        .await?;
    cookies.clear()?;
    Ok(true.into())
}
