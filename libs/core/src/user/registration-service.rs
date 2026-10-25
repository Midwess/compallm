use crate::organization::CreationService;
use crate::persistent::user::{UserRepository, UserRepositoryError};
use compallm_utils::email::normalize_email;

use super::{RegistrationError, User, UserStatus};

pub struct RegistrationService {
    #[allow(clippy::borrowed_box)]
    users: &'static Box<dyn UserRepository>,
    organizations: &'static CreationService,
}

impl RegistrationService {
    #[allow(clippy::borrowed_box)]
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
    /// Uses the caller's database scope and never commits its transaction.
    pub async fn register(
        &self,
        email: &str,
        display_name: &str,
    ) -> Result<User, RegistrationError> {
        let display_name = display_name.trim();
        if display_name.is_empty() || display_name.chars().count() > 256 {
            return Err(RegistrationError::InvalidDisplayName);
        }
        let email = normalize_email(email)?;
        if email.chars().count() > 256 {
            return Err(compallm_utils::error::EmailError::InvalidEmail.into());
        }
        if self.users.find_by_email(&email).await?.is_some() {
            return Err(UserRepositoryError::EmailAlreadyExists.into());
        }
        let user = self
            .users
            .create(User {
                id: hero::distributed_id::try_gen_id().await?,
                email,
                display_name: display_name.into(),
                status: UserStatus::Active,
            })
            .await?;
        self.organizations.create_default(&user).await?;
        Ok(user)
    }
}
