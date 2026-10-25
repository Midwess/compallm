use compallm_core::organization::Organization;
use compallm_utils::{email::verify_email_local_part, text::has_special_characters};

#[test]
fn special_character_validation_uses_the_callers_allowlist() {
    assert!(!has_special_characters("Team-42", "-"));
    assert!(has_special_characters("Team_42", "-"));
    assert!(!has_special_characters("Team_42", "_-"));
    assert!(has_special_characters("Team 42", "-"));
    assert!(has_special_characters("Téam", "-"));
}

#[test]
fn organization_names_are_ascii_and_strictly_shorter_than_64_characters() {
    assert!(Organization::is_valid_name("Team-42"));
    assert!(Organization::is_valid_name(&"a".repeat(63)));
    for invalid in [
        "",
        "Team Name",
        "team_name",
        "team.name",
        "team@name",
        "Téam",
        "team\nname",
        &"a".repeat(64),
    ] {
        assert!(
            !Organization::is_valid_name(invalid),
            "accepted invalid name {invalid:?}"
        );
    }
}

#[test]
fn user_names_follow_unquoted_email_local_part_character_rules() {
    for valid in [
        "James",
        "james.dang",
        "james+dev",
        "james_dang",
        "O'Neil",
        "a!#$%&'*+-/=?^_`{|}~z",
    ] {
        assert!(
            verify_email_local_part(valid).is_ok(),
            "rejected valid local part {valid:?}"
        );
    }
    for invalid in [
        "",
        ".james",
        "james.",
        "james..dang",
        "james dang",
        "james@midwess.com",
        "james<dev>",
        "james\n",
        "jämés",
        "\"james\"",
    ] {
        assert!(
            verify_email_local_part(invalid).is_err(),
            "accepted invalid local part {invalid:?}"
        );
    }
}
