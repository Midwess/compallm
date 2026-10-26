use axum::{
    Extension, Json,
    extract::{State, rejection::JsonRejection},
    http::{HeaderMap, StatusCode, header::CACHE_CONTROL},
    response::IntoResponse,
};
use compallm_core::auth::AuthService;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::error::{ApiError, ErrorResponse};
pub mod delivery;
use delivery::{AuthTokensResponse, TokenDelivery, TokenDeliverySettings};

pub fn router() -> OpenApiRouter<&'static AuthService> {
    OpenApiRouter::new()
        .routes(routes!(signup_email))
        .routes(routes!(confirm_signup_otp))
        .routes(routes!(signin_email))
        .routes(routes!(confirm_signin_otp))
}

/// Email address to receive a signup verification code.
#[derive(Deserialize, ToSchema)]
pub struct SignupEmailRequest {
    /// Valid email address. Addresses from any domain are accepted.
    #[schema(example = "reader@example.com")]
    pub email: String,
}

/// Verification challenge returned after email delivery is requested.
#[derive(Serialize, ToSchema)]
pub struct SignupEmailResponse {
    /// Encrypted challenge to submit with the emailed code. Expires after 10 minutes.
    /// This is not a user access credential and must not be used to call protected APIs.
    #[schema(example = "example-email-challenge")]
    pub token: String,
}

/// Request an email signup code.
///
/// Enqueues a six-digit verification code for the supplied email address and returns an encrypted
/// challenge for confirmation. Submit the challenge and code to
/// `POST /api/auth/signup/email/confirmation/otp` within 10 minutes.
/// This public endpoint does not create a user or establish a signed-in session.
/// Queue acceptance does not guarantee SMTP or inbox delivery. Recipients already pending or sent
/// during the configured cooldown (30 seconds by default) are throttled; keep the earlier challenge. Admission waits up to the configured timeout (30 seconds by default)
/// when the queue is full. Admission failure returns no new challenge.
#[utoipa::path(
    post, path = "/api/auth/signup/email", operation_id = "signupEmail", tag = "auth",
    security(),
    request_body = SignupEmailRequest,
    responses(
        (status = 200, description = "OTP delivery requested; use this token to confirm", body = SignupEmailResponse),
        (status = 400, description = "Invalid email or JSON request. Correct the input before retrying.", body = ErrorResponse, example = json!({"message": "Invalid email address"})),
        (status = 429, description = "Recipient throttled. Keep the existing challenge and retry after the configured cooldown (30 seconds by default).", body = ErrorResponse, example = json!({"message": "Email recipient is throttled; keep the existing challenge and retry later"})),
        (status = 503, description = "Email admission unavailable or queue remained full until the configured timeout (30 seconds by default). Retry later; no challenge was issued.", body = ErrorResponse, example = json!({"message": "Email delivery is unavailable"})),
        (status = 500, description = "Internal error. Retry later.", body = ErrorResponse, example = json!({"message": "Internal server error"}))
    )
)]
async fn signup_email(
    State(auth): State<&'static AuthService>,
    input: Result<Json<SignupEmailRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(input) = input?;
    let token = auth.signup_email(&input.email).await?;
    Ok((
        [(CACHE_CONTROL, "no-store")],
        Json(SignupEmailResponse { token }),
    ))
}

/// Email challenge and code used to confirm signup.
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmSignupOtpRequest {
    /// Encrypted challenge returned by the email signup request.
    #[schema(example = "example-email-challenge")]
    pub token: String,
    /// Six-digit code received by email. Submit it before the challenge expires.
    #[schema(example = "123456")]
    pub code: String,
    /// Credential delivery: cookie (default) sets HttpOnly cookies and requires Origin;
    /// token returns both JWTs in JSON without setting cookies.
    #[serde(default)]
    #[schema(default = "cookie")]
    pub delivery: TokenDelivery,
}

