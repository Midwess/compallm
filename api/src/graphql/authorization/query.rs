use super::{
    PermissionDecision,
    input::{PermissionCheckInput, PermissionResourceType},
};
use crate::{
    graphql::{base_type::Collection, error::Error},
    http::authorization::AuthorizationContext,
};
use compallm_core::{
    authorization::{
        MemberPermission, MemberPermissionError,
        condition::{CheckPermission, ForAction},
        policy::{Action, AttributeName, EntityType},
    },
    persistent::{
        model::ModelRepository, organization::OrganizationRepository, provider::ProviderRepository,
    },
};

fn decision(result: Result<(), MemberPermissionError>) -> Result<bool, Error> {
    match result {
        Ok(()) => Ok(true),
        Err(MemberPermissionError::Denied) => Ok(false),
        Err(_) => Err(Error::internal()),
    }
}
fn id(value: &Option<juniper::ID>) -> Result<Option<u64>, Error> {
    value
        .as_ref()
        .map(|value| {
            value
                .parse::<u64>()
                .ok()
                .filter(|id| *id > 0 && *id <= i64::MAX as u64)
                .ok_or_else(Error::invalid_input)
        })
        .transpose()
}

pub(crate) async fn decisions(
    mut checks: Vec<PermissionCheckInput>,
    rbac: &compallm_core::authorization::rbac::RbacService,
    authority: &AuthorizationContext,
    organizations: &dyn OrganizationRepository,
    providers: &dyn ProviderRepository,
    models: &dyn ModelRepository,
) -> Result<Collection<PermissionDecision>, Error> {
    if checks.len() > 100 {
        return Err(Error::invalid_input());
    }
    use validify::Validify;
    for check in &mut checks {
        check.validify()?;
    }
    let permission = authority.member_permission().await?;
    let mut nodes = Vec::with_capacity(checks.len());
    let mut cache = EvaluationCache::default();
    for (index, check) in checks.iter().enumerate() {
        nodes.push(PermissionDecision {
            index: index as i32,
            allowed: evaluate_cached(
                check,
                &permission,
                rbac,
                organizations,
                providers,
                models,
                &mut cache,
            )
            .await?,
        });
    }
    Collection::complete(
        nodes,
        &format!(
            "permissions:{}:{}:{checks:?}",
            permission.organization_id(),
            permission.user_id()
        ),
        |node| vec![Some(node.index.to_string())],
    )
    .map_err(|_| Error::internal())
}

