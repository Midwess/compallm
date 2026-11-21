use compallm_core::provider as domain;
use juniper::{GraphQLEnum, GraphQLObject};

#[derive(GraphQLEnum)]
pub enum CapabilityStatus {
    Unknown,
    Unsupported,
    Supported,
}
impl<T> From<&domain::CapabilitySupport<T>> for CapabilityStatus {
    fn from(value: &domain::CapabilitySupport<T>) -> Self {
        match value {
            domain::CapabilitySupport::Unknown => Self::Unknown,
            domain::CapabilitySupport::Unsupported => Self::Unsupported,
            domain::CapabilitySupport::Supported { .. } => Self::Supported,
        }
    }
}

#[derive(GraphQLObject)]
pub struct FeatureSupport {
    pub status: CapabilityStatus,
    pub pricing: Option<super::pricing::CapabilityPricing>,
}
impl From<&domain::FeatureSupport> for FeatureSupport {
    fn from(value: &domain::FeatureSupport) -> Self {
        Self {
            status: value.into(),
            pricing: value.pricing().map(Into::into),
        }
    }
}

#[derive(GraphQLObject)]
pub struct VisionSupport {
    pub status: CapabilityStatus,
    pub pricing: Option<super::pricing::CapabilityPricing>,
    pub configuration: Option<VisionConfiguration>,
}
#[derive(GraphQLObject)]
pub struct VisionConfiguration {
    pub formats: Option<Vec<String>>,
    pub max_images: Option<f64>,
}
impl From<&domain::CapabilitySupport<domain::VisionCapability>> for VisionSupport {
    fn from(value: &domain::CapabilitySupport<domain::VisionCapability>) -> Self {
        Self {
            status: value.into(),
            pricing: value.pricing().map(Into::into),
            configuration: value.configuration().map(|value| VisionConfiguration {
                formats: value.formats.clone(),
                max_images: value.max_images.map(f64::from),
            }),
        }
    }
}

#[derive(GraphQLObject)]
pub struct PdfSupport {
    pub status: CapabilityStatus,
    pub pricing: Option<super::pricing::CapabilityPricing>,
    pub configuration: Option<PdfConfiguration>,
}
#[derive(GraphQLObject)]
pub struct PdfConfiguration {
    pub max_files: Option<f64>,
    pub max_pages: Option<f64>,
}
impl From<&domain::CapabilitySupport<domain::PdfCapability>> for PdfSupport {
    fn from(value: &domain::CapabilitySupport<domain::PdfCapability>) -> Self {
        Self {
            status: value.into(),
            pricing: value.pricing().map(Into::into),
            configuration: value.configuration().map(|value| PdfConfiguration {
                max_files: value.max_files.map(f64::from),
                max_pages: value.max_pages.map(f64::from),
            }),
        }
    }
}

#[derive(GraphQLObject)]
pub struct AudioSupport {
    pub status: CapabilityStatus,
    pub pricing: Option<super::pricing::CapabilityPricing>,
    pub configuration: Option<AudioConfiguration>,
}
#[derive(GraphQLObject)]
pub struct AudioConfiguration {
    pub formats: Option<Vec<String>>,
    pub max_duration_ms: Option<f64>,
    pub max_bytes: Option<f64>,
}
impl From<&domain::CapabilitySupport<domain::AudioCapability>> for AudioSupport {
    fn from(value: &domain::CapabilitySupport<domain::AudioCapability>) -> Self {
        Self {
            status: value.into(),
            pricing: value.pricing().map(Into::into),
            configuration: value.configuration().map(|value| AudioConfiguration {
                formats: value.formats.clone(),
                max_duration_ms: value.max_duration_ms.map(f64::from),
                max_bytes: value.max_bytes.map(f64::from),
            }),
        }
    }
}

