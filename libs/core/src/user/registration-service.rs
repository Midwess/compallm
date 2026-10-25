use crate::organization::CreationService;
use crate::persistent::DbContext;
use crate::persistent::user::{UserRepository, UserRepositoryError};
use crate::transaction;
use compallm_utils::email::{normalize_email, verify_email_local_part};

use super::{RegistrationError, User, UserStatus};

pub struct RegistrationService {
    db: &'static DbContext,
    users: &'static Box<dyn UserRepository>,
    organizations: &'static CreationService,
}

impl RegistrationService {
    pub fn new(
        db: &'static DbContext,
        users: &'static Box<dyn UserRepository>,
        organizations: &'static CreationService,
    ) -> Self {
        Self {
            db,
            users,
            organizations,
        }
    }

    /// Create an account after the caller has verified email ownership.
    /// User, default organization, and owner membership succeed atomically.
    /// Joins an existing transaction when invoked by another service.
    pub async fn register(&self, email: &str) -> Result<User, RegistrationError> {
        let email = normalize_email(email)?;
        if email.chars().count() > 256 {
            return Err(compallm_utils::error::EmailError::InvalidEmail.into());
        }
        let display_name = email
            .rsplit_once('@')
            .map(|(name, _)| name.to_owned())
            .ok_or(compallm_utils::error::EmailError::InvalidEmail)?;
        if verify_email_local_part(&display_name).is_err() {
            return Err(RegistrationError::InvalidDisplayName);
        }

        transaction!(self.db, {
            if self.users.find_by_email(&email).await?.is_some() {
                return Err(UserRepositoryError::EmailAlreadyExists.into());
            }
            let user = self
                .users
                .create(User {
                    id: hero::distributed_id::try_gen_id().await?,
                    email,
                    display_name,
                    status: UserStatus::Active,
                })
                .await?;
            self.organizations.create_default(&user).await?;
            Ok(user)
        })
    }
}
