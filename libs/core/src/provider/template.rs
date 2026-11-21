use serde::{Deserialize, Serialize};

use super::configuration::{Authentication, Connection, Headers};
use crate::model::template::ModelTemplate;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProviderTemplate {
    pub id: String,
    pub research_id: String,
    pub name: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub connection: Connection,
    pub authentication_options: Vec<Authentication>,
    pub headers: Headers,
    pub models: Vec<ModelTemplate>,
}