#[derive(Default)]
struct EvaluationCache {
    roles: std::collections::HashMap<u64, Option<compallm_core::organization::Role>>,
    providers: std::collections::HashMap<u64, Option<compallm_core::provider::Provider>>,
    provider_models: std::collections::HashMap<u64, Option<compallm_core::provider::ProviderModel>>,
    models: std::collections::HashMap<u64, Option<compallm_core::model::Model>>,
}
impl EvaluationCache {
    async fn role(
        &mut self,
        repository: &dyn OrganizationRepository,
        org: u64,
        id: u64,
    ) -> Result<Option<compallm_core::organization::Role>, Error> {
        if let Some(value) = self.roles.get(&id) {
            return Ok(value.clone());
        }
        let value = repository
            .find_roles_for_assignment(org, &[id])
            .await
            .map_err(|error| {
                Error::from(compallm_core::authorization::error::AuthorizationError::from(error))
            })?
            .pop();
        self.roles.insert(id, value.clone());
        Ok(value)
    }
    async fn provider(
        &mut self,
        repository: &dyn ProviderRepository,
        org: u64,
        id: u64,
    ) -> Result<Option<compallm_core::provider::Provider>, Error> {
        if let Some(value) = self.providers.get(&id) {
            return Ok(value.clone());
        }
        let value = repository
            .find_by_id(org, id)
            .await
            .map_err(|_| Error::internal())?;
        self.providers.insert(id, value.clone());
        Ok(value)
    }
    async fn provider_model(
        &mut self,
        repository: &dyn ProviderRepository,
        org: u64,
        id: u64,
    ) -> Result<Option<compallm_core::provider::ProviderModel>, Error> {
        if let Some(value) = self.provider_models.get(&id) {
            return Ok(value.clone());
        }
        let value = repository
            .find_model_by_id(org, id)
            .await
            .map_err(|_| Error::internal())?;
        self.provider_models.insert(id, value.clone());
        Ok(value)
    }
    async fn model(
        &mut self,
        repository: &dyn ModelRepository,
        org: u64,
        id: u64,
    ) -> Result<Option<compallm_core::model::Model>, Error> {
        if let Some(value) = self.models.get(&id) {
            return Ok(value.clone());
        }
        let value = repository
            .find_by_id(org, id)
            .await
            .map_err(|_| Error::internal())?;
        self.models.insert(id, value.clone());
        Ok(value)
    }
}
async fn evaluate_cached(
    check: &PermissionCheckInput,
    permission: &MemberPermission,
    rbac: &compallm_core::authorization::rbac::RbacService,
    organizations: &dyn OrganizationRepository,
    providers: &dyn ProviderRepository,
    models: &dyn ModelRepository,
    cache: &mut EvaluationCache,
) -> Result<bool, Error> {
    let action = Action::from(check.action);
    let kind = EntityType::from(check.resource_type);
    let target = id(&check.resource_id)?;
    let role_id = id(&check.role_id)?;
    let member_id = id(&check.member_id)?;
    let provider_model_id = id(&check.provider_model_id)?;
    let context_count = usize::from(role_id.is_some())
        + usize::from(member_id.is_some())
        + usize::from(provider_model_id.is_some());
    let correct_context = match action.context().map(|attribute| attribute.name) {
        None => context_count == 0,
        Some(AttributeName::Role) => role_id.is_some() && context_count == 1,
        Some(AttributeName::Member) => member_id.is_some() && context_count == 1,
        Some(AttributeName::ProviderModel) => provider_model_id.is_some() && context_count == 1,
        _ => false,
    };
    if action.resource() != kind || !correct_context {
        return Err(Error::invalid_input());
    }
    let org = permission.organization_id();
    if kind == EntityType::Organization {
        if target.is_some_and(|id| id != org) {
            return Ok(false);
        }
        return decision(action.check_permission(permission));
    }
    let target = target.ok_or_else(Error::invalid_input)?;
    let read = match check.resource_type {
        PermissionResourceType::Role => Action::RoleRead,
        PermissionResourceType::Group => Action::GroupRead,
        PermissionResourceType::Member => Action::MemberRead,
        PermissionResourceType::Provider => Action::ProviderRead,
        PermissionResourceType::ProviderModel => Action::ProviderModelRead,
        PermissionResourceType::Model => Action::ModelRead,
        PermissionResourceType::Organization => unreachable!(),
    };
    if !decision(read.check_permission(permission))? {
        return Ok(false);
    }
    let provider_model = if let Some(id) = provider_model_id {
        if !decision(Action::ProviderModelRead.check_permission(permission))? {
            return Ok(false);
        }
        let Some(model) = cache.provider_model(providers, org, id).await? else {
            return Ok(false);
        };
        Some(model)
    } else {
        None
    };
    match check.resource_type {
        PermissionResourceType::Role => {
            let Some(role) = cache.role(organizations, org, target).await? else {
                return Ok(false);
            };
            decision((&role).for_action(action).check_permission(permission))
        }
        PermissionResourceType::Group => {
            match rbac
                .check_group_permission(permission, target, action, role_id, member_id)
                .await
            {
                Ok(()) => Ok(true),
                Err(compallm_core::authorization::error::AuthorizationError::Permission(
                    MemberPermissionError::Denied,
                ))
                | Err(compallm_core::authorization::error::AuthorizationError::Persistence(
                    compallm_core::persistent::organization::OrganizationRepositoryError::NotFound,
                )) => Ok(false),
                Err(error) => Err(error.into()),
            }
        }
        PermissionResourceType::Member => {
            match rbac
                .check_member_permission(permission, target, action, role_id)
                .await
            {
                Ok(()) => Ok(true),
                Err(compallm_core::authorization::error::AuthorizationError::Permission(
                    MemberPermissionError::Denied,
                ))
                | Err(compallm_core::authorization::error::AuthorizationError::Persistence(
                    compallm_core::persistent::organization::OrganizationRepositoryError::NotFound,
                )) => Ok(false),
                Err(error) => Err(error.into()),
            }
        }
        PermissionResourceType::Provider => {
            let Some(provider) = cache.provider(providers, org, target).await? else {
                return Ok(false);
            };
            decision((&provider).for_action(action).check_permission(permission))
        }
        PermissionResourceType::ProviderModel => {
            let Some(model) = cache.provider_model(providers, org, target).await? else {
                return Ok(false);
            };
            decision((&model).for_action(action).check_permission(permission))
        }
        PermissionResourceType::Model => {
            let Some(model) = cache.model(models, org, target).await? else {
                return Ok(false);
            };
            if let Some(provider_model) = provider_model {
                decision(
                    (&model, &provider_model)
                        .for_action(action)
                        .check_permission(permission),
                )
            } else {
                decision((&model).for_action(action).check_permission(permission))
            }
        }
        PermissionResourceType::Organization => unreachable!(),
    }
}
