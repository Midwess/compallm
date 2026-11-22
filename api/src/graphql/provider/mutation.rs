use super::{Provider, input::ProviderCreateFromTemplateInput};
use crate::{
    graphql::{base_type::Payload, error::Error},
    http::authorization::AuthorizationContext,
};
use compallm_core::{
    authorization::{condition::CheckPermission, policy::Action},
    provider::*,
};
use compallm_postgres::catalog::repository::{CatalogRepository, CatalogScope};
use validify::{Validate, Validify};

pub async fn create(
    mut input: ProviderCreateFromTemplateInput,
    authority: &AuthorizationContext,
    catalog: &CatalogRepository,
    service: &CreationService<'_>,
    cipher: &Option<ProviderCredentialCipher>,
    storage: &Option<std::sync::Arc<compallm_storage::S3Storage>>,
) -> Result<Payload<Provider>, Error> {
    input.validify()?;
    let permission = authority.member_permission().await?;
    for action in [Action::ProviderCreate, Action::ProviderRead] {
        action
            .check_permission(&permission)
            .map_err(compallm_core::authorization::error::AuthorizationError::from)?;
    }
    let template = catalog
        .provider(&input.provider_template_id)
        .await
        .map_err(|_| Error::internal())?
        .ok_or_else(|| Error::message("This provider template is unavailable."))?;
    let mut connection = template.connection.clone();
    if !matches!(connection.authentication, Some(Authentication::ApiKey(_))) {
        connection.authentication = template
            .authentication_options
            .iter()
            .find(|v| matches!(v, Authentication::ApiKey(_)))
            .cloned();
    }
    if !matches!(connection.authentication, Some(Authentication::ApiKey(_))) {
        return Err(Error::message(
            "This provider does not support API-key creation yet.",
        ));
    }
    connection.base_url = Some(input.base_url);
    connection
        .timeout_ms
        .get_or_insert(DEFAULT_REQUEST_TIMEOUT_MS);
    if connection.endpoint.required_inputs.is_empty() {
        connection.endpoint.mode = EndpointMode::Fixed;
    }
    connection.validate()?;
    let scope = CatalogScope {
        organization_id: permission.organization_id(),
        user_id: permission.user_id(),
    };
    let mut models = Vec::with_capacity(input.models.len());
    for selected in input.models {
        let template = catalog
            .model(&input.provider_template_id, &selected.id, Some(scope))
            .await
            .map_err(|_| Error::internal())?
            .ok_or_else(|| {
                Error::message("An enabled model is unavailable. Reload its saved configuration.")
            })?;
        let mut model = ProviderModel::try_from((&template, selected.protocol.into()))?;
        model.connection.base_url = connection.base_url.clone();
        model
            .connection
            .timeout_ms
            .get_or_insert(DEFAULT_REQUEST_TIMEOUT_MS);
        if model.connection.authentication.is_none() {
            model.connection.authentication = connection.authentication.clone();
        }
        model.connection.endpoint = connection.endpoint.clone();
        model.organization_id = permission.organization_id();
        model.validate()?;
        models.push(model);
    }
    let logo_url = if let Some(id) = input.logo_upload_id {
        let storage = storage
            .as_ref()
            .ok_or_else(|| Error::message("Image uploads are unavailable."))?;
        Some(
            storage
                .provider_logo_uploaded(
                    permission.organization_id(),
                    permission.user_id(),
                    uuid::Uuid::parse_str(&id).map_err(|_| Error::invalid_input())?,
                )
                .await
                .map_err(|_| Error::message("Upload the logo before creating the provider."))?,
        )
    } else {
        match input.logo_url {
            Some(value) if template.logo_url.as_ref() == Some(&value) => Some(value),
            Some(_) => {
                return Err(Error::message(
                    "Choose the provider logo or a verified uploaded image.",
                ));
            }
            None => template.logo_url,
        }
    };
    let key = ProviderApiKey::new(
        &input.api_key,
        cipher.as_ref().ok_or_else(|| {
            Error::message(
                "Provider creation is unavailable until credential encryption is configured.",
            )
        })?,
    )
    .map_err(|_| Error::message("The API key could not be protected."))?;
    let provider = compallm_core::provider::Provider {
        id: 0,
        organization_id: permission.organization_id(),
        name: input.name,
        description: template.description,
        logo_url,
        template_id: Some(input.provider_template_id),
        connection,
        headers: input
            .headers
            .into_iter()
            .map(|v| (v.key, v.value))
            .collect(),
        api_key: key,
        models,
    };
    let created = service
        .create(&permission, provider)
        .await
        .map_err(|error| match error {
            ProviderCreationError::NameAlreadyExists => {
                Error::message("A provider with this name already exists.")
            }
            ProviderCreationError::InvalidInput(v) => v.into(),
            ProviderCreationError::Permission(v) => {
                compallm_core::authorization::error::AuthorizationError::from(v).into()
            }
            _ => Error::internal(),
        })?;
    Ok(Provider::from(created).into())
}

pub async fn logo(
    mut input: crate::graphql::user::input::ImageUploadInput,
    authority: &AuthorizationContext,
    storage: &Option<std::sync::Arc<compallm_storage::S3Storage>>,
) -> Result<Payload<ProviderLogoUpload>, Error> {
    input.validify()?;
    let p = authority.member_permission().await?;
    Action::ProviderCreate
        .check_permission(&p)
        .map_err(compallm_core::authorization::error::AuthorizationError::from)?;
    let constraints = compallm_core::persistent::storage::AvatarUploadConstraints::image(
        input.media_type.into(),
        input.size as u64,
    )
    .map_err(|_| Error::invalid_input())?;
    let id = uuid::Uuid::new_v4();
    let upload = storage
        .as_ref()
        .ok_or_else(|| Error::message("Image uploads are unavailable."))?
        .generate_provider_logo(p.organization_id(), p.user_id(), id, &constraints)
        .await
        .map_err(|_| Error::message("The logo upload could not be prepared."))?;
    Ok(ProviderLogoUpload {
        id: id.to_string(),
        upload: upload.into(),
    }
    .into())
}
#[derive(juniper::GraphQLObject)]
#[graphql(context=crate::graphql::GraphqlContext)]
pub struct ProviderLogoUpload {
    pub id: String,
    pub upload: crate::graphql::user::AvatarUpload,
}
