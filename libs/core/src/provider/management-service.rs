use crate::{
    persistent::{DbContext, provider::ProviderRepository},
    transaction,
};

use super::{
    ApiProtocol, NewProvider, Provider, ProviderApiKey, ProviderManagementError, ProviderUpdate,
};

pub struct ManagementService {
    db: &'static DbContext,
    #[allow(clippy::borrowed_box)]
    providers: &'static Box<dyn ProviderRepository>,
}

impl ManagementService {
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

    /// The API key must already be protected for storage. Plaintext credential
    /// encryption belongs in a focused secret-protection component once its
    /// configuration and key-rotation rules are selected.
    pub async fn create(
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
                .insert(NewProvider {
                    id: hero::distributed_id::try_gen_id().await?,
                    organization_id,
                    name,
                    url,
                    api_key,
                    api_protocol,
                })
                .await?)
        })
    }

    /// Omit `api_key` to preserve the current stored credential.
    pub async fn update(
        &self,
        organization_id: u64,
        provider_id: u64,
        name: &str,
        url: &str,
        api_key: Option<ProviderApiKey>,
        api_protocol: ApiProtocol,
    ) -> Result<Provider, ProviderManagementError> {
        let name = Self::name(name)?;
        let url = Self::url(url)?;
        transaction!(self.db, {
            Ok(self
                .providers
                .update(
                    organization_id,
                    provider_id,
                    ProviderUpdate {
                        name,
                        url,
                        api_key,
                        api_protocol,
                    },
                )
                .await?)
        })
    }

    pub async fn delete(
        &self,
        organization_id: u64,
        provider_id: u64,
    ) -> Result<(), ProviderManagementError> {
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
