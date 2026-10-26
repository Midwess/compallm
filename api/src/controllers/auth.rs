use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::{StatusCode, header::CACHE_CONTROL},
    response::IntoResponse,
};
use compallm_core::auth::AuthService;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::error::{ApiError, ErrorResponse};

pub fn router() -> OpenApiRouter<&'static AuthService> {
    OpenApiRouter::new()
        .routes(routes!(signup_email))
        .routes(routes!(confirm_signup_otp))
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
}

/// User credentials returned by the current signup API.
/// Cookie transport is planned separately and is not implemented by this response.
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthTokensResponse {
    /// Signed access JWT scoped to the user's initial organization and membership role.
    #[schema(example = "example-user-access-token")]
    pub access_token: String,
    /// Refresh JWT used to renew the same organization-scoped session as the access token.
    #[schema(example = "example-refresh-token")]
    pub refresh_token: String,
}

/// Confirm email signup with a verification code.
///
/// Validates the encrypted challenge and emailed code, then creates the user and their personal
/// organization atomically. The display name is derived from the email address.
/// Returns access and refresh credentials in JSON; this endpoint does not set authentication cookies.
/// An existing email is rejected rather than signed in. Request a new code if the challenge expired.
#[utoipa::path(
    post, path = "/api/auth/signup/email/confirmation/otp", operation_id = "confirmSignupOtp", tag = "auth",
    security(),
    request_body = ConfirmSignupOtpRequest,
    responses(
        (status = 201, description = "User created", body = AuthTokensResponse),
        (status = 400, description = "Invalid email or JSON request. Correct the input before retrying.", body = ErrorResponse, example = json!({"message": "Invalid JSON request body"})),
        (status = 401, description = "Invalid or expired challenge or code. Check the code or request a new challenge.", body = ErrorResponse, example = json!({"message": "Invalid or expired token or OTP code"})),
        (status = 403, description = "User or organization access is disabled. Contact the administrator.", body = ErrorResponse, example = json!({"message": "Organization access denied"})),
        (status = 409, description = "Email already registered. Use signin instead of signup.", body = ErrorResponse, example = json!({"message": "Email is already registered"})),
        (status = 500, description = "Database or internal error. Retry later.", body = ErrorResponse, example = json!({"message": "Internal server error"}))
    )
)]
async fn confirm_signup_otp(
    State(auth): State<&'static AuthService>,
    input: Result<Json<ConfirmSignupOtpRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(input) = input?;
    let tokens = auth.signup(&input.token, &input.code).await?;
    Ok((
        StatusCode::CREATED,
        [(CACHE_CONTROL, "no-store")],
        Json(AuthTokensResponse {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
        }),
    ))
}
