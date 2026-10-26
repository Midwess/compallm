use crate::{
    persistent::{DbContext, provider::ProviderRepository},
    transaction,
};

use super::{ApiProtocol, Provider, ProviderApiKey, ProviderManagementError};

/// Manages provider records owned by one organization.
///
/// Provider API integration behavior belongs in a separate integration service.
pub struct OrganizationProviderService {
    db: &'static DbContext,
    #[allow(clippy::borrowed_box)]
    providers: &'static Box<dyn ProviderRepository>,
}

impl OrganizationProviderService {
    #[allow(clippy::borrowed_box)]
    pub fn new(db: &'static DbContext, providers: &'static Box<dyn ProviderRepository>) -> Self {
        Self { db, providers }
    }

    /// The caller must authorize access to the organization before invoking
    /// this service. A limit is mandatory and enforced by the repository.
    pub async fn find_all(
        &self,
        organization_id: u64,
        limit: u64,
    ) -> Result<Vec<Provider>, ProviderManagementError> {
        Ok(self.providers.find_all(organization_id, limit).await?)
    }

    pub async fn add_openai_api(
        &self,
        organization_id: u64,
        name: &str,
        url: &str,
        api_key: ProviderApiKey,
    ) -> Result<Provider, ProviderManagementError> {
        self.add_api(organization_id, name, url, api_key, ApiProtocol::Openai)
            .await
    }

    pub async fn add_anthropic_api(
        &self,
        organization_id: u64,
        name: &str,
        url: &str,
        api_key: ProviderApiKey,
    ) -> Result<Provider, ProviderManagementError> {
        self.add_api(organization_id, name, url, api_key, ApiProtocol::Anthropic)
            .await
    }

    pub async fn add_openai_compatible_api(
        &self,
        organization_id: u64,
        name: &str,
        url: &str,
        api_key: ProviderApiKey,
    ) -> Result<Provider, ProviderManagementError> {
        self.add_api(
            organization_id,
            name,
            url,
            api_key,
            ApiProtocol::OpenaiCompatible,
        )
        .await
    }

    /// The API key must already be protected for storage. Plaintext credential
    /// encryption belongs in a focused secret-protection component once its
    /// configuration and key-rotation rules are selected.
    async fn add_api(
        &self,
        organization_id: u64,
        name: &str,
        url: &str,
        api_key: ProviderApiKey,
        api_protocol: ApiProtocol,
    ) -> Result<Provider, ProviderManagementError> {
        let name = Self::name(name)?;
        let url = Self::url(url)?;
        transaction!(self.db, {
            Ok(self
                .providers
                .insert(
                    Provider {
                        id: hero::distributed_id::try_gen_id().await?,
                        organization_id,
                        name,
                        url,
                        api_protocol,
                    },
                    api_key,
                )
                .await?)
        })
    }

    pub async fn delete(
        &self,
        organization_id: u64,
        provider_id: u64,
    ) -> Result<Provider, ProviderManagementError> {
        transaction!(self.db, {
            Ok(self.providers.delete(organization_id, provider_id).await?)
        })
    }

    fn name(name: &str) -> Result<String, ProviderManagementError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(ProviderManagementError::InvalidName);
        }
        Ok(name.into())
    }

    fn url(url: &str) -> Result<String, ProviderManagementError> {
        let url = url.trim();
        if url.is_empty() {
            return Err(ProviderManagementError::InvalidUrl);
        }
        Ok(url.into())
    }
}