#[derive(GraphQLObject)]
pub struct VideoSupport {
    pub status: CapabilityStatus,
    pub pricing: Option<super::pricing::CapabilityPricing>,
    pub configuration: Option<VideoConfiguration>,
}
#[derive(GraphQLObject)]
pub struct VideoConfiguration {
    pub formats: Option<Vec<String>>,
    pub max_duration_ms: Option<f64>,
    pub max_frames: Option<f64>,
}
impl From<&domain::CapabilitySupport<domain::VideoCapability>> for VideoSupport {
    fn from(value: &domain::CapabilitySupport<domain::VideoCapability>) -> Self {
        Self {
            status: value.into(),
            pricing: value.pricing().map(Into::into),
            configuration: value.configuration().map(|value| VideoConfiguration {
                formats: value.formats.clone(),
                max_duration_ms: value.max_duration_ms.map(f64::from),
                max_frames: value.max_frames.map(f64::from),
            }),
        }
    }
}

#[derive(GraphQLObject)]
pub struct ImageGenerationSupport {
    pub status: CapabilityStatus,
    pub pricing: Option<super::pricing::CapabilityPricing>,
    pub configuration: Option<ImageGenerationConfiguration>,
}
#[derive(GraphQLObject)]
pub struct ImageGenerationConfiguration {
    pub formats: Option<Vec<String>>,
    pub max_images: Option<f64>,
}
impl From<&domain::CapabilitySupport<domain::ImageGenerationCapability>>
    for ImageGenerationSupport
{
    fn from(value: &domain::CapabilitySupport<domain::ImageGenerationCapability>) -> Self {
        Self {
            status: value.into(),
            pricing: value.pricing().map(Into::into),
            configuration: value
                .configuration()
                .map(|value| ImageGenerationConfiguration {
                    formats: value.formats.clone(),
                    max_images: value.max_images.map(f64::from),
                }),
        }
    }
}

#[derive(GraphQLObject)]
pub struct ToolCallingSupport {
    pub status: CapabilityStatus,
    pub pricing: Option<super::pricing::CapabilityPricing>,
    pub configuration: Option<ToolCallingConfiguration>,
}
#[derive(GraphQLObject)]
pub struct ToolCallingConfiguration {
    pub max_tools: Option<f64>,
}
impl From<&domain::CapabilitySupport<domain::ToolCallingCapability>> for ToolCallingSupport {
    fn from(value: &domain::CapabilitySupport<domain::ToolCallingCapability>) -> Self {
        Self {
            status: value.into(),
            pricing: value.pricing().map(Into::into),
            configuration: value.configuration().map(|value| ToolCallingConfiguration {
                max_tools: value.max_tools.map(f64::from),
            }),
        }
    }
}

#[derive(GraphQLObject)]
pub struct StructuredOutputSupport {
    pub status: CapabilityStatus,
    pub pricing: Option<super::pricing::CapabilityPricing>,
    pub configuration: Option<StructuredOutputConfiguration>,
}
#[derive(GraphQLObject)]
pub struct StructuredOutputConfiguration {
    pub json_mode: FeatureSupport,
    pub json_schema: FeatureSupport,
}
impl From<&domain::CapabilitySupport<domain::StructuredOutputCapability>>
    for StructuredOutputSupport
{
    fn from(value: &domain::CapabilitySupport<domain::StructuredOutputCapability>) -> Self {
        Self {
            status: value.into(),
            pricing: value.pricing().map(Into::into),
            configuration: value
                .configuration()
                .map(|value| StructuredOutputConfiguration {
                    json_mode: (&value.json_mode).into(),
                    json_schema: (&value.json_schema).into(),
                }),
        }
    }
}

#[derive(GraphQLObject)]
pub struct SearchSupport {
    pub status: CapabilityStatus,
    pub pricing: Option<super::pricing::CapabilityPricing>,
    pub configuration: Option<SearchConfiguration>,
}
#[derive(GraphQLObject)]
pub struct SearchConfiguration {
    pub max_results: Option<f64>,
}
impl From<&domain::CapabilitySupport<domain::SearchCapability>> for SearchSupport {
    fn from(value: &domain::CapabilitySupport<domain::SearchCapability>) -> Self {
        Self {
            status: value.into(),
            pricing: value.pricing().map(Into::into),
            configuration: value.configuration().map(|value| SearchConfiguration {
                max_results: value.max_results.map(f64::from),
            }),
        }
    }
}

