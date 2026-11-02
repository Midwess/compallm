use compallm_core::provider::ProviderModel;
use compallm_postgres::ProviderModelSchema;

fn provider_model() -> ProviderModelSchema {
    ProviderModelSchema {
        id: 1,
        organization_id: 2,
        provider_id: 3,
        name: "reasoning-model".into(),
        enabled: true,
        input_price_per_million_tokens: 1.into(),
        output_price_per_million_tokens: 2.into(),
        cached_price_per_million_tokens: 0.into(),
        reasoning_effort_values: Some(vec!["low".into(), "high".into()]),
        supports_reasoning_budget: true,
        supports_adaptive_reasoning: false,
        minimum_reasoning_budget_tokens: Some(1),
        maximum_reasoning_budget_tokens: Some(i32::MAX),
    }
}

#[test]
fn mapping_preserves_native_effort_values_without_normalizing() {
    let mut stored = provider_model();
    let values = vec!["low".into(), "HIGH".into(), "provider-specific".into()];
    stored.reasoning_effort_values = Some(values.clone());
    let model = ProviderModel::try_from(stored).unwrap();
    assert!(model.reasoning_effort_values.is_some());
    assert_eq!(model.reasoning_effort_values, Some(values));
    assert_eq!(model.minimum_reasoning_budget_tokens, Some(1));
    assert_eq!(model.maximum_reasoning_budget_tokens, Some(i32::MAX as u32));
}

#[test]
fn mapping_distinguishes_absent_effort_values_from_empty_values() {
    let mut stored = provider_model();
    stored.reasoning_effort_values = None;
    let model = ProviderModel::try_from(stored).unwrap();
    assert!(model.reasoning_effort_values.is_none());

    let mut stored = provider_model();
    stored.reasoning_effort_values = Some(Vec::new());
    let model = ProviderModel::try_from(stored).unwrap();
    assert!(model.reasoning_effort_values.is_some());
    assert_eq!(model.reasoning_effort_values, Some(Vec::new()));
}

#[test]
fn mapping_preserves_absent_budgets_and_rejects_negative_budgets() {
    let mut stored = provider_model();
    stored.minimum_reasoning_budget_tokens = None;
    stored.maximum_reasoning_budget_tokens = None;
    let model = ProviderModel::try_from(stored).unwrap();
    assert_eq!(model.minimum_reasoning_budget_tokens, None);
    assert_eq!(model.maximum_reasoning_budget_tokens, None);

    let mut stored = provider_model();
    stored.minimum_reasoning_budget_tokens = Some(-1);
    assert!(ProviderModel::try_from(stored).is_err());

    let mut stored = provider_model();
    stored.maximum_reasoning_budget_tokens = Some(-1);
    assert!(ProviderModel::try_from(stored).is_err());
}
