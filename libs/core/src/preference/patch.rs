use super::{UserPreferences, values::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PalettePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightPalette>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark: Option<DarkPalette>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CodeFontPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<FontFamily>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<PixelFontSize>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NavigationPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home: Option<Destination>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MemberDisplayPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_format: Option<NamingFormat>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CalendarPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_day: Option<Weekday>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TypographyPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<FontFamily>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<RelativeFontSize>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AccessibilityPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline_links: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_motion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer_cursors: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MessageCompositionPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_emoticons: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_shortcut: Option<KeyCombination>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ThemePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<AppearanceMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub palettes: Option<PalettePatch>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CodeAppearancePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<CodeTheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<CodeFontPatch>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SidebarPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Vec<Destination>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<Vec<super::sidebar::SidebarSection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Vec<super::sidebar::SidebarEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_style: Option<BadgeStyle>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UserPreferencesPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigation: Option<NavigationPatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_display: Option<MemberDisplayPatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar: Option<CalendarPatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typography: Option<TypographyPatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<AccessibilityPatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_composition: Option<MessageCompositionPatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<ThemePatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_appearance: Option<CodeAppearancePatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sidebar: Option<SidebarPatch>,
}

impl UserPreferencesPatch {
    pub fn is_empty(&self) -> bool {
        self.navigation.as_ref().is_none_or(|p| p.home.is_none())
            && self
                .member_display
                .as_ref()
                .is_none_or(|p| p.name_format.is_none())
            && self.calendar.as_ref().is_none_or(|p| p.first_day.is_none())
            && self
                .typography
                .as_ref()
                .is_none_or(|p| p.family.is_none() && p.size.is_none())
            && self.accessibility.as_ref().is_none_or(|p| {
                p.underline_links.is_none()
                    && p.reduce_motion.is_none()
                    && p.pointer_cursors.is_none()
            })
            && self
                .message_composition
                .as_ref()
                .is_none_or(|p| p.convert_emoticons.is_none() && p.send_shortcut.is_none())
            && self.theme.as_ref().is_none_or(|p| {
                p.mode.is_none()
                    && p.palettes
                        .as_ref()
                        .is_none_or(|v| v.light.is_none() && v.dark.is_none())
            })
            && self.code_appearance.as_ref().is_none_or(|p| {
                p.theme.is_none()
                    && p.font
                        .as_ref()
                        .is_none_or(|v| v.family.is_none() && v.size.is_none())
            })
            && self.sidebar.as_ref().is_none_or(|p| {
                p.order.is_none()
                    && p.layout.is_none()
                    && p.badge_style.is_none()
                    && p.visibility.as_ref().is_none_or(Vec::is_empty)
            })
    }

    pub fn apply(
        &self,
        preferences: &mut UserPreferences,
    ) -> Result<(), validify::ValidationErrors> {
        if let Some(patch) = &self.navigation
            && let Some(value) = patch.home
        {
            preferences.navigation.home = value;
        }
        if let Some(patch) = &self.member_display
            && let Some(value) = patch.name_format
        {
            preferences.member_display.name_format = value;
        }
        if let Some(patch) = &self.calendar
            && let Some(value) = patch.first_day
        {
            preferences.calendar.first_day = value;
        }
        if let Some(patch) = &self.typography {
            if let Some(value) = patch.family {
                preferences.typography.family = value;
            }
            if let Some(value) = patch.size {
                preferences.typography.size = value;
            }
        }
        if let Some(patch) = &self.accessibility {
            if let Some(value) = patch.underline_links {
                preferences.accessibility.underline_links = value;
            }
            if let Some(value) = patch.reduce_motion {
                preferences.accessibility.reduce_motion = value;
            }
            if let Some(value) = patch.pointer_cursors {
                preferences.accessibility.pointer_cursors = value;
            }
        }
        if let Some(patch) = &self.message_composition {
            if let Some(value) = patch.convert_emoticons {
                preferences.message_composition.convert_emoticons = value;
            }
            if let Some(value) = patch.send_shortcut {
                preferences.message_composition.send_shortcut = value;
            }
        }
        if let Some(patch) = &self.theme {
            if let Some(value) = patch.mode {
                preferences.theme.mode = value;
            }
            if let Some(value) = &patch.palettes {
                if let Some(value) = value.light {
                    preferences.theme.palettes.light = value;
                }
                if let Some(value) = value.dark {
                    preferences.theme.palettes.dark = value;
                }
            }
        }
        if let Some(patch) = &self.code_appearance {
            if let Some(value) = patch.theme {
                preferences.code_appearance.theme = value;
            }
            if let Some(value) = &patch.font {
                if let Some(value) = value.family {
                    preferences.code_appearance.font.family = value;
                }
                if let Some(value) = value.size {
                    preferences.code_appearance.font.size = value;
                }
            }
        }
        if let Some(patch) = &self.sidebar {
            preferences.sidebar.append_missing_defaults();
            if let Some(order) = &patch.order {
                let mut seen = std::collections::HashSet::new();
                if order
                    .iter()
                    .any(|item| *item == Destination::Chat || !seen.insert(*item))
                {
                    return Err(invalid_patch());
                }
                preferences.sidebar.append_missing_defaults();
                let old = preferences.sidebar.entries.clone();
                preferences.sidebar.entries = order
                    .iter()
                    .map(|item| {
                        old.iter()
                            .find(|entry| entry.item == *item)
                            .expect("known destination")
                            .clone()
                    })
                    .collect();
                preferences
                    .sidebar
                    .entries
                    .extend(old.into_iter().filter(|entry| !seen.contains(&entry.item)));
            }
            if patch.order.is_some() {
                let positions: std::collections::HashMap<_, _> = preferences
                    .sidebar
                    .entries
                    .iter()
                    .enumerate()
                    .map(|(index, entry)| (entry.item, index))
                    .collect();
                for section in &mut preferences.sidebar.layout {
                    section.items.sort_by_key(|item| positions[item]);
                }
            }
            if let Some(layout) = &patch.layout {
                preferences.sidebar.replace_layout(layout.clone())?;
            }
            if let Some(entries) = &patch.visibility {
                let mut seen = std::collections::HashSet::new();
                preferences.sidebar.append_missing_defaults();
                for entry in entries {
                    if entry.item == Destination::Chat || !seen.insert(entry.item) {
                        return Err(invalid_patch());
                    }
                    preferences
                        .sidebar
                        .entries
                        .iter_mut()
                        .find(|saved| saved.item == entry.item)
                        .expect("known destination")
                        .visibility = entry.visibility;
                }
            }
            if let Some(value) = patch.badge_style {
                preferences.sidebar.badge_style = value;
            }
        }
        validify::Validate::validate(preferences)
    }
}

fn invalid_patch() -> validify::ValidationErrors {
    let mut errors = validify::ValidationErrors::new();
    errors.add(validify::ValidationError::new_schema(
        "invalid_sidebar_patch",
    ));
    errors
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UserPreferenceGroup {
    Navigation,
    MemberDisplay,
    Calendar,
    Typography,
    Accessibility,
    MessageComposition,
    Theme,
    CodeAppearance,
    Sidebar,
    All,
}
