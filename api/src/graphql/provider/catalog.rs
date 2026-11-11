use juniper::{GraphQLObject, ID};

use super::input::ProviderApiProtocol;

/// Public catalog defaults; selecting a template creates no configured provider.
#[derive(GraphQLObject)]
pub struct ProviderTemplate {
    /// Global opaque catalog identity, independent of organization resource IDs.
    pub id: ID,
    pub name: String,
    /// Family hint only; configured models select their own text operation.
    pub protocol: ProviderApiProtocol,
    /// Unknown and account-specific addresses remain null.
    pub base_url: Option<String>,
}
