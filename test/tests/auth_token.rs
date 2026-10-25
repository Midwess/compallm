use std::time::Duration;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use compallm_core::auth::{
    AccessToken, AuthError, EmailOtpAccessToken, EmailOtpChallenge, EncryptedAccessToken,
    EncryptedToken, RefreshToken, UserAccessToken, UserTokens,
};
use compallm_utils::time::unix_timestamp;
use hero::token::jwt::create_jwt_token;

const SECRET: &str = "test-signing-secret-at-least-32-bytes";

struct Fixture;

impl Fixture {
    fn challenge() -> EmailOtpChallenge {
        EmailOtpChallenge::new(" James@MIDWESS.COM ", "midwess.com", SECRET).unwrap()
    }

    fn tokens() -> UserTokens {
        UserTokens::new(1, SECRET, Duration::from_secs(60), Duration::from_secs(600)).unwrap()
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
    let mut claims = UserAccessToken::new(1, Duration::from_secs(60)).unwrap();
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
    assert_eq!(UserAccessToken::try_from(user.clone()).unwrap().user_id, 1);
    assert_eq!(RefreshToken::try_from(refresh.clone()).unwrap().user_id, 1);
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
fn organization_email_domain_must_match_exactly() {
    for email in [
        "a@evilmidwess.com",
        "a@midwess.com.evil",
        "a@sub.midwess.com",
        "@midwess.com",
        "not an email",
    ] {
        assert!(matches!(
            EmailOtpChallenge::new(email, "midwess.com", SECRET),
            Err(AuthError::InvalidEmail)
        ));
    }
    let issued = Fixture::challenge();
    let claims =
        EmailOtpAccessToken::try_from(AccessToken::decode(&issued.access_token, SECRET).unwrap())
            .unwrap();
    assert!(matches!(
        EmailOtpChallenge::new(&claims.email, "another.com", SECRET),
        Err(AuthError::InvalidEmail)
    ));
}

#[test]
fn ids_round_trip_as_u64_and_refresh_tokens_have_distinct_ids() {
    let one = UserTokens::new(
        9_007_199_254_740_993,
        SECRET,
        Duration::from_secs(60),
        Duration::from_secs(600),
    )
    .unwrap();
    let two = UserTokens::new(
        9_007_199_254_740_993,
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
        json["data"]["user"]["user_id"].as_u64(),
        Some(9_007_199_254_740_993)
    );
    assert_eq!(
        UserAccessToken::try_from(AccessToken::decode(&one.access_token, SECRET).unwrap())
            .unwrap()
            .user_id,
        9_007_199_254_740_993
    );
    assert_ne!(
        RefreshToken::try_from(AccessToken::decode(&one.refresh_token, SECRET).unwrap())
            .unwrap()
            .token_id,
        RefreshToken::try_from(AccessToken::decode(&two.refresh_token, SECRET).unwrap())
            .unwrap()
            .token_id
    );
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
        UserTokens::new(1, SECRET, Duration::ZERO, Duration::from_secs(60)),
        Err(AuthError::InvalidConfiguration)
    ));
    assert!(matches!(
        UserAccessToken::new(0, Duration::from_secs(60)),
        Err(AuthError::InvalidToken)
    ));
}

#[test]
fn encryption_preserves_the_variant_for_typed_conversion() {
    let cipher = EncryptedAccessToken::new(SECRET).unwrap();
    let user = UserAccessToken::new(1, Duration::from_secs(60)).unwrap();
    let jwt = AccessToken::User(user)
        .encrypt(&cipher)
        .unwrap()
        .encode(SECRET, Duration::from_secs(60))
        .unwrap();
    assert_eq!(
        UserAccessToken::try_from(AccessToken::decode(&jwt, SECRET).unwrap())
            .unwrap()
            .user_id,
        1
    );
    let refresh = RefreshToken::new(1, Duration::from_secs(600)).unwrap();
    let jwt = AccessToken::Refresh(refresh)
        .encrypt(&cipher)
        .unwrap()
        .encode(SECRET, Duration::from_secs(600))
        .unwrap();
    assert_eq!(
        RefreshToken::try_from(AccessToken::decode(&jwt, SECRET).unwrap())
            .unwrap()
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
    let mut user = UserAccessToken::new(1, Duration::from_secs(60)).unwrap();
    user.user_id = 0;
    assert!(matches!(
        AccessToken::User(user).validate(unix_timestamp()),
        Err(AuthError::InvalidToken)
    ));
    let mut otp = EmailOtpAccessToken::new("james@midwess.com".into()).unwrap();
    otp.code = "abcdef".into();
    assert!(matches!(
        AccessToken::EmailOtp(otp).validate(unix_timestamp()),
        Err(AuthError::InvalidToken)
    ));
    let mut refresh = RefreshToken::new(1, Duration::from_secs(60)).unwrap();
    refresh.token_id = "invalid".into();
    assert!(matches!(
        AccessToken::Refresh(refresh).validate(unix_timestamp()),
        Err(AuthError::InvalidToken)
    ));
    let mut future = UserAccessToken::new(1, Duration::from_secs(60)).unwrap();
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