/// Confirm email signup with a verification code.
///
/// Validates the encrypted challenge and emailed code, then creates the user and their personal
/// organization atomically. The display name is derived from the email address.
/// Delivery defaults to cookie: sets both HttpOnly cookies and returns an empty JSON object.
/// Send Origin matching AUTH_PUBLIC_ORIGIN (or the local loopback Host in development).
/// Explicit delivery token returns accessToken and refreshToken in JSON and sets no cookies.
/// The delivery choice is controller-only and is never stored in the signed JWTs.
/// An existing email is rejected rather than signed in. Request a new code if the challenge expired.
#[utoipa::path(
    post, path = "/api/auth/signup/email/confirmation/otp", operation_id = "confirmSignupOtp", tag = "auth",
    security(),
    params(("Origin" = Option<String>, Header, description = "Required for cookie delivery; must match the configured browser origin")),
    request_body(content = ConfirmSignupOtpRequest, example = json!({"token":"example-email-challenge","code":"123456","delivery":"token"})),
    responses(
        (status = 201, description = "User created; cookie mode returns {} and sets both cookies; token mode returns both credentials", body = AuthTokensResponse),
        (status = 400, description = "Invalid email or JSON request. Correct the input before retrying.", body = ErrorResponse, example = json!({"message": "Invalid JSON request body"})),
        (status = 401, description = "Invalid or expired challenge or code. Check the code or request a new challenge.", body = ErrorResponse, example = json!({"message": "Invalid or expired token or OTP code"})),
        (status = 403, description = "Untrusted cookie Origin or user/organization access disabled.", body = ErrorResponse, example = json!({"message": "Organization access denied"})),
        (status = 409, description = "Email already registered. Use signin instead of signup.", body = ErrorResponse, example = json!({"message": "Email is already registered"})),
        (status = 500, description = "Database or internal error. Retry later.", body = ErrorResponse, example = json!({"message": "Internal server error"}))
    )
)]
async fn confirm_signup_otp(
    State(auth): State<&'static AuthService>,
    Extension(settings): Extension<&'static TokenDeliverySettings>,
    headers: HeaderMap,
    input: Result<Json<ConfirmSignupOtpRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(input) = input?;
    input.delivery.validate_origin(&headers, settings)?;
    let tokens = auth.signup(&input.token, &input.code).await?;
    input
        .delivery
        .response(tokens, StatusCode::CREATED, settings)
}

/// Email address to receive a signin verification code.
#[derive(Deserialize, ToSchema)]
pub struct SigninEmailRequest {
    /// Valid email address. Account existence is checked only at confirmation.
    #[schema(example = "reader@example.com")]
    pub email: String,
}

/// Request an email signin code.
///
/// Enqueues a six-digit code and returns a ten-minute encrypted challenge.
/// Admission does not guarantee delivery. Pending recipients and the configured cooldown
/// are shared with signup; throttling returns no replacement challenge.
/// Requesting a code neither creates a user nor checks whether the account exists.
#[utoipa::path(
    post, path = "/api/auth/signin/email", operation_id = "signinEmail", tag = "auth",
    security(), request_body = SigninEmailRequest,
    responses(
        (status = 200, description = "Code queued; submit the challenge with the emailed code", body = SignupEmailResponse),
        (status = 400, description = "Invalid email or JSON request", body = ErrorResponse),
        (status = 429, description = "Recipient throttled; keep the earlier challenge", body = ErrorResponse),
        (status = 503, description = "Email admission unavailable or queue full", body = ErrorResponse)
    )
)]
async fn signin_email(
    State(auth): State<&'static AuthService>,
    input: Result<Json<SigninEmailRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(input) = input?;
    let token = auth.signup_email(&input.email).await?;
    Ok((
        [(CACHE_CONTROL, "no-store")],
        Json(SignupEmailResponse { token }),
    ))
}

/// Email challenge, verification code, and credential delivery for an existing user.
#[derive(Deserialize, ToSchema)]
pub struct ConfirmSigninOtpRequest {
    /// Encrypted challenge returned by the email code request.
    #[schema(example = "example-email-challenge")]
    pub token: String,
    /// Six-digit emailed code, valid for ten minutes.
    #[schema(example = "123456")]
    pub code: String,
    /// Cookie (default) sets HttpOnly cookies; token returns both JWTs in JSON.
    #[serde(default)]
    #[schema(default = "cookie")]
    pub delivery: TokenDelivery,
}

/// Confirm email signin with a verification code.
///
/// Requires an existing active user and an active owner/admin organization membership.
/// Records a new session. Cookie delivery (default) requires an allowed Origin, sets both
/// HttpOnly cookies and returns an empty JSON object. Explicit delivery token returns
/// accessToken and refreshToken in JSON without setting cookies. Delivery is not a JWT claim.
#[utoipa::path(
    post, path = "/api/auth/signin/email/confirmation/otp", operation_id = "confirmSigninOtp", tag = "auth",
    security(),
    params(("Origin" = Option<String>, Header, description = "Required for cookie delivery; must match the configured browser origin")),
    request_body(content = ConfirmSigninOtpRequest, example = json!({"token":"example-email-challenge","code":"123456","delivery":"token"})),
    responses(
        (status = 200, description = "Signed in; cookie mode returns {} and sets both cookies; token mode returns both credentials", body = AuthTokensResponse),
        (status = 400, description = "Invalid JSON request", body = ErrorResponse),
        (status = 401, description = "Invalid or expired challenge or code", body = ErrorResponse),
        (status = 403, description = "Untrusted cookie Origin or user/organization access disabled", body = ErrorResponse),
        (status = 404, description = "No account for the verified email; signup first", body = ErrorResponse),
        (status = 500, description = "Database or internal error", body = ErrorResponse)
    )
)]
async fn confirm_signin_otp(
    State(auth): State<&'static AuthService>,
    Extension(settings): Extension<&'static TokenDeliverySettings>,
    headers: HeaderMap,
    input: Result<Json<ConfirmSigninOtpRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(input) = input?;
    input.delivery.validate_origin(&headers, settings)?;
    let tokens = auth.signin(&input.token, &input.code).await?;
    input.delivery.response(tokens, StatusCode::OK, settings)
}
