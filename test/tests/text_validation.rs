use compallm_core::organization::Organization;
use compallm_core::user::{User, UserStatus};
use compallm_utils::text::has_special_characters;
use validify::Validate;

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
    assert!(Organization::normalize_name("Team-42").is_ok());
    assert!(Organization::normalize_name(&"a".repeat(63)).is_ok());
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
            Organization::normalize_name(invalid).is_err(),
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
            User {
                id: 1,
                email: "a@example.com".into(),
                display_name: valid.into(),
                status: UserStatus::Active
            }
            .validate()
            .is_ok(),
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
            User {
                id: 1,
                email: "a@example.com".into(),
                display_name: invalid.into(),
                status: UserStatus::Active
            }
            .validate()
            .is_err(),
            "accepted invalid local part {invalid:?}"
        );
    }
}

#[test]
fn user_preparation_owns_normalization_and_registration_rules() {
    let user = User::from_email(42, "  James.Dang+dev@MIDWESS.COM  ").unwrap();
    assert_eq!(user.email, "james.dang+dev@midwess.com");
    assert_eq!(user.display_name, "james.dang+dev");
    for email in ["not-an-email", "@example.com", "james dang@example.com"] {
        assert!(User::from_email(42, email).is_err());
    }
    let too_long = User {
        id: 42,
        email: "a@example.com".into(),
        display_name: "a".repeat(257),
        status: UserStatus::Active,
    };
    assert!(too_long.validate().is_err());
}
