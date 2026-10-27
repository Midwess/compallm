use anyhow::Context;
use compallm_core::{model::Model, persistent::model::ModelRepositoryError};

use crate::schema::ModelSchema;

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
        if model.input_price_per_million_tokens.is_sign_negative()
            || model.output_price_per_million_tokens.is_sign_negative()
            || model.cached_price_per_million_tokens.is_sign_negative()
        {
            return Err(ModelRepositoryError::database(anyhow::anyhow!(
                "model prices in database must be non-negative"
            )));
        }

        Ok(Self {
            id,
            organization_id,
            name: model.name,
            input_price_per_million_tokens: model.input_price_per_million_tokens,
            output_price_per_million_tokens: model.output_price_per_million_tokens,
            cached_price_per_million_tokens: model.cached_price_per_million_tokens,
            enabled: model.enabled,
            hidden: model.hidden,
            settings: model.settings,
        })
    }
}
