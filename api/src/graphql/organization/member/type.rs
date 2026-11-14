pub struct Member(pub compallm_core::organization::group::MemberInfo);
impl From<compallm_core::organization::group::MemberInfo> for Member {
    fn from(value: compallm_core::organization::group::MemberInfo) -> Self {
        Self(value)
    }
}
#[juniper::graphql_object(context = crate::graphql::GraphqlContext)]
impl Member {
    /// Direct and group-inherited role identities in the verified organization.
    async fn effective_role_ids(
        &self,
        #[graphql(context)] authority: &crate::http::authorization::AuthorizationContext,
    ) -> Result<Vec<juniper::ID>, crate::graphql::error::Error> {
        use compallm_core::authorization::{
            condition::CheckPermission, error::AuthorizationError, policy::Action,
        };
        let permission = authority.member_permission().await?;
        Action::MemberRead
            .check_permission(&permission)
            .map_err(AuthorizationError::from)?;
        Action::GroupRead
            .check_permission(&permission)
            .map_err(AuthorizationError::from)?;
        Ok(self
            .0
            .effective_role_ids
            .iter()
            .map(|id| juniper::ID::new(id.to_string()))
            .collect())
    }
    /// Group identities in the verified organization; no policy source is exposed.
    async fn group_ids(
        &self,
        #[graphql(context)] authority: &crate::http::authorization::AuthorizationContext,
    ) -> Result<Vec<juniper::ID>, crate::graphql::error::Error> {
        use compallm_core::authorization::{
            condition::CheckPermission, error::AuthorizationError, policy::Action,
        };
        let permission = authority.member_permission().await?;
        Action::MemberRead
            .check_permission(&permission)
            .map_err(AuthorizationError::from)?;
        Action::GroupRead
            .check_permission(&permission)
            .map_err(AuthorizationError::from)?;
        Ok(self
            .0
            .group_ids
            .iter()
            .map(|id| juniper::ID::new(id.to_string()))
            .collect())
    }
    fn id(&self) -> juniper::ID {
        juniper::ID::new(self.0.id.to_string())
    }
    fn display_name(&self) -> &str {
        &self.0.display_name
    }
    fn email(&self) -> &str {
        &self.0.email
    }
    fn available(&self) -> bool {
        self.0.available
    }
}
