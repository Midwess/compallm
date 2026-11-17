use compallm_core::authentication::Session as CoreSession;

/// Session identity projected only after authenticating the selected credential.
///
/// Token material, permission source, and internal freshness state are omitted.
pub struct Session(pub(super) CoreSession, pub(super) Option<String>);

impl From<(CoreSession, String)> for Session {
    fn from((session, name): (CoreSession, String)) -> Self {
        Self(session, Some(name))
    }
}

impl From<CoreSession> for Session {
    fn from(session: CoreSession) -> Self {
        Self(session, None)
    }
}

use juniper::{ID, graphql_object};

#[graphql_object(context = crate::graphql::GraphqlContext)]
impl Session {
    fn expires_at(&self) -> String {
        self.0.expires_at.to_string()
    }

    fn last_seen_at(&self) -> String {
        self.0.last_seen_at.to_string()
    }

    fn signed_in_at(&self) -> String {
        (hero::distributed_id::id_to_unix_timestamp(self.0.id) / 1000).to_string()
    }

    /// IP observed by the API connection at sign-in; it may belong to a proxy or VPN.
    fn ip_address(&self) -> Option<String> {
        self.0.metadata.ip_address.map(|ip| ip.to_string())
    }

    /// User-Agent captured at sign-in, used only for session recognition.
    fn user_agent(&self) -> Option<&str> {
        self.0.metadata.user_agent.as_deref()
    }

    async fn passkey(
        &self,
        #[graphql(context)] user: &crate::http::authentication::UserContext,
        #[graphql(context)] identity: &crate::http::authentication::AuthenticationContext,
        #[graphql(context)] repository: &Box<
            dyn compallm_core::persistent::passkey::PasskeyRepository,
        >,
    ) -> Result<Option<super::super::passkey::PasskeySummary>, crate::graphql::error::Error> {
        let actor = user.require().await?.access();
        if !actor.session.has_same_identity(&self.0) {
            return Err(compallm_core::authentication::AuthError::InvalidSession.into());
        }
        Ok(identity
            .execute(repository.session_passkey(&actor.session))
            .await?
            .map(Into::into))
    }
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

    /// Current name of this login's organization, resolved only when selected.
    async fn organization_name(
        &self,
        #[graphql(context)] user: &crate::http::authentication::UserContext,
        #[graphql(context)] identity: &crate::http::authentication::AuthenticationContext,
        #[graphql(context)] repository: &Box<
            dyn compallm_core::persistent::organization::OrganizationRepository,
        >,
    ) -> Result<String, crate::graphql::error::Error> {
        if user.require().await?.access().session.user_id != self.0.user_id {
            return Err(compallm_core::authentication::AuthError::InvalidSession.into());
        }
        if let Some(name) = &self.1 {
            return Ok(name.clone());
        }
        let organization = identity
            .execute(repository.find_by_id(self.0.organization_id))
            .await
            .map_err(compallm_core::authentication::AuthError::from)?
            .ok_or_else(crate::graphql::error::Error::internal)?;
        Ok(organization.name)
    }
}
