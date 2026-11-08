use compallm_core::organization::{Group, Role};

#[test]
fn role_and_group_names_obey_character_limits_after_trimming() {
    for (limit, normalize) in [
        (
            Role::MAX_NAME_LENGTH,
            Role::normalize_name as fn(&str) -> Result<String, validify::ValidationErrors>,
        ),
        (
            Group::MAX_NAME_LENGTH,
            Group::normalize_name as fn(&str) -> Result<String, validify::ValidationErrors>,
        ),
    ] {
        assert!(normalize(" \t ").is_err());
        assert_eq!(normalize("  Trợ lý  ").unwrap(), "Trợ lý");
        assert!(normalize(&"界".repeat(limit)).is_ok());
        assert!(normalize(&"界".repeat(limit + 1)).is_err());
    }
}
