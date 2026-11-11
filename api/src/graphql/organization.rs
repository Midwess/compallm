use compallm_core::organization::Organization as CoreOrganization;
use juniper::{ID, graphql_object};

/// Public organization identity backed directly by the Core entity.
///
/// Settings and membership administration are separate protected operations.
pub struct Organization(CoreOrganization);

impl From<CoreOrganization> for Organization {
    fn from(organization: CoreOrganization) -> Self {
        Self(organization)
    }
}

#[graphql_object]
impl Organization {
    /// Stable organization identity, serialized as a string.
    fn id(&self) -> ID {
        ID::new(self.0.id.to_string())
    }

    /// Display name; changing this value does not change resource ownership.
    fn name(&self) -> &str {
        &self.0.name
    }
}
