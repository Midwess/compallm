use axum::{
    Extension, Json,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use axum_valid::ValidifiedByRef;
use compallm_contract::ContractSchema;
use compallm_core::auth::{AuthService, EmailOtpAccessToken, UserAccessToken};
use compallm_utils::email;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use validify::Validify;

use crate::{
    error::{ApiError, ErrorResponse},
    middleware::authorization::RequireToken,
};
#[path = "auth/delivery.rs"]
pub mod delivery;
#[path = "auth/validation.rs"]
mod validation;
use delivery::{AuthTokensResponse, TokenDelivery, TokenDeliverySettings, TokenLifetime};
use validation::{AuthInput, ValidatedJson};

pub fn router() -> OpenApiRouter<&'static AuthService> {
    OpenApiRouter::new()
        .routes(routes!(signup_email))
        .routes(routes!(signin_email))
        .routes(routes!(refresh_tokens))
}

/// Email address to receive a signup verification code.
#[derive(Deserialize, ContractSchema, Validify)]
#[serde(deny_unknown_fields)]
pub struct SignupEmailRequest {
    /// Trimmed and ASCII-lowercased before email validation. Any valid domain is accepted.
    #[schema(example = "reader@example.com")]
    #[modify(trim, custom(str::make_ascii_lowercase))]
    #[validate(custom(function = email::validate_email))]
    pub email: String,
    /// Cookie (default) sets the OTP access cookie and returns lifetime metadata; token additionally returns the credential.
    #[serde(default)]
    #[schema(default = "cookie")]
    pub delivery: TokenDelivery,
}

/// Verification challenge returned after email delivery is requested.
#[derive(Serialize, ToSchema)]
pub struct SignupEmailResponse {
    /// OTP challenge validity in both cookie and token delivery.
    pub lifetime: TokenLifetime,
    /// Encrypted OTP access token, present only in token delivery. Expires after 10 minutes.
    /// This is not a user access credential and must not be used to call protected APIs.
    #[schema(example = "example-email-challenge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Request an email signup code.
///
/// Encoded credentials are limited to 3,800 bytes; serialized cookies to 4,096 bytes.
///
/// Rejects an already-registered email with 409 and a signin suggestion before mailing or token
/// delivery. For a new email, enqueues a six-digit verification code and returns an encrypted
/// challenge for confirmation. Submit the challenge and code to
/// `POST /api/auth/signup/email/confirmation/otp` within 10 minutes.
/// Delivery defaults to cookie: set a ten-minute HttpOnly OTP access cookie, clear any refresh
/// cookie and return lifetime metadata. Explicit delivery token returns the challenge in JSON and sets no cookies.
/// Cookie delivery requires an allowed Origin. No signed-in session exists until confirmation.
/// Queue acceptance does not guarantee SMTP or inbox delivery. Recipients already pending or sent
/// during the configured cooldown (30 seconds by default) are throttled; keep the earlier challenge. Admission waits up to the configured timeout (30 seconds by default)
/// when the queue is full. Admission failure returns no new challenge.
#[utoipa::path(
    post, path = "/api/auth/signup/email", operation_id = "signupEmail", tag = "auth",
    security(),
    params(("Origin" = Option<String>, Header, description = "Required for cookie delivery; must match the configured browser origin")),
    request_body(content = SignupEmailRequest, example = json!({"email":"reader@example.com","delivery":"token"})),
    responses(
        (status = 200, description = "OTP delivery requested; use this token to confirm", body = SignupEmailResponse),
        (status = 400, description = "Malformed JSON or invalid field format. Correct the input before retrying.", body = ErrorResponse, example = json!({"message": "Invalid request fields"})),
        (status = 403, description = "Untrusted Origin for cookie delivery", body = ErrorResponse),
        (status = 409, description = "Email already registered; sign in instead. No email or token issued.", body = ErrorResponse, example = json!({"message":"Email is already registered. <a href=\"/login\">Sign in instead.</a>"})),
        (status = 429, description = "Recipient throttled. Keep the existing challenge and retry after the configured cooldown (30 seconds by default).", body = ErrorResponse, example = json!({"message": "Email recipient is throttled; keep the existing challenge and retry later"})),
        (status = 503, description = "Email admission unavailable or queue remained full until the configured timeout (30 seconds by default). Retry later; no challenge was issued.", body = ErrorResponse, example = json!({"message": "Email delivery is unavailable"})),
        (status = 422, description = "Encoded authentication credential exceeds the supported size limit", body = ErrorResponse),
        (status = 500, description = "Internal error. Retry later.", body = ErrorResponse, example = json!({"message": "Internal server error"}))
    )
)]
async fn signup_email(
    State(auth): State<&'static AuthService>,
    Extension(settings): Extension<&'static TokenDeliverySettings>,
    headers: HeaderMap,
    input: ValidatedJson<SignupEmailRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let ValidifiedByRef(Json(input)) = input?;
    input.delivery.validate_origin(&headers, settings)?;
    let token = auth.signup_email(&input.email).await?;
    input.delivery.otp_response(token, settings)
}

