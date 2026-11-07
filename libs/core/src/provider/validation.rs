use std::collections::HashSet;

use validify::{Validate, ValidationError, ValidationErrors};

use super::{ApiKeyAuthentication, Connection, Headers, Provider, ProviderModel, input};

// Schema errors deliberately contain no submitted values, including header values.
pub(super) fn invalid(code: &'static str) -> ValidationErrors {
    let mut errors = ValidationErrors::new();
    errors.add(ValidationError::new_schema(code));
    errors
}

fn headers(value: &Headers) -> Result<(), ValidationErrors> {
    let mut names = HashSet::new();
    for (name, value) in value {
        if input::header_name(name).is_err()
            || input::header_value(value).is_err()
            || !names.insert(name.to_ascii_lowercase())
            || (name.eq_ignore_ascii_case("authorization") && input::authorization(value).is_err())
        {
            return Err(invalid("invalid_headers"));
        }
    }
    Ok(())
}

impl Validate for Connection {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if input::base_url(&self.base_url).is_err() || self.timeout_ms == 0 {
            return Err(invalid("invalid_connection"));
        }
        if self.query.keys().any(|key| input::nonblank(key).is_err()) {
            return Err(invalid("invalid_query"));
        }
        match &self.authentication {
            ApiKeyAuthentication::Header { api_key_header } => {
                if input::header_name(&api_key_header.name).is_err()
                    || input::header_value(&api_key_header.prefix).is_err()
                {
                    return Err(invalid("invalid_authentication"));
                }
            }
            ApiKeyAuthentication::Query { api_key_query } => {
                if input::nonblank(api_key_query).is_err() || self.query.contains_key(api_key_query)
                {
                    return Err(invalid("invalid_authentication"));
                }
            }
        }
        Ok(())
    }
}

impl Validate for Provider {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if self.organization_id == 0 || input::nonblank(&self.name).is_err() {
            return Err(invalid("invalid_provider"));
        }
        self.connection.validate()?;
        headers(&self.headers)?;
        if !self.models.iter().any(|model| model.enabled) {
            return Err(invalid("missing_enabled_model"));
        }
        let mut names = HashSet::new();
        for model in &self.models {
            model.validate()?;
            if !names.insert(model.name.trim()) {
                return Err(invalid("duplicate_model"));
            }
        }
        Ok(())
    }
}

impl Validate for ProviderModel {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if input::nonblank(&self.name).is_err() || !self.protocol.supports(self.service_kind) {
            return Err(invalid("invalid_model"));
        }
        self.connection.validate()?;
        headers(&self.headers)?;
        self.pricing.validate()?;
        if self
            .capabilities
            .iter()
            .enumerate()
            .any(|(index, value)| self.capabilities[..index].contains(value))
        {
            return Err(invalid("duplicate_capability"));
        }
        let limits = &self.limits;
        if [
            limits.context_tokens,
            limits.max_output_tokens,
            limits.max_images_per_request,
            limits.max_duration_ms,
            limits.max_audio_bytes,
        ]
        .contains(&Some(0))
        {
            return Err(invalid("invalid_limits"));
        }
        if let Some(thinking) = &self.thinking {
            if thinking.effort.is_none() && thinking.budget.is_none() {
                return Err(invalid("empty_thinking"));
            }
            if let Some(effort) = &thinking.effort
                && (effort.is_empty()
                    || effort
                        .iter()
                        .enumerate()
                        .any(|(index, value)| effort[..index].contains(value)))
            {
                return Err(invalid("invalid_effort"));
            }
            if let Some(budget) = &thinking.budget
                && (budget.min_tokens == 0
                    || budget.max_tokens < budget.min_tokens
                    || budget.default_tokens.is_some_and(|value| {
                        value < budget.min_tokens || value > budget.max_tokens
                    }))
            {
                return Err(invalid("invalid_thinking_budget"));
            }
        }
        Ok(())
    }
}
