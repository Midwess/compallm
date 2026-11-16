use super::{
    EmailChallenge, SessionCredentials,
    input::{AuthenticationPurpose, CredentialDelivery, EmailCodeInput, EmailInput},
};
use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authentication::{AuthenticationContext, CookieDelivery, UserContext},
};
use compallm_core::{
    authentication::AuthService, organization::invitation_service::InvitationService,
};
use validify::Validify;

/// Request signup email admission. Reject existing accounts before sending or issuing a challenge.
pub(crate) async fn signup_email_request(
    input: EmailInput,
    auth: &AuthService,
    invitations: &InvitationService,
    identity: &AuthenticationContext,
    cookies: &CookieDelivery,
) -> Result<Payload<EmailChallenge>, Error> {
    let mut input = input;
    input.validify()?;
    if input.purpose != Some(AuthenticationPurpose::Login) {
        return Err(Error::invalid_input());
    }
    let mode = input.delivery.unwrap_or_default();
    if mode == CredentialDelivery::Cookie {
        cookies.validate_origin()?;
    }
    (
        match if input.invitation.unwrap_or(false) {
            Some(
                identity
                    .invitation()
                    .await?
                    .ok_or(compallm_core::authentication::AuthError::InvalidInvitation)?,
            )
        } else {
            if mode == CredentialDelivery::Cookie {
                cookies.clear_invitation()?;
            }
            None
        } {
            Some(invitation) => {
                if mode != CredentialDelivery::Cookie {
                    return Err(Error::invalid_input());
                }
                identity
                    .execute(invitations.invitation_email(&invitation, &input.email, true))
                    .await?
            }
            None => identity.execute(auth.signup_email(&input.email)).await?,
        },
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
    invitations: &InvitationService,
    identity: &AuthenticationContext,
    cookies: &CookieDelivery,
    user: &UserContext,
) -> Result<Payload<EmailChallenge>, Error> {
    let mut input = input;
    input.validify()?;
    let mode = input.delivery.unwrap_or_default();
    if mode == CredentialDelivery::Cookie {
        cookies.validate_origin()?;
    }
    if input.purpose == Some(AuthenticationPurpose::PasskeyChange) {
        if input.invitation.unwrap_or(false) {
            return Err(Error::invalid_input());
        }
        let challenge = identity
            .execute(auth.passkey_change_email(user.require().await?.access(), &input.email))
            .await?;
        return EmailChallenge::try_from((
            challenge,
            mode,
            cookies,
            AuthenticationPurpose::PasskeyChange,
        ))
        .map(Into::into);
    }
    (
        match if input.invitation.unwrap_or(false) {
            Some(
                identity
                    .invitation()
                    .await?
                    .ok_or(compallm_core::authentication::AuthError::InvalidInvitation)?,
            )
        } else {
            if mode == CredentialDelivery::Cookie {
                cookies.clear_invitation()?;
            }
            None
        } {
            Some(invitation) => {
                if mode != CredentialDelivery::Cookie {
                    return Err(Error::invalid_input());
                }
                identity
                    .execute(invitations.invitation_email(&invitation, &input.email, false))
                    .await?
            }
            None => identity.execute(auth.signin_email(&input.email)).await?,
        },
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
    invitations: &InvitationService,
    identity: &AuthenticationContext,
    cookies: &CookieDelivery,
) -> Result<Payload<SessionCredentials>, Error> {
    let mut input = input;
    input.validify()?;
    if input.purpose != Some(AuthenticationPurpose::Login) {
        return Err(Error::invalid_input());
    }
    let mode = input.delivery.unwrap_or_default();
    if mode == CredentialDelivery::Cookie {
        cookies.validate_origin()?;
    }
    (
        match if identity.otp().await?.invitation_id.is_some() {
            Some(
                identity
                    .invitation()
                    .await?
                    .ok_or(compallm_core::authentication::AuthError::InvalidInvitation)?,
            )
        } else {
            None
        } {
            Some(invitation) => {
                if mode != CredentialDelivery::Cookie {
                    return Err(Error::invalid_input());
                }
                identity
                    .execute(invitations.invitation_confirm(
                        &invitation,
                        identity.otp().await?,
                        &input.code,
                        true,
                    ))
                    .await?
            }
            None => {
                identity
                    .execute(auth.signup(identity.otp().await?, &input.code))
                    .await?
            }
        },
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
    invitations: &InvitationService,
    identity: &AuthenticationContext,
    cookies: &CookieDelivery,
    user: &UserContext,
) -> Result<Payload<SessionCredentials>, Error> {
    input.validify()?;
    let mode = input.delivery.unwrap_or_default();
    if mode == CredentialDelivery::Cookie {
        cookies.validate_origin()?;
    }
    if input.purpose == Some(AuthenticationPurpose::PasskeyChange) {
        let grant = identity
            .execute(auth.passkey_change_confirm(
                user.require().await?.access(),
                identity.passkey_otp().await?,
                &input.code,
            ))
            .await?;
        return SessionCredentials::try_from((grant, mode, cookies)).map(Into::into);
    }
    (
        match if identity.otp().await?.invitation_id.is_some() {
            Some(
                identity
                    .invitation()
                    .await?
                    .ok_or(compallm_core::authentication::AuthError::InvalidInvitation)?,
            )
        } else {
            None
        } {
            Some(invitation) => {
                if mode != CredentialDelivery::Cookie {
                    return Err(Error::invalid_input());
                }
                identity
                    .execute(invitations.invitation_confirm(
                        &invitation,
                        identity.otp().await?,
                        &input.code,
                        false,
                    ))
                    .await?
            }
            None => {
                identity
                    .execute(auth.signin(identity.otp().await?, &input.code))
                    .await?
            }
        },
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