/// Email challenge and code used to confirm signup.
#[derive(Deserialize, ContractSchema, Validify)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConfirmSignupOtpRequest {
    /// Six ASCII digits received by email; surrounding whitespace is trimmed. Submit before expiry.
    #[schema(example = "123456")]
    #[modify(trim)]
    #[validate(length(equal = 6), custom(function = AuthInput::otp))]
    pub code: String,
    /// Credential delivery: cookie (default) sets HttpOnly cookies and requires Origin;
    /// token returns both CWTs in JSON without setting cookies.
    #[serde(default)]
    #[schema(default = "cookie")]
    pub delivery: TokenDelivery,
}

/// Confirm email signup with a verification code.
///
/// Requires an email-OTP access token in the access cookie or Authorization: Bearer, verified by middleware.
/// JSON contains only code and delivery; token fields in JSON are rejected.
/// Validates the emailed code, then creates the user and their personal
/// organization atomically. The display name is derived from the email address.
/// Delivery defaults to cookie: sets both HttpOnly cookies and returns lifetime metadata.
/// Send Origin matching AUTH_PUBLIC_ORIGIN (or the local loopback Host in development).
/// Explicit delivery token returns accessToken and refreshToken in JSON and sets no cookies.
/// The delivery choice is controller-only and is never stored in the signed CWTs.
/// User access CWTs include the current compact Cedar policy snapshot; refresh CWTs do not.
/// Issuance exceeding 3,800 encoded bytes is rejected without returning credentials.
/// An existing email is rejected rather than signed in. Request a new code if the challenge expired.
#[utoipa::path(
    post, path = "/api/auth/signup/email/confirmation/otp", operation_id = "confirmSignupOtp", tag = "auth",
    security(("emailOtp" = []), ("emailOtpCookie" = [])),
    params(("Origin" = Option<String>, Header, description = "Required for cookie delivery; must match the configured browser origin")),
    request_body(content = ConfirmSignupOtpRequest, example = json!({"code":"123456","delivery":"token"})),
    responses(
        (status = 201, description = "User created; cookie mode returns lifetime metadata and sets both cookies; token mode returns both credentials", body = AuthTokensResponse),
        (status = 400, description = "Malformed JSON or invalid field format. Correct the input before retrying.", body = ErrorResponse, example = json!({"message": "Invalid JSON request body"})),
        (status = 401, description = "Invalid or expired challenge or code. Check the code or request a new challenge.", body = ErrorResponse, example = json!({"message": "Invalid or expired token or OTP code"})),
        (status = 403, description = "Untrusted cookie Origin or user/organization access disabled.", body = ErrorResponse, example = json!({"message": "Organization access denied"})),
        (status = 409, description = "Email already registered. Use signin instead of signup.", body = ErrorResponse, example = json!({"message": "Email is already registered. <a href=\"/login\">Sign in instead.</a>"})),
        (status = 422, description = "Encoded authentication credential exceeds the supported size limit", body = ErrorResponse),
        (status = 500, description = "Database or internal error. Retry later.", body = ErrorResponse, example = json!({"message": "Internal server error"}))
    )
)]
async fn confirm_signup_otp(
    State(auth): State<&'static AuthService>,
    Extension(settings): Extension<&'static TokenDeliverySettings>,
    headers: HeaderMap,
    RequireToken(challenge): RequireToken<EmailOtpAccessToken>,
    input: ValidatedJson<ConfirmSignupOtpRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let ValidifiedByRef(Json(input)) = input?;
    input.delivery.validate_origin(&headers, settings)?;
    let tokens = auth.signup(&challenge, &input.code).await?;
    input
        .delivery
        .response(tokens, StatusCode::CREATED, settings)
}

