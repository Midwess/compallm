use crate::{
    graphql::{GraphqlContext, base_type::Collection, error::Error},
    http::authorization::AuthorizationContext,
};
use compallm_core::{
    authorization::{condition::CheckPermission, error::AuthorizationError, policy::Action},
    persistent::organization::OrganizationRepository,
};
use juniper::ID;

pub struct Group {
    pub id: ID,
    pub name: String,
    pub description: String,
}
impl From<compallm_core::organization::group::GroupInfo> for Group {
    fn from(group: compallm_core::organization::group::GroupInfo) -> Self {
        Self {
            id: ID::new(group.id.to_string()),
            name: group.name,
            description: group.description,
        }
    }
}
impl From<compallm_core::organization::Group> for Group {
    fn from(group: compallm_core::organization::Group) -> Self {
        Self {
            id: ID::new(group.id.to_string()),
            name: group.name,
            description: group.description,
        }
    }
}
#[juniper::graphql_object(context = GraphqlContext, scalar = juniper::DefaultScalarValue)]
impl Group {
    fn id(&self) -> &ID {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    async fn roles(
        &self,
        #[graphql(context)] repository: &Box<dyn OrganizationRepository>,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Collection<super::super::role::Role>, Error> {
        let permission = authority.member_permission().await?;
        Action::RoleRead
            .check_permission(&permission)
            .map_err(AuthorizationError::from)?;
        Action::GroupRead
            .check_permission(&permission)
            .map_err(AuthorizationError::from)?;
        let group = repository
            .find_group(
                permission.organization_id(),
                self.id.parse().map_err(|_| Error::invalid_input())?,
            )
            .await
            .map_err(AuthorizationError::from)?
            .ok_or(compallm_core::persistent::organization::OrganizationRepositoryError::NotFound)
            .map_err(AuthorizationError::from)?;
        let nodes = group.roles.into_iter().map(Into::into).collect();
        Collection::complete(
            nodes,
            &format!(
                "group-roles:{}:{}:{}",
                permission.organization_id(),
                permission.user_id(),
                self.id
            ),
            |role: &super::super::role::Role| vec![Some(role.id.to_string())],
        )
        .map_err(|_| Error::internal())
    }
    async fn member_count(
        &self,
        #[graphql(context)] repository: &Box<dyn OrganizationRepository>,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<String, Error> {
        let permission = authority.member_permission().await?;
        Action::MemberRead
            .check_permission(&permission)
            .map_err(AuthorizationError::from)?;
        Action::GroupRead
            .check_permission(&permission)
            .map_err(AuthorizationError::from)?;
        Ok(repository
            .group_member_count(
                permission.organization_id(),
                self.id.parse().map_err(|_| Error::invalid_input())?,
            )
            .await
            .map_err(AuthorizationError::from)?
            .to_string())
    }
    async fn member_ids(
        &self,
        #[graphql(context)] repository: &Box<dyn OrganizationRepository>,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Vec<ID>, Error> {
        let permission = authority.member_permission().await?;
        Action::MemberRead
            .check_permission(&permission)
            .map_err(AuthorizationError::from)?;
        Action::GroupRead
            .check_permission(&permission)
            .map_err(AuthorizationError::from)?;
        Ok(repository
            .group_member_ids(
                permission.organization_id(),
                self.id.parse().map_err(|_| Error::invalid_input())?,
            )
            .await
            .map_err(AuthorizationError::from)?
            .into_iter()
            .map(|id| ID::new(id.to_string()))
            .collect())
    }
}
