use super::input::*;
use compallm_core::preference as core;
use juniper::graphql_object;
impl From<core::values::Weekday> for PreferenceWeekday {
    fn from(value: core::values::Weekday) -> Self {
        match value {
            core::values::Weekday::Monday => Self::Monday,
            core::values::Weekday::Sunday => Self::Sunday,
            core::values::Weekday::Tuesday => Self::Tuesday,
            core::values::Weekday::Wednesday => Self::Wednesday,
            core::values::Weekday::Thursday => Self::Thursday,
            core::values::Weekday::Friday => Self::Friday,
            core::values::Weekday::Saturday => Self::Saturday,
        }
    }
}
impl From<core::values::DarkPalette> for PreferenceDarkPalette {
    fn from(value: core::values::DarkPalette) -> Self {
        match value {
            core::values::DarkPalette::Dark => Self::Dark,
            core::values::DarkPalette::MagicBlue => Self::MagicBlue,
            core::values::DarkPalette::ClassicDark => Self::ClassicDark,
        }
    }
}
impl From<core::values::AppearanceMode> for PreferenceAppearanceMode {
    fn from(value: core::values::AppearanceMode) -> Self {
        match value {
            core::values::AppearanceMode::System => Self::System,
            core::values::AppearanceMode::Light => Self::Light,
            core::values::AppearanceMode::Dark => Self::Dark,
        }
    }
}
impl From<core::values::Destination> for PreferenceDestination {
    fn from(value: core::values::Destination) -> Self {
        match value {
            core::values::Destination::Chat => Self::Chat,
            core::values::Destination::Models => Self::Models,
            core::values::Destination::Providers => Self::Providers,
            core::values::Destination::Notebooks => Self::Notebooks,
            core::values::Destination::Plugins => Self::Plugins,
            core::values::Destination::Stats => Self::Stats,
            core::values::Destination::ApiKeys => Self::ApiKeys,
            core::values::Destination::Members => Self::Members,
            core::values::Destination::Conversations => Self::Conversations,
        }
    }
}
impl From<core::values::NamingFormat> for PreferenceNamingFormat {
    fn from(value: core::values::NamingFormat) -> Self {
        match value {
            core::values::NamingFormat::FullName => Self::FullName,
            core::values::NamingFormat::Username => Self::Username,
        }
    }
}
impl From<core::values::KeyCombination> for PreferenceKeyCombination {
    fn from(value: core::values::KeyCombination) -> Self {
        match value {
            core::values::KeyCombination::Enter => Self::Enter,
            core::values::KeyCombination::ModifierEnter => Self::ModifierEnter,
        }
    }
}
impl From<core::values::CodeTheme> for PreferenceCodeTheme {
    fn from(value: core::values::CodeTheme) -> Self {
        match value {
            core::values::CodeTheme::FollowInterface => Self::FollowInterface,
            core::values::CodeTheme::System => Self::System,
            core::values::CodeTheme::LinearLight => Self::LinearLight,
            core::values::CodeTheme::LinearDark => Self::LinearDark,
        }
    }
}
impl From<core::values::RelativeFontSize> for PreferenceRelativeFontSize {
    fn from(value: core::values::RelativeFontSize) -> Self {
        match value {
            core::values::RelativeFontSize::Smaller => Self::Smaller,
            core::values::RelativeFontSize::Small => Self::Small,
            core::values::RelativeFontSize::Default => Self::Default,
            core::values::RelativeFontSize::Large => Self::Large,
            core::values::RelativeFontSize::Larger => Self::Larger,
        }
    }
}
impl From<core::values::SidebarVisibility> for PreferenceSidebarVisibility {
    fn from(value: core::values::SidebarVisibility) -> Self {
        match value {
            core::values::SidebarVisibility::Always => Self::Always,
            core::values::SidebarVisibility::WhenBadged => Self::WhenBadged,
            core::values::SidebarVisibility::Hidden => Self::Hidden,
        }
    }
}
impl From<core::values::BadgeStyle> for PreferenceBadgeStyle {
    fn from(value: core::values::BadgeStyle) -> Self {
        match value {
            core::values::BadgeStyle::Dot => Self::Dot,
            core::values::BadgeStyle::Count => Self::Count,
        }
    }
}
impl From<core::values::FontFamily> for PreferenceFontFamily {
    fn from(value: core::values::FontFamily) -> Self {
        match value {
            core::values::FontFamily::Default => Self::Default,
            core::values::FontFamily::Monospace => Self::Monospace,
        }
    }
}
impl From<core::values::PixelFontSize> for PreferencePixelFontSize {
    fn from(value: core::values::PixelFontSize) -> Self {
        match value {
            core::values::PixelFontSize::Px12 => Self::Px12,
            core::values::PixelFontSize::Px13 => Self::Px13,
            core::values::PixelFontSize::Px14 => Self::Px14,
            core::values::PixelFontSize::Px16 => Self::Px16,
        }
    }
}
impl From<core::values::LightPalette> for PreferenceLightPalette {
    fn from(value: core::values::LightPalette) -> Self {
        match value {
            core::values::LightPalette::Light => Self::Light,
            core::values::LightPalette::PureLight => Self::PureLight,
        }
    }
}
pub struct NavigationPreferences(pub core::navigation::NavigationPreferences);
impl From<core::navigation::NavigationPreferences> for NavigationPreferences {
    fn from(value: core::navigation::NavigationPreferences) -> Self {
        Self(value)
    }
}
#[graphql_object]
impl NavigationPreferences {
    fn home(&self) -> PreferenceDestination {
        self.0.home.into()
    }
}
pub struct MemberDisplayPreferences(pub core::member_display::MemberDisplayPreferences);
impl From<core::member_display::MemberDisplayPreferences> for MemberDisplayPreferences {
    fn from(value: core::member_display::MemberDisplayPreferences) -> Self {
        Self(value)
    }
}
#[graphql_object]
impl MemberDisplayPreferences {
    fn name_format(&self) -> PreferenceNamingFormat {
        self.0.name_format.into()
    }
}
pub struct CalendarPreferences(pub core::calendar::CalendarPreferences);
impl From<core::calendar::CalendarPreferences> for CalendarPreferences {
    fn from(value: core::calendar::CalendarPreferences) -> Self {
        Self(value)
    }
}
#[graphql_object]
impl CalendarPreferences {
    fn first_day(&self) -> PreferenceWeekday {
        self.0.first_day.into()
    }
}
pub struct TypographyPreferences(pub core::typography::TypographyPreferences);
impl From<core::typography::TypographyPreferences> for TypographyPreferences {
    fn from(value: core::typography::TypographyPreferences) -> Self {
        Self(value)
    }
}
#[graphql_object]
impl TypographyPreferences {
    fn family(&self) -> PreferenceFontFamily {
        self.0.family.into()
    }
    fn size(&self) -> PreferenceRelativeFontSize {
        self.0.size.into()
    }
}
pub struct AccessibilityPreferences(pub core::accessibility::AccessibilityPreferences);
impl From<core::accessibility::AccessibilityPreferences> for AccessibilityPreferences {
    fn from(value: core::accessibility::AccessibilityPreferences) -> Self {
        Self(value)
    }
}
#[graphql_object]
impl AccessibilityPreferences {
    fn underline_links(&self) -> bool {
        self.0.underline_links
    }
    fn reduce_motion(&self) -> bool {
        self.0.reduce_motion
    }
    fn pointer_cursors(&self) -> bool {
        self.0.pointer_cursors
    }
}
pub struct MessageCompositionPreferences(
    pub core::message_composition::MessageCompositionPreferences,
);
impl From<core::message_composition::MessageCompositionPreferences>
    for MessageCompositionPreferences
{
    fn from(value: core::message_composition::MessageCompositionPreferences) -> Self {
        Self(value)
    }
}
#[graphql_object]
impl MessageCompositionPreferences {
    fn convert_emoticons(&self) -> bool {
        self.0.convert_emoticons
    }
    fn send_shortcut(&self) -> PreferenceKeyCombination {
        self.0.send_shortcut.into()
    }
}
pub struct PalettePreferences(
    pub core::values::ColorPalette<core::values::LightPalette, core::values::DarkPalette>,
);
impl From<core::values::ColorPalette<core::values::LightPalette, core::values::DarkPalette>>
    for PalettePreferences
{
    fn from(
        value: core::values::ColorPalette<core::values::LightPalette, core::values::DarkPalette>,
    ) -> Self {
        Self(value)
    }
}
#[graphql_object]
impl PalettePreferences {
    fn light(&self) -> PreferenceLightPalette {
        self.0.light.into()
    }
    fn dark(&self) -> PreferenceDarkPalette {
        self.0.dark.into()
    }
}
pub struct CodeFontPreferences(
    pub core::values::Font<core::values::FontFamily, core::values::PixelFontSize>,
);
impl From<core::values::Font<core::values::FontFamily, core::values::PixelFontSize>>
    for CodeFontPreferences
{
    fn from(
        value: core::values::Font<core::values::FontFamily, core::values::PixelFontSize>,
    ) -> Self {
        Self(value)
    }
}
#[graphql_object]
impl CodeFontPreferences {
    fn family(&self) -> PreferenceFontFamily {
        self.0.family.into()
    }
    fn size(&self) -> PreferencePixelFontSize {
        self.0.size.into()
    }
}
pub struct ThemePreferences(pub core::theme::ThemePreferences);
impl From<core::theme::ThemePreferences> for ThemePreferences {
    fn from(value: core::theme::ThemePreferences) -> Self {
        Self(value)
    }
}
#[graphql_object]
impl ThemePreferences {
    fn mode(&self) -> PreferenceAppearanceMode {
        self.0.mode.into()
    }
    fn palettes(&self) -> PalettePreferences {
        self.0.palettes.clone().into()
    }
}
pub struct CodeAppearancePreferences(pub core::code_appearance::CodeAppearancePreferences);
impl From<core::code_appearance::CodeAppearancePreferences> for CodeAppearancePreferences {
    fn from(value: core::code_appearance::CodeAppearancePreferences) -> Self {
        Self(value)
    }
}
#[graphql_object]
impl CodeAppearancePreferences {
    fn theme(&self) -> PreferenceCodeTheme {
        self.0.theme.into()
    }
    fn font(&self) -> CodeFontPreferences {
        self.0.font.clone().into()
    }
}
pub struct SidebarEntry(pub core::sidebar::SidebarEntry);
impl From<core::sidebar::SidebarEntry> for SidebarEntry {
    fn from(value: core::sidebar::SidebarEntry) -> Self {
        Self(value)
    }
}
#[graphql_object]
impl SidebarEntry {
    fn item(&self) -> PreferenceDestination {
        self.0.item.into()
    }
    fn visibility(&self) -> PreferenceSidebarVisibility {
        self.0.visibility.into()
    }
}
impl From<core::values::SidebarCategory> for PreferenceSidebarCategory {
    fn from(value: core::values::SidebarCategory) -> Self {
        match value {
            core::values::SidebarCategory::Workspace => Self::Workspace,
            core::values::SidebarCategory::Routers => Self::Routers,
            core::values::SidebarCategory::Organization => Self::Organization,
            core::values::SidebarCategory::Conversations => Self::Conversations,
        }
    }
}
pub struct SidebarLayoutSection(pub core::sidebar::SidebarSection);
impl From<core::sidebar::SidebarSection> for SidebarLayoutSection {
    fn from(value: core::sidebar::SidebarSection) -> Self {
        Self(value)
    }
}
#[graphql_object]
impl SidebarLayoutSection {
    fn category(&self) -> PreferenceSidebarCategory {
        self.0.category.into()
    }
    fn items(&self) -> Vec<PreferenceDestination> {
        self.0.items.iter().copied().map(Into::into).collect()
    }
}
pub struct SidebarPreferences(pub core::sidebar::SidebarPreferences);
impl From<core::sidebar::SidebarPreferences> for SidebarPreferences {
    fn from(value: core::sidebar::SidebarPreferences) -> Self {
        Self(value)
    }
}
#[graphql_object]
impl SidebarPreferences {
    fn layout(&self) -> Vec<SidebarLayoutSection> {
        self.0.layout.iter().cloned().map(Into::into).collect()
    }
    fn badge_style(&self) -> PreferenceBadgeStyle {
        self.0.badge_style.into()
    }
    fn entries(&self) -> Vec<SidebarEntry> {
        self.0.entries.iter().cloned().map(Into::into).collect()
    }
}
pub struct UserPreferences(pub core::UserPreferences);
impl From<core::UserPreferences> for UserPreferences {
    fn from(value: core::UserPreferences) -> Self {
        Self(value)
    }
}
#[graphql_object]
impl UserPreferences {
    fn navigation(&self) -> NavigationPreferences {
        self.0.navigation.clone().into()
    }
    fn sidebar(&self) -> SidebarPreferences {
        self.0.sidebar.clone().into()
    }
    fn member_display(&self) -> MemberDisplayPreferences {
        self.0.member_display.clone().into()
    }
    fn calendar(&self) -> CalendarPreferences {
        self.0.calendar.clone().into()
    }
    fn theme(&self) -> ThemePreferences {
        self.0.theme.clone().into()
    }
    fn typography(&self) -> TypographyPreferences {
        self.0.typography.clone().into()
    }
    fn accessibility(&self) -> AccessibilityPreferences {
        self.0.accessibility.clone().into()
    }
    fn message_composition(&self) -> MessageCompositionPreferences {
        self.0.message_composition.clone().into()
    }
    fn code_appearance(&self) -> CodeAppearancePreferences {
        self.0.code_appearance.clone().into()
    }
}
