use super::CapabilityPricing;
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

pub type EffortMapping = BTreeMap<EffortLevel, String>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

pub static ALL_CAPABILITIES: [Capability; 12] = [
    Capability::Text,
    Capability::Vision,
    Capability::Pdf,
    Capability::Audio,
    Capability::Video,
    Capability::ImageGeneration,
    Capability::AudioGeneration,
    Capability::Search,
    Capability::Reasoning,
    Capability::ToolCall,
    Capability::ForcedToolChoice,
    Capability::StructuredOutput,
];

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Thinking {
    pub effort_mapping: EffortMapping,
    #[serde(default)]
    pub default_effort: Option<EffortLevel>,
    pub budget: Option<ThinkingBudget>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum CapabilitySupport<T> {
    #[default]
    Unknown,
    Unsupported,
    Supported {
        configuration: T,
        #[serde(default)]
        pricing: Option<CapabilityPricing>,
    },
}

impl<T> CapabilitySupport<T> {
    pub fn supported(configuration: T, pricing: Option<CapabilityPricing>) -> Self {
        Self::Supported {
            configuration,
            pricing,
        }
    }
    pub fn pricing(&self) -> Option<&CapabilityPricing> {
        match self {
            Self::Supported { pricing, .. } => pricing.as_ref(),
            _ => None,
        }
    }

    pub fn configuration(&self) -> Option<&T> {
        match self {
            Self::Supported { configuration, .. } => Some(configuration),
            _ => None,
        }
    }

    pub fn is_supported(&self) -> bool {
        matches!(self, Self::Supported { .. })
    }
    pub fn is_unsupported(&self) -> bool {
        matches!(self, Self::Unsupported)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BasicCapability {}

pub type FeatureSupport = CapabilitySupport<BasicCapability>;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TextCapabilities {
    pub input: CapabilitySupport<TextInput>,
    pub output: CapabilitySupport<TextGeneration>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TextInput {
    pub max_tokens: Option<u32>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TextGeneration {
    pub max_tokens: Option<u32>,
    pub thinking: CapabilitySupport<Thinking>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VisionCapability {
    pub formats: Option<Vec<String>>,
    pub max_images: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PdfCapability {
    pub max_files: Option<u32>,
    pub max_pages: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AudioCapability {
    pub formats: Option<Vec<String>>,
    pub max_duration_ms: Option<u32>,
    pub max_bytes: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AudioCapabilities {
    pub input: CapabilitySupport<AudioCapability>,
    pub output: CapabilitySupport<AudioCapability>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VideoCapability {
    pub formats: Option<Vec<String>>,
    pub max_duration_ms: Option<u32>,
    pub max_frames: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ImageGenerationCapability {
    pub formats: Option<Vec<String>>,
    pub max_images: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ToolCallingCapability {
    pub max_tools: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ToolCapabilities {
    pub calling: CapabilitySupport<ToolCallingCapability>,
    pub forced_choice: FeatureSupport,
    pub parallel_calls: FeatureSupport,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StructuredOutputCapability {
    pub json_mode: FeatureSupport,
    pub json_schema: FeatureSupport,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SearchCapability {
    pub max_results: Option<u32>,
}

// This view borrows the same fields stored on each entity; it owns no capability state.
pub(crate) struct CapabilityFields<'a> {
    pub text: &'a TextCapabilities,
    pub vision: &'a CapabilitySupport<VisionCapability>,
    pub pdf: &'a CapabilitySupport<PdfCapability>,
    pub audio: &'a AudioCapabilities,
    pub video: &'a CapabilitySupport<VideoCapability>,
    pub image_generation: &'a CapabilitySupport<ImageGenerationCapability>,
    pub tools: &'a ToolCapabilities,
    pub structured_output: &'a CapabilitySupport<StructuredOutputCapability>,
    pub search: &'a CapabilitySupport<SearchCapability>,
}

impl CapabilityFields<'_> {
    pub fn validate(
        &self,
        billing: Option<&super::ModelBilling>,
    ) -> Result<(), validify::ValidationErrors> {
        use validify::Validate;
        for price in [
            self.text.input.pricing(),
            self.text.output.pricing(),
            self.vision.pricing(),
            self.pdf.pricing(),
            self.audio.input.pricing(),
            self.audio.output.pricing(),
            self.video.pricing(),
            self.image_generation.pricing(),
            self.tools.calling.pricing(),
            self.tools.forced_choice.pricing(),
            self.tools.parallel_calls.pricing(),
            self.structured_output.pricing(),
            self.search.pricing(),
        ]
        .into_iter()
        .flatten()
        {
            if matches!(price, CapabilityPricing::ModelBilling) && billing.is_none() {
                return Err(super::validation::invalid("unresolved_model_billing"));
            }
            price.validate()?;
        }
        let mut limits = Vec::new();
        if let Some(v) = self.text.input.configuration() {
            limits.push(v.max_tokens);
        }
        if let Some(v) = self.text.output.configuration() {
            limits.push(v.max_tokens);
            if let Some(p) = v.thinking.pricing() {
                if matches!(p, CapabilityPricing::ModelBilling) && billing.is_none() {
                    return Err(super::validation::invalid("unresolved_model_billing"));
                }
                p.validate()?;
            }
        }
        let mut formats = Vec::new();
        if let Some(v) = self.vision.configuration() {
            limits.push(v.max_images);
            formats.push(&v.formats);
        }
        if let Some(v) = self.pdf.configuration() {
            limits.extend([v.max_files, v.max_pages]);
        }
        for v in [
            self.audio.input.configuration(),
            self.audio.output.configuration(),
        ]
        .into_iter()
        .flatten()
        {
            limits.extend([v.max_duration_ms, v.max_bytes]);
            formats.push(&v.formats);
        }
        if let Some(v) = self.video.configuration() {
            limits.extend([v.max_duration_ms, v.max_frames]);
            formats.push(&v.formats);
        }
        if let Some(v) = self.image_generation.configuration() {
            limits.push(v.max_images);
            formats.push(&v.formats);
        }
        if let Some(v) = self.tools.calling.configuration() {
            limits.push(v.max_tools);
        }
        if let Some(v) = self.search.configuration() {
            limits.push(v.max_results);
        }
        if limits.contains(&Some(0))
            || formats
                .into_iter()
                .flatten()
                .any(|values| values.is_empty() || values.iter().any(|v| v.trim().is_empty()))
        {
            return Err(super::validation::invalid(
                "invalid_capability_configuration",
            ));
        }
        Ok(())
    }
    pub fn supported(&self) -> Vec<Capability> {
        self.select(false)
    }
    pub fn unknown(&self) -> Vec<Capability> {
        let supported = self.supported();
        let unsupported = self.unsupported();
        super::ALL_CAPABILITIES
            .into_iter()
            .filter(|value| !supported.contains(value) && !unsupported.contains(value))
            .collect()
    }
    pub fn unsupported(&self) -> Vec<Capability> {
        self.select(true)
    }
    fn select(&self, unsupported: bool) -> Vec<Capability> {
        let states = [
            (
                Capability::Text,
                (self.text.input.is_supported() || self.text.output.is_supported()),
                (self.text.input.is_unsupported() && self.text.output.is_unsupported()),
            ),
            (
                Capability::Vision,
                self.vision.is_supported(),
                self.vision.is_unsupported(),
            ),
            (
                Capability::Pdf,
                self.pdf.is_supported(),
                self.pdf.is_unsupported(),
            ),
            (
                Capability::Audio,
                self.audio.input.is_supported(),
                self.audio.input.is_unsupported(),
            ),
            (
                Capability::Video,
                self.video.is_supported(),
                self.video.is_unsupported(),
            ),
            (
                Capability::ImageGeneration,
                self.image_generation.is_supported(),
                self.image_generation.is_unsupported(),
            ),
            (
                Capability::AudioGeneration,
                self.audio.output.is_supported(),
                self.audio.output.is_unsupported(),
            ),
            (
                Capability::Search,
                self.search.is_supported(),
                self.search.is_unsupported(),
            ),
            (
                Capability::Reasoning,
                self.text
                    .output
                    .configuration()
                    .is_some_and(|v| v.thinking.is_supported()),
                self.text
                    .output
                    .configuration()
                    .is_some_and(|v| v.thinking.is_unsupported()),
            ),
            (
                Capability::ToolCall,
                self.tools.calling.is_supported(),
                self.tools.calling.is_unsupported(),
            ),
            (
                Capability::ForcedToolChoice,
                self.tools.forced_choice.is_supported(),
                self.tools.forced_choice.is_unsupported(),
            ),
            (
                Capability::StructuredOutput,
                self.structured_output.is_supported(),
                self.structured_output.is_unsupported(),
            ),
        ];
        states
            .into_iter()
            .filter_map(|(cap, supported, absent)| {
                if if unsupported { absent } else { supported } {
                    Some(cap)
                } else {
                    None
                }
            })
            .collect()
    }
}
