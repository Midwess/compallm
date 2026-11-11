use super::{Role, input::RoleCreateInput};
use crate::{graphql::error::Error, http::authorization::AuthorizationContext};
use compallm_core::authorization::{condition::CheckPermission, policy::Action, rbac::RbacService};
use validify::Validify;

pub struct Mutation;

#[juniper::graphql_object(name = "RoleMutation", context = crate::graphql::GraphqlContext)]
impl Mutation {
    pub(crate) async fn create(
        mut input: RoleCreateInput,
        #[graphql(context)] service: &RbacService,
        #[graphql(context)] authority: &AuthorizationContext,
    ) -> Result<Role, Error> {
        input.validify()?;
        let permission = authority.member_permission().await?;
        Action::RoleRead
            .check_permission(&permission)
            .map_err(compallm_core::authorization::error::AuthorizationError::from)?;
        let role = service
            .new_role(
                &permission,
                &input.name,
                &input.description,
                input.cedar_source,
            )
            .await?;
        Ok(role.into())
    }
}
