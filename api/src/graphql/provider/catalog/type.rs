use juniper::{ID, graphql_object};

use super::super::input::ProviderApiProtocol;

pub struct ProviderTemplate {
    /// Global opaque catalog identity, independent of organization resource IDs.
    pub id: ID,
    pub name: String,
    /// Family hint only; configured models select their own text operation.
    pub protocol: ProviderApiProtocol,
    /// Unknown and account-specific addresses remain null.
    pub base_url: Option<String>,
}

/// Public catalog defaults; selecting a template creates no configured provider.
#[graphql_object]
impl ProviderTemplate {
    /// Global opaque catalog identity, independent of organization resource IDs.
    fn id(&self) -> &ID {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    /// Family hint only; configured models select their own text operation.
    fn protocol(&self) -> ProviderApiProtocol {
        self.protocol
    }
    /// Unknown and account-specific addresses remain null.
    fn base_url(&self) -> Option<&str> {
        self.base_url.as_deref()
    }
}
