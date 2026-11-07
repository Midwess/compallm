//! OpenAPI declarations only. These functions are never mounted as Axum routes.
use crate::error::ErrorResponse;
use compallm_contract::provider::api::*;

/// List provider registry.
///
/// Schema only; this operation has no runtime handler. Requires an owner or
/// administrator in the verified session's organization. Never returns API keys
/// or custom header values. Cookie writes require the trusted same-origin policy.
#[utoipa::path(
    get, path = "/api/provider-registry", operation_id = "listProviderRegistry",
    tag = "Provider registry",
    responses(
        (status = 200, description = "Planned success response", body = ProviderRegistryListResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Organization role or origin denied", body = ErrorResponse),
        (status = 500, description = "Internal failure", body = ErrorResponse),
    ),
    security(("bearerAuth" = []), ("accessCookie" = []))
)]
#[allow(dead_code)]
fn list_registry() {}

/// Get provider registry detail.
///
/// Schema only; this operation has no runtime handler. Requires an owner or
/// administrator in the verified session's organization. Never returns API keys
/// or custom header values. Cookie writes require the trusted same-origin policy.
#[utoipa::path(
    get, path = "/api/provider-registry/{registryId}", operation_id = "getProviderRegistry",
    tag = "Provider registry",
    params(("registryId" = String, Path, description = "Opaque registry provider ID")),
    responses(
        (status = 200, description = "Planned success response", body = ProviderRegistryDetailResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Organization role or origin denied", body = ErrorResponse),
        (status = 404, description = "Resource not found", body = ErrorResponse),
        (status = 500, description = "Internal failure", body = ErrorResponse),
    ),
    security(("bearerAuth" = []), ("accessCookie" = []))
)]
#[allow(dead_code)]
fn registry_detail() {}

/// List saved providers.
///
/// Schema only; this operation has no runtime handler. Requires an owner or
/// administrator in the verified session's organization. Never returns API keys
/// or custom header values. Cookie writes require the trusted same-origin policy.
#[utoipa::path(
    get, path = "/api/providers", operation_id = "listProviders",
    tag = "Providers",
    responses(
        (status = 200, description = "Planned success response", body = ProviderListResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Organization role or origin denied", body = ErrorResponse),
        (status = 500, description = "Internal failure", body = ErrorResponse),
    ),
    security(("bearerAuth" = []), ("accessCookie" = []))
)]
#[allow(dead_code)]
fn list_providers() {}

/// Get saved provider detail.
///
/// Schema only; this operation has no runtime handler. Requires an owner or
/// administrator in the verified session's organization. Never returns API keys
/// or custom header values. Cookie writes require the trusted same-origin policy.
#[utoipa::path(
    get, path = "/api/providers/{providerId}", operation_id = "getProvider",
    tag = "Providers",
    params(("providerId" = ProviderResourceId, Path, description = "Positive decimal-string provider ID in the current organization")),
    responses(
        (status = 200, description = "Planned success response", body = ProviderDetailResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Organization role or origin denied", body = ErrorResponse),
        (status = 404, description = "Resource not found", body = ErrorResponse),
        (status = 500, description = "Internal failure", body = ErrorResponse),
    ),
    security(("bearerAuth" = []), ("accessCookie" = []))
)]
#[allow(dead_code)]
fn provider_detail() {}

/// Create providers atomically.
///
/// Schema only; this operation has no runtime handler. Requires an owner or
/// administrator in the verified session's organization. Never returns API keys
/// or custom header values. Cookie writes require the trusted same-origin policy.
/// Requires a nonempty providers array. The entire batch succeeds or fails; no upstream calls are made.
#[utoipa::path(
    post, path = "/api/providers", operation_id = "createProviders",
    tag = "Providers",
    request_body = CreateProvidersRequest,
    responses(
        (status = 201, description = "Planned success response", body = CreateProvidersResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Organization role or origin denied", body = ErrorResponse),
        (status = 409, description = "Provider name conflict", body = ErrorResponse),
        (status = 500, description = "Internal failure", body = ErrorResponse),
    ),
    security(("bearerAuth" = []), ("accessCookie" = []))
)]
#[allow(dead_code)]
fn create_providers() {}

/// Replace provider configuration.
///
/// Schema only; this operation has no runtime handler. Requires an owner or
/// administrator in the verified session's organization. Never returns API keys
/// or custom header values. Cookie writes require the trusted same-origin policy.
/// Replaces name, base URL, headers, and models. Omitted apiKey preserves the stored credential.
#[utoipa::path(
    put, path = "/api/providers/{providerId}", operation_id = "replaceProvider",
    tag = "Providers",
    params(("providerId" = ProviderResourceId, Path, description = "Positive decimal-string provider ID in the current organization")),
    request_body = ReplaceProviderRequest,
    responses(
        (status = 200, description = "Planned success response", body = ProviderDetailResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Organization role or origin denied", body = ErrorResponse),
        (status = 404, description = "Resource not found", body = ErrorResponse),
        (status = 409, description = "Provider name conflict", body = ErrorResponse),
        (status = 500, description = "Internal failure", body = ErrorResponse),
    ),
    security(("bearerAuth" = []), ("accessCookie" = []))
)]
#[allow(dead_code)]
fn replace_provider() {}

/// Partially update provider configuration.
///
/// Schema only; this operation has no runtime handler. Requires an owner or
/// administrator in the verified session's organization. Never returns API keys
/// or custom header values. Cookie writes require the trusted same-origin policy.
/// Omitted fields preserve existing values. Supplied collections replace the collection. Null is invalid.
#[utoipa::path(
    patch, path = "/api/providers/{providerId}", operation_id = "patchProvider",
    tag = "Providers",
    params(("providerId" = ProviderResourceId, Path, description = "Positive decimal-string provider ID in the current organization")),
    request_body = PatchProviderRequest,
    responses(
        (status = 200, description = "Planned success response", body = ProviderDetailResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Organization role or origin denied", body = ErrorResponse),
        (status = 404, description = "Resource not found", body = ErrorResponse),
        (status = 409, description = "Provider name conflict", body = ErrorResponse),
        (status = 500, description = "Internal failure", body = ErrorResponse),
    ),
    security(("bearerAuth" = []), ("accessCookie" = []))
)]
#[allow(dead_code)]
fn patch_provider() {}

/// Delete provider.
///
/// Schema only; this operation has no runtime handler. Requires an owner or
/// administrator in the verified session's organization. Never returns API keys
/// or custom header values. Cookie writes require the trusted same-origin policy.
/// Removes the connection, provider models, and their bindings; logical models and usage history remain.
#[utoipa::path(
    delete, path = "/api/providers/{providerId}", operation_id = "deleteProvider",
    tag = "Providers",
    params(("providerId" = ProviderResourceId, Path, description = "Positive decimal-string provider ID in the current organization")),
    responses(
        (status = 204, description = "No content"),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Organization role or origin denied", body = ErrorResponse),
        (status = 404, description = "Resource not found", body = ErrorResponse),
        (status = 500, description = "Internal failure", body = ErrorResponse),
    ),
    security(("bearerAuth" = []), ("accessCookie" = []))
)]
#[allow(dead_code)]
fn delete_provider() {}

#[derive(utoipa::OpenApi)]
#[openapi(paths(
    list_registry,
    registry_detail,
    list_providers,
    provider_detail,
    create_providers,
    replace_provider,
    patch_provider,
    delete_provider
))]
pub struct ProviderApi;