/// Email address to receive a signin verification code.
#[derive(Deserialize, ContractSchema, Validify)]
#[serde(deny_unknown_fields)]
pub struct SigninEmailRequest {
    /// Trimmed and ASCII-lowercased before email validation. Account existence is checked only at confirmation.
    #[schema(example = "reader@example.com")]
    #[modify(trim, custom(str::make_ascii_lowercase))]
    #[validate(custom(function = email::validate_email))]
    pub email: String,
    /// Cookie (default) sets the OTP access cookie and returns lifetime metadata; token additionally returns the credential.
    #[serde(default)]
    #[schema(default = "cookie")]
    pub delivery: TokenDelivery,
}

/// Request an email signin code.
///
/// Encoded credentials are limited to 3,800 bytes; serialized cookies to 4,096 bytes.
///
/// Enqueues a six-digit code and returns a ten-minute encrypted challenge.
/// Admission does not guarantee delivery. Pending recipients and the configured cooldown
/// are shared with signup; throttling returns no replacement challenge.
/// Cookie delivery (default) sets the ten-minute OTP access cookie, clears any refresh cookie,
/// and returns lifetime metadata; explicit token delivery returns the challenge in JSON without setting cookies.
/// Cookie delivery requires an allowed Origin. The normalized email must belong to an existing active user before mail admission. Unknown emails return 404 with a signup link; disabled users return 403. Rejections issue no challenge or cookies.
#[utoipa::path(
    post, path = "/api/auth/signin/email", operation_id = "signinEmail", tag = "auth",
    security(),
    params(("Origin" = Option<String>, Header, description = "Required for cookie delivery; must match the configured browser origin")),
    request_body(content = SigninEmailRequest, example = json!({"email":"reader@example.com","delivery":"token"})),
    responses(
        (status = 200, description = "Code queued; submit the challenge with the emailed code", body = SignupEmailResponse),
        (status = 404, description = "Email is not registered; signup first. No email or challenge issued.", body = ErrorResponse, example = json!({"message": "Email is not registered. <a href=\"/signup\">Sign up here.</a>"})),
        (status = 400, description = "Malformed JSON or invalid field format", body = ErrorResponse),
        (status = 403, description = "Untrusted Origin for cookie delivery", body = ErrorResponse),
        (status = 429, description = "Recipient throttled; keep the earlier challenge", body = ErrorResponse),
        (status = 422, description = "Encoded authentication credential exceeds the supported size limit", body = ErrorResponse),
        (status = 503, description = "Email admission unavailable or queue full", body = ErrorResponse)
    )
)]
async fn signin_email(
    State(auth): State<&'static AuthService>,
    Extension(settings): Extension<&'static TokenDeliverySettings>,
    headers: HeaderMap,
    input: ValidatedJson<SigninEmailRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let ValidifiedByRef(Json(input)) = input?;
    input.delivery.validate_origin(&headers, settings)?;
    let token = auth.signin_email(&input.email).await?;
    input.delivery.otp_response(token, settings)
}

/// Email challenge, verification code, and credential delivery for an existing user.
#[derive(Deserialize, ContractSchema, Validify)]
#[serde(deny_unknown_fields)]
pub struct ConfirmSigninOtpRequest {
    /// Six ASCII digits received by email; surrounding whitespace is trimmed. Valid for ten minutes.
    #[schema(example = "123456")]
    #[modify(trim)]
    #[validate(length(equal = 6), custom(function = AuthInput::otp))]
    pub code: String,
    /// Cookie (default) sets HttpOnly cookies; token returns both CWTs in JSON.
    #[serde(default)]
    #[schema(default = "cookie")]
    pub delivery: TokenDelivery,
}

