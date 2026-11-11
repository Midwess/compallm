//! Metadata shared by Rust validation exporters.
/// Add only compatible constraints; conflicts indicate a broken export declaration.
pub fn merge_constraints(target: &mut serde_json::Value, patch: serde_json::Value) {
    for (key, value) in patch.as_object().expect("constraint object") {
        if let Some(previous) = target.get(key) {
            assert_eq!(previous, value, "conflicting exported constraint: {key}");
        }
        target[key] = value.clone();
    }
}
