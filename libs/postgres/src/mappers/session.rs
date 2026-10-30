use crate::UserSessionSchema;
use anyhow::Context;
use compallm_core::{auth::Session, persistent::session::SessionRepositoryError};

impl TryFrom<UserSessionSchema> for Session {
    type Error = SessionRepositoryError;

    fn try_from(model: UserSessionSchema) -> Result<Self, Self::Error> {
        let id = crate::from_bigint(model.id).context("invalid session ID")?;
        let user_id = crate::from_bigint(model.user_id).context("invalid session user ID")?;
        let organization_id =
            crate::from_bigint(model.organization_id).context("invalid session organization ID")?;
        let revoked_at = model
            .revoked_at
            .map(|at| u64::try_from(at.timestamp()))
            .transpose()
            .context("invalid session revocation timestamp")?;
        Ok(Self {
            id,
            user_id,
            organization_id,
            revoked_at,
        })
    }
}
