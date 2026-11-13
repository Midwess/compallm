use compallm_core::user::User as CoreUser;

/// Public identity fields backed directly by the existing Core entity.
///
/// Email requires verified ownership; account status and authentication metadata stay private.
pub struct User(pub(super) CoreUser);

impl From<CoreUser> for User {
    fn from(user: CoreUser) -> Self {
        Self(user)
    }
}

use juniper::{ID, graphql_object};

#[graphql_object(context = super::super::GraphqlContext)]
impl User {
    /// Public avatar URL; the object may not exist yet and clients must show a fallback.
    fn avatar_url(
        &self,
        #[graphql(context)] storage: &Option<std::sync::Arc<compallm_storage::S3Storage>>,
    ) -> Option<String> {
        super::avatar_url(self.0.avatar_path.as_deref()?, storage)
    }

    /// Stable user identity, serialized as a string.
    fn id(&self) -> ID {
        ID::new(self.0.id.to_string())
    }

    fn username(&self) -> &str {
        &self.0.username
    }

    /// Email is readable only by the verified account owner and cannot be updated here.
    async fn email(
        &self,
        #[graphql(context)] user: &crate::http::authentication::UserContext,
    ) -> Result<&str, crate::graphql::error::Error> {
        let actor = user.require().await?;
        if actor.access().session.user_id != self.0.id {
            return Err(compallm_core::user::ProfileError::Denied.into());
        }
        Ok(&self.0.email)
    }

    /// Display name; this field does not establish identity or uniqueness.
    fn display_name(&self) -> &str {
        &self.0.display_name
    }
}

pub struct AvatarUpload(compallm_core::persistent::storage::user_space::AvatarUpload);
impl From<compallm_core::persistent::storage::user_space::AvatarUpload> for AvatarUpload {
    fn from(value: compallm_core::persistent::storage::user_space::AvatarUpload) -> Self {
        Self(value)
    }
}

#[derive(juniper::GraphQLObject)]
pub struct UploadFormField {
    pub name: String,
    pub value: String,
}

#[graphql_object(context = crate::graphql::GraphqlContext)]
impl AvatarUpload {
    fn url(&self) -> &str {
        &self.0.url
    }
    fn fields(&self) -> Vec<UploadFormField> {
        self.0
            .fields
            .iter()
            .map(|(name, value)| UploadFormField {
                name: name.clone(),
                value: value.clone(),
            })
            .collect()
    }
    /// Expiration as Unix seconds, encoded as a string.
    fn expires_at(&self) -> String {
        self.0
            .expires_at
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string()
    }
}
