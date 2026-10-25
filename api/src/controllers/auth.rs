use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::{StatusCode, header::CACHE_CONTROL},
    response::IntoResponse,
};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    di::Di,
    dtos::auth::{
        AuthTokensResponse, ConfirmSignupOtpRequest, ErrorResponse, SignupEmailRequest,
        SignupEmailResponse,
    },
    error::ApiError,
};

pub fn router() -> OpenApiRouter<Di> {
    OpenApiRouter::new()
        .routes(routes!(signup_email))
        .routes(routes!(confirm_signup_otp))
}

#[utoipa::path(
    post, path = "/api/auth/signup/email", operation_id = "signupEmail", tag = "auth",
    request_body = SignupEmailRequest,
    responses(
        (status = 200, description = "OTP delivery requested; use this token to confirm", body = SignupEmailResponse),
        (status = 400, description = "Invalid email or request", body = ErrorResponse),
        (status = 503, description = "Email delivery unavailable", body = ErrorResponse)
    )
)]
async fn signup_email(
    State(di): State<Di>,
    input: Result<Json<SignupEmailRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(input) = input?;
    let token = di.auth_service().signup_email(&input.email).await?;
    Ok((
        [(CACHE_CONTROL, "no-store")],
        Json(SignupEmailResponse { token }),
    ))
}

#[utoipa::path(
    post, path = "/api/auth/signup/email/confirmation/otp", operation_id = "confirmSignupOtp", tag = "auth",
    request_body = ConfirmSignupOtpRequest,
    responses(
        (status = 201, description = "User created", body = AuthTokensResponse),
        (status = 400, description = "Invalid display name or request", body = ErrorResponse),
        (status = 401, description = "Invalid or expired token or OTP", body = ErrorResponse),
        (status = 409, description = "Email already registered", body = ErrorResponse),
        (status = 500, description = "Database or internal error", body = ErrorResponse)
    )
)]
async fn confirm_signup_otp(
    State(di): State<Di>,
    input: Result<Json<ConfirmSignupOtpRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(input) = input?;
    let tokens = di
        .auth_service()
        .signup(&input.token, &input.code, &input.display_name)
        .await?;
    Ok((
        StatusCode::CREATED,
        [(CACHE_CONTROL, "no-store")],
        Json(AuthTokensResponse {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
        }),
    ))
}
