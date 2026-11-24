use super::{ModelCatalog, ProviderCatalog, input::*};
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
    filter: Option<ProviderCatalogFilterInput>,
    pagination: compallm_query::Pagination,
    order_by: Option<Vec<ProviderCatalogOrderInput>>,
    repository: &CatalogRepository,
) -> Result<Collection<ProviderCatalog>, Error> {
    if compallm_core::provider::catalog::CatalogSearch::try_from(
        search.as_deref().unwrap_or_default(),
    )
    .is_err()
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
            None,
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
) -> Result<Option<Payload<ProviderCatalog>>, Error> {
    Ok(repository
        .provider(&id)
        .await
        .map_err(|_| Error::internal())?
        .map(|node| Payload::from(ProviderCatalog::from(node))))
}
#[db_context::root]
pub async fn model(
    provider_catalog_id: ID,
    id: ID,
    repository: &CatalogRepository,
) -> Result<Option<Payload<ModelCatalog>>, Error> {
    Ok(repository
        .model(&provider_catalog_id, &id, None)
        .await
        .map_err(|_| Error::internal())?
        .map(|node| Payload::from(ModelCatalog::from(node))))
}

use crate::http::authorization::AuthorizationContext;
use compallm_postgres::catalog::repository::CatalogScope;
async fn owner(authority: &AuthorizationContext) -> Result<CatalogScope, Error> {
    let p = authority.member_permission().await?;
    Ok(CatalogScope {
        organization_id: p.organization_id(),
        user_id: p.user_id(),
    })
}
pub async fn effective_get(
    id: ID,
    authority: &AuthorizationContext,
    repository: &CatalogRepository,
) -> Result<Option<Payload<ProviderCatalog>>, Error> {
    let scope = owner(authority).await?;
    Ok(repository
        .provider(&id)
        .await
        .map_err(|_| Error::internal())?
        .map(|v| ProviderCatalog(v, Some(scope)).into()))
}
pub async fn effective_model(
    provider: ID,
    id: ID,
    authority: &AuthorizationContext,
    repository: &CatalogRepository,
) -> Result<Option<Payload<ModelCatalog>>, Error> {
    let scope = owner(authority).await?;
    Ok(repository
        .model(&provider, &id, Some(scope))
        .await
        .map_err(|_| Error::internal())?
        .map(|v| ModelCatalog(v).into()))
}
pub async fn effective_list(
    search: Option<String>,
    first: Option<i32>,
    after: Option<String>,
    authority: &AuthorizationContext,
    repository: &CatalogRepository,
) -> Result<Collection<ProviderCatalog>, Error> {
    let scope = owner(authority).await?;
    if compallm_core::provider::catalog::CatalogSearch::try_from(
        search.as_deref().unwrap_or_default(),
    )
    .is_err()
    {
        return Err(Error::invalid_input());
    }
    let result = repository
        .list(
            Box::new(ProviderCatalogFilterInput::default()),
            compallm_query::Pagination {
                first: Some(first.unwrap_or(20)),
                after: after.map(compallm_query::Cursor),
                ..Default::default()
            },
            vec![compallm_query::OrderBy {
                field: "name".into(),
                direction: compallm_query::OrderByDirection::AscNullsLast,
            }],
            &format!(
                "effective-templates:{}:{}",
                scope.organization_id, scope.user_id
            ),
            search.as_deref(),
            Some(scope),
        )
        .await
        .map_err(|error| match error {
            compallm_postgres::query::QueryCompileError::Database(_) => Error::internal(),
            _ => Error::invalid_input(),
        })?;
    let mut output: Collection<ProviderCatalog> = result.into();
    for edge in &mut output.edges {
        edge.node.1 = Some(scope);
    }
    Ok(output)
}
