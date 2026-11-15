use crate::authentication::{AuthError, token::InvitationToken};

#[derive(Clone, Debug)]
pub struct Invitation {
    pub id: u64,
    pub organization_id: u64,
    pub email: String,
    pub inviter_id: u64,
    pub expires_at: u64,
    pub status: String,
    pub accepted_at: Option<u64>,
    pub accepted_by: Option<u64>,
    pub revoked_at: Option<u64>,
}

impl Invitation {
    pub fn validate(&self, claims: &InvitationToken, now: u64) -> Result<(), AuthError> {
        claims.validate(now)?;
        if self.id != claims.invitation_id
            || self.organization_id != claims.organization_id
            || self.email != claims.email
            || self.expires_at != claims.validity.expires_at
            || self.status != "pending"
            || self.accepted_at.is_some()
            || self.accepted_by.is_some()
            || self.revoked_at.is_some()
        {
            return Err(AuthError::InvalidInvitation);
        }
        Ok(())
    }
}
