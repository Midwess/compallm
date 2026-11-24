use compallm_core::{
    model::{CatalogCustomization, ModelCatalog},
    provider::*,
};
use serde_json::json;

fn template() -> ModelCatalog {
    let price = CapabilityPricing::Usage(CapabilityUsagePricing::Fixed(CapabilityRates::Token(
        CapabilityTokenRate {
            rate: Rate {
                amount: Some("0.125000000000000001".parse().unwrap()),
                currency: Some("USD".into()),
                quantity: Some(1_000_000),
            },
            cache: Some(CachePricing::Unsupported),
        },
    )));
    ModelCatalog{custom_fields:Default::default(),id:"test".into(),provider_catalog_id:"openai".into(),research_id:Some("research".into()),name:"original".into(),display_name:None,description:None,service_kind:Some(ServiceKind::Llm),connections:[(Protocol::Openai,serde_json::from_value(json!({"baseUrl":"https://api.example/v1","path":"/chat/completions","timeoutMs":30000,"authentication":{"type":"api_key","configuration":{"placement":{"apiKeyHeader":{"name":"Authorization","prefix":"Bearer "}},"requiredInputs":[]}},"endpoint":{"mode":"fixed","urlTemplate":null,"requiredInputs":[]}})).unwrap())].into(),headers:Default::default(),body_defaults:Default::default(),text:TextCapabilities{input:CapabilitySupport::supported(TextInput{max_tokens:Some(128000)},Some(price.clone())),output:CapabilitySupport::supported(TextGeneration{max_tokens:Some(16000),thinking:CapabilitySupport::supported(Thinking{budget:Some(ThinkingBudget{min_tokens:100,max_tokens:10000,default_tokens:Some(500)}),..Default::default()},None)},Some(price))},vision:Default::default(),pdf:Default::default(),audio:Default::default(),video:Default::default(),image_generation:Default::default(),tools:Default::default(),structured_output:Default::default(),search:Default::default(),limits:ModelLimits{context_tokens:Some(128000)},billing:None,route_selection:None}
}
#[test]
fn private_edits_preserve_unexposed_configuration_and_restore_individual_defaults() {
    let original = template();
    let mut personal = original.clone();
    CatalogCustomization {
        name: Some("custom-name".into()),
        limits: Some((Some(256000), Some(200000), Some(64000))),
        ..Default::default()
    }
    .apply(&mut personal, Some(&original))
    .unwrap();
    assert_eq!(personal.text.input.pricing(), original.text.input.pricing());
    assert_eq!(
        personal.text.output.configuration().unwrap().thinking,
        original.text.output.configuration().unwrap().thinking
    );
    CatalogCustomization {
        limits: Some((None, None, None)),
        ..Default::default()
    }
    .apply(&mut personal, Some(&original))
    .unwrap();
    assert_eq!(personal.limits, original.limits);
    assert_eq!(personal.text, original.text);
    assert_eq!(personal.name, "custom-name");
}
#[test]
fn creating_a_configured_model_copies_configuration_independently_of_future_template_edits() {
    let mut original = template();
    let configured = ProviderModel::try_from((&original, Protocol::Openai)).unwrap();
    CatalogCustomization {
        input_price: Some(Some("1.75".parse().unwrap())),
        ..Default::default()
    }
    .apply(&mut original, None)
    .unwrap();
    assert_ne!(
        configured.text.input.pricing(),
        original.text.input.pricing()
    );
    assert_eq!(
        configured.text.input.pricing(),
        template().text.input.pricing()
    );
    assert_eq!(configured.template.unwrap().model_id, "test");
}
