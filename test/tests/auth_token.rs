use std::time::Duration;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use compallm_core::auth::{
    AccessToken, AuthError, EmailOtpAccessToken, EmailOtpChallenge, EncryptedAccessToken,
    EncryptedToken, RefreshToken, Session, UserAccessToken, UserTokens,
};
use compallm_core::organization::Role;
use compallm_utils::time::unix_timestamp;
use hero::token::jwt::create_jwt_token;

const SECRET: &str = "test-signing-secret-at-least-32-bytes";

struct Fixture;

impl Fixture {
    fn challenge() -> EmailOtpChallenge {
        EmailOtpChallenge::new(" James@MIDWESS.COM ", SECRET).unwrap()
    }

    fn tokens() -> UserTokens {
        UserTokens::new(
            &Session::new(1, 7, 84, "Research team"),
            Role::OWNER,
            empty_snapshot(),
            SECRET,
            Duration::from_secs(60),
            Duration::from_secs(600),
        )
        .unwrap()
    }
}

#[test]
fn email_jwt_exposes_ciphertext_not_email_or_code() {
    let challenge = Fixture::challenge();
    let decoded = URL_SAFE_NO_PAD
        .decode(challenge.access_token.split('.').nth(1).unwrap())
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&decoded).unwrap();
    assert!(
        json["data"]["encrypted"]
            .as_str()
            .unwrap()
            .starts_with("v1.")
    );
    let body = String::from_utf8(decoded).unwrap();
    assert!(!body.contains("james@midwess.com"));
    assert!(!body.contains("\"code\""));
    let claims = EmailOtpAccessToken::try_from(
        AccessToken::decode(&challenge.access_token, SECRET).unwrap(),
    )
    .unwrap();
    claims
        .verify_code(&challenge.code, unix_timestamp())
        .unwrap();
    assert_eq!(claims.email, "james@midwess.com");
    assert_eq!(claims.validity.expires_at - claims.validity.issued_at, 600);
}

#[test]
fn otp_comparison_and_expiration_are_strict() {
    let claims = EmailOtpAccessToken::new("james@midwess.com".into()).unwrap();
    let now = claims.validity.issued_at;
    assert!(claims.verify_code(&claims.code, now + 599).is_ok());
    assert!(matches!(
        claims.verify_code(&claims.code, now + 600),
        Err(AuthError::ExpiredToken)
    ));
    assert!(matches!(
        claims.verify_code("wrong", now),
        Err(AuthError::IncorrectCode)
    ));
    let wrong = if claims.code == "000000" {
        "000001"
    } else {
        "000000"
    };
    assert!(matches!(
        claims.verify_code(wrong, now),
        Err(AuthError::IncorrectCode)
    ));
}

#[test]
fn old_otp_pairs_remain_valid_after_another_is_issued() {
    let old = Fixture::challenge();
    let new = Fixture::challenge();
    assert_ne!(old.access_token, new.access_token);
    for challenge in [&old, &new, &old] {
        let claims = EmailOtpAccessToken::try_from(
            AccessToken::decode(&challenge.access_token, SECRET).unwrap(),
        )
        .unwrap();
        claims
            .verify_code(&challenge.code, unix_timestamp())
            .unwrap();
    }
}

