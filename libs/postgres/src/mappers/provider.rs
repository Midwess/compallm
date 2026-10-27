use anyhow::Context;
use compallm_core::{
    persistent::provider::ProviderRepositoryError,
    provider::{ApiProtocol, Provider},
};

use crate::schema::{ApiProtocolSchema, ProviderSchema};

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
        })
    }
}
