use compallm_core::{model::ModelTemplate, provider::*};
use serde_json::json;

#[test]
fn capability_lists_derive_from_fields_without_treating_unknown_as_unsupported() {
    let mut model:ModelTemplate=serde_json::from_value(json!({
        "id":"model","providerTemplateId":"provider","researchId":"research","name":"model",
        "displayName":null,"description":null,"serviceKind":"llm","connections":{},"headers":{},"bodyDefaults":{},
        "text":{"status":"unknown"},"vision":{"status":"unknown"},"pdf":{"status":"unknown"},
        "audio":{"input":{"status":"unknown"},"output":{"status":"unknown"}},"video":{"status":"unknown"},
        "imageGeneration":{"status":"unknown"},"thinking":{"status":"unknown"},
        "tools":{"calling":{"status":"unknown"},"forcedChoice":{"status":"unknown"},"parallelCalls":{"status":"unknown"}},
        "structuredOutput":{"status":"unknown"},"search":{"status":"unknown"},"limits":{},"pricing":null,"routeSelection":null
    })).unwrap();
    assert_eq!(model.unknown_capabilities(), ALL_CAPABILITIES);
    assert!(model.supported_capabilities().is_empty());
    assert!(model.unsupported_capabilities().is_empty());
    model.vision = CapabilitySupport::Supported(VisionCapability::default());
    model.tools.calling = CapabilitySupport::Supported(ToolCallingCapability::default());
    model.audio.output = CapabilitySupport::Unsupported;
    model.thinking = CapabilitySupport::Supported(Thinking::default());
    assert_eq!(
        model.supported_capabilities(),
        vec![
            Capability::Vision,
            Capability::Reasoning,
            Capability::ToolCall
        ]
    );
    assert_eq!(
        model.unsupported_capabilities(),
        vec![Capability::AudioGeneration]
    );
    let encoded = serde_json::to_value(&model).unwrap();
    assert_eq!(
        model,
        serde_json::from_value::<ModelTemplate>(encoded).unwrap()
    );
}

#[test]
fn reasoning_support_does_not_require_configurable_effort_or_budget() {
    let value = CapabilitySupport::Supported(Thinking::default());
    assert!(value.is_supported());
    assert!(value.configuration().unwrap().effort_mapping.is_empty());
    assert!(value.configuration().unwrap().budget.is_none());
    let mut configured = Thinking::default();
    configured
        .effort_mapping
        .insert(EffortLevel::Low, "minimal".into());
    configured.default_effort = Some(EffortLevel::Low);
    configured.budget = Some(ThinkingBudget {
        min_tokens: 100,
        max_tokens: 8192,
        default_tokens: Some(1024),
    });
    let support = CapabilitySupport::Supported(configured);
    let json = serde_json::to_value(&support).unwrap();
    assert_eq!(
        support,
        serde_json::from_value::<CapabilitySupport<Thinking>>(json).unwrap()
    );
}