/// Confirm email signin with a verification code.
///
/// Requires an email-OTP access token in the access cookie or Authorization: Bearer, verified by middleware.
/// JSON contains only code and delivery; token fields in JSON are rejected.
/// Requires an existing active user and an active organization membership.
/// Records a new session. Cookie delivery (default) requires an allowed Origin, sets both
/// HttpOnly cookies and returns lifetime metadata. Explicit delivery token returns
/// accessToken and refreshToken in JSON without setting cookies. Delivery is not a CWT claim.
/// User access CWTs require a current compact Cedar policy snapshot; refresh CWTs do not.
/// Oversized authority returns 422 without recording a login session.
#[utoipa::path(
    post, path = "/api/auth/signin/email/confirmation/otp", operation_id = "confirmSigninOtp", tag = "auth",
    security(("emailOtp" = []), ("emailOtpCookie" = [])),
    params(("Origin" = Option<String>, Header, description = "Required for cookie delivery; must match the configured browser origin")),
    request_body(content = ConfirmSigninOtpRequest, example = json!({"code":"123456","delivery":"token"})),
    responses(
        (status = 200, description = "Signed in; cookie mode returns lifetime metadata and sets both cookies; token mode returns both credentials", body = AuthTokensResponse),
        (status = 400, description = "Malformed JSON or invalid field format", body = ErrorResponse),
        (status = 401, description = "Invalid or expired challenge or code", body = ErrorResponse),
        (status = 403, description = "Untrusted cookie Origin or user/organization access disabled", body = ErrorResponse),
        (status = 404, description = "No account for the verified email; signup first", body = ErrorResponse),
        (status = 422, description = "Encoded authentication credential exceeds the supported size limit", body = ErrorResponse),
        (status = 500, description = "Database or internal error", body = ErrorResponse)
    )
)]
async fn confirm_signin_otp(
    State(auth): State<&'static AuthService>,
    Extension(settings): Extension<&'static TokenDeliverySettings>,
    headers: HeaderMap,
    RequireToken(challenge): RequireToken<EmailOtpAccessToken>,
    input: ValidatedJson<ConfirmSigninOtpRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let ValidifiedByRef(Json(input)) = input?;
    input.delivery.validate_origin(&headers, settings)?;
    let tokens = auth.signin(&challenge, &input.code).await?;
    input.delivery.response(tokens, StatusCode::OK, settings)
}

/// Identity and organization scope from the current signed access credential.
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUserResponse {
    /// Current login session identifier.
    pub session_id: u64,
    /// Authenticated user identifier.
    pub user_id: u64,
    /// Organization selected for this session.
    pub organization_id: u64,
    /// Organization name snapshot from the signed session; refreshed when tokens renew.
    pub organization_name: String,
}

pub fn protected_router() -> OpenApiRouter<&'static AuthService> {
    OpenApiRouter::new()
        .routes(routes!(current_user))
        .routes(routes!(signout))
        .routes(routes!(confirm_signup_otp))
        .routes(routes!(confirm_signin_otp))
}

/// Read the current authenticated identity and organization scope.
///
/// Accepts a user access CWT through Authorization Bearer or the configured access cookie.
/// An explicit Authorization header takes precedence and never falls back to cookies.
/// Cookie mode renews missing or expired access through the refresh cookie and returns
/// replacement HttpOnly cookies when Origin is trusted, or a safe request has
/// Sec-Fetch-Site: same-origin. No credentials are returned in JSON.
/// Middleware performs one scoped session-version lookup for valid user access.
/// Matching versions reuse signed claims; stale claims rebuild before this handler runs.
/// Cookie renewal requires a matching valid refresh cookie and trusted Origin.
/// Stale bearer access returns 401 as expired. Present a valid refresh CWT to
/// POST /api/auth/refresh, update both credentials, then retry the original request.
/// Temporary session-storage errors return 503 and preserve browser credentials.
#[utoipa::path(
    get, path = "/api/auth/me", operation_id = "currentUser", tag = "auth",
    security(("bearerAuth" = []), ("accessCookie" = [])),
    responses(
        (status = 200, description = "Current identity; stale cookie authority renews automatically, stale bearer access returns 401", body = CurrentUserResponse),
        (status = 401, description = "Missing, invalid, expired, or wrong-variant credentials", body = ErrorResponse),
        (status = 403, description = "Untrusted renewal Origin or disabled account/membership", body = ErrorResponse),
        (status = 422, description = "Encoded authentication credential exceeds the supported size limit", body = ErrorResponse),
        (status = 503, description = "Session validation temporarily unavailable; cookies are preserved for retry", body = ErrorResponse),
        (status = 500, description = "Internal validation or policy construction failure", body = ErrorResponse)
    )
)]
async fn current_user(
    RequireToken(claims): RequireToken<UserAccessToken>,
) -> Result<Json<CurrentUserResponse>, ApiError> {
    Ok(Json(CurrentUserResponse {
        session_id: claims.session.id,
        user_id: claims.session.user_id,
        organization_id: claims.session.organization_id,
        organization_name: claims.session.organization_name,
    }))
}

