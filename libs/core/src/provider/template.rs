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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CatalogSearch {
    pub terms: Vec<String>,
}
impl TryFrom<&str> for CatalogSearch {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.chars().count() > 256 || value.split_whitespace().count() > 8 {
            return Err("Search supports up to 256 characters and 8 terms.");
        }
        let mut terms = value
            .split_whitespace()
            .map(str::to_lowercase)
            .collect::<Vec<_>>();
        terms.sort_unstable();
        terms.dedup();
        Ok(Self { terms })
    }
}
