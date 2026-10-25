use anyhow::Context;
use compallm_core::{
    persistent::user::UserRepositoryError,
    user::{User, UserStatus},
};

use crate::schema::UserSchema;

impl TryFrom<UserSchema> for User {
    type Error = UserRepositoryError;

    fn try_from(model: UserSchema) -> Result<Self, Self::Error> {
        let id = crate::from_bigint(model.id).context("invalid user ID in database")?;
        if id == 0 {
            return Err(UserRepositoryError::database(anyhow::anyhow!(
                "user ID in database must be positive"
            )));
        }
        let status = match model.status.as_str() {
            "active" => UserStatus::Active,
            "disabled" => UserStatus::Disabled,
            status => {
                return Err(UserRepositoryError::database(anyhow::anyhow!(
                    "unknown user status: {status}"
                )));
            }
        };
        Ok(Self {
            id,
            email: model.email,
            display_name: model.display_name,
            status,
        })
    }
}
