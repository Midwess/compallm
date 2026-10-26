pub mod auth;

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
        SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
    );
    let mut otp = Http::new(HttpAuthScheme::Bearer);
    otp.description = Some("Encrypted email OTP challenge returned by the code request. Requires the email-OTP token variant; user access and refresh tokens are rejected. Supply it as Authorization: Bearer <challenge>.".into());
    components.add_security_scheme("emailOtp", SecurityScheme::Http(otp));
    components.add_security_scheme("accessCookie", SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::with_description(
        "__Host-access", "HttpOnly access cookie; nonsecure local development uses compallm-access. Automatic renewal additionally requires the configured refresh cookie."
    ))));
    OpenApiRouter::with_openapi(openapi)
        .merge(public_router())
        .merge(protected_router())
}

pub fn openapi() -> OpenApi {
    api_router().into_openapi()
}
