use super::{ModelTemplate, ProviderTemplate, input::*};
use crate::{
    graphql::{
        base_type::{Collection, Payload},
        error::Error,
    },
    http::authorization::AuthorizationContext,
};
use compallm_postgres::catalog::repository::CatalogRepository;
use juniper::ID;

pub async fn list(
    filter: Option<ProviderTemplateFilterInput>,
    pagination: compallm_query::Pagination,
    order_by: Option<Vec<ProviderTemplateOrderInput>>,
    repository: &CatalogRepository,
    authority: &AuthorizationContext,
) -> Result<Collection<ProviderTemplate>, Error> {
    let member = authority.member_permission().await?;
    let ordering = match order_by {
        Some(order) => order
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| Error::invalid_input())?,
        None => vec![compallm_query::OrderBy {
            field: "name".into(),
            direction: compallm_query::OrderByDirection::AscNullsLast,
        }],
    };
    let scope = format!(
        "provider-templates:{}:{}",
        member.organization_id(),
        member.user_id()
    );
    let result = repository
        .list(
            Box::new(filter.unwrap_or_default()),
            pagination,
            ordering,
            &scope,
        )
        .await
        .map_err(|error| match error {
            compallm_postgres::query::QueryCompileError::Database(_) => Error::internal(),
            _ => Error::invalid_input(),
        })?;
    Ok(result.into())
}
pub async fn get(
    id: ID,
    repository: &CatalogRepository,
    authority: &AuthorizationContext,
) -> Result<Option<Payload<ProviderTemplate>>, Error> {
    authority.member_permission().await?;
    Ok(repository
        .provider(&id)
        .await
        .map_err(|_| Error::internal())?
        .map(|node| Payload::from(ProviderTemplate::from(node))))
}
pub async fn model(
    provider_template_id: ID,
    id: ID,
    repository: &CatalogRepository,
    authority: &AuthorizationContext,
) -> Result<Option<Payload<ModelTemplate>>, Error> {
    authority.member_permission().await?;
    Ok(repository
        .model(&provider_template_id, &id)
        .await
        .map_err(|_| Error::internal())?
        .filter(|v| {
            ModelTemplate::compatible(
                &compallm_core::provider_template::ModelTemplateSummary::from(v),
            )
        })
        .map(|node| Payload::from(ModelTemplate::from(node))))
}
