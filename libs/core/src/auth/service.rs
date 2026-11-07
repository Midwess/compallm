use std::time::Duration;

use super::Session;
use super::{
    AuthError,
    token::{
        AccessToken, EmailOtpAccessToken, EmailOtpChallenge, EncryptedAccessToken, IssuedEmailOtp,
        RefreshToken, UserAccessToken, UserTokens,
    },
};
use crate::authorization::PolicySnapshot;
use crate::mail::{EmailError, EmailRequest, EmailService, EmailTemplate, OtpCode};
use crate::organization::{Organization, OrganizationMember};
use crate::persistent::session::SessionRepository;
use crate::persistent::{DbContext, organization::OrganizationRepository, user::UserRepository};
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
    /// Return true for current access, false for stale access; reject missing/revoked/ahead state.
    pub async fn validate_access(&self, claims: &UserAccessToken) -> Result<bool, AuthError> {
        claims.validate(unix_timestamp())?;
        let version = self
            .sessions
            .authorization_version(&claims.session)
            .await?
            .ok_or(AuthError::InvalidSession)?;
        if claims.session.authorization_version > version {
            return Err(AuthError::InvalidSession);
        }
        Ok(claims.session.authorization_version == version)
    }

    /// Invalidate selected members' session snapshots in one organization.
    /// The caller determines affected users and owns the transaction containing
    /// authority changes. This increments versions without revoking or renewing.
    pub async fn invalidate_permissions(
        &self,
        organization_id: u64,
        user_ids: &[u64],
    ) -> Result<u64, AuthError> {
        Ok(self
            .sessions
            .invalidate_permissions(organization_id, user_ids)
            .await?)
    }

    /// Build compact authority from a trusted, already-loaded membership.
    /// Session revalidation belongs to the caller; JWT size checks belong to issuance.
    pub fn create_policy_snapshot(
        &self,
        membership: &OrganizationMember,
    ) -> Result<PolicySnapshot, AuthError> {
        Ok(PolicySnapshot::new(membership)?)
    }

    /// Revoke only the persisted session identified by the verified access credential.
    pub async fn signout(&self, claims: &UserAccessToken) -> Result<(), AuthError> {
        claims.validate(unix_timestamp())?;
        let mut session = self
            .sessions
            .find(&claims.session)
            .await?
            .ok_or(AuthError::InvalidSession)?;
        if !session.has_same_identity(&claims.session) {
            return Err(AuthError::InvalidSession);
        }
        if session.revoked_at.is_some() {
            return Ok(());
        }
        let previous = session.clone();
        session.revoke();
        if !self.sessions.update(&session, &previous).await? {
            return Err(AuthError::InvalidSession);
        }
        Ok(())
    }

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
    pub async fn signup_email(&self, email: &str) -> Result<IssuedEmailOtp, AuthError> {
        let email = compallm_utils::email::normalize_email(email);
        if self.users.find_by_email(&email).await?.is_some() {
            return Err(crate::persistent::user::UserRepositoryError::EmailAlreadyExists.into());
        }
        self.send_email_otp(&email).await
    }

    /// Require an existing active account before admitting signin email.
    pub async fn signin_email(&self, email: &str) -> Result<IssuedEmailOtp, AuthError> {
        let email = compallm_utils::email::normalize_email(email);
        compallm_utils::email::validate_email(&email).map_err(|_| AuthError::InvalidEmail)?;
        let user = self
            .users
            .find_by_email(&email)
            .await?
            .ok_or(AuthError::UserNotFound)?;
        if user.status != UserStatus::Active {
            return Err(AuthError::UserDisabled);
        }
        self.send_email_otp(&email).await
    }

    async fn send_email_otp(&self, email: &str) -> Result<IssuedEmailOtp, AuthError> {
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
        Ok(IssuedEmailOtp {
            token: challenge.access_token,
            validity: challenge.validity,
        })
    }

    pub async fn signin(
        &self,
        claims: &EmailOtpAccessToken,
        code: &str,
    ) -> Result<UserTokens, AuthError> {
        claims.verify_code(code, unix_timestamp())?;
        let user = self
            .users
            .find_by_email(&claims.email)
            .await?
            .ok_or(AuthError::UserNotFound)?;
        if user.status != UserStatus::Active {
            return Err(AuthError::UserDisabled);
        }
        let (membership, organization) = self
            .organizations
            .find_first_membership(user.id)
            .await?
            .ok_or(AuthError::OrganizationAccessDenied)?;
        self.start_session(user.id, membership, organization).await
    }

    pub async fn signup(
        &self,
        claims: &EmailOtpAccessToken,
        code: &str,
    ) -> Result<UserTokens, AuthError> {
        claims.verify_code(code, unix_timestamp())?;
        crate::transaction!(self.db, {
            let user = self.registration.register(&claims.email).await?;
            if user.status != UserStatus::Active {
                return Err(AuthError::UserDisabled);
            }
            let (membership, organization) = self
                .organizations
                .find_first_membership(user.id)
                .await?
                .ok_or(AuthError::OrganizationAccessDenied)?;
            self.start_session(user.id, membership, organization).await
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
        let (session, membership) = self
            .sessions
            .find_policy_membership(&claims.session)
            .await?
            .ok_or(AuthError::InvalidSession)?;
        session.validate_credential(&claims)?;
        if claims.session.authorization_version > session.authorization_version
            || membership.user_id != session.user_id
            || membership.organization_id != session.organization_id
        {
            return Err(AuthError::InvalidSession);
        }
        let snapshot = self.create_policy_snapshot(&membership)?;
        let renewed = session.refresh(&claims, self.lifetimes.refresh)?;
        UserTokens::from_refresh(&renewed, snapshot, &self.jwt_secret, self.lifetimes.user)
    }

    pub async fn switch_organization(
        &self,
        refresh_token: &str,
        organization_id: u64,
    ) -> Result<UserTokens, AuthError> {
        let claims = RefreshToken::try_from(AccessToken::decode(refresh_token, &self.jwt_secret)?)?;
        let mut session = self
            .sessions
            .find(&claims.session)
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

        let (membership, organization) = self
            .organizations
            .find_membership(organization_id, user.id)
            .await?
            .ok_or(AuthError::OrganizationAccessDenied)?;

        let previous = session.clone();
        session.switch_organization(&claims, organization_id, organization.name)?;

        let tokens = UserTokens::new(
            &session,
            self.create_policy_snapshot(&membership)?,
            &self.jwt_secret,
            self.lifetimes.user,
            self.lifetimes.refresh,
        )?;

        if !self.sessions.update(&session, &previous).await? {
            return Err(AuthError::InvalidSession);
        }

        Ok(tokens)
    }

    async fn start_session(
        &self,
        user_id: u64,
        membership: OrganizationMember,
        organization: Organization,
    ) -> Result<UserTokens, AuthError> {
        let session = Session::new(user_id, membership.organization_id, organization.name).await?;
        let tokens = UserTokens::new(
            &session,
            self.create_policy_snapshot(&membership)?,
            &self.jwt_secret,
            self.lifetimes.user,
            self.lifetimes.refresh,
        )?;
        self.sessions.create(session).await?;
        Ok(tokens)
    }
}