/// Sign out of the current session.
///
/// Revokes the organization/user-scoped session selected by a verified user access token.
/// Cookie requests require a trusted Origin and clear both HttpOnly authentication cookies.
/// Bearer requests revoke the selected session without changing browser cookies.
/// Core revocation is idempotent; repeated HTTP signout is rejected by revoked-session middleware validation.
/// Other sessions remain active. The next access freshness lookup rejects this revoked session.
#[utoipa::path(
    post, path = "/api/auth/signout", operation_id = "signOut", tag = "auth",
    security(("bearerAuth" = []), ("accessCookie" = [])),
    responses(
        (status = 204, description = "Session revoked; cookie requests clear authentication cookies"),
        (status = 401, description = "Missing or invalid user credential or session", body = ErrorResponse),
        (status = 403, description = "Untrusted cookie Origin", body = ErrorResponse),
        (status = 422, description = "Encoded authentication credential exceeds the supported size limit", body = ErrorResponse),
        (status = 503, description = "Session persistence unavailable; credentials are preserved for retry", body = ErrorResponse)
    )
)]
async fn signout(
    State(auth): State<&'static AuthService>,
    Extension(settings): Extension<&'static TokenDeliverySettings>,
    headers: HeaderMap,
    RequireToken(claims): RequireToken<UserAccessToken>,
) -> Result<impl IntoResponse, ApiError> {
    auth.signout(&claims).await?;
    let mut response_headers = HeaderMap::new();
    if !headers.contains_key(axum::http::header::AUTHORIZATION) {
        settings.clear_cookies(&mut response_headers)?;
    }
    Ok((StatusCode::NO_CONTENT, response_headers))
}

/// Explicitly refresh bearer credentials.
///
/// Send the unexpired refresh CWT as Authorization: Bearer. No request body is
/// required. Access/OTP credentials and cookie-only requests are rejected.
/// Revalidates current session/user/membership and rebuilds the permission snapshot.
/// Returns both CWTs and lifetime metadata in JSON; no cookies are set or cleared.
/// A stale authorization version on the refresh CWT is allowed; revoked, expired,
/// foreign or removed credentials cannot renew. Save both credentials and retry
/// the original request after stale/expired access returns 401.
#[utoipa::path(
    post, path = "/api/auth/refresh", operation_id = "refreshTokens", tag = "auth",
    security(("refreshToken" = [])),
    responses(
        (status = 200, description = "Current access and refresh CWTs with lifetime metadata; no Set-Cookie", body = AuthTokensResponse),
        (status = 401, description = "Missing explicit refresh bearer, wrong token type, expired/revoked/foreign credential, or unavailable membership", body = ErrorResponse),
        (status = 422, description = "Encoded credential exceeds the supported size limit", body = ErrorResponse),
        (status = 503, description = "Session storage unavailable; retry with the existing refresh credential", body = ErrorResponse),
        (status = 500, description = "Invalid stored policy or internal failure", body = ErrorResponse)
    )
)]
async fn refresh_tokens(
    State(auth): State<&'static AuthService>,
    Extension(settings): Extension<&'static TokenDeliverySettings>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let encoded = crate::middleware::authorization::Authorization::bearer(&headers)?
        .ok_or(ApiError::Unauthenticated)?;
    let tokens = auth.refresh(encoded).await?;
    TokenDelivery::Token.response(tokens, StatusCode::OK, settings)
}