#[test]
fn jwt_signature_and_payload_tampering_are_rejected() {
    let challenge = Fixture::challenge();
    assert!(matches!(
        AccessToken::decode(
            &challenge.access_token,
            "another-signing-secret-at-least-32-bytes"
        ),
        Err(AuthError::InvalidToken)
    ));
    let mut parts: Vec<String> = challenge
        .access_token
        .split('.')
        .map(str::to_owned)
        .collect();
    parts[1] = URL_SAFE_NO_PAD.encode(br#"{"data":{"user":{}},"exp":9999999999,"iat":1}"#);
    assert!(matches!(
        AccessToken::decode(&parts.join("."), SECRET),
        Err(AuthError::InvalidToken)
    ));
}

#[test]
fn ciphertext_requires_its_configured_secret_and_integrity_tag() {
    let claims = EmailOtpAccessToken::new("james@midwess.com".into()).unwrap();
    let cipher = EncryptedAccessToken::new(SECRET).unwrap();
    // The cipher can work directly with an inner struct; it knows no enum.
    let payload = cipher.encrypt(&claims).unwrap();
    let decrypted: EmailOtpAccessToken = cipher.decrypt(&payload).unwrap();
    assert_eq!(decrypted.code, claims.code);
    assert!(matches!(
        EncryptedAccessToken::new("another-signing-secret-at-least-32-bytes")
            .unwrap()
            .decrypt::<EmailOtpAccessToken>(&payload),
        Err(AuthError::TokenCrypto(_))
    ));
    let mut bytes = URL_SAFE_NO_PAD
        .decode(payload.0.strip_prefix("v1.").unwrap())
        .unwrap();
    *bytes.last_mut().unwrap() ^= 1;
    let altered = EncryptedToken(format!("v1.{}", URL_SAFE_NO_PAD.encode(bytes)));
    assert!(matches!(
        cipher.decrypt::<EmailOtpAccessToken>(&altered),
        Err(AuthError::TokenCrypto(_))
    ));
}

#[test]
fn valid_jwt_cannot_extend_expired_inner_claims() {
    let mut claims = UserAccessToken::new(
        Session::new(1, 7, 84, "Research team"),
        Role::OWNER,
        empty_snapshot(),
        Duration::from_secs(60),
    )
    .unwrap();
    claims.validity.issued_at -= 61;
    claims.validity.expires_at -= 61;
    let jwt =
        create_jwt_token(AccessToken::User(claims), SECRET, Duration::from_secs(600)).unwrap();
    assert!(matches!(
        AccessToken::decode(&jwt, SECRET),
        Err(AuthError::ExpiredToken)
    ));
}

#[test]
fn token_variants_cannot_authorize_each_other() {
    let challenge = Fixture::challenge();
    let otp = AccessToken::decode(&challenge.access_token, SECRET).unwrap();
    assert!(matches!(
        UserAccessToken::try_from(otp.clone()),
        Err(AuthError::WrongTokenVariant)
    ));
    assert!(matches!(
        RefreshToken::try_from(otp),
        Err(AuthError::WrongTokenVariant)
    ));
    let issued = Fixture::tokens();
    let user = AccessToken::decode(&issued.access_token, SECRET).unwrap();
    let refresh = AccessToken::decode(&issued.refresh_token, SECRET).unwrap();
    assert_eq!(
        UserAccessToken::try_from(user.clone())
            .unwrap()
            .session
            .user_id,
        1
    );
    assert_eq!(
        RefreshToken::try_from(refresh.clone())
            .unwrap()
            .session
            .user_id,
        1
    );
    assert!(matches!(
        EmailOtpAccessToken::try_from(user.clone()),
        Err(AuthError::WrongTokenVariant)
    ));
    assert!(matches!(
        RefreshToken::try_from(user),
        Err(AuthError::WrongTokenVariant)
    ));
    assert!(matches!(
        UserAccessToken::try_from(refresh),
        Err(AuthError::WrongTokenVariant)
    ));
}

#[test]
fn email_otp_accepts_valid_addresses_from_any_domain() {
    for email in [
        "a@gmail.com",
        "a@example.org",
        "a@sub.midwess.com",
        "a@evilmidwess.com",
        "a@midwess.com.evil",
    ] {
        let challenge = EmailOtpChallenge::new(email, SECRET).unwrap();
        let claims = EmailOtpAccessToken::try_from(
            AccessToken::decode(&challenge.access_token, SECRET).unwrap(),
        )
        .unwrap();
        assert_eq!(claims.email, email);
    }
    for email in ["@midwess.com", "not an email", "a@@example.org"] {
        assert!(matches!(
            EmailOtpChallenge::new(email, SECRET),
            Err(AuthError::InvalidEmail)
        ));
    }
}

#[test]
fn ids_round_trip_as_u64_in_access_and_refresh_sessions() {
    let one = UserTokens::new(
        &Session::new(9_007_199_254_740_993, 7, 84, "Research team"),
        Role::OWNER,
        empty_snapshot(),
        SECRET,
        Duration::from_secs(60),
        Duration::from_secs(600),
    )
    .unwrap();
    let payload = URL_SAFE_NO_PAD
        .decode(one.access_token.split('.').nth(1).unwrap())
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&payload).unwrap();
    assert_eq!(
        json["data"]["user"]["session"]["user_id"].as_u64(),
        Some(9_007_199_254_740_993)
    );
    assert_eq!(
        UserAccessToken::try_from(AccessToken::decode(&one.access_token, SECRET).unwrap())
            .unwrap()
            .session
            .user_id,
        9_007_199_254_740_993
    );
    let first =
        RefreshToken::try_from(AccessToken::decode(&one.refresh_token, SECRET).unwrap()).unwrap();
    assert_eq!(first.session.id, 7);
    assert_eq!(first.session.user_id, 9_007_199_254_740_993);
    assert_eq!(first.session.organization_id, 84);
    assert_eq!(
        json["data"]["user"]["session"].as_object().unwrap().len(),
        6
    );
}

