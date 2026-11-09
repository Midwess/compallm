use async_graphql::{ID, Object};
use compallm_core::user::User as CoreUser;

/// Public identity fields backed directly by the existing Core entity.
///
/// Email, account status, and authentication metadata require separate contracts.
pub struct User(CoreUser);

impl From<CoreUser> for User {
    fn from(user: CoreUser) -> Self {
        Self(user)
    }
}

#[Object]
impl User {
    /// Stable user identity, serialized as a string.
    async fn id(&self) -> ID {
        ID(self.0.id.to_string())
    }

    /// Display name; this field does not establish identity or uniqueness.
    async fn display_name(&self) -> &str {
        &self.0.display_name
    }
}
