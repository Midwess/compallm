#![allow(dead_code)]
use crate::test_codec;

use compallm_core::authentication::{
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
        access_token: test_codec::encode(&AccessToken::User(user), secret, user_ttl)?,
        refresh_token: test_codec::encode(&AccessToken::Refresh(refresh), secret, refresh_ttl)?,
    })
}

pub fn compressed_new(
    session: &Session,
    snapshot: PolicySnapshot,
    secret: &str,
    user_ttl: Duration,
    refresh_ttl: Duration,
) -> Result<UserTokens, AuthError> {
    new(session, snapshot, secret, user_ttl, refresh_ttl)
}
