use anyhow::Context;
use compallm_core::{
    persistent::provider::ProviderRepositoryError,
    provider::{Provider, ProviderModel},
};

use crate::schema::{ProviderModelSchema, ProviderSchema};
use validify::Validate;

impl TryFrom<ProviderModelSchema> for ProviderModel {
    type Error = ProviderRepositoryError;

    fn try_from(model: ProviderModelSchema) -> Result<Self, Self::Error> {
        let id = crate::from_bigint(model.id).context("invalid provider model ID in database")?;
        let organization_id = crate::from_bigint(model.organization_id)
            .context("invalid provider model organization ID in database")?;
        let provider_id = crate::from_bigint(model.provider_id)
            .context("invalid provider model provider ID in database")?;
        let pricing: compallm_core::provider::Pricing = serde_json::from_value(model.pricing)
            .context("invalid provider model pricing in database")?;
        pricing
            .validate()
            .context("invalid provider model prices in database")?;

        Ok(Self {
            id,
            organization_id,
            provider_id,
            name: model.name,
            enabled: model.enabled,
            service_kind: serde_json::from_value(model.service_kind.into())
                .context("invalid provider model service kind in database")?,
            protocol: serde_json::from_value(model.protocol.into())
                .context("invalid provider model protocol in database")?,
            connection: serde_json::from_value(model.connection)
                .context("invalid provider model connection in database")?,
            headers: serde_json::from_value(model.headers)
                .context("invalid provider model headers in database")?,
            capabilities: serde_json::from_value(model.capabilities)
                .context("invalid provider model capabilities in database")?,
            limits: serde_json::from_value(model.limits)
                .context("invalid provider model limits in database")?,
            thinking: model
                .thinking
                .map(serde_json::from_value)
                .transpose()
                .context("invalid provider model thinking in database")?,
            pricing,
        })
    }
}

impl TryFrom<ProviderSchema> for Provider {
    type Error = ProviderRepositoryError;

    fn try_from(model: ProviderSchema) -> Result<Self, Self::Error> {
        let id = crate::from_bigint(model.id).context("invalid provider ID in database")?;
        let organization_id = crate::from_bigint(model.organization_id)
            .context("invalid provider organization ID in database")?;
        if id == 0 || organization_id == 0 {
            return Err(ProviderRepositoryError::database(anyhow::anyhow!(
                "provider and organization IDs in database must be positive"
            )));
        }

        Ok(Self {
            id,
            organization_id,
            name: model.name,
            connection: serde_json::from_value(model.connection)
                .context("invalid provider connection in database")?,
            headers: serde_json::from_value(model.headers)
                .context("invalid provider headers in database")?,
            models: Vec::new(),
        })
    }
}
