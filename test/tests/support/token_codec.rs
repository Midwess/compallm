#![allow(dead_code)]
#[path = "compression.rs"]
mod compression;

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

pub fn sign_unchecked(token: &AccessToken, secret: &str, lifetime: Duration) -> String {
    let payload = serde_json::json!({"compressed": {"dictionary_version": 1, "data": compression::compress(&dictionary(), &serde_json::to_string(token).unwrap()).unwrap()}});
    hero::token::jwt::create_jwt_token(&payload, secret, lifetime).unwrap()
}

pub fn unpack(jwt: &str) -> serde_json::Value {
    let payload: AccessToken =
        hero::token::jwt::decode_jwt_token(jwt, "test-signing-secret-at-least-32-bytes").unwrap();
    let AccessToken::Compressed(payload) = payload else {
        panic!("expected compressed token")
    };
    serde_json::from_str(&compression::decompress(&dictionary(), &payload.data).unwrap()).unwrap()
}
