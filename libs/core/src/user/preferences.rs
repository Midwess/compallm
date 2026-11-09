//! Personal preference values. Persistence and update authorization are separate.

use serde::{Deserialize, Serialize};

/// A reusable font choice whose supported values depend on its context.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Font<Family, Size> {
    pub family: Family,
    pub size: Size,
}

/// Light and dark palettes used by an appearance choice.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ColorPalette<Light, Dark> {
    pub light: Light,
    pub dark: Dark,
}

/// A visual mode and its context-specific palette choices.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Visual<Mode, Light, Dark> {
    pub mode: Mode,
    pub palettes: ColorPalette<Light, Dark>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HomeDestination {
    #[default]
    Chat,
    Models,
    Providers,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct NavigationPreferences {
    pub home: HomeDestination,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberNameFormat {
    #[default]
    FullName,
    Username,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MemberDisplayPreferences {
    pub name_format: MemberNameFormat,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirstDayOfWeek {
    #[default]
    Monday,
    Sunday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CalendarPreferences {
    pub first_day: FirstDayOfWeek,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceMode {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LightPalette {
    #[default]
    Light,
    PureLight,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DarkPalette {
    #[default]
    Dark,
    MagicBlue,
    ClassicDark,
}

pub type ThemePreferences = Visual<AppearanceMode, LightPalette, DarkPalette>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterfaceFontFamily {
    #[default]
    Default,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterfaceFontSize {
    Smaller,
    Small,
    #[default]
    Default,
    Large,
    Larger,
}

pub type TypographyPreferences = Font<InterfaceFontFamily, InterfaceFontSize>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AccessibilityPreferences {
    pub underline_links: bool,
    pub reduce_motion: bool,
    pub pointer_cursors: bool,
}

impl Default for AccessibilityPreferences {
    fn default() -> Self {
        Self {
            underline_links: false,
            reduce_motion: true,
            pointer_cursors: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SendShortcut {
    Enter,
    /// Command on macOS and Control on other platforms.
    #[default]
    ModifierEnter,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MessageCompositionPreferences {
    pub convert_emoticons: bool,
    pub send_shortcut: SendShortcut,
}

impl Default for MessageCompositionPreferences {
    fn default() -> Self {
        Self {
            convert_emoticons: true,
            send_shortcut: SendShortcut::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeTheme {
    #[default]
    FollowInterface,
    System,
    LinearLight,
    LinearDark,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeFontFamily {
    #[default]
    Default,
    Monospace,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeFontSize {
    #[default]
    Px12,
    Px13,
    Px14,
    Px16,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CodeAppearancePreferences {
    pub theme: CodeTheme,
    pub font: Font<CodeFontFamily, CodeFontSize>,
}

/// A personal sidebar item, independent of any organization.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SidebarItem {
    Providers,
    Models,
    Notebooks,
    Plugins,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SidebarVisibility {
    #[default]
    Always,
    WhenBadged,
    Hidden,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeStyle {
    #[default]
    Dot,
    Count,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SidebarEntry {
    pub item: SidebarItem,
    pub visibility: SidebarVisibility,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SidebarPreferences {
    pub badge_style: BadgeStyle,
    /// Entries are in display order.
    pub entries: Vec<SidebarEntry>,
}

impl Default for SidebarPreferences {
    fn default() -> Self {
        let items = [
            SidebarItem::Providers,
            SidebarItem::Models,
            SidebarItem::Notebooks,
            SidebarItem::Plugins,
        ];
        Self {
            badge_style: BadgeStyle::default(),
            entries: items
                .into_iter()
                .map(|item| SidebarEntry {
                    item,
                    visibility: SidebarVisibility::default(),
                })
                .collect(),
        }
    }
}

impl validify::Validate for SidebarPreferences {
    fn validate(&self) -> Result<(), validify::ValidationErrors> {
        let mut seen = std::collections::HashSet::new();
        if self.entries.iter().any(|entry| !seen.insert(entry.item)) {
            let mut errors = validify::ValidationErrors::new();
            errors.add(validify::ValidationError::new_schema(
                "duplicate_sidebar_item",
            ));
            return Err(errors);
        }
        Ok(())
    }
}

impl SidebarPreferences {
    /// Append newly introduced items in their default order without disturbing saved choices.
    pub fn append_missing_defaults(&mut self) {
        for entry in Self::default().entries {
            if !self
                .entries
                .iter()
                .any(|existing| existing.item == entry.item)
            {
                self.entries.push(entry);
            }
        }
    }
}

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
        validify::Validate::validate(&self.sidebar)
    }
}
