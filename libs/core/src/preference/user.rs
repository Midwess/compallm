use super::{
    accessibility::AccessibilityPreferences, calendar::CalendarPreferences,
    code_appearance::CodeAppearancePreferences, member_display::MemberDisplayPreferences,
    message_composition::MessageCompositionPreferences, navigation::NavigationPreferences,
    sidebar::SidebarPreferences, theme::ThemePreferences, typography::TypographyPreferences,
};
use serde::{Deserialize, Serialize};

/// Preferences belonging to a single user; identity is supplied by the owner context.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UserPreferences {
    pub navigation: NavigationPreferences,
    pub sidebar: SidebarPreferences,
    pub member_display: MemberDisplayPreferences,
    pub calendar: CalendarPreferences,
    pub theme: ThemePreferences,
    pub typography: TypographyPreferences,
    pub accessibility: AccessibilityPreferences,
    pub message_composition: MessageCompositionPreferences,
    pub code_appearance: CodeAppearancePreferences,
}

impl validify::Validate for UserPreferences {
    fn validate(&self) -> Result<(), validify::ValidationErrors> {
        validify::Validate::validate(&self.navigation)?;
        validify::Validate::validate(&self.typography)?;
        validify::Validate::validate(&self.sidebar)
    }
}
