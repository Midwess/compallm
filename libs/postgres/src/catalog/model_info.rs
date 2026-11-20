use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Version-one retrieval metadata. These records describe sources and never execute requests.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelInfoSource {
    pub kind: SourceKind,
    pub url: Option<String>,
    pub access: SourceAccess,
    pub status: SourceStatus,
    pub fields: Vec<ModelInfoField>,
    pub request: Option<SourceRequest>,
    pub response: Option<SourceResponse>,
    pub pagination: Option<PagePagination>,
    pub revision: Option<String>,
    pub notes: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    Api,
    Documentation,
    Repository,
    Manual,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceAccess {
    Public,
    ProviderCredential,
    Manual,
    Unknown,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceStatus {
    Observed,
    Documented,
    Reference,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelInfoField {
    BaseUrl,
    Pricing,
    Specification,
    UpstreamModelId,
    DisplayName,
    Description,
    Protocols,
    Paths,
    Capabilities,
    MaxContextTokens,
    MaxInputTokens,
    MaxOutputTokens,
    EffortMapping,
    InputPrice,
    OutputPrice,
    CachedPrice,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRequest {
    pub method: RetrievalMethod,
    pub authentication: SourceAuthentication,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetrievalMethod {
    #[serde(rename = "GET")]
    Get,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceAuthentication {
    None,
    ProviderApiKey,
    ProviderOauth,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceResponse {
    pub format: ResponseFormat,
    pub object_pointer: String,
    pub identity_pointer: String,
    pub identity_value: Option<String>,
    pub fields: BTreeMap<ModelInfoField, FieldLocation>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat {
    ModelList,
    ModelObject,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldLocation {
    pub pointer: String,
    pub unit: SourceUnit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceUnit {
    Identifier,
    Text,
    Tokens,
    UsdPerToken,
    UsdPerMillionTokens,
    ProviderNative,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PagePagination {
    pub page_parameter: String,
    pub page_size_parameter: String,
    pub page_size: u16,
    pub total_pages_pointer: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct ModelInfoSources(pub Vec<ModelInfoSource>);

impl TryFrom<serde_json::Value> for ModelInfoSources {
    type Error = &'static str;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let sources: Vec<ModelInfoSource> =
            serde_json::from_value(value).map_err(|_| "invalid model-info source shape")?;
        if sources.is_empty() || sources.len() > 32 {
            return Err("model-info sources must contain 1 to 32 records");
        }
        for source in &sources {
            if source.notes.trim().is_empty() || source.notes.chars().count() > 4096 {
                return Err("source needs bounded explanatory notes");
            }
            let fields: BTreeSet<_> = source.fields.iter().copied().collect();
            if fields.len() != source.fields.len() {
                return Err("duplicate source field");
            }
            match source.kind {
                SourceKind::Manual => {
                    if source.url.is_some()
                        || source.request.is_some()
                        || source.response.is_some()
                        || source.pagination.is_some()
                        || source.revision.is_some()
                        || source.access != SourceAccess::Manual
                        || source.status != SourceStatus::Reference
                    {
                        return Err("manual source cannot declare a remote request");
                    }
                }
                SourceKind::Repository | SourceKind::Documentation => {
                    if source
                        .url
                        .as_deref()
                        .is_none_or(|value| value.trim().is_empty())
                    {
                        return Err("source reference is required");
                    }
                    if source.access == SourceAccess::Manual {
                        return Err("reference source cannot have manual access");
                    }
                }
                SourceKind::Api => {
                    let url =
                        url::Url::parse(source.url.as_deref().ok_or("source URL is required")?)
                            .map_err(|_| "invalid source URL")?;
                    if !matches!(url.scheme(), "http" | "https")
                        || url.host_str().is_none()
                        || !url.username().is_empty()
                        || url.password().is_some()
                        || url.fragment().is_some()
                    {
                        return Err("source URL must be HTTP(S) without credentials or fragment");
                    }
                    if url.query_pairs().any(|(key, _)| {
                        matches!(
                            key.to_ascii_lowercase().as_str(),
                            "key"
                                | "api_key"
                                | "apikey"
                                | "token"
                                | "access_token"
                                | "authorization"
                                | "password"
                                | "secret"
                        )
                    }) {
                        return Err("source URL cannot contain credential query parameters");
                    }
                    if source.access == SourceAccess::Manual {
                        return Err("remote source cannot have manual access");
                    }
                }
            }
            if source.kind == SourceKind::Repository
                && source
                    .revision
                    .as_deref()
                    .is_none_or(|revision| revision.trim().is_empty())
            {
                return Err("repository source needs a revision");
            }
            if source.kind != SourceKind::Api {
                if source.request.is_some()
                    || source.response.is_some()
                    || source.pagination.is_some()
                {
                    return Err("only API sources declare request/response mappings");
                }
                continue;
            }
            let request = source.request.as_ref().ok_or("API request is required")?;
            if source.access == SourceAccess::Public
                && request.authentication != SourceAuthentication::None
            {
                return Err("public source cannot require credentials");
            }
            if source.access == SourceAccess::ProviderCredential
                && !matches!(
                    request.authentication,
                    SourceAuthentication::ProviderApiKey | SourceAuthentication::ProviderOauth
                )
            {
                return Err("credential source needs an authentication method");
            }
            if source.status == SourceStatus::Observed && source.response.is_none() {
                return Err("observed API needs a known response mapping");
            }
            if let Some(response) = &source.response {
                if !pointer(&response.object_pointer)
                    || !pointer(&response.identity_pointer)
                    || response.identity_pointer.is_empty()
                    || response.fields.is_empty()
                    || fields != response.fields.keys().copied().collect()
                {
                    return Err("invalid API identity or field mapping");
                }
                for (field, location) in &response.fields {
                    if !pointer(&location.pointer) {
                        return Err("invalid JSON pointer");
                    }
                    if matches!(
                        field,
                        ModelInfoField::MaxContextTokens
                            | ModelInfoField::MaxInputTokens
                            | ModelInfoField::MaxOutputTokens
                    ) && location.unit != SourceUnit::Tokens
                    {
                        return Err("token limits require token units");
                    }
                    if matches!(
                        field,
                        ModelInfoField::InputPrice
                            | ModelInfoField::OutputPrice
                            | ModelInfoField::CachedPrice
                    ) && !matches!(
                        location.unit,
                        SourceUnit::UsdPerToken
                            | SourceUnit::UsdPerMillionTokens
                            | SourceUnit::ProviderNative
                    ) {
                        return Err("prices need explicit source units");
                    }
                }
            }
            if let Some(pagination) = &source.pagination
                && (source
                    .response
                    .as_ref()
                    .is_none_or(|response| response.format != ResponseFormat::ModelList)
                    || pagination.page_parameter.trim().is_empty()
                    || pagination.page_size_parameter.trim().is_empty()
                    || pagination.page_size == 0
                    || pagination.page_size > 1000
                    || !pointer(&pagination.total_pages_pointer))
            {
                return Err("invalid model-list pagination");
            }
        }
        Ok(Self(sources))
    }
}

fn pointer(value: &str) -> bool {
    if value.len() > 1024
        || (!value.is_empty() && !value.starts_with('/'))
        || value.chars().any(char::is_control)
    {
        return false;
    }
    let mut bytes = value.bytes();
    while let Some(byte) = bytes.next() {
        if byte == b'~' && !matches!(bytes.next(), Some(b'0' | b'1')) {
            return false;
        }
    }
    true
}
