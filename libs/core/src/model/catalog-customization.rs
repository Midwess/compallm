use super::ModelCatalog;
use crate::provider::*;
use validify::ValidationErrors;

#[derive(Default)]
pub struct CatalogCustomization {
    pub name: Option<String>,
    pub protocol: Option<Protocol>,
    pub path: Option<String>,
    pub limits: Option<(Option<u32>, Option<u32>, Option<u32>)>,
    pub input_price: Option<Option<rust_decimal::Decimal>>,
    pub output_price: Option<Option<rust_decimal::Decimal>>,
    pub cached_price: Option<Option<rust_decimal::Decimal>>,
    pub capabilities: Option<Vec<Capability>>,
    pub effort_mapping: Option<EffortMapping>,
    pub headers: Option<Headers>,
    pub body_defaults: Option<BodyDefaults>,
}

fn fixed(amount: rust_decimal::Decimal, cache: Option<CachePricing>) -> CapabilityPricing {
    CapabilityPricing::Usage(CapabilityUsagePricing::Fixed(CapabilityRates::Token(
        CapabilityTokenRate {
            rate: Rate {
                amount: Some(amount),
                currency: Some("USD".into()),
                quantity: Some(1_000_000),
            },
            cache,
        },
    )))
}
fn set_support<T: Default>(value: &mut CapabilitySupport<T>, enabled: bool) {
    if !enabled && value.is_supported() {
        *value = CapabilitySupport::Unsupported;
    } else if enabled && !value.is_supported() {
        *value = CapabilitySupport::supported(T::default(), None);
    }
}
impl CatalogCustomization {
    pub fn apply(
        self,
        model: &mut ModelCatalog,
        default: Option<&ModelCatalog>,
    ) -> Result<(), ValidationErrors> {
        for (field, enabled) in [
            (
                "contextLimitEnabled",
                self.limits.as_ref().map(|v| v.0.is_some()),
            ),
            (
                "inputLimitEnabled",
                self.limits.as_ref().map(|v| v.1.is_some()),
            ),
            (
                "outputLimitEnabled",
                self.limits.as_ref().map(|v| v.2.is_some()),
            ),
            (
                "inputPriceEnabled",
                self.input_price.as_ref().map(Option::is_some),
            ),
            (
                "outputPriceEnabled",
                self.output_price.as_ref().map(Option::is_some),
            ),
            (
                "cachedPriceEnabled",
                self.cached_price.as_ref().map(Option::is_some),
            ),
        ] {
            if let Some(enabled) = enabled {
                if enabled {
                    model.custom_fields.insert(field.into());
                } else {
                    model.custom_fields.remove(field);
                }
            }
        }
        if let Some(name) = self.name {
            model.name = name;
        }
        if let Some(path) = self.path {
            let protocol = self
                .protocol
                .ok_or_else(|| super::invalid_catalog("Choose a text operation."))?;
            model
                .connections
                .entry(protocol)
                .or_insert_with(|| Connection {
                    base_url: None,
                    path: None,
                    timeout_ms: None,
                    query: Default::default(),
                    authentication: None,
                    endpoint: EndpointRequirements {
                        mode: EndpointMode::Unknown,
                        url_template: None,
                        required_inputs: vec![],
                    },
                })
                .path = Some(path);
        }
        if let Some((context, input, output)) = self.limits {
            model.limits.context_tokens =
                context.or_else(|| default.and_then(|v| v.limits.context_tokens));
            if let CapabilitySupport::Supported { configuration, .. } = &mut model.text.input {
                configuration.max_tokens = input.or_else(|| {
                    default
                        .and_then(|v| v.text.input.configuration())
                        .and_then(|v| v.max_tokens)
                });
            }
            if let CapabilitySupport::Supported { configuration, .. } = &mut model.text.output {
                configuration.max_tokens = output.or_else(|| {
                    default
                        .and_then(|v| v.text.output.configuration())
                        .and_then(|v| v.max_tokens)
                });
            }
        }
        if let Some(value) = self.input_price
            && let CapabilitySupport::Supported { pricing, .. } = &mut model.text.input
        {
            *pricing = match value {
                Some(amount) => Some(fixed(
                    amount,
                    match pricing.as_ref() {
                        Some(CapabilityPricing::Usage(CapabilityUsagePricing::Fixed(
                            CapabilityRates::Token(v),
                        ))) => v.cache.clone(),
                        _ => None,
                    },
                )),
                None => default.and_then(|v| v.text.input.pricing()).cloned(),
            };
        }
        if let Some(value) = self.output_price
            && let CapabilitySupport::Supported { pricing, .. } = &mut model.text.output
        {
            *pricing = match value {
                Some(amount) => Some(fixed(amount, None)),
                None => default.and_then(|v| v.text.output.pricing()).cloned(),
            };
        }
        if let Some(value) = self.cached_price {
            if let CapabilitySupport::Supported {
                pricing:
                    Some(CapabilityPricing::Usage(CapabilityUsagePricing::Fixed(
                        CapabilityRates::Token(v),
                    ))),
                ..
            } = &mut model.text.input
            {
                v.cache = match value {
                    Some(amount) => Some(CachePricing::SeparateRate(CacheTokenRate {
                        rate: Rate {
                            amount: Some(amount),
                            currency: Some("USD".into()),
                            quantity: Some(1_000_000),
                        },
                    })),
                    None => match default.and_then(|v| v.text.input.pricing()) {
                        Some(CapabilityPricing::Usage(CapabilityUsagePricing::Fixed(
                            CapabilityRates::Token(v),
                        ))) => v.cache.clone(),
                        _ => None,
                    },
                };
            } else if value.is_some() {
                return Err(super::invalid_catalog(
                    "A custom cached-input price requires fixed input pricing.",
                ));
            }
        }
        if let Some(capabilities) = self.capabilities {
            if !capabilities.contains(&Capability::Text) {
                return Err(super::invalid_catalog(
                    "Text models require the text capability.",
                ));
            }
            set_support(
                &mut model.vision,
                capabilities.contains(&Capability::Vision),
            );
            set_support(&mut model.pdf, capabilities.contains(&Capability::Pdf));
            set_support(
                &mut model.audio.input,
                capabilities.contains(&Capability::Audio),
            );
            set_support(
                &mut model.audio.output,
                capabilities.contains(&Capability::AudioGeneration),
            );
            set_support(&mut model.video, capabilities.contains(&Capability::Video));
            set_support(
                &mut model.image_generation,
                capabilities.contains(&Capability::ImageGeneration),
            );
            set_support(
                &mut model.search,
                capabilities.contains(&Capability::Search),
            );
            set_support(
                &mut model.tools.calling,
                capabilities.contains(&Capability::ToolCall),
            );
            set_support(
                &mut model.tools.forced_choice,
                capabilities.contains(&Capability::ForcedToolChoice),
            );
            set_support(
                &mut model.structured_output,
                capabilities.contains(&Capability::StructuredOutput),
            );
            if let CapabilitySupport::Supported { configuration, .. } = &mut model.text.output {
                set_support(
                    &mut configuration.thinking,
                    capabilities.contains(&Capability::Reasoning),
                );
            }
        }
        if let Some(mapping) = self.effort_mapping
            && let CapabilitySupport::Supported { configuration, .. } = &mut model.text.output
        {
            if let CapabilitySupport::Supported {
                configuration: thinking,
                ..
            } = &mut configuration.thinking
            {
                thinking.effort_mapping = mapping;
                if thinking
                    .default_effort
                    .is_some_and(|v| !thinking.effort_mapping.contains_key(&v))
                {
                    thinking.default_effort = None;
                }
            } else if !mapping.is_empty() {
                return Err(super::invalid_catalog(
                    "Enable thinking before setting effort levels.",
                ));
            }
        }
        if let Some(headers) = self.headers {
            model.headers = headers;
        }
        if let Some(body) = self.body_defaults {
            model.body_defaults = body;
        }
        model.validate_configuration()
    }
}
impl TryFrom<(&ModelCatalog, Protocol)> for ProviderModel {
    type Error = ValidationErrors;
    fn try_from((template, protocol): (&ModelCatalog, Protocol)) -> Result<Self, Self::Error> {
        Ok(ProviderModel {
            id: 0,
            organization_id: 0,
            provider_id: 0,
            name: template.name.clone(),
            display_name: template.display_name.clone(),
            description: template.description.clone(),
            template: Some(ModelCatalogReference {
                provider_id: template.provider_catalog_id.clone(),
                model_id: template.id.clone(),
            }),
            enabled: true,
            service_kind: template
                .service_kind
                .ok_or_else(|| super::invalid_catalog("Choose a supported text model."))?,
            protocol,
            connection: template
                .connections
                .get(&protocol)
                .cloned()
                .ok_or_else(|| super::invalid_catalog("Choose a supported text operation."))?,
            headers: template.headers.clone(),
            body_defaults: template.body_defaults.clone(),
            text: template.text.clone(),
            vision: template.vision.clone(),
            pdf: template.pdf.clone(),
            audio: template.audio.clone(),
            video: template.video.clone(),
            image_generation: template.image_generation.clone(),
            tools: template.tools.clone(),
            structured_output: template.structured_output.clone(),
            search: template.search.clone(),
            limits: template.limits.clone(),
            billing: template.billing.clone(),
            route_selection: template.route_selection.clone(),
        })
    }
}
impl ModelCatalog {
    pub fn validate_configuration(&self) -> Result<(), ValidationErrors> {
        let context = self.limits.context_tokens;
        for (field, cap, message) in [
            (
                "input",
                self.text.input.configuration().and_then(|v| v.max_tokens),
                "Input limit cannot exceed the context window.",
            ),
            (
                "output",
                self.text.output.configuration().and_then(|v| v.max_tokens),
                "Output limit cannot exceed the context window.",
            ),
        ] {
            if context.zip(cap).is_some_and(|(context, cap)| cap > context) {
                return Err(super::invalid_catalog_field(field, message));
            }
        }
        if self.service_kind != Some(ServiceKind::Llm)
            || !self.text.input.is_supported()
            || !self.text.output.is_supported()
        {
            return Err(super::invalid_catalog("Choose a text-generation model."));
        }
        self.capability_fields().validate(self.billing.as_ref())?;
        Ok(())
    }
}
