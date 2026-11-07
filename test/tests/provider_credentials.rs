use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use compallm_core::provider::{ProviderApiKey, ProviderApiKeyError, ProviderCredentialCipher};

fn cipher() -> ProviderCredentialCipher {
    ProviderCredentialCipher::new("provider-credential-test-secret-at-least-32-bytes").unwrap()
}

#[test]
fn storage_round_trip_keeps_ciphertext_until_explicit_read() {
    let cipher = cipher();
    let key = ProviderApiKey::new("upstream-secret", &cipher).unwrap();
    let encrypted = String::from(key.clone());
    assert!(encrypted.starts_with("v1."));
    assert!(!encrypted.contains("upstream-secret"));
    assert!(!format!("{key:?}").contains("upstream-secret"));
    assert!(!format!("{key:?}").contains(&encrypted));
    let restored = ProviderApiKey::try_from(encrypted.clone()).unwrap();
    assert_eq!(restored.read(&cipher).unwrap(), "upstream-secret");
    assert_eq!(restored.read(&cipher).unwrap(), "upstream-secret");
    assert_eq!(String::from(restored), encrypted);
}

#[test]
fn encrypting_the_same_key_again_uses_fresh_ciphertext() {
    let cipher = cipher();
    let first = ProviderApiKey::new("upstream-secret", &cipher).unwrap();
    let second = ProviderApiKey::new("upstream-secret", &cipher).unwrap();
    assert_ne!(first, second);
    assert_eq!(first.read(&cipher).unwrap(), second.read(&cipher).unwrap());
}

#[test]
fn wrong_key_and_tampered_storage_fail_without_exposing_values() {
    let cipher = cipher();
    let key = ProviderApiKey::new("upstream-secret", &cipher).unwrap();
    let wrong =
        ProviderCredentialCipher::new("different-provider-credential-secret-at-least-32-bytes")
            .unwrap();
    assert_eq!(key.read(&wrong), Err(ProviderApiKeyError::DecryptionFailed));
    let encrypted = String::from(key);
    let bytes = URL_SAFE_NO_PAD
        .decode(encrypted.strip_prefix("v1.").unwrap())
        .unwrap();
    // Authenticate both the nonce and ciphertext/tag, rather than just decoding.
    for index in [0, 12, bytes.len() - 1] {
        let mut altered = bytes.clone();
        altered[index] ^= 1;
        let tampered =
            ProviderApiKey::try_from(format!("v1.{}", URL_SAFE_NO_PAD.encode(altered))).unwrap();
        let error = tampered.read(&cipher).unwrap_err();
        assert_eq!(error, ProviderApiKeyError::DecryptionFailed);
        assert!(!error.to_string().contains("upstream-secret"));
        assert!(!format!("{error:?}").contains(&encrypted));
    }
}

#[test]
fn storage_rejects_plaintext_unknown_versions_and_truncated_values() {
    for value in ["", "upstream-secret", "v2.AAAA", "v1.invalid!", "v1.AAAA"] {
        assert_eq!(
            ProviderApiKey::try_from(value.to_owned()),
            Err(ProviderApiKeyError::InvalidCiphertext)
        );
    }
    let nonce_and_tag_only = format!("v1.{}", URL_SAFE_NO_PAD.encode([0u8; 28]));
    assert_eq!(
        ProviderApiKey::try_from(nonce_and_tag_only),
        Err(ProviderApiKeyError::InvalidCiphertext)
    );
}

#[test]
fn construction_rejects_invalid_keys_and_short_encryption_secrets() {
    assert!(matches!(
        ProviderCredentialCipher::new("too-short"),
        Err(ProviderApiKeyError::InvalidConfiguration)
    ));
    for value in ["", " ", "api key", "key\n"] {
        assert_eq!(
            ProviderApiKey::new(value, &cipher()),
            Err(ProviderApiKeyError::InvalidKey)
        );
    }
}
