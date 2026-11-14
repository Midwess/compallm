use compallm_core::authentication::Session as CoreSession;

/// Session identity projected only after authenticating the selected credential.
///
/// Token material, permission source, and internal freshness state are omitted.
pub struct Session(pub(super) CoreSession, pub(super) String);

impl From<(CoreSession, String)> for Session {
    fn from((session, name): (CoreSession, String)) -> Self {
        Self(session, name)
    }
}

use juniper::{ID, graphql_object};

#[graphql_object(context = crate::graphql::GraphqlContext)]
impl Session {
    /// Public avatar for this user; clients show a fallback while absent or loading.
    fn avatar_url(
        &self,
        #[graphql(context)] storage: &Option<std::sync::Arc<compallm_storage::S3Storage>>,
    ) -> Option<String> {
        use compallm_core::persistent::storage::user_space::UserStorageSpace;
        crate::graphql::user::avatar_url(
            &compallm_storage::S3UserSpace::get_avatar_path(self.0.user_id),
            storage,
        )
    }

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

    /// Current organization name.
    fn organization_name(&self) -> &str {
        &self.1
    }
}
