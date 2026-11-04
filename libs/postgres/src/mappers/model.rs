use anyhow::Context;
use compallm_core::{model::Model, persistent::model::ModelRepositoryError};

use crate::schema::ModelSchema;
use validify::Validate;

impl TryFrom<ModelSchema> for Model {
    type Error = ModelRepositoryError;

    fn try_from(model: ModelSchema) -> Result<Self, Self::Error> {
        let id = crate::from_bigint(model.id).context("invalid model ID in database")?;
        let organization_id = crate::from_bigint(model.organization_id)
            .context("invalid model organization ID in database")?;
        if id == 0 || organization_id == 0 {
            return Err(ModelRepositoryError::database(anyhow::anyhow!(
                "model and organization IDs in database must be positive"
            )));
        }
        let pricing: compallm_core::provider::Pricing =
            serde_json::from_value(model.pricing).context("invalid model pricing in database")?;
        pricing
            .validate()
            .context("invalid model prices in database")?;
        Ok(Self {
            id,
            organization_id,
            name: model.name,
            service_kind: serde_json::from_value(model.service_kind.into())
                .context("invalid model service kind in database")?,
            enabled: model.enabled,
            capabilities: serde_json::from_value(model.capabilities)
                .context("invalid model capabilities in database")?,
            limits: serde_json::from_value(model.limits)
                .context("invalid model limits in database")?,
            thinking: model
                .thinking
                .map(serde_json::from_value)
                .transpose()
                .context("invalid model thinking in database")?,
            pricing,
        })
    }
}
