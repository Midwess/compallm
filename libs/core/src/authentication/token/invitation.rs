use super::TokenValidity;
use crate::authentication::AuthError;
use compallm_utils::email;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct InvitationToken {
    pub invitation_id: u64,
    pub organization_id: u64,
    pub email: String,
    pub validity: TokenValidity,
}

impl InvitationToken {
    pub const TTL_SECONDS: u64 = 7 * 24 * 60 * 60;

    pub fn validate(&self, now: u64) -> Result<(), AuthError> {
        self.validity.validate(now)?;
        if self.invitation_id == 0
            || self.organization_id == 0
            || self.invitation_id > i64::MAX as u64
            || self.organization_id > i64::MAX as u64
            || self.validity.expires_at - self.validity.issued_at != Self::TTL_SECONDS
            || email::validate_email(&self.email).is_err()
            || email::normalize_email(&self.email) != self.email
        {
            return Err(AuthError::InvalidToken);
        }
        Ok(())
    }
}

impl TryFrom<super::AccessToken> for InvitationToken {
    type Error = AuthError;
    fn try_from(value: super::AccessToken) -> Result<Self, Self::Error> {
        match value {
            super::AccessToken::Invitation(token) => Ok(token),
            _ => Err(AuthError::WrongTokenVariant),
        }
    }
}
