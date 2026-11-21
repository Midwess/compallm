use compallm_core::{
    model::Model,
    provider::{ALL_CAPABILITIES, Capability, FreePricing, ModelLimits, Pricing, ServiceKind},
};

#[test]
fn unsupported_capabilities_follow_the_models_supported_list() {
    let mut model = Model {
        id: 1,
        organization_id: 1,
        name: "review-model".into(),
        service_kind: ServiceKind::Llm,
        enabled: true,
        capabilities: vec![],
        limits: ModelLimits::default(),
        thinking: None,
        pricing: Pricing::Free(FreePricing {
            description: None,
            allowances: vec![],
        }),
    };
    assert_eq!(model.unsupported_capabilities(), ALL_CAPABILITIES);
    model.capabilities = vec![Capability::Vision, Capability::ToolCall];
    let unsupported = model.unsupported_capabilities();
    assert_eq!(
        unsupported.len() + model.capabilities.len(),
        ALL_CAPABILITIES.len()
    );
    for capability in ALL_CAPABILITIES {
        assert_ne!(
            model.capabilities.contains(&capability),
            unsupported.contains(&capability)
        );
    }
    model.capabilities = ALL_CAPABILITIES.to_vec();
    assert!(model.unsupported_capabilities().is_empty());
}
