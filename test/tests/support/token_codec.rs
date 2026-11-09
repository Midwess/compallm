#![allow(dead_code)]

use compallm_core::{
    auth::{AccessToken, AuthError, EmailOtpChallenge},
    compression::{DCompression, DictionaryType},
    persistent::dictionary::DictionaryRepository,
};
use std::time::Duration;

const CONTENT: &str =
    "Compallm::Action:: Compallm::Role:: organization_id user_id policy_snapshot cedar_source";

pub fn dictionary() -> DCompression {
    DCompression::new(DictionaryType::AccessToken, 1, CONTENT).unwrap()
}

struct Repository(DCompression);

#[async_trait::async_trait]
impl DictionaryRepository for Repository {
    async fn latest(&self, _: DictionaryType) -> anyhow::Result<Option<&DCompression>> {
        Ok(Some(&self.0))
    }
    async fn get(&self, _: DictionaryType, version: u32) -> anyhow::Result<Option<&DCompression>> {
        Ok((version == self.0.version).then_some(&self.0))
    }
}

pub fn encode(token: &AccessToken, secret: &str, lifetime: Duration) -> Result<String, AuthError> {
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        token
                            .clone()
                            .compress(&dictionary())
                            .await?
                            .encode(secret, lifetime)
                    })
            })
            .join()
            .expect("token test thread failed")
    })
}

pub fn decode(encoded: &str, secret: &str) -> Result<AccessToken, AuthError> {
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(AccessToken::decode(
                        encoded,
                        secret,
                        &Repository(dictionary()),
                    ))
            })
            .join()
            .expect("token test thread failed")
    })
}

pub fn challenge(email: &str, secret: &str) -> Result<EmailOtpChallenge, AuthError> {
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(EmailOtpChallenge::new(email, secret, &dictionary()))
            })
            .join()
            .expect("token test thread failed")
    })
}

pub fn sign_raw(token: &AccessToken, secret: &str, lifetime: Duration) -> String {
    use base64::Engine;
    use coset::{
        CborSerializable, CoseMac0Builder, HeaderBuilder,
        cwt::{ClaimsSetBuilder, Timestamp},
        iana,
    };
    use hmac::{Hmac, Mac};
    let now = compallm_utils::time::unix_timestamp() as i64;
    let mut payload = Vec::new();
    ciborium::ser::into_writer(token, &mut payload).unwrap();
    let claims = ClaimsSetBuilder::new()
        .issued_at(Timestamp::WholeSeconds(now))
        .expiration_time(Timestamp::WholeSeconds(now + lifetime.as_secs() as i64))
        .private_claim(-65537, ciborium::value::Value::Bytes(payload))
        .build()
        .to_vec()
        .unwrap();
    let mac = CoseMac0Builder::new()
        .protected(
            HeaderBuilder::new()
                .algorithm(iana::Algorithm::HMAC_256_256)
                .build(),
        )
        .payload(claims)
        .create_tag(b"", |bytes| {
            let mut mac = <Hmac<sha2::Sha256> as Mac>::new_from_slice(secret.as_bytes()).unwrap();
            mac.update(bytes);
            mac.finalize().into_bytes().to_vec()
        })
        .build()
        .to_vec()
        .unwrap();
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(mac)
}

pub fn sign_unchecked(token: &AccessToken, secret: &str, lifetime: Duration) -> String {
    let mut bytes = Vec::new();
    ciborium::ser::into_writer(token, &mut bytes).unwrap();
    let data = std::thread::scope(|scope| {
        scope
            .spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(dictionary().compress_bytes(&bytes))
            })
            .join()
            .unwrap()
            .unwrap()
    });
    sign_raw(
        &AccessToken::Compressed(compallm_core::auth::token::CompressedToken {
            dictionary_version: 1,
            data,
        }),
        secret,
        lifetime,
    )
}

pub fn envelope(encoded: &str) -> AccessToken {
    use base64::Engine;
    use coset::{CborSerializable, CoseMac0, cwt::ClaimsSet};
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(encoded)
        .unwrap();
    let cose = CoseMac0::from_slice(&bytes).unwrap();
    let claims = ClaimsSet::from_slice(cose.payload.as_deref().unwrap()).unwrap();
    let ciborium::value::Value::Bytes(token) = &claims.rest[0].1 else {
        panic!("missing token")
    };
    ciborium::de::from_reader(token.as_slice()).unwrap()
}

pub fn unpack(encoded: &str) -> serde_json::Value {
    let token = envelope(encoded);
    let AccessToken::Compressed(payload) = token else {
        panic!("expected compressed token")
    };
    let source = std::thread::scope(|scope| {
        scope
            .spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(dictionary().decompress_bytes(&payload.data))
            })
            .join()
            .unwrap()
            .unwrap()
    });
    let token: AccessToken = ciborium::de::from_reader(source.as_slice()).unwrap();
    serde_json::to_value(token).unwrap()
}
