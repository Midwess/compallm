use compallm_core::provider::{
    Capability, EffortLevel, Pricing, Protocol, ProviderModel, ServiceKind,
};
use compallm_postgres::ProviderModelSchema;
use serde_json::json;

fn provider_model() -> ProviderModelSchema {
    ProviderModelSchema {
        id: 1,
        organization_id: 2,
        provider_id: 3,
        name: "reasoning-model".into(),
        enabled: true,
        service_kind: "llm".into(),
        protocol: "openai".into(),
        base_url: Some("https://model.example/v1".into()),
        connection: None,
        headers: json!({"x-project": "test"}),
        capabilities: json!(["text", "reasoning"]),
        limits: json!({"contextTokens": 32768}),
        thinking: Some(
            json!({"effort": ["low", "high"], "budget": {"minTokens": 1, "maxTokens": 8192}}),
        ),
        pricing: json!({"token": {
            "inputUsdPerMillionTokens": "1.12345678",
            "outputUsdPerMillionTokens": "2.00000000",
            "cachedUsdPerMillionTokens": "0.00000000",
        }}),
    }
}

#[test]
fn maps_typed_configuration_and_exact_decimal_prices() {
    let model = ProviderModel::try_from(provider_model()).unwrap();
    assert_eq!(model.service_kind, ServiceKind::Llm);
    assert_eq!(model.protocol, Protocol::Openai);
    assert_eq!(model.base_url.as_deref(), Some("https://model.example/v1"));
    assert_eq!(model.headers["x-project"], "test");
    assert_eq!(
        model.capabilities,
        [Capability::Text, Capability::Reasoning]
    );
    assert_eq!(model.limits.context_tokens, Some(32768));
    let thinking = model.thinking.unwrap();
    assert_eq!(
        thinking.effort.unwrap(),
        [EffortLevel::Low, EffortLevel::High]
    );
    assert_eq!(thinking.budget.unwrap().max_tokens, 8192);
    let Pricing::Token(prices) = model.pricing else {
        panic!("expected token prices")
    };
    assert_eq!(
        prices.input_usd_per_million_tokens.to_string(),
        "1.12345678"
    );
    assert_eq!(
        prices.cached_usd_per_million_tokens.to_string(),
        "0.00000000"
    );
}

#[test]
fn preserves_null_thinking_and_complete_connection_override() {
    let mut stored = provider_model();
    stored.thinking = None;
    stored.connection = Some(json!({
        "baseUrl": "https://override.example", "timeoutMs": 10000,
        "authentication": {"apiKeyQuery": "key"}
    }));
    let model = ProviderModel::try_from(stored).unwrap();
    assert!(model.thinking.is_none());
    assert_eq!(model.connection.unwrap().timeout_ms, 10000);
}

#[test]
fn rejects_unknown_protocols_unknown_json_fields_and_nonstring_prices() {
    let mut stored = provider_model();
    stored.protocol = "unknown".into();
    assert!(ProviderModel::try_from(stored).is_err());
    let mut stored = provider_model();
    stored.thinking = Some(json!({"effort": ["low"], "unknown": true}));
    assert!(ProviderModel::try_from(stored).is_err());
    let mut stored = provider_model();
    stored.pricing["token"]["inputUsdPerMillionTokens"] = json!(1.2);
    assert!(ProviderModel::try_from(stored).is_err());
}

#[test]
fn maps_non_token_pricing_without_discarding_units_or_conditions() {
    let mut stored = provider_model();
    stored.service_kind = "image".into();
    stored.thinking = None;
    stored.pricing =
        json!({"image": {"rates": [{"usdPerImage": "0.025", "conditions": {"quality": "high"}}]}});
    let model = ProviderModel::try_from(stored).unwrap();
    let Pricing::Image(prices) = model.pricing else {
        panic!("expected image prices")
    };
    assert_eq!(prices.rates[0].usd_per_image.to_string(), "0.025");
    assert_eq!(prices.rates[0].conditions.quality.as_deref(), Some("high"));
}
