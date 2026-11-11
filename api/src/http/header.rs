use super::authentication::RequestError;
use axum::http::{
    HeaderMap,
    header::{AUTHORIZATION, COOKIE},
};
use cookie::Cookie;

pub(super) fn bearer(headers: &HeaderMap) -> Result<Option<&str>, RequestError> {
    let mut values = headers.get_all(AUTHORIZATION).iter();
    let Some(value) = values.next() else {
        return Ok(None);
    };
    if values.next().is_some() {
        return Err(RequestError::Unauthenticated);
    }
    let value = value.to_str().map_err(|_| RequestError::Unauthenticated)?;
    let mut parts = value.split_ascii_whitespace();
    match (parts.next(), parts.next(), parts.next()) {
        (Some(scheme), Some(token), None) if scheme.eq_ignore_ascii_case("Bearer") => {
            Ok(Some(token))
        }
        _ => Err(RequestError::Unauthenticated),
    }
}

pub(super) fn cookie(headers: &HeaderMap, name: &str) -> Result<Option<String>, RequestError> {
    let mut value = None;
    for header in headers.get_all(COOKIE) {
        let header = header.to_str().map_err(|_| RequestError::Unauthenticated)?;
        for cookie in Cookie::split_parse(header) {
            let cookie = cookie.map_err(|_| RequestError::Unauthenticated)?;
            if cookie.name() == name && value.replace(cookie.value().to_owned()).is_some() {
                return Err(RequestError::Unauthenticated);
            }
        }
    }
    Ok(value)
}