#[test]
fn refresh_requires_an_embedded_session_and_rejects_legacy_claims() {
    let claims = RefreshToken::from_session(
        &Session::new(1, 7, 84, "Research team"),
        Duration::from_secs(60),
    )
    .unwrap();
    let mut serialized = serde_json::to_value(&claims).unwrap();
    serialized.as_object_mut().unwrap().remove("session");
    let jwt = create_jwt_token(
        serde_json::json!({"refresh":serialized}),
        SECRET,
        Duration::from_secs(60),
    )
    .unwrap();
    assert!(AccessToken::decode(&jwt, SECRET).is_err());
    assert_eq!(serde_json::to_value(claims).unwrap()["session"]["id"], 7);
}

#[test]
fn access_and_refresh_tokens_include_the_organization_name_and_require_it() {
    let tokens = Fixture::tokens();
    for encoded in [&tokens.access_token, &tokens.refresh_token] {
        let claims = AccessToken::decode(encoded, SECRET).unwrap();
        let mut payload = serde_json::to_value(&claims).unwrap();
        let variant = if payload.get("user").is_some() {
            "user"
        } else {
            "refresh"
        };
        let session = &mut payload[variant]["session"];
        assert_eq!(session["organization_id"], 84);
        assert_eq!(session["organization_name"], "Research team");
        session.as_object_mut().unwrap().remove("organization_name");
        let legacy = create_jwt_token(payload, SECRET, Duration::from_secs(60)).unwrap();
        assert!(AccessToken::decode(&legacy, SECRET).is_err());
    }
}

#[test]
fn malformed_tokens_and_invalid_lifetimes_are_rejected() {
    for input in ["", "not.jwt", "a.b.c"] {
        assert!(matches!(
            AccessToken::decode(input, SECRET),
            Err(AuthError::InvalidToken)
        ));
    }
    assert!(matches!(
        UserTokens::new(
            &Session::new(1, 7, 84, "Research team"),
            Role::OWNER,
            empty_snapshot(),
            SECRET,
            Duration::ZERO,
            Duration::from_secs(600)
        ),
        Err(AuthError::InvalidConfiguration)
    ));
}

#[test]
fn encryption_preserves_the_variant_for_typed_conversion() {
    let cipher = EncryptedAccessToken::new(SECRET).unwrap();
    let user = UserAccessToken::new(
        Session::new(1, 7, 84, "Research team"),
        Role::OWNER,
        empty_snapshot(),
        Duration::from_secs(60),
    )
    .unwrap();
    let jwt = AccessToken::User(user)
        .encrypt(&cipher)
        .unwrap()
        .encode(SECRET, Duration::from_secs(60))
        .unwrap();
    assert_eq!(
        UserAccessToken::try_from(AccessToken::decode(&jwt, SECRET).unwrap())
            .unwrap()
            .session
            .user_id,
        1
    );
    let refresh = RefreshToken::from_session(
        &Session::new(1, 7, 84, "Research team"),
        Duration::from_secs(600),
    )
    .unwrap();
    let jwt = AccessToken::Refresh(refresh)
        .encrypt(&cipher)
        .unwrap()
        .encode(SECRET, Duration::from_secs(600))
        .unwrap();
    assert_eq!(
        RefreshToken::try_from(AccessToken::decode(&jwt, SECRET).unwrap())
            .unwrap()
            .session
            .user_id,
        1
    );
}

