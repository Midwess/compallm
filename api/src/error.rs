use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use compallm_core::{auth::AuthError, mail::EmailError, persistent::user::UserRepositoryError};

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct ErrorResponse {
    /// Human-readable explanation of the failed request. Use the HTTP status to classify the error.
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("authentication required")]
    Unauthenticated,
    #[error("access denied")]
    Forbidden,
    #[error("untrusted browser origin")]
    UntrustedOrigin,
    #[error("invalid response header")]
    Header(#[from] axum::http::header::InvalidHeaderValue),
    #[error(transparent)]
    Auth(#[from] AuthError),
    #[error("invalid request input")]
    Input(#[from] axum_valid::ValidifyRejection<JsonRejection>),
}

impl ApiError {
    /// Authentication does not disclose whether the credential's user still exists.
    pub fn into_authentication_response(self) -> Response {
        match self {
            Self::Auth(AuthError::UserNotFound) => Self::Unauthenticated.into_response(),
            error => error.into_response(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::Unauthenticated => (StatusCode::UNAUTHORIZED, "Authentication required"),
            Self::Forbidden => (StatusCode::FORBIDDEN, "Access denied"),
            Self::UntrustedOrigin => (StatusCode::FORBIDDEN, "Untrusted browser origin"),
            Self::Input(axum_valid::ValidifyRejection::Inner(_)) => {
                (StatusCode::BAD_REQUEST, "Invalid JSON request body")
            }
            Self::Input(axum_valid::ValidifyRejection::Valid(_)) => {
                (StatusCode::BAD_REQUEST, "Invalid request fields")
            }
            Self::Auth(AuthError::InvalidEmail) => {
                (StatusCode::BAD_REQUEST, "Invalid email address")
            }
            Self::Auth(AuthError::InvalidDisplayName) => (
                StatusCode::BAD_REQUEST,
                "User name must contain 1 to 256 email local-part characters; dots cannot be first, last, or repeated",
            ),
            Self::Auth(AuthError::Persistence(UserRepositoryError::EmailAlreadyExists)) => (
                StatusCode::CONFLICT,
                "Email is already registered. Please sign in instead.",
            ),
            Self::Auth(AuthError::UserDisabled) => (StatusCode::FORBIDDEN, "User is disabled"),
            Self::Auth(AuthError::OrganizationAccessDenied) => {
                (StatusCode::FORBIDDEN, "Organization access denied")
            }
            Self::Auth(AuthError::UserNotFound) => (StatusCode::NOT_FOUND, "User not found"),
            Self::Auth(AuthError::Email(EmailError::Throttled)) => (
                StatusCode::TOO_MANY_REQUESTS,
                "Email recipient is throttled; keep the existing challenge and retry later",
            ),
            Self::Auth(AuthError::Email(EmailError::QueueTimeout)) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Email queue is full; retry later",
            ),
            Self::Auth(AuthError::Email(EmailError::Unavailable | EmailError::Delivery(_))) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Email delivery is unavailable",
            ),
            Self::Auth(
                AuthError::InvalidSession
                | AuthError::InvalidToken
                | AuthError::ExpiredToken
                | AuthError::WrongTokenVariant
                | AuthError::IncorrectCode
                | AuthError::TokenEncoding(_)
                | AuthError::TokenPayload(_)
                | AuthError::TokenCrypto(_),
            ) => (
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token or OTP code",
            ),
            _ => {
                tracing::error!(error = %self, "signup request failed");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };
        (
            status,
            [(axum::http::header::CACHE_CONTROL, "no-store")],
            Json(ErrorResponse {
                message: message.into(),
            }),
        )
            .into_response()
    }
}
