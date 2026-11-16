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
    message: std::borrow::Cow<'static, str>,
}

impl From<AuthError> for Error {
    fn from(error: AuthError) -> Self {
        let (code, status, message) = match error {
            AuthError::ReauthenticationRequired => (
                "REAUTHENTICATION_REQUIRED",
                403,
                "Verify your identity to continue",
            ),
            AuthError::InvalidPasskey => ("PASSKEY_INVALID", 401, "Passkey verification failed"),
            AuthError::PasskeyConflict => (
                "PASSKEY_CONFLICT",
                409,
                "This passkey is already registered",
            ),
            AuthError::PasskeyLimit => ("PASSKEY_LIMIT", 409, "You can add up to 20 passkeys"),
            AuthError::PasskeyUnavailable => {
                ("PASSKEY_UNAVAILABLE", 503, "Passkeys are not configured")
            }
            AuthError::InvalidInvitation => (
                "INVITATION_UNAVAILABLE",
                410,
                "Invitation is expired, revoked, consumed, or unavailable",
            ),
            AuthError::OrganizationRepository(
                compallm_core::persistent::organization::OrganizationRepositoryError::InvalidQuery,
            ) => ("BAD_USER_INPUT", 400, "Invalid collection query"),
            AuthError::InvitationConflict => (
                "INVITATION_CONFLICT",
                409,
                "Recipient already belongs to this organization or has a pending invitation",
            ),
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
            AuthError::UserDisabled => (
                "ACCOUNT_DISABLED",
                403,
                "This account is disabled and cannot sign in or receive an invitation",
            ),
            AuthError::OrganizationAccessDenied => ("FORBIDDEN", 403, "Organization access denied"),
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
            AuthError::PasskeyPersistence(_)
            | AuthError::Preferences(_)
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
            message: message.into(),
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
            message: message.into(),
        }
    }
}

impl From<validify::ValidationErrors> for Error {
    fn from(errors: validify::ValidationErrors) -> Self {
        Self {
            code: "BAD_USER_INPUT",
            status: 400,
            message: errors
                .errors()
                .iter()
                .find_map(|error| error.message())
                .map(Into::into)
                .unwrap_or_else(|| "Check the highlighted fields and try again.".into()),
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
            message: "Invalid request fields".into(),
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
                message: "Authorization state is invalid. Please sign in again.".into(),
            },
        }
    }
}

impl From<compallm_core::authorization::error::AuthorizationError> for Error {
    fn from(error: compallm_core::authorization::error::AuthorizationError) -> Self {
        match error {
            E::Persistence(compallm_core::persistent::organization::OrganizationRepositoryError::InvalidQuery) => Self::invalid_input(),
            E::GroupDeletion(_) => Self { code: "GROUP_ASSIGNED", status: 409, message: "Remove all members from this group before deleting it".into() },
            E::Persistence(compallm_core::persistent::organization::OrganizationRepositoryError::Conflict) => Self { code: "CONFLICT", status: 409, message: "Assignments changed; reload and retry".into() },
            E::RoleDeletion(compallm_core::organization::role::RoleDeletionError::Assigned) => Self { code: "ROLE_ASSIGNED", status: 409, message: "Remove all members and groups assigned to this role before deleting it".into() },
            E::RoleDeletion(compallm_core::organization::role::RoleDeletionError::OwnerRole) => Self { code: "FORBIDDEN", status: 403, message: "The owner role cannot be deleted".into() },
            E::Membership(compallm_core::organization::member::MembershipAssignmentError::SelfRoleChange) => Self { code: "SELF_ROLE_CHANGE", status: 403, message: "You cannot change your own role".into() },
            E::Membership(compallm_core::organization::member::MembershipAssignmentError::OwnerChangeDenied) => Self { code: "FORBIDDEN", status: 403, message: "Permission denied".into() },
            E::Membership(compallm_core::organization::member::MembershipAssignmentError::OwnerRole) => Self { code: "LAST_OWNER", status: 409, message: "The last active owner cannot be unassigned".into() },
            E::Membership(compallm_core::organization::member::MembershipAssignmentError::ForeignOrganization) => Self::invalid_input(),
            E::Permission(MemberPermissionError::Denied) => Self { code: "FORBIDDEN", status: 403, message: "Permission denied".into() },
            E::Group(compallm_core::organization::group::GroupCreationError::IdGeneration(_)) | E::Role(compallm_core::organization::role::RoleCreationError::IdGeneration(_)) => Self { code: "SERVICE_UNAVAILABLE", status: 503, message: "Authorization operation is temporarily unavailable".into() },
            E::Group(_) | E::Role(_) | E::UnsupportedPolicy => Self::invalid_input(),
            E::Persistence(compallm_core::persistent::organization::OrganizationRepositoryError::NameAlreadyExists) => Self { code: "CONFLICT", status: 409, message: "Name already exists in this organization".into() },
            E::Persistence(compallm_core::persistent::organization::OrganizationRepositoryError::NotFound) => Self { code: "NOT_FOUND", status: 404, message: "Resource not found".into() },
            _ => Self { code: "SERVICE_UNAVAILABLE", status: 503, message: "Authorization operation is temporarily unavailable".into() },
        }
    }
}

