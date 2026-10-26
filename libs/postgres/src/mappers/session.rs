use crate::UserSessionSchema;
use anyhow::{Context, ensure};
use compallm_core::{auth::Session, persistent::session::SessionRepositoryError};

impl TryFrom<UserSessionSchema> for Session {
    type Error = SessionRepositoryError;

    fn try_from(model: UserSessionSchema) -> Result<Self, Self::Error> {
        let id = crate::from_bigint(model.id).context("invalid session ID")?;
        let user_id = crate::from_bigint(model.user_id).context("invalid session user ID")?;
        let created_at =
            u64::try_from(model.created_at.timestamp()).context("invalid login timestamp")?;
        let last_used_at = u64::try_from(model.last_used_at.timestamp())
            .context("invalid session activity timestamp")?;
        let expires_at =
            u64::try_from(model.expires_at.timestamp()).context("invalid session expiry")?;
        let revoked_at = model
            .revoked_at
            .map(|at| u64::try_from(at.timestamp()))
            .transpose()
            .context("invalid session revocation timestamp")?;
        let validation = (|| -> anyhow::Result<()> {
            ensure!(id > 0 && user_id > 0, "session IDs must be positive");
            ensure!(
                created_at <= last_used_at && last_used_at < expires_at,
                "invalid session timestamps"
            );
            ensure!(
                revoked_at.is_none_or(|at| at >= created_at),
                "session revoked before creation"
            );
            ensure!(
                model.refresh_token_id.len() == 43
                    && model
                        .refresh_token_id
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || b"_-".contains(&byte)),
                "invalid refresh credential identifier"
            );
            Ok(())
        })();
        validation?;
        Ok(Self {
            id,
            user_id,
            refresh_token_id: model.refresh_token_id,
            created_at,
            last_used_at,
            expires_at,
            revoked_at,
        })
    }
}
