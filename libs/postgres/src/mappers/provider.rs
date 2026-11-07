use anyhow::Context;
use compallm_core::{
    persistent::provider::ProviderRepositoryError,
    provider::{Provider, ProviderApiKey, ProviderModel},
};
use sea_orm::{DbBackend, Statement};

use crate::schema::{ProviderModelSchema, ProviderSchema};
use validify::Validate;

/// Prepared PostgreSQL insert and the explicitly supplied model configuration.
pub(crate) struct ProviderInsert {
    pub statement: Statement,
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
        let mut models = Vec::with_capacity(provider.models.len());
        for model in &provider.models {
            if model.id == 0
                || model.organization_id != provider.organization_id
                || model.provider_id != provider.id
            {
                return Err(ProviderRepositoryError::database(anyhow::anyhow!(
                    "provider model identity or ownership is invalid"
                )));
            }
            models.push(serde_json::json!({
                "id": crate::to_bigint(model.id).context("provider model ID exceeds BIGINT")?,
                "name": model.name, "enabled": model.enabled,
                "service_kind": model.service_kind, "protocol": model.protocol,
                "connection": model.connection, "headers": model.headers,
                "capabilities": model.capabilities, "limits": model.limits,
                "thinking": model.thinking, "pricing": model.pricing,
            }));
        }
        let statement = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            WITH created_provider AS (
                INSERT INTO public.providers (id, organization_id, name, connection, headers, credentials)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING *
            ), created_models AS (
                INSERT INTO public.provider_models (
                    id, organization_id, provider_id, name, enabled, service_kind, protocol,
                    connection, headers, capabilities, limits, thinking, pricing
                )
                SELECT m.id, p.organization_id, p.id, m.name, m.enabled, m.service_kind, m.protocol,
                    m.connection, m.headers, m.capabilities, m.limits, m.thinking, m.pricing
                FROM created_provider p
                CROSS JOIN jsonb_to_recordset($7::jsonb) AS m (
                    id bigint, name text, enabled boolean, service_kind text, protocol text,
                    connection jsonb, headers jsonb, capabilities jsonb, limits jsonb,
                    thinking jsonb, pricing jsonb
                )
                RETURNING id
            )
            SELECT * FROM created_provider
        "#,
            [
                crate::to_bigint(provider.id)
                    .context("provider ID exceeds BIGINT")?
                    .into(),
                crate::to_bigint(provider.organization_id)
                    .context("organization ID exceeds BIGINT")?
                    .into(),
                provider.name.into(),
                serde_json::to_value(provider.connection)
                    .context("invalid provider connection")?
                    .into(),
                serde_json::to_value(provider.headers)
                    .context("invalid provider headers")?
                    .into(),
                String::from(provider.api_key).into(),
                serde_json::Value::Array(models).into(),
            ],
        );
        Ok(Self {
            statement,
            models: provider.models,
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
