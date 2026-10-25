use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct SignupEmailRequest {
    pub email: String,
}

#[derive(Serialize, ToSchema)]
pub struct SignupEmailResponse {
    pub token: String,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmSignupOtpRequest {
    pub token: String,
    pub code: String,
    pub display_name: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthTokensResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
}
