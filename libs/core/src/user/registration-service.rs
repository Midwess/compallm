use crate::organization::CreationService;
use crate::persistent::DbContext;
use crate::persistent::user::{UserRepository, UserRepositoryError};
use crate::transaction;

use super::{RegistrationError, User};

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
        let user = User::from_email(hero::distributed_id::try_gen_id().await?, email)?;
        if self.users.find_by_email(&user.email).await?.is_some() {
            return Err(UserRepositoryError::EmailAlreadyExists.into());
        }

        transaction!(self.db, {
            let user = self.users.create(user).await?;
            self.organizations.create_default(&user).await?;
            Ok(user)
        })
    }
}