#[test]
fn plaintext_otp_and_nested_encryption_are_rejected() {
    let claims = EmailOtpAccessToken::new("james@midwess.com".into()).unwrap();
    let plaintext = AccessToken::EmailOtp(claims.clone());
    assert!(matches!(
        plaintext.encode(SECRET, Duration::from_secs(600)),
        Err(AuthError::WrongTokenVariant)
    ));
    let jwt = create_jwt_token(plaintext, SECRET, Duration::from_secs(600)).unwrap();
    assert!(matches!(
        AccessToken::decode(&jwt, SECRET),
        Err(AuthError::WrongTokenVariant)
    ));
    let cipher = EncryptedAccessToken::new(SECRET).unwrap();
    let encrypted = AccessToken::EmailOtp(claims).encrypt(&cipher).unwrap();
    assert!(matches!(
        encrypted.clone().encrypt(&cipher),
        Err(AuthError::InvalidToken)
    ));
    // Even valid authenticated nested ciphertext cannot bypass enum validation.
    let nested = AccessToken::Encrypted(cipher.encrypt(&encrypted).unwrap());
    let jwt = nested.encode(SECRET, Duration::from_secs(600)).unwrap();
    assert!(matches!(
        AccessToken::decode(&jwt, SECRET),
        Err(AuthError::InvalidToken)
    ));
}

#[test]
fn injected_secret_controls_both_jwt_and_encryption() {
    assert!(matches!(
        EncryptedAccessToken::new("short"),
        Err(AuthError::InvalidConfiguration)
    ));
    let challenge = Fixture::challenge();
    assert_eq!(
        EmailOtpAccessToken::try_from(
            AccessToken::decode(&challenge.access_token, SECRET).unwrap()
        )
        .unwrap()
        .email,
        "james@midwess.com"
    );
}

#[test]
fn encrypted_token_errors_preserve_their_source() {
    use std::error::Error;
    let error = EncryptedAccessToken::new(SECRET)
        .unwrap()
        .decrypt::<EmailOtpAccessToken>(&EncryptedToken("v1.!!!!".into()))
        .err()
        .unwrap();
    assert!(matches!(&error, AuthError::TokenEncoding(_)));
    assert!(error.source().unwrap().is::<base64::DecodeError>());
}

#[test]
fn enum_checks_common_timestamps_then_inner_invariants() {
    let mut otp = EmailOtpAccessToken::new("james@midwess.com".into()).unwrap();
    otp.code = "abcdef".into();
    assert!(matches!(
        AccessToken::EmailOtp(otp).validate(unix_timestamp()),
        Err(AuthError::InvalidToken)
    ));
    let mut future = UserAccessToken::new(
        Session::new(1, 7, 84, "Research team"),
        Role::OWNER,
        empty_snapshot(),
        Duration::from_secs(60),
    )
    .unwrap();
    future.validity.issued_at += 100;
    future.validity.expires_at += 100;
    assert!(matches!(
        AccessToken::User(future).validate(unix_timestamp()),
        Err(AuthError::InvalidToken)
    ));
}

#[test]
fn token_validity_owns_expiration_and_timestamp_validation() {
    use compallm_core::auth::token::TokenValidity;

    let validity = TokenValidity::new(Duration::from_secs(60)).unwrap();
    let now = validity.issued_at;
    assert!(validity.validate(now).is_ok());
    assert!(validity.validate(now + 59).is_ok());
    assert!(matches!(
        validity.validate(now + 60),
        Err(AuthError::ExpiredToken)
    ));
    assert!(matches!(
        validity.validate(now - 1),
        Err(AuthError::InvalidToken)
    ));
    assert!(matches!(
        TokenValidity::new(Duration::ZERO),
        Err(AuthError::InvalidConfiguration)
    ));
    assert!(matches!(
        TokenValidity::new(Duration::from_secs(u64::MAX)),
        Err(AuthError::InvalidConfiguration)
    ));

    for expires_at in [999, 1000] {
        let invalid = TokenValidity {
            issued_at: 1000,
            expires_at,
        };
        assert!(matches!(
            invalid.validate(unix_timestamp()),
            Err(AuthError::InvalidToken)
        ));
    }
}