impl Error {
    pub(super) fn internal() -> Self {
        Self {
            code: "INTERNAL_SERVER_ERROR",
            status: 500,
            message: "Request failed".into(),
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
                message: "Preference state is temporarily unavailable".into(),
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
            ProfileError::Invalid(errors) => errors.into(),
            ProfileError::InvalidOperation => Self::invalid_input(),
            ProfileError::Persistence(UserRepositoryError::UsernameAlreadyExists) => Self {
                code: "USERNAME_TAKEN",
                status: 409,
                message: "Username is already in use".into(),
            },
            ProfileError::Denied | ProfileError::Persistence(UserRepositoryError::NotFound) => {
                Self {
                    code: "FORBIDDEN",
                    status: 403,
                    message: "Profile operation denied".into(),
                }
            }
            ProfileError::Storage(StorageError::NotFound) => Self {
                code: "AVATAR_NOT_UPLOADED",
                status: 409,
                message: "Upload the avatar before saving the profile".into(),
            },
            _ => Self {
                code: "SERVICE_UNAVAILABLE",
                status: 503,
                message: "Profile operation is temporarily unavailable".into(),
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
                message: "Organization profile update denied".into(),
            },
            ProfileError::Invalid | ProfileError::Storage(StorageError::InvalidRequest) => {
                Self::invalid_input()
            }
            ProfileError::Persistence(OrganizationRepositoryError::NotFound) => Self {
                code: "NOT_FOUND",
                status: 404,
                message: "Organization not found".into(),
            },
            ProfileError::Storage(StorageError::NotFound) => Self {
                code: "AVATAR_NOT_UPLOADED",
                status: 409,
                message: "Upload the organization avatar before saving".into(),
            },
            _ => Self {
                code: "SERVICE_UNAVAILABLE",
                status: 503,
                message: "Organization profile is temporarily unavailable".into(),
            },
        }
    }
}

impl From<compallm_core::analytics::AnalyticsError> for Error {
    fn from(e: compallm_core::analytics::AnalyticsError) -> Self {
        use compallm_core::analytics::AnalyticsError as E;
        match e {
            E::Invalid(e) => e.into(),
            E::Authentication(e) => e.into(),
            E::Persistence(_) => Self {
                code: "SERVICE_UNAVAILABLE",
                status: 503,
                message: "Analytics is temporarily unavailable".into(),
            },
        }
    }
}
impl From<compallm_core::onboarding::OnboardingError> for Error {
    fn from(e: compallm_core::onboarding::OnboardingError) -> Self {
        use compallm_core::onboarding::OnboardingError as E;
        match e {
            E::Invalid(e) => e.into(),
            E::Authentication(e) => e.into(),
            E::Permission(e) => {
                compallm_core::authorization::error::AuthorizationError::from(e).into()
            }
            E::Requirements => Self {
                code: "ONBOARDING_REQUIREMENTS",
                status: 422,
                message: "Complete the required profile fields before continuing".into(),
            },
            E::Transition => Self {
                code: "ONBOARDING_CONFLICT",
                status: 409,
                message: "Reload onboarding before continuing".into(),
            },
            _ => Self {
                code: "SERVICE_UNAVAILABLE",
                status: 503,
                message: "Onboarding is temporarily unavailable".into(),
            },
        }
    }
}
