//! Hero JWT issuance and verification. Email delivery and persisted sessions
//! are separate workflow steps and are not wired into this service yet.

use super::{
    AccessToken, EmailOtpAccessToken, EncryptedAccessToken, RefreshToken, UserAccessToken,
};
use crate::auth::AuthError;
use hero::token::jwt::{create_jwt_token, decode_jwt_token};
use std::time::Duration;

/// Server-only delivery result. Send only access_token to the browser and code
/// through email. Intentionally not serializable or Debug.
pub struct EmailOtpChallenge {
    pub access_token: String,
    pub code: String,
}

pub struct UserTokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct TokenCodec {
    jwt_secret: String,
    allowed_email_domain: String,
    encrypted: EncryptedAccessToken,
}

impl TokenCodec {
    /// Startup injects the one secret used for signing and payload encryption.
    pub fn new(jwt_secret: String, allowed_email_domain: &str) -> Result<Self, AuthError> {
        let domain = allowed_email_domain.trim().to_ascii_lowercase();
        if jwt_secret.len() < 32
            || !email_address::EmailAddress::is_valid(format!("user@{domain}").as_str())
        {
            return Err(AuthError::InvalidConfiguration);
        }
        let encrypted = EncryptedAccessToken::new(&jwt_secret)?;
        Ok(Self {
            jwt_secret,
            allowed_email_domain: domain,
            encrypted,
        })
    }

    pub fn issue_email_otp(&self, email: &str) -> Result<EmailOtpChallenge, AuthError> {
        let email = self.normalize_email(email)?;
        let now = compallm_utils::time::unix_timestamp();
        let claims = EmailOtpAccessToken::new(email, now)?;
        let code = claims.code.clone();
        let encrypted = self.encrypted.encrypt(AccessToken::EmailOtp(claims))?;
        let access_token = create_jwt_token(
            encrypted,
            &self.jwt_secret,
            Duration::from_secs(EmailOtpAccessToken::TTL_SECONDS),
        )
        .map_err(|_| AuthError::InvalidToken)?;
        Ok(EmailOtpChallenge { access_token, code })
    }

    /// Hero verifies signature and expiry first. Inner claims then enforce strict
    /// expiry without the JWT library's default clock leeway.
    pub fn parse_access_token(&self, encoded: &str) -> Result<AccessToken, AuthError> {
        let token = self.parse_signed_token(encoded)?;
        if let AccessToken::Encrypted(payload) = token {
            return self.encrypted.decrypt(payload);
        }
        // OTP claims are allowed only inside authenticated encryption.
        if matches!(token, AccessToken::EmailOtp(_)) {
            return Err(AuthError::WrongTokenVariant);
        }
        token.validate_plain(compallm_utils::time::unix_timestamp())?;
        Ok(token)
    }

    fn parse_signed_token(&self, encoded: &str) -> Result<AccessToken, AuthError> {
        if encoded.len() > 16 * 1024 {
            return Err(AuthError::InvalidToken);
        }
        decode_jwt_token(encoded, &self.jwt_secret).map_err(|_| AuthError::InvalidToken)
    }

    pub fn parse_email_otp(&self, encoded: &str) -> Result<EmailOtpAccessToken, AuthError> {
        let AccessToken::EmailOtp(claims) = self.parse_access_token(encoded)? else {
            return Err(AuthError::WrongTokenVariant);
        };
        self.normalize_email(&claims.email)?;
        Ok(claims)
    }

    pub fn verify_email_otp(
        &self,
        encoded: &str,
        code: &str,
    ) -> Result<EmailOtpAccessToken, AuthError> {
        let claims = self.parse_email_otp(encoded)?;
        claims.verify_code(code, compallm_utils::time::unix_timestamp())?;
        Ok(claims)
    }

    pub fn parse_user(&self, encoded: &str) -> Result<UserAccessToken, AuthError> {
        match self.parse_access_token(encoded)? {
            AccessToken::User(claims) => Ok(claims),
            _ => Err(AuthError::WrongTokenVariant),
        }
    }

    /// Requires a separate check of persisted session/refresh state before use.
    pub fn parse_refresh(&self, encoded: &str) -> Result<RefreshToken, AuthError> {
        match self.parse_access_token(encoded)? {
            AccessToken::Refresh(claims) => Ok(claims),
            _ => Err(AuthError::WrongTokenVariant),
        }
    }

    /// Call after the workflow authorizes the user/session. Lifetimes are explicit
    /// until application session policy is agreed.
    pub fn issue_user_tokens(
        &self,
        user_id: u64,
        session_id: u64,
        user_lifetime: Duration,
        refresh_lifetime: Duration,
    ) -> Result<UserTokens, AuthError> {
        let now = compallm_utils::time::unix_timestamp();
        let refresh = RefreshToken::new(user_id, session_id, now, refresh_lifetime)?;
        Ok(UserTokens {
            access_token: self.issue_user_access_token(user_id, session_id, user_lifetime)?,
            refresh_token: create_jwt_token(
                AccessToken::Refresh(refresh),
                &self.jwt_secret,
                refresh_lifetime,
            )
            .map_err(|_| AuthError::InvalidToken)?,
        })
    }

    pub fn issue_user_access_token(
        &self,
        user_id: u64,
        session_id: u64,
        lifetime: Duration,
    ) -> Result<String, AuthError> {
        let user = UserAccessToken::new(
            user_id,
            session_id,
            compallm_utils::time::unix_timestamp(),
            lifetime,
        )?;
        create_jwt_token(AccessToken::User(user), &self.jwt_secret, lifetime)
            .map_err(|_| AuthError::InvalidToken)
    }

    fn normalize_email(&self, email: &str) -> Result<String, AuthError> {
        let email = email.trim().to_ascii_lowercase();
        if !email_address::EmailAddress::is_valid(&email)
            || email.rsplit_once('@').map(|(_, domain)| domain)
                != Some(self.allowed_email_domain.as_str())
        {
            return Err(AuthError::InvalidEmail);
        }
        Ok(email)
    }
}
