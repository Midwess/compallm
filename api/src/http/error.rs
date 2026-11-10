use axum::{
    Json,
    http::{
        HeaderValue, StatusCode,
        header::{CACHE_CONTROL, WWW_AUTHENTICATE},
    },
    response::{IntoResponse, Response},
};
use compallm_core::authentication::AuthError;

#[derive(Clone, Copy)]
pub enum RequestError {
    Unauthenticated,
    Expired,
    ExpiredChallenge,
    Origin,
    Unavailable,
    CredentialTooLarge,
    Internal,
}

impl From<AuthError> for RequestError {
    fn from(error: AuthError) -> Self {
        match error {
            AuthError::ExpiredToken => Self::Expired,
            AuthError::TokenTooLarge { .. } => Self::CredentialTooLarge,
            AuthError::SessionPersistence(_)
            | AuthError::OrganizationRepository(_)
            | AuthError::Connection(_)
            | AuthError::DictionaryUnavailable
            | AuthError::Persistence(_) => Self::Unavailable,
            AuthError::InvalidConfiguration
            | AuthError::InvalidClock(_)
            | AuthError::IdGeneration(_)
            | AuthError::Unexpected(_)
            | AuthError::Organization(_)
            | AuthError::Email(_) => Self::Internal,
            _ => Self::Unauthenticated,
        }
    }
}

impl IntoResponse for RequestError {
    fn into_response(self) -> Response {
        let expired = matches!(self, Self::Expired);
        let (status, code, message) = match self {
            Self::Unauthenticated => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHENTICATED",
                "Authentication required",
            ),
            Self::Expired => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHENTICATED",
                "Access token expired; refresh explicitly",
            ),
            Self::ExpiredChallenge => (
                StatusCode::UNAUTHORIZED,
                "EXPIRED_TOKEN",
                "Email challenge expired; request a new code",
            ),
            Self::Origin => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN",
                "Untrusted browser origin",
            ),
            Self::Unavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "SERVICE_UNAVAILABLE",
                "Authorization state is temporarily unavailable",
            ),
            Self::CredentialTooLarge => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "CREDENTIAL_TOO_LARGE",
                "Authentication credential exceeds the supported size limit",
            ),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
                "Request failed",
            ),
        };
        let mut response = error_response(status, code, message);
        if status == StatusCode::UNAUTHORIZED {
            response.headers_mut().insert(WWW_AUTHENTICATE, HeaderValue::from_static(if expired {
                "Bearer error=\"invalid_token\", error_description=\"Access token expired; refresh explicitly\""
            } else { "Bearer" }));
        }
        response
    }
}

fn error_response(status: StatusCode, code: &'static str, message: &'static str) -> Response {
    no_store(
        (
            status,
            Json(serde_json::json!({
                "errors": [{ "message": message, "extensions": { "code": code } }]
            })),
        )
            .into_response(),
    )
}

pub(super) fn no_store(mut response: Response) -> Response {
    response
        .headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}