#[test]
fn both_tokens_link_to_the_same_organization_scoped_session() {
    let one = UserTokens::new(
        &Session::new(1, 7, 84, "Research team"),
        Role::OWNER,
        empty_snapshot(),
        SECRET,
        Duration::from_secs(60),
        Duration::from_secs(600),
    )
    .unwrap();
    let two = UserTokens::new(
        &Session::new(1, 7, 85, "Research team"),
        Role::MEMBER,
        empty_snapshot(),
        SECRET,
        Duration::from_secs(60),
        Duration::from_secs(600),
    )
    .unwrap();
    for (jwt, variant) in [(&one.access_token, "user"), (&one.refresh_token, "refresh")] {
        let payload = URL_SAFE_NO_PAD
            .decode(jwt.split('.').nth(1).unwrap())
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&payload).unwrap();
        let claims = &json["data"][variant];
        assert_eq!(claims["session"]["id"], 7);
        assert_eq!(claims["session"]["user_id"], 1);
        assert_eq!(claims["session"]["organization_id"], 84);
        for field in ["session_id", "user_id", "organization_id", "token_id"] {
            assert!(claims.get(field).is_none(), "duplicated claim: {field}");
        }
    }
    assert_ne!(one.access_token, two.access_token);
    let first =
        UserAccessToken::try_from(AccessToken::decode(&one.access_token, SECRET).unwrap()).unwrap();
    let second =
        UserAccessToken::try_from(AccessToken::decode(&two.access_token, SECRET).unwrap()).unwrap();
    assert_eq!(first.session.organization_id, 84);
    assert_eq!(first.organization_role, Role::OWNER);
    assert_eq!(second.session.organization_id, 85);
    assert_eq!(second.organization_role, Role::MEMBER);
    let payload = URL_SAFE_NO_PAD
        .decode(one.refresh_token.split('.').nth(1).unwrap())
        .unwrap();
    let refresh: serde_json::Value = serde_json::from_slice(&payload).unwrap();
    assert_eq!(refresh["data"]["refresh"]["session"]["user_id"], 1);
    assert_eq!(refresh["data"]["refresh"]["session"]["organization_id"], 84);
    assert_eq!(
        refresh["data"]["refresh"]["session"]["id"],
        first.session.id
    );
    let refresh_claims =
        RefreshToken::try_from(AccessToken::decode(&one.refresh_token, SECRET).unwrap()).unwrap();
    assert_eq!(first.session, refresh_claims.session);
    assert!(
        refresh["data"]["refresh"]
            .get("organization_role")
            .is_none()
    );
}

#[test]
fn access_tokens_reject_unknown_roles() {
    let mut claims = serde_json::to_value(
        UserAccessToken::new(
            Session::new(1, 7, 84, "Research team"),
            Role::OWNER,
            empty_snapshot(),
            Duration::from_secs(60),
        )
        .unwrap(),
    )
    .unwrap();
    claims["organization_role"] = serde_json::json!("super_admin");
    let jwt = create_jwt_token(
        serde_json::json!({"user": claims}),
        SECRET,
        Duration::from_secs(60),
    )
    .unwrap();
    assert!(matches!(
        AccessToken::decode(&jwt, SECRET),
        Err(AuthError::InvalidToken)
    ));
}

#[test]
fn issued_token_metadata_matches_the_encoded_claims() {
    let tokens = Fixture::tokens();
    let access =
        UserAccessToken::try_from(AccessToken::decode(&tokens.access_token, SECRET).unwrap())
            .unwrap();
    let refresh =
        RefreshToken::try_from(AccessToken::decode(&tokens.refresh_token, SECRET).unwrap())
            .unwrap();
    assert_eq!(tokens.access_validity.issued_at, access.validity.issued_at);
    assert_eq!(
        tokens.access_validity.expires_at,
        access.validity.expires_at
    );
    assert_eq!(
        tokens.refresh_validity.issued_at,
        refresh.validity.issued_at
    );
    assert_eq!(
        tokens.refresh_validity.expires_at,
        refresh.validity.expires_at
    );
    let challenge = EmailOtpChallenge::new("reader@example.com", SECRET).unwrap();
    let claims = EmailOtpAccessToken::try_from(
        AccessToken::decode(&challenge.access_token, SECRET).unwrap(),
    )
    .unwrap();
    assert_eq!(challenge.validity.expires_at, claims.validity.expires_at);
}

