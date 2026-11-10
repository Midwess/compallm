use compallm_core::authentication::{
    AuthService, EmailOtpAccessToken, IssuedEmailOtp, TokenValidity, UserTokens,
};
use juniper::{GraphQLObject, graphql_object};
use validify::Validify;

use super::{CredentialDelivery, EmailCodeInput, EmailInput};
use crate::{
    graphql::{GraphqlContext, error::Error},
    middleware::authentication::{CookieDelivery, RequestIdentity},
};

/// Exact Unix-second validity and client throttle values encoded as decimal strings.
#[derive(GraphQLObject)]
pub struct TokenLifetime {
    pub issued_at: String,
    pub expires_at: String,
    pub expires_in_seconds: String,
    pub throttle_seconds: String,
}

impl From<(TokenValidity, u64)> for TokenLifetime {
    fn from((validity, throttle): (TokenValidity, u64)) -> Self {
        Self {
            issued_at: validity.issued_at.to_string(),
            expires_at: validity.expires_at.to_string(),
            expires_in_seconds: validity
                .expires_at
                .saturating_sub(compallm_utils::time::unix_timestamp())
                .to_string(),
            throttle_seconds: throttle.to_string(),
        }
    }
}

/// Email admission succeeded; cookie delivery never exposes the OTP credential.
#[derive(GraphQLObject)]
pub struct EmailChallenge {
    pub lifetime: TokenLifetime,
    pub token: Option<String>,
}

/// Timing is always returned; credential values exist only in explicit TOKEN delivery.
#[derive(GraphQLObject)]
pub struct SessionCredentials {
    pub access_lifetime: TokenLifetime,
    pub refresh_lifetime: TokenLifetime,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl TryFrom<(IssuedEmailOtp, CredentialDelivery, &CookieDelivery)> for EmailChallenge {
    type Error = Error;
    fn try_from(
        (issued, mode, cookies): (IssuedEmailOtp, CredentialDelivery, &CookieDelivery),
    ) -> Result<Self, Error> {
        let remaining = issued
            .validity
            .expires_at
            .saturating_sub(compallm_utils::time::unix_timestamp());
        let lifetime = (issued.validity, EmailOtpAccessToken::THROTTLE_SECONDS).into();
        let token = match mode {
            CredentialDelivery::Token => Some(issued.token),
            CredentialDelivery::Cookie => {
                cookies.set_otp(issued.token, remaining)?;
                None
            }
        };
        Ok(Self { lifetime, token })
    }
}

impl TryFrom<(UserTokens, CredentialDelivery, &CookieDelivery)> for SessionCredentials {
    type Error = Error;
    fn try_from(
        (tokens, mode, cookies): (UserTokens, CredentialDelivery, &CookieDelivery),
    ) -> Result<Self, Error> {
        let access_lifetime = (tokens.access_validity.clone(), 0).into();
        let refresh_lifetime = (tokens.refresh_validity.clone(), 0).into();
        let (access_token, refresh_token) = match mode {
            CredentialDelivery::Token => (Some(tokens.access_token), Some(tokens.refresh_token)),
            CredentialDelivery::Cookie => {
                cookies.set_user(tokens)?;
                (None, None)
            }
        };
        Ok(Self {
            access_lifetime,
            refresh_lifetime,
            access_token,
            refresh_token,
        })
    }
}

pub struct Mutation;

#[graphql_object(context = GraphqlContext)]
impl Mutation {
    /// Request signup email admission. Reject existing accounts before sending or issuing a challenge.
    async fn signup_email_request(
        input: EmailInput,
        #[graphql(context)] auth: &AuthService,
        #[graphql(context)] cookies: &CookieDelivery,
    ) -> Result<EmailChallenge, Error> {
        let mut input = input;
        input.validify()?;
        let mode = input.delivery.unwrap_or_default();
        if mode == CredentialDelivery::Cookie {
            cookies.validate_origin()?;
        }
        (auth.signup_email(&input.email).await?, mode, cookies).try_into()
    }

    /// Request signin email admission for an existing active account.
    async fn signin_email_request(
        input: EmailInput,
        #[graphql(context)] auth: &AuthService,
        #[graphql(context)] cookies: &CookieDelivery,
    ) -> Result<EmailChallenge, Error> {
        let mut input = input;
        input.validify()?;
        let mode = input.delivery.unwrap_or_default();
        if mode == CredentialDelivery::Cookie {
            cookies.validate_origin()?;
        }
        (auth.signin_email(&input.email).await?, mode, cookies).try_into()
    }

    /// Confirm the middleware-verified OTP challenge and atomically create the account and session.
    async fn signup_email_confirm(
        input: EmailCodeInput,
        #[graphql(context)] auth: &AuthService,
        #[graphql(context)] identity: &RequestIdentity,
        #[graphql(context)] cookies: &CookieDelivery,
    ) -> Result<SessionCredentials, Error> {
        let mut input = input;
        input.validify()?;
        let mode = input.delivery.unwrap_or_default();
        if mode == CredentialDelivery::Cookie {
            cookies.validate_origin()?;
        }
        (
            auth.signup(identity.otp()?, &input.code).await?,
            mode,
            cookies,
        )
            .try_into()
    }

    /// Confirm an existing user's middleware-verified OTP challenge and create a scoped session.
    async fn signin_email_confirm(
        mut input: EmailCodeInput,
        #[graphql(context)] auth: &AuthService,
        #[graphql(context)] identity: &RequestIdentity,
        #[graphql(context)] cookies: &CookieDelivery,
    ) -> Result<SessionCredentials, Error> {
        input.validify()?;
        let mode = input.delivery.unwrap_or_default();
        if mode == CredentialDelivery::Cookie {
            cookies.validate_origin()?;
        }
        (
            auth.signin(identity.otp()?, &input.code).await?,
            mode,
            cookies,
        )
            .try_into()
    }

    /// Explicit refresh Bearer only. Rebuild current authority; never set browser cookies.
    async fn session_refresh(
        #[graphql(context)] auth: &AuthService,
        #[graphql(context)] identity: &RequestIdentity,
        #[graphql(context)] cookies: &CookieDelivery,
    ) -> Result<SessionCredentials, Error> {
        (
            auth.refresh(identity.refresh()?).await?,
            CredentialDelivery::Token,
            cookies,
        )
            .try_into()
    }

    /// Revoke this verified login only. Clear cookie credentials after successful revocation.
    async fn session_revoke(
        #[graphql(context)] auth: &AuthService,
        #[graphql(context)] identity: &RequestIdentity,
        #[graphql(context)] cookies: &CookieDelivery,
    ) -> Result<bool, Error> {
        auth.signout(identity.user()?.access()).await?;
        cookies.clear()?;
        Ok(true)
    }
}
