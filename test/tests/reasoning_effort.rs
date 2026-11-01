use compallm_core::provider::ReasoningEffort;

#[test]
fn reasoning_effort_accepts_only_portable_scale_values() {
    let minimum = ReasoningEffort::new(ReasoningEffort::MIN).unwrap();
    let maximum = ReasoningEffort::new(ReasoningEffort::MAX).unwrap();

    assert_eq!(minimum.value(), 1);
    assert_eq!(maximum.value(), 255);
    assert!(ReasoningEffort::new(0).is_err());
    assert!(minimum < maximum);

    for value in ReasoningEffort::MIN..=ReasoningEffort::MAX {
        assert_eq!(ReasoningEffort::new(value).unwrap().value(), value);
    }
}
