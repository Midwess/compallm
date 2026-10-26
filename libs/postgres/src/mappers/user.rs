use anyhow::Context;
use compallm_core::{
    persistent::user::UserRepositoryError,
    user::{User, UserStatus},
};

use crate::schema::{UserSchema, UserStatusSchema};

impl TryFrom<UserSchema> for User {
    type Error = UserRepositoryError;

    fn try_from(model: UserSchema) -> Result<Self, Self::Error> {
        let id = crate::from_bigint(model.id).context("invalid user ID in database")?;
        let status = match model.status {
            UserStatusSchema::Active => UserStatus::Active,
            UserStatusSchema::Disabled => UserStatus::Disabled,
        };
        Ok(Self {
            id,
            email: model.email,
            display_name: model.display_name,
            status,
        })
    }
}
