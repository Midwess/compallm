use crate::organization::CreationService;
use crate::persistent::DbContext;
use crate::repository::user::{UserQuery, UserRepository, UserRepositoryError};
use crate::transaction;
use modql::filter::ListOptions;

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
        let user = User::from_email(email).await?;
        if self
            .users
            .one(
                UserQuery {
                    email: Some(user.email.clone()),
                    ..Default::default()
                },
                ListOptions::default(),
            )
            .await?
            .is_some()
        {
            return Err(UserRepositoryError::EmailAlreadyExists.into());
        }

        transaction!(DbContext::current()?, {
            let user = self.users.create(user).await?;
            self.organizations.create_default(&user).await?;
            Ok(user)
        })
    }
}
