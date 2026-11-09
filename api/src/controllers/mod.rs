pub mod auth;
mod provider;

use compallm_core::auth::AuthService;
use utoipa::openapi::OpenApi;
use utoipa_axum::router::OpenApiRouter;

#[derive(utoipa::OpenApi)]
struct ApiDoc;

pub fn public_router() -> OpenApiRouter<&'static AuthService> {
    auth::router()
}

pub fn protected_router() -> OpenApiRouter<&'static AuthService> {
    auth::protected_router()
}

pub fn api_router() -> OpenApiRouter<&'static AuthService> {
    use utoipa::openapi::security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme, SecurityScheme};
    let mut openapi = <ApiDoc as utoipa::OpenApi>::openapi();
    let components = openapi.components.get_or_insert_with(Default::default);
    components.add_security_scheme(
        "bearerAuth",
        SecurityScheme::Http({
            let mut bearer = Http::new(HttpAuthScheme::Bearer);
            bearer.description = Some("User access CWT. Requests check the scoped session version. Stale or expired bearer access returns 401; call POST /api/auth/refresh with a valid refresh CWT, then retry. No automatic bearer replacement or cookie fallback.".into());
            bearer
        }),
    );
    let mut refresh = Http::new(HttpAuthScheme::Bearer);
    refresh.description = Some("Refresh CWT sent explicitly as Authorization: Bearer to POST /api/auth/refresh. Access and OTP tokens are rejected; cookie-only credentials cannot renew through this endpoint.".into());
    components.add_security_scheme("refreshToken", SecurityScheme::Http(refresh));
    let mut otp = Http::new(HttpAuthScheme::Bearer);
    otp.description = Some("Encrypted email OTP challenge returned by the code request. Requires the email-OTP token variant; user access and refresh tokens are rejected. Supply it as Authorization: Bearer <challenge>.".into());
    components.add_security_scheme("emailOtp", SecurityScheme::Http(otp));
    components.add_security_scheme("emailOtpCookie", SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::with_description(
        "__Host-access", "HttpOnly OTP access cookie from the email code request. Nonsecure local development uses compallm-access. This token can confirm OTP but cannot authorize normal user APIs."
    ))));
    components.add_security_scheme("accessCookie", SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::with_description(
        "__Host-access", "HttpOnly access cookie; nonsecure local development uses compallm-access. Automatic renewal additionally requires the configured refresh cookie."
    ))));
    OpenApiRouter::with_openapi(openapi)
        .merge(public_router())
        .merge(protected_router())
}

pub fn openapi() -> OpenApi {
    let mut api = api_router().into_openapi();
    api.merge(<compallm_contract::provider::InputContracts as utoipa::OpenApi>::openapi());
    api.merge(<compallm_contract::provider::api::ApiContracts as utoipa::OpenApi>::openapi());
    api.merge(<provider::ProviderApi as utoipa::OpenApi>::openapi());
    api
}
