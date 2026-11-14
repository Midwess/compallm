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
            AuthError::InvalidEmail | AuthError::InvalidUsername => {
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
            AuthError::Preferences(_)
            | AuthError::SessionPersistence(_)
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
            E::Persistence(compallm_core::persistent::organization::OrganizationRepositoryError::InvalidQuery) => Self::invalid_input(),
            E::GroupDeletion(_) => Self { code: "GROUP_ASSIGNED", status: 409, message: "Remove all members from this group before deleting it" },
            E::Persistence(compallm_core::persistent::organization::OrganizationRepositoryError::Conflict) => Self { code: "CONFLICT", status: 409, message: "Assignments changed; reload and retry" },
            E::RoleDeletion(compallm_core::organization::role::RoleDeletionError::Assigned) => Self { code: "ROLE_ASSIGNED", status: 409, message: "Remove all members and groups assigned to this role before deleting it" },
            E::RoleDeletion(compallm_core::organization::role::RoleDeletionError::OwnerRole) => Self { code: "FORBIDDEN", status: 403, message: "The owner role cannot be deleted" },
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

impl From<compallm_core::preference::UserPreferenceError> for Error {
    fn from(error: compallm_core::preference::UserPreferenceError) -> Self {
        use compallm_core::preference::UserPreferenceError;
        match error {
            UserPreferenceError::Invalid(_) | UserPreferenceError::InvalidReset => {
                Self::invalid_input()
            }
            UserPreferenceError::Authentication(error) => error.into(),
            UserPreferenceError::Persistence(_) => Self {
                code: "SERVICE_UNAVAILABLE",
                status: 503,
                message: "Preference state is temporarily unavailable",
            },
        }
    }
}

impl From<compallm_core::user::ProfileError> for Error {
    fn from(error: compallm_core::user::ProfileError) -> Self {
        use compallm_core::persistent::storage::user_space::StorageError;
        use compallm_core::user::ProfileError;
        match error {
            ProfileError::Authentication(error) => error.into(),
            ProfileError::Invalid(_) | ProfileError::InvalidOperation => Self::invalid_input(),
            ProfileError::Persistence(UserRepositoryError::UsernameAlreadyExists) => Self {
                code: "USERNAME_TAKEN",
                status: 409,
                message: "Username is already in use",
            },
            ProfileError::Denied | ProfileError::Persistence(UserRepositoryError::NotFound) => {
                Self {
                    code: "FORBIDDEN",
                    status: 403,
                    message: "Profile operation denied",
                }
            }
            ProfileError::Storage(StorageError::NotFound) => Self {
                code: "AVATAR_NOT_UPLOADED",
                status: 409,
                message: "Upload the avatar before saving the profile",
            },
            _ => Self {
                code: "SERVICE_UNAVAILABLE",
                status: 503,
                message: "Profile operation is temporarily unavailable",
            },
        }
    }
}

impl From<compallm_core::organization::profile_service::ProfileError> for Error {
    fn from(error: compallm_core::organization::profile_service::ProfileError) -> Self {
        use compallm_core::{
            authorization::MemberPermissionError,
            organization::profile_service::ProfileError,
            persistent::{organization::OrganizationRepositoryError, storage::StorageError},
        };
        match error {
            ProfileError::Permission(MemberPermissionError::Denied) => Self {
                code: "FORBIDDEN",
                status: 403,
                message: "Organization profile update denied",
            },
            ProfileError::Invalid | ProfileError::Storage(StorageError::InvalidRequest) => {
                Self::invalid_input()
            }
            ProfileError::Persistence(OrganizationRepositoryError::NotFound) => Self {
                code: "NOT_FOUND",
                status: 404,
                message: "Organization not found",
            },
            ProfileError::Storage(StorageError::NotFound) => Self {
                code: "AVATAR_NOT_UPLOADED",
                status: 409,
                message: "Upload the organization avatar before saving",
            },
            _ => Self {
                code: "SERVICE_UNAVAILABLE",
                status: 503,
                message: "Organization profile is temporarily unavailable",
            },
        }
    }
}
