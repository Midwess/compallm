//! Request-local authorization information supplied after authentication.
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use std::sync::Arc;
use tokio::sync::OnceCell;

use compallm_core::authorization::{MemberPermission, MemberPermissionError};

use super::authentication::{RequestError, UserContext};

/// Shares the authenticated request facts without evaluating permissions.
#[derive(Clone)]
pub struct AuthorizationContext {
    user: UserContext,
    permission: Arc<OnceCell<Result<MemberPermission, MemberPermissionError>>>,
}

pub enum AuthorizationError {
    Authentication(RequestError),
    InvalidAuthority,
}

impl IntoResponse for AuthorizationError {
    fn into_response(self) -> Response {
        match self {
            Self::Authentication(error) => error.into_response(),
            Self::InvalidAuthority => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

impl AuthorizationContext {
    /// Prepare organization-bound permission once, using only verified, current access.
    pub async fn member_permission(&self) -> Result<&MemberPermission, AuthorizationError> {
        let user = self
            .user
            .require()
            .await
            .map_err(AuthorizationError::Authentication)?;
        self.permission
            .get_or_init(|| async { MemberPermission::try_from(user.access().clone()) })
            .await
            .as_ref()
            .map_err(|_| AuthorizationError::InvalidAuthority)
    }
}

pub(super) fn context(user: UserContext) -> AuthorizationContext {
    AuthorizationContext {
        user,
        permission: Arc::new(OnceCell::new()),
    }
}
