use compallm_core::onboarding as core;
use juniper::{GraphQLEnum, GraphQLObject};
#[derive(GraphQLObject)]
pub struct OnboardingLink {
    pub label: String,
    pub href: String,
}
#[derive(GraphQLObject)]
pub struct OrganizationOnboardingSettings {
    pub welcome: String,
    pub links: Vec<OnboardingLink>,
    pub destination: super::input::OnboardingDestination,
    pub artwork_color: Option<String>,
    pub require_full_name: bool,
    pub require_avatar: bool,
}
impl From<core::Settings> for OrganizationOnboardingSettings {
    fn from(v: core::Settings) -> Self {
        Self {
            welcome: v.welcome,
            links: v
                .links
                .into_iter()
                .map(|l| OnboardingLink {
                    label: l.label,
                    href: l.href,
                })
                .collect(),
            destination: v.destination.into(),
            artwork_color: v.artwork_color,
            require_full_name: v.require_full_name,
            require_avatar: v.require_avatar,
        }
    }
}
#[derive(GraphQLEnum)]
pub enum OnboardingFlow {
    Personal,
    InvitedNew,
    InvitedExisting,
}
#[derive(GraphQLEnum)]
pub enum OnboardingStep {
    Profile,
    Workspace,
    Work,
    Preferences,
    Introduction,
    Ready,
}
#[derive(GraphQLObject)]
pub struct OnboardingContext {
    pub required: bool,
    pub flow: OnboardingFlow,
    pub step: OnboardingStep,
    pub settings: OrganizationOnboardingSettings,
    pub should_present: bool,
    pub has_been_presented: bool,
}
impl From<(core::Context, u64)> for OnboardingContext {
    fn from((v, session): (core::Context, u64)) -> Self {
        Self {
            required: v.required,
            has_been_presented: !v.progress.presented_session_ids.is_empty(),
            should_present: v.required && !v.progress.presented_session_ids.contains(&session),
            flow: match v.progress.flow {
                core::Flow::Personal => OnboardingFlow::Personal,
                core::Flow::InvitedNew => OnboardingFlow::InvitedNew,
                core::Flow::InvitedExisting => OnboardingFlow::InvitedExisting,
            },
            step: match v.progress.step {
                core::Step::Profile => OnboardingStep::Profile,
                core::Step::Workspace => OnboardingStep::Workspace,
                core::Step::Work => OnboardingStep::Work,
                core::Step::Preferences => OnboardingStep::Preferences,
                core::Step::Introduction => OnboardingStep::Introduction,
                core::Step::Ready => OnboardingStep::Ready,
            },
            settings: v.progress.settings.into(),
        }
    }
}
