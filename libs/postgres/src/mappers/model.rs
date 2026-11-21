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
            text: serde_json::from_value(model.text)
                .context("invalid stored text configuration")?,
            vision: serde_json::from_value(model.vision)
                .context("invalid stored vision configuration")?,
            pdf: serde_json::from_value(model.pdf).context("invalid stored pdf configuration")?,
            audio: serde_json::from_value(model.audio)
                .context("invalid stored audio configuration")?,
            video: serde_json::from_value(model.video)
                .context("invalid stored video configuration")?,
            image_generation: serde_json::from_value(model.image_generation)
                .context("invalid stored image_generation configuration")?,
            tools: serde_json::from_value(model.tools)
                .context("invalid stored tools configuration")?,
            structured_output: serde_json::from_value(model.structured_output)
                .context("invalid stored structured_output configuration")?,
            search: serde_json::from_value(model.search)
                .context("invalid stored search configuration")?,
            limits: serde_json::from_value(model.limits)
                .context("invalid model limits in database")?,
            thinking: serde_json::from_value(model.thinking)
                .context("invalid stored thinking configuration")?,
            pricing,
        })
    }
}