#[derive(GraphQLObject)]
pub struct ThinkingSupport {
    pub status: CapabilityStatus,
    pub pricing: Option<super::pricing::CapabilityPricing>,
    pub configuration: Option<ThinkingConfiguration>,
}
#[derive(GraphQLObject)]
pub struct ThinkingConfiguration {
    pub effort_mapping: Vec<super::r#type::TemplateEffort>,
    pub default_effort: Option<super::r#type::TemplateEffortLevel>,
    pub budget: Option<ThinkingBudget>,
}
impl From<&domain::CapabilitySupport<domain::Thinking>> for ThinkingSupport {
    fn from(value: &domain::CapabilitySupport<domain::Thinking>) -> Self {
        Self {
            status: value.into(),
            pricing: value.pricing().map(Into::into),
            configuration: value.configuration().map(|value| ThinkingConfiguration {
                effort_mapping: value
                    .effort_mapping
                    .iter()
                    .map(|(level, value)| super::r#type::TemplateEffort {
                        level: (*level).into(),
                        value: value.clone(),
                    })
                    .collect(),
                default_effort: value.default_effort.map(Into::into),
                budget: value.budget.as_ref().map(Into::into),
            }),
        }
    }
}

#[derive(GraphQLObject)]
pub struct ThinkingBudget {
    pub min_tokens: f64,
    pub max_tokens: f64,
    pub default_tokens: Option<f64>,
}
impl From<&domain::ThinkingBudget> for ThinkingBudget {
    fn from(v: &domain::ThinkingBudget) -> Self {
        Self {
            min_tokens: f64::from(v.min_tokens),
            max_tokens: f64::from(v.max_tokens),
            default_tokens: v.default_tokens.map(f64::from),
        }
    }
}
#[derive(GraphQLObject)]
pub struct AudioCapabilities {
    pub input: AudioSupport,
    pub output: AudioSupport,
}
impl From<&domain::AudioCapabilities> for AudioCapabilities {
    fn from(v: &domain::AudioCapabilities) -> Self {
        Self {
            input: (&v.input).into(),
            output: (&v.output).into(),
        }
    }
}
#[derive(GraphQLObject)]
pub struct ToolCapabilities {
    pub calling: ToolCallingSupport,
    pub forced_choice: FeatureSupport,
    pub parallel_calls: FeatureSupport,
}
impl From<&domain::ToolCapabilities> for ToolCapabilities {
    fn from(v: &domain::ToolCapabilities) -> Self {
        Self {
            calling: (&v.calling).into(),
            forced_choice: (&v.forced_choice).into(),
            parallel_calls: (&v.parallel_calls).into(),
        }
    }
}

#[derive(GraphQLObject)]
pub struct TextCapabilities {
    pub input: TextInputSupport,
    pub output: TextGenerationSupport,
}
#[derive(GraphQLObject)]
pub struct TextInputSupport {
    pub status: CapabilityStatus,
    pub configuration: Option<TextInputConfiguration>,
    pub pricing: Option<super::pricing::CapabilityPricing>,
}
#[derive(GraphQLObject)]
pub struct TextInputConfiguration {
    pub max_tokens: Option<f64>,
}
#[derive(GraphQLObject)]
pub struct TextGenerationSupport {
    pub status: CapabilityStatus,
    pub configuration: Option<TextGenerationConfiguration>,
    pub pricing: Option<super::pricing::CapabilityPricing>,
}
#[derive(GraphQLObject)]
pub struct TextGenerationConfiguration {
    pub max_tokens: Option<f64>,
    pub thinking: ThinkingSupport,
}
impl From<&domain::TextCapabilities> for TextCapabilities {
    fn from(v: &domain::TextCapabilities) -> Self {
        Self {
            input: TextInputSupport {
                status: (&v.input).into(),
                configuration: v.input.configuration().map(|v| TextInputConfiguration {
                    max_tokens: v.max_tokens.map(f64::from),
                }),
                pricing: v.input.pricing().map(Into::into),
            },
            output: TextGenerationSupport {
                status: (&v.output).into(),
                configuration: v
                    .output
                    .configuration()
                    .map(|v| TextGenerationConfiguration {
                        max_tokens: v.max_tokens.map(f64::from),
                        thinking: (&v.thinking).into(),
                    }),
                pricing: v.output.pricing().map(Into::into),
            },
        }
    }
}
