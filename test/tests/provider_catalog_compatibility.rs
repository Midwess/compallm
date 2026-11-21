use compallm_api::graphql::provider::catalog::ModelTemplate;
use compallm_core::{model::ModelTemplate as DomainModelTemplate, provider::*};
use serde_json::json;
#[path = "support/capability_pricing.rs"]
mod pricing_fixture;
fn template() -> DomainModelTemplate {
    let mut old = json!({"id":"example","researchId":"research-example","providerTemplateId":"example","name":"example/model","displayName":null,"description":null,"service_kind":"llm","serviceKind":"llm","connections":{"openai":{"baseUrl":"https://example.com/v1","path":"/chat/completions","timeoutMs":null,"query":{},"authentication":null,"endpoint":{"mode":"fixed","urlTemplate":null,"requiredInputs":[]}}},"headers":{},"bodyDefaults":{},"text":{"status":"supported","configuration":{}},"vision":{"status":"unknown"},"pdf":{"status":"unknown"},"audio":{"input":{"status":"unknown"},"output":{"status":"unknown"}},"video":{"status":"unknown"},"image_generation":{"status":"unknown"},"thinking":{"status":"unknown"},"tools":{"calling":{"status":"unknown"},"forcedChoice":{"status":"unknown"},"parallelCalls":{"status":"unknown"}},"structured_output":{"status":"unknown"},"search":{"status":"unknown"},"limits":{"contextTokens":128000,"maxInputTokens":128000,"maxOutputTokens":16000},"pricing":{"usage":{"fixed":{"token":{"input":{"amount":"0.15","currency":"USD","quantity":1000000},"output":{"amount":"0.6","currency":"USD","quantity":1000000},"cache":{"type":"separate_rate","configuration":{"rate":{"amount":"0.075","currency":"USD","quantity":1000000}}}}}}},"route_selection":null});
    let converted = pricing_fixture::convert(old.clone());
    let object = old.as_object_mut().unwrap();
    for field in [
        "service_kind",
        "thinking",
        "pricing",
        "image_generation",
        "structured_output",
        "route_selection",
    ] {
        object.remove(field);
    }
    for (key, value) in converted.as_object().unwrap() {
        object.insert(
            match key.as_str() {
                "image_generation" => "imageGeneration",
                "structured_output" => "structuredOutput",
                "route_selection" => "routeSelection",
                other => other,
            }
            .into(),
            value.clone(),
        );
    }
    serde_json::from_value(old).unwrap()
}
#[test]
fn selectable_templates_require_representable_token_prices_without_filling_unknown_rates() {
    let mut model = template();
    assert!(ModelTemplate::compatible(&model));
    let CapabilitySupport::Supported {
        pricing:
            Some(CapabilityPricing::Usage(CapabilityUsagePricing::Fixed(CapabilityRates::Token(
                mut rates,
            )))),
        configuration,
    } = model.text.input.clone()
    else {
        panic!()
    };
    for cache in [
        Some(CachePricing::StandardInputRate),
        Some(CachePricing::Included),
    ] {
        rates.cache = cache;
        model.text.input = CapabilitySupport::supported(
            configuration.clone(),
            Some(CapabilityPricing::Usage(CapabilityUsagePricing::Fixed(
                CapabilityRates::Token(rates.clone()),
            ))),
        );
        assert!(ModelTemplate::compatible(&model));
    }
    for cache in [None, Some(CachePricing::Unsupported)] {
        rates.cache = cache;
        model.text.input = CapabilitySupport::supported(
            configuration.clone(),
            Some(CapabilityPricing::Usage(CapabilityUsagePricing::Fixed(
                CapabilityRates::Token(rates.clone()),
            ))),
        );
        assert!(!ModelTemplate::compatible(&model));
    }
    rates.cache = Some(CachePricing::Included);
    rates.rate.currency = Some("CNY".into());
    model.text.input = CapabilitySupport::supported(
        configuration.clone(),
        Some(CapabilityPricing::Usage(CapabilityUsagePricing::Fixed(
            CapabilityRates::Token(rates.clone()),
        ))),
    );
    assert!(!ModelTemplate::compatible(&model));
    rates.rate.currency = Some("USD".into());
    rates.rate.amount = None;
    model.text.input = CapabilitySupport::supported(
        configuration,
        Some(CapabilityPricing::Usage(CapabilityUsagePricing::Fixed(
            CapabilityRates::Token(rates),
        ))),
    );
    assert!(!ModelTemplate::compatible(&model));
}
#[test]
fn unsupported_services_connections_and_conditional_prices_remain_unselectable() {
    let original = template();
    let mut model = original.clone();
    model.service_kind = Some(ServiceKind::Image);
    assert!(!ModelTemplate::compatible(&model));
    model = original.clone();
    model.connections.clear();
    assert!(!ModelTemplate::compatible(&model));
    model = original.clone();
    model.connections.values_mut().next().unwrap().path = None;
    assert!(!ModelTemplate::compatible(&model));
    model = original.clone();
    model.limits.context_tokens = None;
    assert!(!ModelTemplate::compatible(&model));
    model = original;
    model.text.input = CapabilitySupport::supported(
        TextInput::default(),
        Some(CapabilityPricing::Usage(
            CapabilityUsagePricing::TieredToken(CapabilityTokenTiers {
                measured_by: TokenTierBasis::InputTokens,
                tiers: vec![],
            }),
        )),
    );
    assert!(!ModelTemplate::compatible(&model));
    model.text.input = CapabilitySupport::Unknown;
    assert!(!ModelTemplate::compatible(&model));
}
