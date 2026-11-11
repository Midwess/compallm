use anyhow::Context;
use compallm_core::{
    persistent::provider::ProviderRepositoryError,
    provider::{Provider, ProviderApiKey, ProviderModel},
};
use sea_orm::{EntityTrait, ModelTrait, Set};

use crate::schema::{ProviderModelSchema, ProviderSchema};
use validify::Validate;

type ProviderActiveModel = <<ProviderSchema as ModelTrait>::Entity as EntityTrait>::ActiveModel;
type ProviderModelActiveModel =
    <<ProviderModelSchema as ModelTrait>::Entity as EntityTrait>::ActiveModel;

/// Prepared provider and model writes; conversion finishes before a transaction starts.
pub(crate) struct ProviderInsert {
    pub provider: ProviderActiveModel,
    pub model_writes: Vec<ProviderModelActiveModel>,
    pub models: Vec<ProviderModel>,
}

impl TryFrom<Provider> for ProviderInsert {
    type Error = ProviderRepositoryError;

    fn try_from(provider: Provider) -> Result<Self, Self::Error> {
        if provider.id == 0 || provider.organization_id == 0 {
            return Err(ProviderRepositoryError::database(anyhow::anyhow!(
                "provider and organization IDs must be positive"
            )));
        }
        let mut model_writes = Vec::with_capacity(provider.models.len());
        for model in &provider.models {
            if model.id == 0
                || model.organization_id != provider.organization_id
                || model.provider_id != provider.id
            {
                return Err(ProviderRepositoryError::database(anyhow::anyhow!(
                    "provider model identity or ownership is invalid"
                )));
            }
            model_writes.push(ProviderModelActiveModel::try_from(model)?);
        }
        Ok(Self {
            provider: ProviderActiveModel {
                id: Set(crate::to_bigint(provider.id).context("provider ID exceeds BIGINT")?),
                organization_id: Set(crate::to_bigint(provider.organization_id)
                    .context("organization ID exceeds BIGINT")?),
                name: Set(provider.name),
                connection: Set(serde_json::to_value(provider.connection)
                    .context("invalid provider connection")?),
                headers: Set(
                    serde_json::to_value(provider.headers).context("invalid provider headers")?
                ),
                credentials: Set(String::from(provider.api_key)),
            },
            model_writes,
            models: provider.models,
        })
    }
}

impl TryFrom<&ProviderModel> for ProviderModelActiveModel {
    type Error = ProviderRepositoryError;

    fn try_from(model: &ProviderModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Set(crate::to_bigint(model.id).context("provider model ID exceeds BIGINT")?),
            organization_id: Set(crate::to_bigint(model.organization_id)
                .context("organization ID exceeds BIGINT")?),
            provider_id: Set(
                crate::to_bigint(model.provider_id).context("provider ID exceeds BIGINT")?
            ),
            name: Set(model.name.clone()),
            enabled: Set(model.enabled),
            service_kind: Set(model.service_kind.into()),
            protocol: Set(serde_json::to_value(model.protocol)
                .context("invalid provider model protocol")?
                .as_str()
                .context("invalid provider model protocol")?
                .to_owned()),
            connection: Set(
                serde_json::to_value(&model.connection).context("invalid model connection")?
            ),
            headers: Set(serde_json::to_value(&model.headers).context("invalid model headers")?),
            capabilities: Set(
                serde_json::to_value(&model.capabilities).context("invalid model capabilities")?
            ),
            limits: Set(serde_json::to_value(&model.limits).context("invalid model limits")?),
            thinking: Set(model
                .thinking
                .as_ref()
                .map(serde_json::to_value)
                .transpose()
                .context("invalid model thinking")?),
            pricing: Set(serde_json::to_value(&model.pricing).context("invalid model pricing")?),
        })
    }
}

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
            api_key: ProviderApiKey::try_from(model.credentials)
                .context("invalid encrypted provider credential in database")?,
            headers: serde_json::from_value(model.headers)
                .context("invalid provider headers in database")?,
            models: Vec::new(),
        })
    }
}
