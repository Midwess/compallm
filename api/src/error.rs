use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use compallm_core::{auth::AuthError, mail::EmailError, persistent::user::UserRepositoryError};

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    Auth(#[from] AuthError),
    #[error("invalid JSON request body")]
    Json(#[from] JsonRejection),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::Json(_) => (StatusCode::BAD_REQUEST, "Invalid JSON request body"),
            Self::Auth(AuthError::InvalidEmail) => {
                (StatusCode::BAD_REQUEST, "Invalid email address")
            }
            Self::Auth(AuthError::InvalidDisplayName) => (
                StatusCode::BAD_REQUEST,
                "User name must contain 1 to 256 email local-part characters; dots cannot be first, last, or repeated",
            ),
            Self::Auth(AuthError::Persistence(UserRepositoryError::EmailAlreadyExists)) => {
                (StatusCode::CONFLICT, "Email is already registered")
            }
            Self::Auth(AuthError::UserDisabled) => (StatusCode::FORBIDDEN, "User is disabled"),
            Self::Auth(AuthError::OrganizationAccessDenied) => {
                (StatusCode::FORBIDDEN, "Organization access denied")
            }
            Self::Auth(AuthError::UserNotFound) => (StatusCode::NOT_FOUND, "User not found"),
            Self::Auth(AuthError::Email(EmailError::Unavailable | EmailError::Delivery(_))) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Email delivery is unavailable",
            ),
            Self::Auth(
                AuthError::InvalidSession
                | AuthError::RefreshTokenReused
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
