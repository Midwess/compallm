use crate::UserSessionSchema;
use anyhow::Context;
use compallm_core::{auth::Session, persistent::session::SessionRepositoryError};

pub fn from_database(
    model: UserSessionSchema,
    organization_name: String,
) -> Result<Session, SessionRepositoryError> {
    let id = crate::from_bigint(model.id).context("invalid session ID")?;
    let user_id = crate::from_bigint(model.user_id).context("invalid session user ID")?;
    let organization_id =
        crate::from_bigint(model.organization_id).context("invalid session organization ID")?;
    let revoked_at = model
        .revoked_at
        .map(|at| u64::try_from(at.timestamp()))
        .transpose()
        .context("invalid session revocation timestamp")?;
    Ok(Session {
        id,
        user_id,
        organization_id,
        organization_name,
        revoked_at,
    })
}
