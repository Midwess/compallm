use compallm_api::graphql::provider::catalog::ModelTemplate;
use compallm_core::{model::ModelTemplate as DomainModelTemplate, provider::*};
use serde_json::json;

fn template() -> DomainModelTemplate {
    serde_json::from_value(json!({
        "id":"example","researchId":"research-example","providerTemplateId":"example",
        "name":"example/model","displayName":null,"description":null,"serviceKind":"llm",
        "connections":{"openai":{
            "baseUrl":"https://example.com/v1","path":"/chat/completions","timeoutMs":null,
            "query":{},"authentication":null,
            "endpoint":{"mode":"fixed","urlTemplate":null,"requiredInputs":[]}
        }},
        "headers":{},"bodyDefaults":{},"text":{"status":"supported","configuration":{}},"vision":{"status":"unknown"},"pdf":{"status":"unknown"},
        "audio":{"input":{"status":"unknown"},"output":{"status":"unknown"}},"video":{"status":"unknown"},
        "imageGeneration":{"status":"unknown"},"tools":{"calling":{"status":"unknown"},"forcedChoice":{"status":"unknown"},"parallelCalls":{"status":"unknown"}},
        "structuredOutput":{"status":"unknown"},"search":{"status":"unknown"},
        "limits":{"contextTokens":128000,"maxInputTokens":128000,"maxOutputTokens":16000},
        "thinking":{"status":"unknown"},"pricing":{"usage":{"fixed":{"token":{
            "input":{"amount":"0.15","currency":"USD","quantity":1000000},
            "output":{"amount":"0.6","currency":"USD","quantity":1000000},
            "cache":{"type":"separate_rate","configuration":{"rate":{"amount":"0.075","currency":"USD","quantity":1000000}}}
        }}}},"routeSelection":null
    })).unwrap()
}

#[test]
fn selectable_templates_require_representable_token_prices_without_filling_unknown_rates() {
    let mut model = template();
    assert!(ModelTemplate::compatible(&model));
    let Some(Pricing::Usage(UsagePricing::Fixed(UsageRates::Token(mut rates)))) =
        model.pricing.take()
    else {
        panic!()
    };
    for cache in [
        Some(CachePricing::StandardInputRate),
        Some(CachePricing::Included),
    ] {
        rates.cache = cache;
        model.pricing = Some(Pricing::Usage(UsagePricing::Fixed(UsageRates::Token(
            rates.clone(),
        ))));
        assert!(ModelTemplate::compatible(&model));
    }
    for cache in [None, Some(CachePricing::Unsupported)] {
        rates.cache = cache;
        model.pricing = Some(Pricing::Usage(UsagePricing::Fixed(UsageRates::Token(
            rates.clone(),
        ))));
        assert!(!ModelTemplate::compatible(&model));
    }
    rates.cache = Some(CachePricing::Included);
    rates.input.currency = Some("CNY".into());
    model.pricing = Some(Pricing::Usage(UsagePricing::Fixed(UsageRates::Token(
        rates.clone(),
    ))));
    assert!(!ModelTemplate::compatible(&model));
    rates.input.currency = Some("USD".into());
    rates.input.amount = None;
    model.pricing = Some(Pricing::Usage(UsagePricing::Fixed(UsageRates::Token(
        rates,
    ))));
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
    model.pricing = Some(Pricing::Usage(UsagePricing::TieredToken(
        TieredTokenPricing {
            measured_by: TokenTierBasis::InputTokens,
            tiers: vec![],
        },
    )));
    assert!(!ModelTemplate::compatible(&model));
    model.pricing = None;
    assert!(!ModelTemplate::compatible(&model));
}
