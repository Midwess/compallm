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
    permission: Arc<OnceCell<Result<Arc<MemberPermission>, MemberPermissionError>>>,
}

#[derive(Debug)]
pub enum AuthorizationError {
    Authentication(RequestError),
    InvalidAuthority,
}

impl std::fmt::Display for AuthorizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Authentication(_) => "authentication failed",
            Self::InvalidAuthority => "member authority could not be prepared",
        })
    }
}

impl std::error::Error for AuthorizationError {}

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
    pub async fn member_permission(&self) -> Result<Arc<MemberPermission>, AuthorizationError> {
        let user = self
            .user
            .require()
            .await
            .map_err(AuthorizationError::Authentication)?;
        self.permission
            .get_or_init(|| async {
                MemberPermission::try_from(user.access().clone()).map(Arc::new)
            })
            .await
            .as_ref()
            .cloned()
            .map_err(|_| AuthorizationError::InvalidAuthority)
    }
}

pub(super) fn context(user: UserContext) -> AuthorizationContext {
    AuthorizationContext {
        user,
        permission: Arc::new(OnceCell::new()),
    }
}
