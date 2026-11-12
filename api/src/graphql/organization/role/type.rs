pub struct Role {
    pub id: juniper::ID,
    pub name: String,
    pub description: String,
    pub color: String,
}

impl From<compallm_core::organization::Role> for Role {
    fn from(role: compallm_core::organization::Role) -> Self {
        let color = role.color();
        Self {
            id: juniper::ID::new(role.id.to_string()),
            name: role.name,
            description: role.description,
            color,
        }
    }
}

#[juniper::graphql_object(context = crate::graphql::GraphqlContext)]
impl Role {
    async fn cedar_source(
        &self,
        #[graphql(context)] repository: &Box<
            dyn compallm_core::persistent::organization::OrganizationRepository,
        >,
        #[graphql(context)] authority: &crate::http::authorization::AuthorizationContext,
    ) -> Result<Option<String>, crate::graphql::error::Error> {
        use compallm_core::authorization::{
            condition::{CheckPermission, ForAction},
            error::AuthorizationError,
            policy::Action,
        };
        let permission = authority.member_permission().await?;
        let id = self
            .id
            .parse::<u64>()
            .map_err(|_| crate::graphql::error::Error::invalid_input())?;
        let role = compallm_core::organization::Role {
            id,
            organization_id: permission.organization_id(),
            name: self.name.clone(),
            description: self.description.clone(),
            cedar_source: String::new(),
            org_owner: false,
        };
        role.for_action(Action::RolePolicyRead)
            .check_permission(&permission)
            .map_err(AuthorizationError::from)?;
        Ok(repository
            .find_role(permission.organization_id(), id)
            .await
            .map_err(AuthorizationError::from)?
            .map(|role| role.cedar_source))
    }
    fn id(&self) -> &juniper::ID {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn color(&self) -> &str {
        &self.color
    }
}
