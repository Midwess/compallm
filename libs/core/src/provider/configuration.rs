use std::collections::BTreeMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub type Headers = BTreeMap<String, String>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceKind {
    Llm,
    Image,
    Video,
    Embedding,
    Tts,
    Stt,
}

impl From<ServiceKind> for String {
    fn from(value: ServiceKind) -> Self {
        match value {
            ServiceKind::Llm => "llm",
            ServiceKind::Image => "image",
            ServiceKind::Video => "video",
            ServiceKind::Embedding => "embedding",
            ServiceKind::Tts => "tts",
            ServiceKind::Stt => "stt",
        }
        .into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    Openai,
    OpenaiResponses,
    Anthropic,
    Gemini,
    Minimax,
    Openrouter,
    Xai,
}

impl Protocol {
    pub const fn supports(self, service: ServiceKind) -> bool {
        matches!(
            (service, self),
            (
                ServiceKind::Llm,
                Self::Openai | Self::OpenaiResponses | Self::Anthropic | Self::Gemini
            ) | (ServiceKind::Image, Self::Openai | Self::Gemini)
                | (ServiceKind::Embedding | ServiceKind::Stt, Self::Openai)
                | (
                    ServiceKind::Tts,
                    Self::Openai | Self::Minimax | Self::Openrouter
                )
                | (ServiceKind::Video, Self::Xai)
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Connection {
    pub base_url: String,
    pub timeout_ms: u32,
    #[serde(default)]
    pub query: BTreeMap<String, String>,
    pub authentication: ApiKeyAuthentication,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields, rename_all_fields = "camelCase")]
pub enum ApiKeyAuthentication {
    Header { api_key_header: ApiKeyHeader },
    Query { api_key_query: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ApiKeyHeader {
    pub name: String,
    pub prefix: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    Text,
    Vision,
    Pdf,
    Audio,
    Video,
    ImageGeneration,
    AudioGeneration,
    Search,
    Reasoning,
    ToolCall,
    ForcedToolChoice,
    StructuredOutput,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ModelLimits {
    pub context_tokens: Option<u32>,
    pub max_output_tokens: Option<u32>,
    pub max_images_per_request: Option<u32>,
    pub max_duration_ms: Option<u32>,
    pub max_audio_bytes: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Thinking {
    pub effort: Option<Vec<EffortLevel>>,
    pub budget: Option<ThinkingBudget>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffortLevel {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ThinkingBudget {
    pub min_tokens: u32,
    pub max_tokens: u32,
    pub default_tokens: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Pricing {
    Token(TokenPricing),
    Audio(AudioPricing),
    Image(ImagePricing),
    Video(VideoPricing),
    Request(RequestPricing),
}

impl validify::Validate for Pricing {
    fn validate(&self) -> Result<(), validify::ValidationErrors> {
        let amounts: Vec<Decimal> = match self {
            Self::Token(value) => vec![
                value.input_usd_per_million_tokens,
                value.output_usd_per_million_tokens,
                value.cached_usd_per_million_tokens,
            ],
            Self::Audio(AudioPricing::Duration(value)) => vec![value.usd_per_second],
            Self::Audio(AudioPricing::Character(value)) => vec![value.usd_per_million_characters],
            Self::Image(value) => value.rates.iter().map(|rate| rate.usd_per_image).collect(),
            Self::Video(value) => value.rates.iter().map(|rate| rate.usd_per_second).collect(),
            Self::Request(value) => vec![value.usd_per_request],
        };
        if amounts.is_empty()
            || amounts.iter().any(|amount| {
                amount.is_sign_negative()
                    || amount.scale() > super::input::PRICE_SCALE
                    || *amount >= Decimal::from(10_u64.pow(super::input::PRICE_INTEGER_DIGITS))
            })
        {
            let mut errors = validify::ValidationErrors::new();
            errors.add(validify::ValidationError::new_schema("invalid_pricing"));
            return Err(errors);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TokenPricing {
    #[serde(with = "rust_decimal::serde::str")]
    pub input_usd_per_million_tokens: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub output_usd_per_million_tokens: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub cached_usd_per_million_tokens: Decimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AudioPricing {
    Duration(AudioDurationPricing),
    Character(AudioCharacterPricing),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AudioDurationPricing {
    #[serde(with = "rust_decimal::serde::str")]
    pub usd_per_second: Decimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AudioCharacterPricing {
    #[serde(with = "rust_decimal::serde::str")]
    pub usd_per_million_characters: Decimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ImagePricing {
    pub rates: Vec<ImageRate>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ImageRate {
    #[serde(with = "rust_decimal::serde::str")]
    pub usd_per_image: Decimal,
    #[serde(default)]
    pub conditions: RateConditions,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VideoPricing {
    pub rates: Vec<VideoRate>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VideoRate {
    #[serde(with = "rust_decimal::serde::str")]
    pub usd_per_second: Decimal,
    #[serde(default)]
    pub conditions: RateConditions,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RequestPricing {
    #[serde(with = "rust_decimal::serde::str")]
    pub usd_per_request: Decimal,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RateConditions {
    pub resolution: Option<String>,
    pub quality: Option<String>,
    pub audio: Option<bool>,
    pub service_tier: Option<String>,
}
