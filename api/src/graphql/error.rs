use crate::http::authentication::RequestError;
use compallm_core::{
    authentication::AuthError,
    authorization::{MemberPermissionError, error::AuthorizationError as E},
    mail::EmailError,
    persistent::user::UserRepositoryError,
};
use juniper::{FieldError, IntoFieldError, ScalarValue, graphql_value};

/// Safe operation failure, with no submitted values or internal diagnostics.
pub struct Error {
    code: &'static str,
    status: i32,
    message: &'static str,
}

impl From<AuthError> for Error {
    fn from(error: AuthError) -> Self {
        let (code, status, message) = match error {
            AuthError::Persistence(UserRepositoryError::EmailAlreadyExists) => (
                "EMAIL_REGISTERED",
                409,
                "Email is already registered. <a href=\"/login\">Sign in instead.</a>",
            ),
            AuthError::UserNotFound => (
                "USER_NOT_FOUND",
                404,
                "Email is not registered. <a href=\"/signup\">Sign up here.</a>",
            ),
            AuthError::UserDisabled | AuthError::OrganizationAccessDenied => {
                ("FORBIDDEN", 403, "Organization access denied")
            }
            AuthError::IncorrectCode => ("INCORRECT_CODE", 401, "Incorrect verification code"),
            AuthError::ExpiredToken => ("EXPIRED_TOKEN", 401, "Authentication credential expired"),
            AuthError::InvalidSession
            | AuthError::InvalidToken
            | AuthError::WrongTokenVariant
            | AuthError::TokenEncoding(_)
            | AuthError::TokenPayload(_)
            | AuthError::TokenCrypto(_) => ("UNAUTHENTICATED", 401, "Authentication required"),
            AuthError::InvalidEmail | AuthError::InvalidDisplayName => {
                ("BAD_USER_INPUT", 400, "Invalid request fields")
            }
            AuthError::Email(EmailError::Throttled) => (
                "THROTTLED",
                429,
                "Email recipient is throttled; keep the existing challenge and retry later",
            ),
            AuthError::Email(_) => ("SERVICE_UNAVAILABLE", 503, "Email delivery is unavailable"),
            AuthError::TokenTooLarge { .. } => (
                "CREDENTIAL_TOO_LARGE",
                422,
                "Authentication credential exceeds the supported size limit",
            ),
            AuthError::SessionPersistence(_)
            | AuthError::OrganizationRepository(_)
            | AuthError::Connection(_)
            | AuthError::DictionaryUnavailable
            | AuthError::Persistence(_) => (
                "SERVICE_UNAVAILABLE",
                503,
                "Authentication state is temporarily unavailable",
            ),
            _ => ("INTERNAL_SERVER_ERROR", 500, "Request failed"),
        };
        Self {
            code,
            status,
            message,
        }
    }
}

impl From<RequestError> for Error {
    fn from(error: RequestError) -> Self {
        let (code, status, message) = match error {
            RequestError::Origin => ("FORBIDDEN", 403, "Untrusted browser origin"),
            RequestError::Unauthenticated | RequestError::Expired => {
                ("UNAUTHENTICATED", 401, "Authentication required")
            }
            RequestError::ExpiredChallenge => (
                "EXPIRED_TOKEN",
                401,
                "Email challenge expired; request a new code",
            ),
            RequestError::Unavailable => (
                "SERVICE_UNAVAILABLE",
                503,
                "Authentication state is temporarily unavailable",
            ),
            RequestError::CredentialTooLarge => (
                "CREDENTIAL_TOO_LARGE",
                422,
                "Authentication credential exceeds the supported size limit",
            ),
            RequestError::Internal => ("INTERNAL_SERVER_ERROR", 500, "Request failed"),
        };
        Self {
            code,
            status,
            message,
        }
    }
}

impl From<validify::ValidationErrors> for Error {
    fn from(_: validify::ValidationErrors) -> Self {
        Self {
            code: "BAD_USER_INPUT",
            status: 400,
            message: "Invalid request fields",
        }
    }
}

impl<S: ScalarValue> IntoFieldError<S> for Error {
    fn into_field_error(self) -> FieldError<S> {
        FieldError::new(
            self.message,
            graphql_value!({"code": self.code, "status": self.status}),
        )
    }
}

impl Error {
    pub(crate) fn invalid_input() -> Self {
        Self {
            code: "BAD_USER_INPUT",
            status: 400,
            message: "Invalid request fields",
        }
    }
}

impl From<crate::http::authorization::AuthorizationError> for Error {
    fn from(error: crate::http::authorization::AuthorizationError) -> Self {
        match error {
            crate::http::authorization::AuthorizationError::Authentication(error) => error.into(),
            crate::http::authorization::AuthorizationError::InvalidAuthority => Self {
                code: "INVALID_AUTHORIZATION_STATE",
                status: 401,
                message: "Authorization state is invalid. Please sign in again.",
            },
        }
    }
}

impl From<compallm_core::authorization::error::AuthorizationError> for Error {
    fn from(error: compallm_core::authorization::error::AuthorizationError) -> Self {
        match error {
            E::Permission(MemberPermissionError::Denied) => Self { code: "FORBIDDEN", status: 403, message: "Permission denied" },
            E::Group(compallm_core::organization::group::GroupCreationError::IdGeneration(_)) | E::Role(compallm_core::organization::role::RoleCreationError::IdGeneration(_)) => Self { code: "SERVICE_UNAVAILABLE", status: 503, message: "Authorization operation is temporarily unavailable" },
            E::Group(_) | E::Role(_) | E::UnsupportedPolicy => Self::invalid_input(),
            E::Persistence(compallm_core::persistent::organization::OrganizationRepositoryError::NameAlreadyExists) => Self { code: "CONFLICT", status: 409, message: "Name already exists in this organization" },
            E::Persistence(compallm_core::persistent::organization::OrganizationRepositoryError::NotFound) => Self { code: "NOT_FOUND", status: 404, message: "Resource not found" },
            _ => Self { code: "SERVICE_UNAVAILABLE", status: 503, message: "Authorization operation is temporarily unavailable" },
        }
    }
}

impl Error {
    pub(super) fn internal() -> Self {
        Self {
            code: "INTERNAL_SERVER_ERROR",
            status: 500,
            message: "Request failed",
        }
    }
}