#[test]
fn wire_budget_checks_actual_encoded_bytes_on_issuance_and_input() {
    let lifetime = Duration::from_secs(60);
    let claims = |name_len| {
        UserAccessToken::new(
            Session::new(1, 7, 84, "a".repeat(name_len)),
            Role::MEMBER,
            empty_snapshot(),
            lifetime,
        )
        .unwrap()
    };
    // Find the largest name whose actual signed credential fits; base64 has steps.
    let mut low: usize = 0;
    let mut high: usize = 4_000;
    while low < high {
        let middle = (low + high).div_ceil(2);
        let encoded =
            create_jwt_token(AccessToken::User(claims(middle)), SECRET, lifetime).unwrap();
        if encoded.len() <= AccessToken::MAX_ENCODED_BYTES {
            low = middle;
        } else {
            high = middle - 1;
        }
    }
    let accepted = AccessToken::User(claims(low))
        .encode(SECRET, lifetime)
        .unwrap();
    assert!(accepted.len() <= AccessToken::MAX_ENCODED_BYTES);
    assert!(AccessToken::decode(&accepted, SECRET).is_ok());
    assert!(matches!(
        AccessToken::User(claims(low + 1)).encode(SECRET, lifetime),
        Err(AuthError::TokenTooLarge { .. })
    ));
    let oversized = create_jwt_token(AccessToken::User(claims(low + 1)), SECRET, lifetime).unwrap();
    assert!(matches!(
        AccessToken::decode(&oversized, SECRET),
        Err(AuthError::TokenTooLarge { .. })
    ));
}

#[test]
fn plain_snapshot_budget_measures_real_jwt_including_unicode_and_escaping() -> anyhow::Result<()> {
    use compallm_core::{authorization::PolicySnapshot, organization::OrganizationMember};
    for role in Role::defaults(84) {
        let snapshot = PolicySnapshot::new(&OrganizationMember {
            organization_id: 84,
            user_id: 1,
            roles: vec![role.clone()],
            groups: vec![],
        })?;
        let access = UserAccessToken::new(
            Session::new(1, 7, 84, "Nghiên cứu \"Team\""),
            role.name.to_lowercase(),
            snapshot.clone(),
            Duration::from_secs(60),
        )?;
        let mut proposed = serde_json::to_value(&access)?;
        proposed["policy_snapshot"] = serde_json::to_value(&snapshot)?;
        let candidate = create_jwt_token(
            serde_json::json!({"user": proposed}),
            SECRET,
            Duration::from_secs(60),
        )?;
        let measured = AccessToken::User(access)
            .encode(SECRET, Duration::from_secs(60))
            .map(|encoded| encoded.len());
        println!(
            "{}: plain Cedar {} bytes; snapshot JSON {} bytes; proposed JWT {} / {} bytes",
            role.name,
            snapshot.cedar_source.len(),
            serde_json::to_vec(&snapshot)?.len(),
            candidate.len(),
            AccessToken::MAX_ENCODED_BYTES
        );
        if candidate.len() > AccessToken::MAX_ENCODED_BYTES {
            assert!(
                matches!(measured, Err(AuthError::TokenTooLarge {size, limit}) if size == candidate.len() && limit == AccessToken::MAX_ENCODED_BYTES)
            );
        } else {
            assert_eq!(measured?, candidate.len());
        }
        assert_eq!(
            snapshot
                .cedar_source
                .parse::<cedar_policy::PolicySet>()?
                .policies()
                .count(),
            cedar_policy::PolicySet::try_from(&role)?.policies().count()
        );
    }
    Ok(())
}

fn empty_snapshot() -> compallm_core::authorization::PolicySnapshot {
    compallm_core::authorization::PolicySnapshot {
        cedar_source: String::new(),
        role_ids: vec![],
        group_roles: std::collections::BTreeMap::new(),
    }
}

#[test]
fn access_requires_the_signed_snapshot_and_refresh_omits_it() {
    let tokens = Fixture::tokens();
    let access =
        UserAccessToken::try_from(AccessToken::decode(&tokens.access_token, SECRET).unwrap())
            .unwrap();
    let mut payload = serde_json::to_value(AccessToken::User(access)).unwrap();
    assert!(payload["user"]["policy_snapshot"].is_object());
    payload["user"]
        .as_object_mut()
        .unwrap()
        .remove("policy_snapshot");
    let legacy = create_jwt_token(payload, SECRET, Duration::from_secs(60)).unwrap();
    assert!(matches!(
        AccessToken::decode(&legacy, SECRET),
        Err(AuthError::InvalidToken)
    ));
    let refresh = AccessToken::decode(&tokens.refresh_token, SECRET).unwrap();
    assert!(
        serde_json::to_value(refresh).unwrap()["refresh"]
            .get("policy_snapshot")
            .is_none()
    );
}
