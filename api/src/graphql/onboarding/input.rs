use compallm_validation_derive::ExportValidation;
use juniper::{GraphQLEnum, GraphQLInputObject};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validify::Validify;
#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OnboardingDestination {
    Chat,
    Models,
    Providers,
}
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OnboardingLinkInput {
    #[validate(length(min = 1, max = 128))]
    pub label: String,
    #[validate(custom(function = compallm_utils::input::web_url), length(max = 2048))]
    pub href: String,
}
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OrganizationOnboardingSettingsInput {
    #[validate(length(max = 4096))]
    pub welcome: String,
    #[validate(length(max = 20))]
    #[validify]
    pub links: Vec<OnboardingLinkInput>,
    pub destination: OnboardingDestination,
    #[validate(custom(function = compallm_utils::input::hex_color))]
    pub artwork_color: Option<String>,
    pub require_full_name: bool,
    pub require_avatar: bool,
}
impl From<OrganizationOnboardingSettingsInput> for compallm_core::onboarding::Settings {
    fn from(v: OrganizationOnboardingSettingsInput) -> Self {
        Self {
            welcome: v.welcome,
            links: v.links.into_iter().map(Into::into).collect(),
            destination: v.destination.into(),
            artwork_color: v.artwork_color,
            require_full_name: v.require_full_name,
            require_avatar: v.require_avatar,
        }
    }
}
impl From<OnboardingLinkInput> for compallm_core::onboarding::UsefulLink {
    fn from(v: OnboardingLinkInput) -> Self {
        Self {
            label: v.label,
            href: v.href,
        }
    }
}
impl From<OnboardingDestination> for compallm_core::onboarding::Destination {
    fn from(v: OnboardingDestination) -> Self {
        match v {
            OnboardingDestination::Chat => Self::Chat,
            OnboardingDestination::Models => Self::Models,
            OnboardingDestination::Providers => Self::Providers,
        }
    }
}
impl From<compallm_core::onboarding::Destination> for OnboardingDestination {
    fn from(v: compallm_core::onboarding::Destination) -> Self {
        match v {
            compallm_core::onboarding::Destination::Chat => Self::Chat,
            compallm_core::onboarding::Destination::Models => Self::Models,
            compallm_core::onboarding::Destination::Providers => Self::Providers,
        }
    }
}
