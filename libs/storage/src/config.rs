use serde::Deserialize;

/// Credentials deliberately do not implement Debug.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S3Settings {
    pub enabled: bool,
    pub region: String,
    pub endpoint: String,
    pub upload_endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub force_path_style: bool,
    pub bucket: String,
    pub public_base_url: String,
    pub timeout_seconds: u64,
    pub allowed_origins: Vec<String>,
}
