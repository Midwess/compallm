use super::{ModelCatalog, input::*};
use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authorization::AuthorizationContext,
};
use compallm_core::{
    model::{CatalogCustomization, ModelCatalog as Domain},
    provider::*,
};
use compallm_postgres::catalog::repository::{CatalogRepository, CatalogScope};
use juniper::ID;
use validify::Validify;

impl TryFrom<ModelCatalogCustomizeInput> for CatalogCustomization {
    type Error = Error;
    fn try_from(v: ModelCatalogCustomizeInput) -> Result<Self, Error> {
        let price=|v:Option<ModelCatalogPriceInput>|->Result<Option<Option<rust_decimal::Decimal>>,Error>{
   v.map(|v|if v.enabled {v.value.ok_or_else(||Error::message("Enter a custom price.")).and_then(|v|v.parse().map(Some).map_err(|_|Error::message("Enter a valid custom price.")))}else{Ok(None)}).transpose()
  };
        Ok(Self {
            name: v.name,
            protocol: v.protocol.map(Into::into),
            path: v.path,
            limits: v.limits.map(|v| {
                (
                    v.context.map(|v| v.get()),
                    v.input.map(|v| v.get()),
                    v.output.map(|v| v.get()),
                )
            }),
            input_price: price(v.input_price)?,
            output_price: price(v.output_price)?,
            cached_price: price(v.cached_price)?,
            capabilities: v
                .capabilities
                .map(|v| v.into_iter().map(Into::into).collect()),
            effort_mapping: v.effort_mapping.map(|v| {
                [
                    (EffortLevel::Low, v.low),
                    (EffortLevel::Medium, v.medium),
                    (EffortLevel::High, v.high),
                    (EffortLevel::Xhigh, v.xhigh),
                    (EffortLevel::Max, v.max),
                ]
                .into_iter()
                .filter_map(|(k, v)| v.map(|v| (k, v)))
                .collect()
            }),
            headers: v
                .headers
                .map(|v| v.into_iter().map(|v| (v.key, v.value)).collect()),
            body_defaults: v
                .body_defaults
                .map(|v| {
                    v.into_iter()
                        .map(|v| {
                            Ok((
                                v.key,
                                serde_json::from_str(&v.value)
                                    .map_err(|_| Error::message("Enter a valid JSON value."))?,
                            ))
                        })
                        .collect::<Result<_, Error>>()
                })
                .transpose()?,
        })
    }
}
pub async fn customize(
    provider: ID,
    id: ID,
    mut input: ModelCatalogCustomizeInput,
    authority: &AuthorizationContext,
    repository: &CatalogRepository,
) -> Result<Payload<ModelCatalog>, Error> {
    input.validify()?;
    let changed = serde_json::to_value(&input)
        .map_err(|_| Error::invalid_input())?
        .as_object()
        .ok_or_else(Error::invalid_input)?
        .iter()
        .filter(|(_, v)| !v.is_null())
        .map(|(key, _)| {
            match key.as_str() {
                "inputPrice" => "input_price",
                "outputPrice" => "output_price",
                "cachedPrice" => "cached_price",
                "effortMapping" => "effort_mapping",
                "bodyDefaults" => "body_defaults",
                _ => key.as_str(),
            }
            .to_owned()
        })
        .collect::<Vec<_>>();
    let permission = authority.member_permission().await?;
    let scope = CatalogScope {
        organization_id: permission.organization_id(),
        user_id: permission.user_id(),
    };
    let personal = repository
        .personal_row(&provider, &id, scope)
        .await
        .map_err(|_| Error::internal())?;
    let global = repository
        .global_row(&provider, &id)
        .await
        .map_err(|_| Error::internal())?;
    let base = global.as_ref().map(|v| v.row_id);
    let default = global
        .clone()
        .map(Domain::try_from)
        .transpose()
        .map_err(|_| Error::internal())?;
    let row = personal
        .or(global)
        .ok_or_else(|| Error::message("This catalog model is unavailable."))?;
    let mut model = Domain::try_from(row).map_err(|_| Error::internal())?;
    CatalogCustomization::try_from(input)?.apply(&mut model, default.as_ref())?;
    let result = repository
        .save_personal(model, scope, base, &changed)
        .await
        .map_err(|_| Error::internal())?;
    Ok(ModelCatalog(result).into())
}
pub async fn create(
    provider: ID,
    mut input: ModelCatalogCreateInput,
    authority: &AuthorizationContext,
    repository: &CatalogRepository,
) -> Result<Payload<ModelCatalog>, Error> {
    input.validify()?;
    let input = input.configuration;
    let changed = serde_json::to_value(&input)
        .map_err(|_| Error::invalid_input())?
        .as_object()
        .ok_or_else(Error::invalid_input)?
        .iter()
        .filter(|(_, v)| !v.is_null())
        .map(|(key, _)| {
            match key.as_str() {
                "inputPrice" => "input_price",
                "outputPrice" => "output_price",
                "cachedPrice" => "cached_price",
                "effortMapping" => "effort_mapping",
                "bodyDefaults" => "body_defaults",
                _ => key.as_str(),
            }
            .to_owned()
        })
        .collect::<Vec<_>>();
    let permission = authority.member_permission().await?;
    let scope = CatalogScope {
        organization_id: permission.organization_id(),
        user_id: permission.user_id(),
    };
    let parent = repository
        .provider(&provider)
        .await
        .map_err(|_| Error::internal())?
        .ok_or_else(|| Error::message("This catalog provider is unavailable."))?;
    let protocol = input
        .protocol
        .ok_or_else(|| Error::message("Choose a text operation."))?;
    let protocol: Protocol = protocol.into();
    let mut connection = parent.connection;
    if !matches!(connection.authentication, Some(Authentication::ApiKey(_))) {
        connection.authentication = parent
            .authentication_options
            .into_iter()
            .find(|v| matches!(v, Authentication::ApiKey(_)));
    }
    let mut model = Domain {
        id: format!("custom:{}", uuid::Uuid::new_v4()),
        custom_fields: Default::default(),
        research_id: None,
        provider_catalog_id: provider.to_string(),
        name: String::new(),
        display_name: None,
        description: None,
        service_kind: Some(ServiceKind::Llm),
        connections: [(protocol, connection)].into(),
        headers: Default::default(),
        body_defaults: Default::default(),
        text: TextCapabilities {
            input: CapabilitySupport::supported(TextInput::default(), None),
            output: CapabilitySupport::supported(TextGeneration::default(), None),
        },
        vision: Default::default(),
        pdf: Default::default(),
        audio: Default::default(),
        video: Default::default(),
        image_generation: Default::default(),
        tools: Default::default(),
        structured_output: Default::default(),
        search: Default::default(),
        limits: Default::default(),
        billing: None,
        route_selection: None,
    };
    CatalogCustomization::try_from(input)?.apply(&mut model, None)?;
    if model.name.is_empty()
        || model
            .connections
            .get(&protocol)
            .and_then(|v| v.path.as_ref())
            .is_none()
    {
        return Err(Error::message(
            "Enter a model identifier and endpoint path.",
        ));
    }
    if !matches!(
        model
            .connections
            .get(&protocol)
            .and_then(|v| v.authentication.as_ref()),
        Some(Authentication::ApiKey(_))
    ) {
        return Err(Error::message(
            "Choose an API-key provider for this text model.",
        ));
    }
    Ok(ModelCatalog(
        repository
            .save_personal(model, scope, None, &changed)
            .await
            .map_err(|_| Error::internal())?,
    )
    .into())
}
pub async fn reset(
    provider: ID,
    id: ID,
    authority: &AuthorizationContext,
    repository: &CatalogRepository,
) -> Result<Payload<ModelCatalog>, Error> {
    let permission = authority.member_permission().await?;
    let scope = CatalogScope {
        organization_id: permission.organization_id(),
        user_id: permission.user_id(),
    };
    let global = repository
        .global_row(&provider, &id)
        .await
        .map_err(|_| Error::internal())?
        .ok_or_else(|| Error::message("Custom models must be deleted instead of reset."))?;
    repository
        .delete_personal(&provider, &id, scope, false)
        .await
        .map_err(|_| Error::internal())?;
    Ok(ModelCatalog(Domain::try_from(global).map_err(|_| Error::internal())?).into())
}
pub async fn delete(
    provider: ID,
    id: ID,
    authority: &AuthorizationContext,
    repository: &CatalogRepository,
) -> Result<Payload<bool>, Error> {
    let permission = authority.member_permission().await?;
    let scope = CatalogScope {
        organization_id: permission.organization_id(),
        user_id: permission.user_id(),
    };
    Ok(repository
        .delete_personal(&provider, &id, scope, true)
        .await
        .map_err(|_| Error::internal())?
        .into())
}
