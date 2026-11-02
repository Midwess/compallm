use anyhow::Context;
use compallm_core::{
    persistent::provider::ProviderRepositoryError,
    provider::{ApiProtocol, Provider, ProviderModel},
};

use crate::schema::{ApiProtocolSchema, ProviderModelSchema, ProviderSchema};

impl TryFrom<ProviderModelSchema> for ProviderModel {
    type Error = ProviderRepositoryError;

    fn try_from(model: ProviderModelSchema) -> Result<Self, Self::Error> {
        let id = crate::from_bigint(model.id).context("invalid provider model ID in database")?;
        let organization_id = crate::from_bigint(model.organization_id)
            .context("invalid provider model organization ID in database")?;
        let provider_id = crate::from_bigint(model.provider_id)
            .context("invalid provider model provider ID in database")?;
        let minimum_reasoning_budget_tokens = model
            .minimum_reasoning_budget_tokens
            .map(u32::try_from)
            .transpose()
            .context("invalid minimum reasoning budget in database")?;
        let maximum_reasoning_budget_tokens = model
            .maximum_reasoning_budget_tokens
            .map(u32::try_from)
            .transpose()
            .context("invalid maximum reasoning budget in database")?;

        Ok(Self {
            id,
            organization_id,
            provider_id,
            name: model.name,
            enabled: model.enabled,
            input_price_per_million_tokens: model.input_price_per_million_tokens,
            output_price_per_million_tokens: model.output_price_per_million_tokens,
            cached_price_per_million_tokens: model.cached_price_per_million_tokens,
            reasoning_effort_values: model.reasoning_effort_values,
            supports_reasoning_budget: model.supports_reasoning_budget,
            supports_adaptive_reasoning: model.supports_adaptive_reasoning,
            minimum_reasoning_budget_tokens,
            maximum_reasoning_budget_tokens,
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
            url: model.url,
            api_protocol: match model.api_protocol {
                ApiProtocolSchema::Anthropic => ApiProtocol::Anthropic,
                ApiProtocolSchema::Openai => ApiProtocol::Openai,
                ApiProtocolSchema::OpenaiCompatible => ApiProtocol::OpenaiCompatible,
            },
            enabled: model.enabled,
            models: Vec::new(),
        })
    }
}
