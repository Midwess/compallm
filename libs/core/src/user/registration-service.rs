use crate::organization::CreationService;
use crate::persistent::DbContext;
use crate::persistent::user::{UserRepository, UserRepositoryError};
use crate::transaction;

use super::{RegistrationError, User};

pub struct RegistrationService {
    users: &'static Box<dyn UserRepository>,
    organizations: &'static CreationService,
}

impl RegistrationService {
    pub fn new(
        users: &'static Box<dyn UserRepository>,
        organizations: &'static CreationService,
    ) -> Self {
        Self {
            users,
            organizations,
        }
    }

    /// Create an account after the caller has verified email ownership.
    /// User, default organization, and owner membership succeed atomically.
    /// Joins an existing transaction when invoked by another service.
    pub async fn register(&self, email: &str) -> Result<User, RegistrationError> {
        let mut user = User::from_email(email).await?;
        if self.users.find_by_email(&user.email).await?.is_some() {
            return Err(UserRepositoryError::EmailAlreadyExists.into());
        }

        transaction!(DbContext::current()?, {
            let mut attempt = 0_u64;
            let user = loop {
                user.username = User::username_candidate(&user.email, attempt);
                match self.users.create(user.clone()).await {
                    Ok(user) => break user,
                    Err(UserRepositoryError::UsernameAlreadyExists) => {
                        attempt += 1;
                    }
                    Err(error) => return Err(error.into()),
                }
            };
            self.organizations.create_default(&user).await?;
            Ok(user)
        })
    }
}
