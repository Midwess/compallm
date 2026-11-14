use compallm_core::organization::Organization as CoreOrganization;

pub struct Organization {
    pub id: u64,
    pub name: String,
    pub avatar_path: Option<String>,
}

impl From<CoreOrganization> for Organization {
    fn from(organization: CoreOrganization) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
            avatar_path: organization.avatar_path,
        }
    }
}

#[juniper::graphql_object(context = crate::graphql::GraphqlContext)]
impl Organization {
    fn avatar_url(
        &self,
        #[graphql(context)] storage: &Option<std::sync::Arc<compallm_storage::S3Storage>>,
    ) -> Option<String> {
        crate::graphql::user::avatar_url(self.avatar_path.as_deref()?, storage)
    }
    async fn can_update_profile(
        &self,
        #[graphql(context)] authority: &crate::http::authorization::AuthorizationContext,
    ) -> Result<bool, crate::graphql::error::Error> {
        use compallm_core::authorization::{
            MemberPermissionError, condition::CheckPermission, policy::Action,
        };
        let permission = match authority.member_permission().await {
            Ok(permission) => permission,
            Err(crate::http::authorization::AuthorizationError::Authentication(
                crate::http::authentication::RequestError::Unauthenticated,
            )) => return Ok(false),
            Err(error) => return Err(error.into()),
        };
        if permission.organization_id() != self.id {
            return Ok(false);
        }
        match Action::OrganizationSettingsUpdate.check_permission(&permission) {
            Ok(()) => Ok(true),
            Err(MemberPermissionError::Denied) => Ok(false),
            Err(error) => {
                Err(compallm_core::organization::profile_service::ProfileError::from(error).into())
            }
        }
    }
    fn id(&self) -> juniper::ID {
        juniper::ID::new(self.id.to_string())
    }
    fn name(&self) -> &str {
        &self.name
    }
}
