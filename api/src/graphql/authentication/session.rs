use compallm_core::authentication::Session as CoreSession;
use juniper::{ID, graphql_object};

/// Session identity projected only after authenticating the selected credential.
///
/// Token material, permission source, and internal freshness state are omitted.
pub struct Session(CoreSession);

impl From<CoreSession> for Session {
    fn from(session: CoreSession) -> Self {
        Self(session)
    }
}

#[graphql_object]
impl Session {
    /// Login session identity, serialized as a string without numeric precision loss.
    fn id(&self) -> ID {
        ID::new(self.0.id.to_string())
    }

    /// User owning this verified session.
    fn user_id(&self) -> ID {
        ID::new(self.0.user_id.to_string())
    }

    /// Organization selected for this verified session.
    fn organization_id(&self) -> ID {
        ID::new(self.0.organization_id.to_string())
    }

    /// Signed display snapshot; the name never establishes organization authority.
    fn organization_name(&self) -> &str {
        &self.0.organization_name
    }
}
