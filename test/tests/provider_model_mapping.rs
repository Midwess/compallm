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
        display_name: None,
        description: None,
        template: None,
        body_defaults: json!({}),
        route_selection: None,
        enabled: true,
        service_kind: "llm".into(),
        protocol: "openai".into(),
        connection: json!({
            "baseUrl": "https://model.example/v1", "timeoutMs": 30000,
            "authentication":{"type":"api_key","configuration":{"placement":{"apiKeyQuery":"key"},"requiredInputs":[]}},"endpoint":{"mode":"fixed","urlTemplate":null,"requiredInputs":[]}
        }),
        headers: json!({"x-project": "test"}),
        text: json!({"status": "supported", "configuration": {}}),
        vision: json!({"status": "unknown"}),
        pdf: json!({"status": "unknown"}),
        audio: json!({"input": {"status": "unknown"}, "output": {"status": "unknown"}}),
        video: json!({"status": "unknown"}),
        image_generation: json!({"status": "unknown"}),
        tools: json!({"calling": {"status": "unknown"}, "forcedChoice": {"status": "unknown"}, "parallelCalls": {"status": "unknown"}}),
        structured_output: json!({"status": "unknown"}),
        search: json!({"status": "unknown"}),
        limits: json!({"contextTokens": 32768}),
        thinking: json!({"status":"supported","configuration":{"effortMapping":{"low":"low","high":"high"},"defaultEffort":null,"budget":{"minTokens":1,"maxTokens":8192}}}),
        pricing: json!({"usage":{"fixed":{"token":{"input":{"amount":"1.12345678","currency":"USD","quantity":1000000},"output":{"amount":"2.00000000","currency":"USD","quantity":1000000},"cache":{"type":"separate_rate","configuration":{"rate":{"amount":"0.00000000","currency":"USD","quantity":1000000}}}}}}}),
    }
}

#[test]
fn maps_typed_configuration_and_exact_decimal_prices() {
    let model = ProviderModel::try_from(provider_model()).unwrap();
    assert_eq!(model.service_kind, ServiceKind::Llm);
    assert_eq!(model.protocol, Protocol::Openai);
    assert_eq!(
        model.connection.base_url.as_deref(),
        Some("https://model.example/v1")
    );
    assert_eq!(model.headers["x-project"], "test");
    assert_eq!(
        model.supported_capabilities(),
        [Capability::Text, Capability::Reasoning]
    );
    assert_eq!(model.limits.context_tokens, Some(32768));
    let thinking = model.thinking.configuration().unwrap();
    assert_eq!(
        thinking.effort_mapping.keys().copied().collect::<Vec<_>>(),
        [EffortLevel::Low, EffortLevel::High]
    );
    assert_eq!(thinking.budget.as_ref().unwrap().max_tokens, 8192);
    let Pricing::Usage(compallm_core::provider::UsagePricing::Fixed(
        compallm_core::provider::UsageRates::Token(prices),
    )) = model.pricing
    else {
        panic!("expected token prices")
    };
    assert_eq!(prices.input.amount.unwrap().to_string(), "1.12345678");
    assert_eq!(
        match prices.cache.unwrap() {
            compallm_core::provider::CachePricing::SeparateRate(v) =>
                v.rate.amount.unwrap().to_string(),
            _ => panic!(),
        },
        "0.00000000"
    );
}

#[test]
fn preserves_null_thinking_and_required_complete_connection() {
    let mut stored = provider_model();
    stored.thinking = json!({"status":"unknown"});
    stored.connection = json!({
        "baseUrl": "https://override.example", "timeoutMs": 10000,
        "authentication":{"type":"api_key","configuration":{"placement":{"apiKeyQuery":"key"},"requiredInputs":[]}},"endpoint":{"mode":"fixed","urlTemplate":null,"requiredInputs":[]}
    });
    let model = ProviderModel::try_from(stored).unwrap();
    assert!(matches!(
        model.thinking,
        compallm_core::provider::CapabilitySupport::Unknown
    ));
    assert_eq!(model.connection.timeout_ms, Some(10000));
}

#[test]
fn rejects_null_or_incomplete_connection() {
    for connection in [
        serde_json::Value::Null,
        json!({"baseUrl": "https://model.example"}),
    ] {
        let mut stored = provider_model();
        stored.connection = connection;
        assert!(ProviderModel::try_from(stored).is_err());
    }
}

#[test]
fn rejects_unknown_protocols_unknown_json_fields_and_nonstring_prices() {
    let mut stored = provider_model();
    stored.protocol = "unknown".into();
    assert!(ProviderModel::try_from(stored).is_err());
    let mut stored = provider_model();
    stored.thinking = json!({"status":"supported","configuration":{"effortMapping":{"low":"low"},"unknown":true}});
    assert!(ProviderModel::try_from(stored).is_err());
    let mut stored = provider_model();
    stored.pricing["usage"]["fixed"]["token"]["input"]["amount"] = json!(1.2);
    assert!(ProviderModel::try_from(stored).is_err());
}

#[test]
fn maps_non_token_pricing_without_discarding_units_or_conditions() {
    let mut stored = provider_model();
    stored.service_kind = "image".into();
    stored.thinking = json!({"status":"unknown"});
    stored.pricing = json!({"usage":{"fixed":{"image":{"rates":[{"rate":{"amount":"0.025","currency":"USD","quantity":1},"conditions":{"quality":"high"}}]}}}});
    let model = ProviderModel::try_from(stored).unwrap();
    let Pricing::Usage(compallm_core::provider::UsagePricing::Fixed(
        compallm_core::provider::UsageRates::Image(prices),
    )) = model.pricing
    else {
        panic!("expected image prices")
    };
    assert_eq!(prices.rates[0].rate.amount.unwrap().to_string(), "0.025");
    assert_eq!(prices.rates[0].conditions.quality.as_deref(), Some("high"));
}
