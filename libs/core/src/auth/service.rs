use std::time::Duration;

use super::Session;
use super::{
    AuthError,
    token::{
        AccessToken, EmailOtpAccessToken, EmailOtpChallenge, EncryptedAccessToken, RefreshToken,
        UserAccessToken, UserTokens,
    },
};
use crate::mail::{EmailError, EmailRequest, EmailService, EmailTemplate, OtpCode};
use crate::organization::OrganizationMember;
use crate::persistent::session::SessionRepository;
use crate::persistent::{DbContext, organization::OrganizationRepository, user::UserRepository};
use crate::transaction;
use crate::user::{RegistrationService, UserStatus};
use compallm_utils::time::unix_timestamp;

/// Explicit policy supplied by application configuration.
pub struct TokenLifetimes {
    pub user: Duration,
    pub refresh: Duration,
}

pub struct AuthService {
    db: &'static DbContext,
    users: &'static Box<dyn UserRepository>,
    organizations: &'static Box<dyn OrganizationRepository>,
    sessions: &'static Box<dyn SessionRepository>,
    registration: &'static RegistrationService,
    jwt_secret: String,
    lifetimes: TokenLifetimes,
    email: Option<&'static Box<dyn EmailService>>,
}

impl AuthService {
    pub fn new(
        db: &'static DbContext,
        users: &'static Box<dyn UserRepository>,
        organizations: &'static Box<dyn OrganizationRepository>,
        sessions: &'static Box<dyn SessionRepository>,
        registration: &'static RegistrationService,
        jwt_secret: String,
        lifetimes: TokenLifetimes,
        email: Option<&'static Box<dyn EmailService>>,
    ) -> Result<Self, AuthError> {
        EncryptedAccessToken::new(&jwt_secret)?;
        Ok(Self {
            db,
            users,
            organizations,
            sessions,
            registration,
            jwt_secret,
            lifetimes,
            email,
        })
    }

    /// Reject existing accounts before requesting OTP delivery for signup.
    pub async fn signup_email(&self, email: &str) -> Result<String, AuthError> {
        let email = compallm_utils::email::normalize_email(email);
        if self.users.find_by_email(&email).await?.is_some() {
            return Err(crate::persistent::user::UserRepositoryError::EmailAlreadyExists.into());
        }
        self.send_email_otp(&email).await
    }

    /// Request an encrypted OTP access token without disclosing account existence.
    pub async fn signin_email(&self, email: &str) -> Result<String, AuthError> {
        self.send_email_otp(email).await
    }

    async fn send_email_otp(&self, email: &str) -> Result<String, AuthError> {
        let challenge = EmailOtpChallenge::new(email, &self.jwt_secret)?;
        let sender = self.email.as_ref().ok_or(EmailError::Unavailable)?;
        sender
            .send(EmailRequest {
                to: compallm_utils::email::normalize_email(email),
                subject: "Your Compallm verification code".into(),
                text: EmailTemplate::OtpCode(OtpCode::new(
                    challenge.code,
                    Duration::from_secs(EmailOtpAccessToken::TTL_SECONDS),
                )),
            })
            .await?;
        Ok(challenge.access_token)
    }

    pub async fn signin(
        &self,
        claims: &EmailOtpAccessToken,
        code: &str,
    ) -> Result<UserTokens, AuthError> {
        claims.verify_code(code, unix_timestamp())?;
        transaction!(self.db, {
            let user = self
                .users
                .find_by_email(&claims.email)
                .await?
                .ok_or(AuthError::UserNotFound)?;
            if user.status != UserStatus::Active {
                return Err(AuthError::UserDisabled);
            }
            let membership = self
                .organizations
                .find_first_admin_membership(user.id)
                .await?
                .ok_or(AuthError::OrganizationAccessDenied)?;
            self.start_session(user.id, membership).await
        })
    }

    pub async fn signup(
        &self,
        claims: &EmailOtpAccessToken,
        code: &str,
    ) -> Result<UserTokens, AuthError> {
        claims.verify_code(code, unix_timestamp())?;
        transaction!(self.db, {
            let user = self.registration.register(&claims.email).await?;
            if user.status != UserStatus::Active {
                return Err(AuthError::UserDisabled);
            }
            let membership = self
                .organizations
                .find_first_admin_membership(user.id)
                .await?
                .ok_or(AuthError::OrganizationAccessDenied)?;
            self.start_session(user.id, membership).await
        })
    }

    /// Verify the envelope and inner claims, preserving the concrete token variant.
    pub fn authenticate_token(&self, token: &str) -> Result<AccessToken, AuthError> {
        AccessToken::decode(token, &self.jwt_secret)
    }

    /// Verify signed access claims without a database read; session changes apply on renewal.
    pub fn authenticate_access(&self, token: &str) -> Result<UserAccessToken, AuthError> {
        UserAccessToken::try_from(self.authenticate_token(token)?)
    }

    /// Read-only renewal: validates JWT/session state and never consumes the credential.
    pub async fn refresh(&self, refresh_token: &str) -> Result<UserTokens, AuthError> {
        let claims = RefreshToken::try_from(AccessToken::decode(refresh_token, &self.jwt_secret)?)?;
        let session = self
            .sessions
            .find(claims.session.reference())
            .await?
            .ok_or(AuthError::InvalidSession)?;
        let renewed = session.refresh(&claims, self.lifetimes.refresh)?;
        let user = self
            .users
            .find_by_id(claims.session.user_id)
            .await?
            .ok_or(AuthError::UserNotFound)?;
        if user.status != UserStatus::Active {
            return Err(AuthError::UserDisabled);
        }
        let membership = self
            .organizations
            .find_membership(session.organization_id, user.id)
            .await?
            .ok_or(AuthError::OrganizationAccessDenied)?;
        UserTokens::from_refresh(
            &renewed,
            membership.role,
            &self.jwt_secret,
            self.lifetimes.user,
        )
    }

    pub async fn switch_organization(
        &self,
        refresh_token: &str,
        organization_id: u64,
    ) -> Result<UserTokens, AuthError> {
        let claims = RefreshToken::try_from(AccessToken::decode(refresh_token, &self.jwt_secret)?)?;
        transaction!(self.db, {
            let mut session = self
                .sessions
                .find(claims.session.reference())
                .await?
                .ok_or(AuthError::InvalidSession)?;
            session.validate_credential(&claims)?;
            let user = self
                .users
                .find_by_id(claims.session.user_id)
                .await?
                .ok_or(AuthError::UserNotFound)?;
            if user.status != UserStatus::Active {
                return Err(AuthError::UserDisabled);
            }
            let membership = self
                .organizations
                .find_membership(organization_id, user.id)
                .await?
                .ok_or(AuthError::OrganizationAccessDenied)?;
            let previous = session.clone();
            session.switch_organization(&claims, organization_id)?;
            let tokens = UserTokens::new(
                &session,
                membership.role,
                &self.jwt_secret,
                self.lifetimes.user,
                self.lifetimes.refresh,
            )?;
            if !self.sessions.update(&session, &previous).await? {
                return Err(AuthError::InvalidSession);
            }
            Ok(tokens)
        })
    }

    async fn start_session(
        &self,
        user_id: u64,
        membership: OrganizationMember,
    ) -> Result<UserTokens, AuthError> {
        let id = hero::distributed_id::try_gen_id().await?;
        let session = Session::new(user_id, id, membership.organization_id);
        let tokens = UserTokens::new(
            &session,
            membership.role,
            &self.jwt_secret,
            self.lifetimes.user,
            self.lifetimes.refresh,
        )?;
        self.sessions.create(session).await?;
        Ok(tokens)
    }
}
