use async_graphql::{ID, Object};
use compallm_core::auth::Session as CoreSession;

/// Session identity projected only after authenticating the selected credential.
///
/// Token material, permission source, and internal freshness state are omitted.
pub struct Session(CoreSession);

impl From<CoreSession> for Session {
    fn from(session: CoreSession) -> Self {
        Self(session)
    }
}

#[Object]
impl Session {
    /// Login session identity, serialized as a string without numeric precision loss.
    async fn id(&self) -> ID {
        ID(self.0.id.to_string())
    }

    /// User owning this verified session.
    async fn user_id(&self) -> ID {
        ID(self.0.user_id.to_string())
    }

    /// Organization selected for this verified session.
    async fn organization_id(&self) -> ID {
        ID(self.0.organization_id.to_string())
    }

    /// Signed display snapshot; the name never establishes organization authority.
    async fn organization_name(&self) -> &str {
        &self.0.organization_name
    }
}
