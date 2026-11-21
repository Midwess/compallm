use super::{ModelTemplate, ProviderTemplate, input::*};
use crate::graphql::{
    base_type::{Collection, Payload},
    error::Error,
};
use compallm_core::persistent::db_context;
use compallm_postgres::catalog::repository::CatalogRepository;
use juniper::ID;

#[db_context::root]
pub async fn list(
    search: Option<String>,
    filter: Option<ProviderTemplateFilterInput>,
    pagination: compallm_query::Pagination,
    order_by: Option<Vec<ProviderTemplateOrderInput>>,
    repository: &CatalogRepository,
) -> Result<Collection<ProviderTemplate>, Error> {
    if search
        .as_ref()
        .is_some_and(|value| value.chars().count() > 256 || value.split_whitespace().count() > 8)
    {
        return Err(Error::invalid_input());
    }
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
    let scope = "provider-templates";
    let result = repository
        .list(
            Box::new(filter.unwrap_or_default()),
            pagination,
            ordering,
            scope,
            search.as_deref(),
        )
        .await
        .map_err(|error| match error {
            compallm_postgres::query::QueryCompileError::Database(_) => Error::internal(),
            _ => Error::invalid_input(),
        })?;
    Ok(result.into())
}
#[db_context::root]
pub async fn get(
    id: ID,
    repository: &CatalogRepository,
) -> Result<Option<Payload<ProviderTemplate>>, Error> {
    Ok(repository
        .provider(&id)
        .await
        .map_err(|_| Error::internal())?
        .map(|node| Payload::from(ProviderTemplate::from(node))))
}
#[db_context::root]
pub async fn model(
    provider_template_id: ID,
    id: ID,
    repository: &CatalogRepository,
) -> Result<Option<Payload<ModelTemplate>>, Error> {
    Ok(repository
        .model(&provider_template_id, &id)
        .await
        .map_err(|_| Error::internal())?
        .map(|node| Payload::from(ModelTemplate::from(node))))
}
