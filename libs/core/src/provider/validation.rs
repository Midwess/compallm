use std::collections::HashSet;

use compallm_utils::input;
use validify::{Validate, ValidationError, ValidationErrors};

use super::{
    ApiKeyAuthentication, Authentication, CachePricing, Connection, Headers, Pricing, Provider,
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
        self.pricing.validate()?;
        self.capability_fields().validate()?;
        let limits = &self.limits;
        if [
            limits.context_tokens,
            limits.max_input_tokens,
            limits.max_output_tokens,
        ]
        .contains(&Some(0))
        {
            return Err(invalid("invalid_limits"));
        }
        if let Some(context) = limits.context_tokens
            && [limits.max_input_tokens, limits.max_output_tokens]
                .into_iter()
                .flatten()
                .any(|cap| cap > context)
        {
            return Err(invalid("limit_exceeds_context"));
        }
        if let Some(selection) = &self.route_selection {
            selection.validate()?;
        }
        if let Some(thinking) = self.thinking.configuration() {
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

impl Validate for super::TokenPricing {
    fn validate(&self) -> Result<(), ValidationErrors> {
        self.input.validate()?;
        self.output.validate()?;
        match &self.cache {
            Some(CachePricing::SeparateRate(rate)) => rate.rate.validate()?,
            Some(
                CachePricing::StandardInputRate
                | CachePricing::Included
                | CachePricing::Unsupported,
            ) => {}
            None => return Err(invalid("unresolved_cache_pricing")),
        }
        Ok(())
    }
}

impl Validate for super::UsageRates {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            Self::Token(value) => value.validate(),
            Self::Audio(super::AudioPricing::Duration(value)) => value.rate.validate(),
            Self::Audio(super::AudioPricing::Character(value)) => value.rate.validate(),
            Self::Audio(super::AudioPricing::Utf8Bytes(value)) => value.rate.validate(),
            Self::Image(value) => {
                if value.rates.is_empty() {
                    return Err(invalid("empty_image_rates"));
                }
                for rate in &value.rates {
                    rate.rate.validate()?;
                }
                Ok(())
            }
            Self::Video(value) => {
                if value.rates.is_empty() {
                    return Err(invalid("empty_video_rates"));
                }
                for rate in &value.rates {
                    rate.rate.validate()?;
                }
                Ok(())
            }
            Self::Request(value) => value.rate.validate(),
            Self::Credit(value) => value.validate(),
        }
    }
}

impl Validate for super::CreditPricing {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if self.rates.is_empty() {
            return Err(invalid("empty_credit_rates"));
        }
        for rate in &self.rates {
            let credits = rate
                .credits
                .ok_or_else(|| invalid("unresolved_credit_rate"))?;
            if rate.quantity.is_none_or(|value| value == 0) {
                return Err(invalid("unresolved_credit_quantity"));
            }
            validate_amounts(&[credits])?;
        }
        let conversion = self
            .conversion
            .as_ref()
            .ok_or_else(|| invalid("unresolved_credit_conversion"))?;
        validate_currency(conversion.currency.as_deref())?;
        let amount = conversion
            .amount_per_credit
            .ok_or_else(|| invalid("unresolved_credit_conversion"))?;
        if amount <= rust_decimal::Decimal::ZERO {
            return Err(invalid("invalid_credit_conversion"));
        }
        validate_amounts(&[amount])
    }
}

impl Validate for super::TieredTokenPricing {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if self.tiers.is_empty() {
            return Err(invalid("empty_price_tiers"));
        }
        let mut from = 0;
        for (index, tier) in self.tiers.iter().enumerate() {
            if tier.from_tokens != from
                || tier.to_tokens.is_some_and(|end| end <= tier.from_tokens)
                || (tier.to_tokens.is_none() != (index + 1 == self.tiers.len()))
            {
                return Err(invalid("invalid_price_tiers"));
            }
            tier.rates.validate()?;
            if let Some(end) = tier.to_tokens {
                from = end;
            }
        }
        Ok(())
    }
}

impl Validate for super::ScheduledRates {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            Self::Fixed(value) => value.validate(),
            Self::TieredToken(value) => value.validate(),
        }
    }
}

impl Validate for super::ScheduledPricing {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if self.timezone != "UTC" || self.slots.is_empty() {
            return Err(invalid("invalid_price_schedule"));
        }
        let mut hour = 0;
        for slot in &self.slots {
            if slot.from_hour != hour || slot.to_hour <= slot.from_hour || slot.to_hour > 24 {
                return Err(invalid("invalid_price_schedule"));
            }
            hour = slot.to_hour;
            slot.pricing.validate()?;
        }
        if hour != 24 {
            return Err(invalid("incomplete_price_schedule"));
        }
        Ok(())
    }
}

impl Validate for super::UsagePricing {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            Self::Fixed(value) => value.validate(),
            Self::TieredToken(value) => value.validate(),
            Self::Scheduled(value) => value.validate(),
        }
    }
}

impl Validate for Pricing {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            Self::Usage(value) => value.validate(),
            Self::Subscription(value) => {
                if input::nonblank(&value.product).is_err()
                    || value
                        .plan
                        .as_deref()
                        .is_none_or(|plan| input::nonblank(plan).is_err())
                {
                    return Err(invalid("invalid_subscription"));
                }
                let fee = value
                    .fee
                    .as_ref()
                    .ok_or_else(|| invalid("unresolved_subscription_fee"))?;
                if fee.period.is_none() {
                    return Err(invalid("unresolved_subscription_period"));
                }
                validate_currency(fee.currency.as_deref())?;
                validate_amounts(&[fee
                    .amount
                    .ok_or_else(|| invalid("unresolved_subscription_fee"))?])?;
                if let Some(overage) = &value.overage {
                    overage.validate()?;
                }
                Ok(())
            }
            Self::Free(_) => Ok(()),
        }
    }
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
        option
            .pricing
            .as_ref()
            .ok_or_else(|| invalid("unresolved_route_pricing"))?
            .validate()
    }
}
