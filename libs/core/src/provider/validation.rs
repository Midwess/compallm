use std::collections::HashSet;

use compallm_utils::input;
use validify::{Validate, ValidationError, ValidationErrors};

use super::{
    ApiKeyAuthentication, Authentication, CachePricing, Connection, Headers, Provider,
    ProviderModel,
};

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

fn validate_key_placement(
    value: &ApiKeyAuthentication,
    query: &std::collections::BTreeMap<String, String>,
) -> Result<(), ValidationErrors> {
    match value {
        ApiKeyAuthentication::Header { api_key_header } => {
            if input::header_name(&api_key_header.name).is_err()
                || input::header_value(&api_key_header.prefix).is_err()
            {
                return Err(invalid("invalid_authentication"));
            }
        }
        ApiKeyAuthentication::Query { api_key_query } => {
            if input::nonblank(api_key_query).is_err() || query.contains_key(api_key_query) {
                return Err(invalid("invalid_authentication"));
            }
        }
    }
    Ok(())
}

impl Validate for Connection {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if self
            .base_url
            .as_deref()
            .is_none_or(|url| input::base_url(url).is_err())
            || self.timeout_ms.is_none_or(|timeout| timeout == 0)
            || self.endpoint.mode == super::EndpointMode::Unknown
        {
            return Err(invalid("incomplete_connection"));
        }
        if self.query.keys().any(|key| input::nonblank(key).is_err()) {
            return Err(invalid("invalid_query"));
        }
        if let Some(path) = &self.path
            && (path.len() > 40
                || input::endpoint_path(path).is_err()
                || path.split('/').any(|segment| segment == ".."))
        {
            return Err(invalid("invalid_endpoint_path"));
        }
        match &self.authentication {
            Some(Authentication::ApiKey(configuration)) => {
                let placement = configuration
                    .placement
                    .as_ref()
                    .ok_or_else(|| invalid("unresolved_api_key_placement"))?;
                validate_key_placement(placement, &self.query)?;
            }
            Some(Authentication::OAuth(configuration)) => {
                use super::OAuthFlow;
                let flow = configuration
                    .flow
                    .as_ref()
                    .ok_or_else(|| invalid("unresolved_oauth_flow"))?;
                if configuration
                    .token_url
                    .as_deref()
                    .is_none_or(|url| input::base_url(url).is_err())
                    || configuration
                        .authorization_url
                        .as_deref()
                        .is_some_and(|url| input::base_url(url).is_err())
                    || configuration
                        .device_authorization_url
                        .as_deref()
                        .is_some_and(|url| input::base_url(url).is_err())
                    || (matches!(flow, OAuthFlow::AuthorizationCode)
                        && configuration.authorization_url.is_none())
                    || (matches!(flow, OAuthFlow::DeviceCode)
                        && configuration.device_authorization_url.is_none())
                {
                    return Err(invalid("incomplete_oauth_configuration"));
                }
                let placement = configuration
                    .token_placement
                    .as_ref()
                    .ok_or_else(|| invalid("unresolved_token_placement"))?;
                validate_key_placement(placement, &self.query)?;
            }
            Some(Authentication::Custom(configuration)) => {
                if configuration
                    .adapter
                    .as_deref()
                    .is_none_or(|adapter| input::nonblank(adapter).is_err())
                {
                    return Err(invalid("unresolved_authentication_adapter"));
                }
            }
            Some(Authentication::None) => {}
            None => return Err(invalid("unresolved_authentication")),
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
        if let Some(billing) = &self.billing {
            billing.validate()?;
        }
        self.capability_fields().validate(self.billing.as_ref())?;
        if self.billing.is_none() {
            let complete = match self.service_kind {
                super::ServiceKind::Llm => {
                    self.text.input.pricing().is_some() && self.text.output.pricing().is_some()
                }
                super::ServiceKind::Embedding | super::ServiceKind::Rerank => {
                    self.text.input.pricing().is_some()
                }
                super::ServiceKind::Image => self.image_generation.pricing().is_some(),
                super::ServiceKind::Video => self.video.pricing().is_some(),
                super::ServiceKind::Tts => self.audio.output.pricing().is_some(),
                super::ServiceKind::Stt => self.audio.input.pricing().is_some(),
                super::ServiceKind::Audio => {
                    self.audio.input.pricing().is_some() || self.audio.output.pricing().is_some()
                }
            };
            if !complete {
                return Err(invalid("unresolved_pricing"));
            }
        }

        let limits = &self.limits;
        if [
            limits.context_tokens,
            self.text.input.configuration().and_then(|v| v.max_tokens),
            self.text.output.configuration().and_then(|v| v.max_tokens),
        ]
        .contains(&Some(0))
        {
            return Err(invalid("invalid_limits"));
        }
        if let Some(context) = limits.context_tokens
            && [
                self.text.input.configuration().and_then(|v| v.max_tokens),
                self.text.output.configuration().and_then(|v| v.max_tokens),
            ]
            .into_iter()
            .flatten()
            .any(|cap| cap > context)
        {
            return Err(invalid("limit_exceeds_context"));
        }
        if let Some(selection) = &self.route_selection {
            selection.validate()?;
        }
        if let Some(thinking) = self
            .text
            .output
            .configuration()
            .and_then(|v| v.thinking.configuration())
        {
            if thinking
                .default_effort
                .is_some_and(|level| !thinking.effort_mapping.contains_key(&level))
            {
                return Err(invalid("invalid_default_effort"));
            }
            if thinking
                .effort_mapping
                .values()
                .any(|value| value.len() > 25 || input::effort_value(value).is_err())
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

fn validate_amounts(amounts: &[rust_decimal::Decimal]) -> Result<(), ValidationErrors> {
    if amounts.is_empty()
        || amounts.iter().any(|amount| {
            input::price(&amount.to_string()).is_err()
                || amount.scale() > input::PRICE_SCALE
                || *amount >= rust_decimal::Decimal::from(10_u64.pow(input::PRICE_INTEGER_DIGITS))
        })
    {
        return Err(invalid("invalid_pricing"));
    }
    Ok(())
}

impl Validate for super::Rate {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let amount = self.amount.ok_or_else(|| invalid("unresolved_rate"))?;
        validate_currency(self.currency.as_deref())?;
        if self.quantity.is_none_or(|quantity| quantity == 0) {
            return Err(invalid("unresolved_rate_quantity"));
        }
        validate_amounts(&[amount])
    }
}

fn validate_currency(currency: Option<&str>) -> Result<(), ValidationErrors> {
    if currency
        .is_none_or(|value| value.len() != 3 || !value.bytes().all(|c| c.is_ascii_uppercase()))
    {
        return Err(invalid("invalid_currency"));
    }
    Ok(())
}

impl Validate for super::RouteSelection {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut ids = HashSet::new();
        for option in &self.options {
            if input::nonblank(&option.id).is_err()
                || input::nonblank(&option.provider).is_err()
                || input::upstream_model_id(&option.upstream_model_id).is_err()
                || !ids.insert(&option.id)
            {
                return Err(invalid("invalid_route_option"));
            }
        }
        let selected = self
            .selected
            .as_ref()
            .ok_or_else(|| invalid("unresolved_route_selection"))?;
        let option = self
            .options
            .iter()
            .find(|option| &option.id == selected)
            .ok_or_else(|| invalid("invalid_route_selection"))?;
        for pricing in [option.text.input.pricing(), option.text.output.pricing()]
            .into_iter()
            .flatten()
        {
            if matches!(pricing, super::CapabilityPricing::ModelBilling) && option.billing.is_none()
            {
                return Err(invalid("unresolved_model_billing"));
            }
            pricing.validate()?;
        }
        if let Some(billing) = &option.billing {
            billing.validate()?;
        }
        Ok(())
    }
}

impl Validate for super::CapabilityPricing {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            Self::Usage(v) => v.validate(),
            Self::Free | Self::Included | Self::ModelBilling => Ok(()),
        }
    }
}
impl Validate for super::CapabilityTokenRate {
    fn validate(&self) -> Result<(), ValidationErrors> {
        self.rate.validate()?;
        if let Some(CachePricing::SeparateRate(v) | CachePricing::EstimatedRate(v)) = &self.cache {
            v.rate.validate()?;
        }
        Ok(())
    }
}
impl Validate for super::CapabilityRates {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            Self::Token(v) => v.validate(),
            Self::Audio(v) => match v {
                super::AudioPricing::Duration(v) => v.rate.validate(),
                super::AudioPricing::Character(v) => v.rate.validate(),
                super::AudioPricing::Utf8Bytes(v) => v.rate.validate(),
            },
            Self::Image(v) => {
                if v.rates.is_empty() {
                    return Err(invalid("empty_image_rates"));
                }
                for r in &v.rates {
                    r.rate.validate()?;
                }
                Ok(())
            }
            Self::Video(v) => {
                if v.rates.is_empty() {
                    return Err(invalid("empty_video_rates"));
                }
                for r in &v.rates {
                    r.rate.validate()?;
                }
                Ok(())
            }
            Self::Request(v) => v.rate.validate(),
            Self::Credit(v) => {
                if v.rates.is_empty() {
                    return Err(invalid("empty_credit_rates"));
                }
                for rate in &v.rates {
                    validate_amounts(&[rate
                        .credits
                        .ok_or_else(|| invalid("unresolved_credit_rate"))?])?;
                    if rate.quantity.is_none_or(|v| v == 0) {
                        return Err(invalid("unresolved_credit_quantity"));
                    }
                }
                Ok(())
            }
        }
    }
}
impl Validate for super::CapabilityTokenTiers {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if self.tiers.is_empty() {
            return Err(invalid("empty_price_tiers"));
        }
        let mut from = 0;
        for (index, tier) in self.tiers.iter().enumerate() {
            if tier.from_tokens != from
                || tier.to_tokens.is_some_and(|v| v <= from)
                || (tier.to_tokens.is_none() != (index + 1 == self.tiers.len()))
            {
                return Err(invalid("invalid_price_tiers"));
            }
            tier.rate.validate()?;
            if let Some(end) = tier.to_tokens {
                from = end;
            }
        }
        Ok(())
    }
}
impl Validate for super::CapabilityUsagePricing {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            Self::Fixed(v) => v.validate(),
            Self::TieredToken(v) => v.validate(),
            Self::Scheduled(v) => {
                if v.timezone != "UTC" || v.slots.is_empty() {
                    return Err(invalid("invalid_price_schedule"));
                }
                let mut hour = 0;
                for slot in &v.slots {
                    if slot.from_hour != hour || slot.to_hour <= hour || slot.to_hour > 24 {
                        return Err(invalid("invalid_price_schedule"));
                    }
                    match &slot.pricing {
                        super::CapabilityScheduledRates::Fixed(v) => v.validate()?,
                        super::CapabilityScheduledRates::TieredToken(v) => v.validate()?,
                    };
                    hour = slot.to_hour;
                }
                if hour != 24 {
                    return Err(invalid("incomplete_price_schedule"));
                }
                Ok(())
            }
        }
    }
}
impl Validate for super::ModelBilling {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            Self::Subscription(v) => {
                if input::nonblank(&v.product).is_err()
                    || v.plan
                        .as_deref()
                        .is_none_or(|v| input::nonblank(v).is_err())
                {
                    return Err(invalid("invalid_subscription"));
                }
                let fee = v
                    .fee
                    .as_ref()
                    .ok_or_else(|| invalid("unresolved_subscription_fee"))?;
                if fee.period.is_none() {
                    return Err(invalid("unresolved_subscription_period"));
                }
                validate_currency(fee.currency.as_deref())?;
                validate_amounts(&[fee
                    .amount
                    .ok_or_else(|| invalid("unresolved_subscription_fee"))?])
            }
            Self::Free(_) => Ok(()),
            Self::Request(v) => v.rate.validate(),
            Self::Credit(v) => {
                validate_currency(v.currency.as_deref())?;
                let amount = v
                    .amount_per_credit
                    .ok_or_else(|| invalid("unresolved_credit_conversion"))?;
                if amount <= rust_decimal::Decimal::ZERO {
                    return Err(invalid("invalid_credit_conversion"));
                }
                validate_amounts(&[amount])
            }
        }
    }
}
