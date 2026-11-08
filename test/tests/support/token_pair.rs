#![allow(dead_code)]
#[path = "compression.rs"]
mod test_compression;

use compallm_core::auth::{
    AccessToken, AuthError, RefreshToken, Session, UserAccessToken, UserTokens,
};
use compallm_core::authorization::PolicySnapshot;
use std::time::Duration;

pub fn new(
    session: &Session,
    snapshot: PolicySnapshot,
    secret: &str,
    user_ttl: Duration,
    refresh_ttl: Duration,
) -> Result<UserTokens, AuthError> {
    let refresh = RefreshToken::from_session(session, refresh_ttl)?;
    let user = UserAccessToken::new(session.clone(), snapshot, user_ttl)?;
    Ok(UserTokens {
        access_validity: user.validity.clone(),
        refresh_validity: refresh.validity.clone(),
        access_token: AccessToken::User(user).encode(secret, user_ttl)?,
        refresh_token: AccessToken::Refresh(refresh).encode(secret, refresh_ttl)?,
    })
}

pub fn compressed_new(
    session: &Session,
    snapshot: PolicySnapshot,
    secret: &str,
    user_ttl: Duration,
    refresh_ttl: Duration,
) -> Result<UserTokens, AuthError> {
    let refresh = RefreshToken::from_session(session, refresh_ttl)?;
    let user = UserAccessToken::new(session.clone(), snapshot, user_ttl)?;
    let dictionary = compallm_core::compression::DCompression::new(
        compallm_core::compression::DictionaryType::AccessToken,
        1,
        "Compallm::Action:: Compallm::Role:: organization_id user_id policy_snapshot cedar_source",
    )?;
    let wrap = |token: AccessToken, ttl| -> Result<String, AuthError> {
        let payload = serde_json::json!({"dictionary_version": 1, "data": test_compression::compress(&dictionary, &serde_json::to_string(&token)?)?});
        let jwt = hero::token::jwt::create_jwt_token(&payload, secret, ttl)
            .map_err(|_| AuthError::InvalidToken)?;
        AccessToken::check_size(jwt.len())?;
        Ok(jwt)
    };
    Ok(UserTokens {
        access_validity: user.validity.clone(),
        refresh_validity: refresh.validity.clone(),
        access_token: wrap(AccessToken::User(user), user_ttl)?,
        refresh_token: wrap(AccessToken::Refresh(refresh), refresh_ttl)?,
    })
}
