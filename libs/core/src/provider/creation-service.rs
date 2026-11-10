use std::collections::HashSet;

use validify::Validate;

use crate::{
    persistent::{DbContext, provider::ProviderRepository},
    transaction,
};

use super::{Provider, ProviderCreationError, validation::invalid};

/// Creates organization-owned connections and their configured models.
///
/// Callers authorize owner/admin access, set the parent organization from trusted
/// context, and protect credentials before calling. No upstream calls are made.
pub struct CreationService<'a> {
    providers: &'a dyn ProviderRepository,
}

impl<'a> CreationService<'a> {
    pub fn new(providers: &'a dyn ProviderRepository) -> Self {
        Self { providers }
    }

    /// Input identities are replaced; the entity carries its encrypted credential.
    pub async fn create(&self, provider: Provider) -> Result<Provider, ProviderCreationError> {
        let mut created = self.create_many(vec![provider]).await?;
        Ok(created.remove(0))
    }

    /// One provider uses the connection scope; multiple providers share a transaction.
    pub async fn create_many(
        &self,
        mut providers: Vec<Provider>,
    ) -> Result<Vec<Provider>, ProviderCreationError> {
        let organization_id = providers
            .first()
            .ok_or_else(|| invalid("missing_provider"))?
            .organization_id;
        let mut names = HashSet::new();
        for provider in &mut providers {
            provider.name = provider.name.trim().to_owned();
            for model in &mut provider.models {
                model.name = model.name.trim().to_owned();
            }
            provider.validate()?;
            if provider.organization_id != organization_id {
                return Err(invalid("organization_mismatch").into());
            }
            if !names.insert(provider.name.clone()) {
                return Err(ProviderCreationError::NameAlreadyExists);
            }
        }

        for provider in &mut providers {
            provider.initialize_ids().await?;
        }

        let db = DbContext::current()?;
        if providers.len() == 1 {
            return Ok(vec![self.providers.insert(providers.remove(0)).await?]);
        }

        transaction!(db, {
            let mut created = Vec::with_capacity(providers.len());
            for provider in providers {
                created.push(self.providers.insert(provider).await?);
            }
            Ok(created)
        })
    }
}
