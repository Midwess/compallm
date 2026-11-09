use async_graphql::{Context, Error, ErrorExtensions, Object};

use super::Session;
use crate::middleware::authentication::VerifiedAccess;

/// Authentication-related queries sharing the request's verified access context.
#[derive(Default)]
pub struct AuthQuery;

#[Object]
impl AuthQuery {
    /// Read the selected login's verified identity and organization-name snapshot.
    ///
    /// Requires active, current user access. No token material or permission source
    /// is exposed, and no additional user/organization query is performed.
    async fn session(&self, ctx: &Context<'_>) -> async_graphql::Result<Session> {
        let claims = ctx.data_opt::<VerifiedAccess>().ok_or_else(|| {
            Error::new("Authentication required")
                .extend_with(|_, extensions| extensions.set("code", "UNAUTHENTICATED"))
        })?;
        Ok(claims.access().session.clone().into())
    }
}
