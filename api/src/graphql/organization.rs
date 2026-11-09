use async_graphql::{ID, Object};
use compallm_core::organization::Organization as CoreOrganization;

/// Public organization identity backed directly by the Core entity.
///
/// Settings and membership administration are separate protected operations.
pub struct Organization(CoreOrganization);

impl From<CoreOrganization> for Organization {
    fn from(organization: CoreOrganization) -> Self {
        Self(organization)
    }
}

#[Object]
impl Organization {
    /// Stable organization identity, serialized as a string.
    async fn id(&self) -> ID {
        ID(self.0.id.to_string())
    }

    /// Display name; changing this value does not change resource ownership.
    async fn name(&self) -> &str {
        &self.0.name
    }
}
