use compallm_validation_derive::ExportValidation;
use juniper::{GraphQLEnum, GraphQLInputObject, Nullable};
use serde::{Deserialize, Serialize};
use validify::Validify;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, GraphQLEnum, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PreferenceWeekday {
    Monday,
    Sunday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
impl From<PreferenceWeekday> for compallm_core::preference::values::Weekday {
    fn from(value: PreferenceWeekday) -> Self {
        match value {
            PreferenceWeekday::Monday => Self::Monday,
            PreferenceWeekday::Sunday => Self::Sunday,
            PreferenceWeekday::Tuesday => Self::Tuesday,
            PreferenceWeekday::Wednesday => Self::Wednesday,
            PreferenceWeekday::Thursday => Self::Thursday,
            PreferenceWeekday::Friday => Self::Friday,
            PreferenceWeekday::Saturday => Self::Saturday,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, GraphQLEnum, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PreferenceDarkPalette {
    Dark,
    MagicBlue,
    ClassicDark,
}
impl From<PreferenceDarkPalette> for compallm_core::preference::values::DarkPalette {
    fn from(value: PreferenceDarkPalette) -> Self {
        match value {
            PreferenceDarkPalette::Dark => Self::Dark,
            PreferenceDarkPalette::MagicBlue => Self::MagicBlue,
            PreferenceDarkPalette::ClassicDark => Self::ClassicDark,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, GraphQLEnum, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PreferenceAppearanceMode {
    System,
    Light,
    Dark,
}
impl From<PreferenceAppearanceMode> for compallm_core::preference::values::AppearanceMode {
    fn from(value: PreferenceAppearanceMode) -> Self {
        match value {
            PreferenceAppearanceMode::System => Self::System,
            PreferenceAppearanceMode::Light => Self::Light,
            PreferenceAppearanceMode::Dark => Self::Dark,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, GraphQLEnum, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PreferenceDestination {
    Chat,
    Models,
    Providers,
    Notebooks,
    Plugins,
}
impl From<PreferenceDestination> for compallm_core::preference::values::Destination {
    fn from(value: PreferenceDestination) -> Self {
        match value {
            PreferenceDestination::Chat => Self::Chat,
            PreferenceDestination::Models => Self::Models,
            PreferenceDestination::Providers => Self::Providers,
            PreferenceDestination::Notebooks => Self::Notebooks,
            PreferenceDestination::Plugins => Self::Plugins,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, GraphQLEnum, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PreferenceNamingFormat {
    FullName,
    Username,
}
impl From<PreferenceNamingFormat> for compallm_core::preference::values::NamingFormat {
    fn from(value: PreferenceNamingFormat) -> Self {
        match value {
            PreferenceNamingFormat::FullName => Self::FullName,
            PreferenceNamingFormat::Username => Self::Username,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, GraphQLEnum, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PreferenceKeyCombination {
    Enter,
    ModifierEnter,
}
impl From<PreferenceKeyCombination> for compallm_core::preference::values::KeyCombination {
    fn from(value: PreferenceKeyCombination) -> Self {
        match value {
            PreferenceKeyCombination::Enter => Self::Enter,
            PreferenceKeyCombination::ModifierEnter => Self::ModifierEnter,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, GraphQLEnum, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PreferenceCodeTheme {
    FollowInterface,
    System,
    LinearLight,
    LinearDark,
}
impl From<PreferenceCodeTheme> for compallm_core::preference::values::CodeTheme {
    fn from(value: PreferenceCodeTheme) -> Self {
        match value {
            PreferenceCodeTheme::FollowInterface => Self::FollowInterface,
            PreferenceCodeTheme::System => Self::System,
            PreferenceCodeTheme::LinearLight => Self::LinearLight,
            PreferenceCodeTheme::LinearDark => Self::LinearDark,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, GraphQLEnum, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PreferenceRelativeFontSize {
    Smaller,
    Small,
    Default,
    Large,
    Larger,
}
impl From<PreferenceRelativeFontSize> for compallm_core::preference::values::RelativeFontSize {
    fn from(value: PreferenceRelativeFontSize) -> Self {
        match value {
            PreferenceRelativeFontSize::Smaller => Self::Smaller,
            PreferenceRelativeFontSize::Small => Self::Small,
            PreferenceRelativeFontSize::Default => Self::Default,
            PreferenceRelativeFontSize::Large => Self::Large,
            PreferenceRelativeFontSize::Larger => Self::Larger,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, GraphQLEnum, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PreferenceSidebarVisibility {
    Always,
    WhenBadged,
    Hidden,
}
impl From<PreferenceSidebarVisibility> for compallm_core::preference::values::SidebarVisibility {
    fn from(value: PreferenceSidebarVisibility) -> Self {
        match value {
            PreferenceSidebarVisibility::Always => Self::Always,
            PreferenceSidebarVisibility::WhenBadged => Self::WhenBadged,
            PreferenceSidebarVisibility::Hidden => Self::Hidden,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, GraphQLEnum, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PreferenceBadgeStyle {
    Dot,
    Count,
}
impl From<PreferenceBadgeStyle> for compallm_core::preference::values::BadgeStyle {
    fn from(value: PreferenceBadgeStyle) -> Self {
        match value {
            PreferenceBadgeStyle::Dot => Self::Dot,
            PreferenceBadgeStyle::Count => Self::Count,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, GraphQLEnum, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PreferenceFontFamily {
    Default,
    Monospace,
}
impl From<PreferenceFontFamily> for compallm_core::preference::values::FontFamily {
    fn from(value: PreferenceFontFamily) -> Self {
        match value {
            PreferenceFontFamily::Default => Self::Default,
            PreferenceFontFamily::Monospace => Self::Monospace,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, GraphQLEnum, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PreferencePixelFontSize {
    Px12,
    Px13,
    Px14,
    Px16,
}
impl From<PreferencePixelFontSize> for compallm_core::preference::values::PixelFontSize {
    fn from(value: PreferencePixelFontSize) -> Self {
        match value {
            PreferencePixelFontSize::Px12 => Self::Px12,
            PreferencePixelFontSize::Px13 => Self::Px13,
            PreferencePixelFontSize::Px14 => Self::Px14,
            PreferencePixelFontSize::Px16 => Self::Px16,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, GraphQLEnum, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PreferenceLightPalette {
    Light,
    PureLight,
}
impl From<PreferenceLightPalette> for compallm_core::preference::values::LightPalette {
    fn from(value: PreferenceLightPalette) -> Self {
        match value {
            PreferenceLightPalette::Light => Self::Light,
            PreferenceLightPalette::PureLight => Self::PureLight,
        }
    }
}
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SidebarVisibilityPatchInput {
    pub item: PreferenceDestination,
    pub visibility: PreferenceSidebarVisibility,
}
impl From<SidebarVisibilityPatchInput> for compallm_core::preference::sidebar::SidebarEntry {
    fn from(value: SidebarVisibilityPatchInput) -> Self {
        Self {
            item: value.item.into(),
            visibility: value.visibility.into(),
        }
    }
}

fn serialize_field<S: serde::Serializer, T: Serialize>(
    value: &Nullable<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Nullable::Some(value) => value.serialize(serializer),
        _ => serializer.serialize_none(),
    }
}
fn deserialize_field<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Nullable<T>, D::Error> {
    Option::<T>::deserialize(deserializer)
        .map(|value| value.map_or(Nullable::ExplicitNull, Nullable::Some))
}
fn supplied<T>(value: Nullable<T>) -> Result<Option<T>, crate::graphql::error::Error> {
    match value {
        Nullable::Some(value) => Ok(Some(value)),
        Nullable::ImplicitNull => Ok(None),
        Nullable::ExplicitNull => Err(crate::graphql::error::Error::invalid_input()),
    }
}
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PalettePatchInput {
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<PreferenceLightPalette>", extend("x-compallm-non-null" = true))]
    pub light: Nullable<PreferenceLightPalette>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<PreferenceDarkPalette>", extend("x-compallm-non-null" = true))]
    pub dark: Nullable<PreferenceDarkPalette>,
}
impl TryFrom<PalettePatchInput> for compallm_core::preference::patch::PalettePatch {
    type Error = crate::graphql::error::Error;
    fn try_from(value: PalettePatchInput) -> Result<Self, Self::Error> {
        Ok(Self {
            light: supplied(value.light)?.map(Into::into),
            dark: supplied(value.dark)?.map(Into::into),
        })
    }
}
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CodeFontPatchInput {
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<PreferenceFontFamily>", extend("x-compallm-non-null" = true))]
    pub family: Nullable<PreferenceFontFamily>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<PreferencePixelFontSize>", extend("x-compallm-non-null" = true))]
    pub size: Nullable<PreferencePixelFontSize>,
}
impl TryFrom<CodeFontPatchInput> for compallm_core::preference::patch::CodeFontPatch {
    type Error = crate::graphql::error::Error;
    fn try_from(value: CodeFontPatchInput) -> Result<Self, Self::Error> {
        Ok(Self {
            family: supplied(value.family)?.map(Into::into),
            size: supplied(value.size)?.map(Into::into),
        })
    }
}
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NavigationPatchInput {
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<PreferenceDestination>", extend("x-compallm-non-null" = true))]
    pub home: Nullable<PreferenceDestination>,
}
impl TryFrom<NavigationPatchInput> for compallm_core::preference::patch::NavigationPatch {
    type Error = crate::graphql::error::Error;
    fn try_from(value: NavigationPatchInput) -> Result<Self, Self::Error> {
        Ok(Self {
            home: supplied(value.home)?.map(Into::into),
        })
    }
}
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MemberDisplayPatchInput {
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<PreferenceNamingFormat>", extend("x-compallm-non-null" = true))]
    pub name_format: Nullable<PreferenceNamingFormat>,
}
impl TryFrom<MemberDisplayPatchInput> for compallm_core::preference::patch::MemberDisplayPatch {
    type Error = crate::graphql::error::Error;
    fn try_from(value: MemberDisplayPatchInput) -> Result<Self, Self::Error> {
        Ok(Self {
            name_format: supplied(value.name_format)?.map(Into::into),
        })
    }
}
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CalendarPatchInput {
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<PreferenceWeekday>", extend("x-compallm-non-null" = true))]
    pub first_day: Nullable<PreferenceWeekday>,
}
impl TryFrom<CalendarPatchInput> for compallm_core::preference::patch::CalendarPatch {
    type Error = crate::graphql::error::Error;
    fn try_from(value: CalendarPatchInput) -> Result<Self, Self::Error> {
        Ok(Self {
            first_day: supplied(value.first_day)?.map(Into::into),
        })
    }
}
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TypographyPatchInput {
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<PreferenceFontFamily>", extend("x-compallm-non-null" = true))]
    pub family: Nullable<PreferenceFontFamily>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<PreferenceRelativeFontSize>", extend("x-compallm-non-null" = true))]
    pub size: Nullable<PreferenceRelativeFontSize>,
}
impl TryFrom<TypographyPatchInput> for compallm_core::preference::patch::TypographyPatch {
    type Error = crate::graphql::error::Error;
    fn try_from(value: TypographyPatchInput) -> Result<Self, Self::Error> {
        Ok(Self {
            family: supplied(value.family)?.map(Into::into),
            size: supplied(value.size)?.map(Into::into),
        })
    }
}
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AccessibilityPatchInput {
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<bool>", extend("x-compallm-non-null" = true))]
    pub underline_links: Nullable<bool>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<bool>", extend("x-compallm-non-null" = true))]
    pub reduce_motion: Nullable<bool>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<bool>", extend("x-compallm-non-null" = true))]
    pub pointer_cursors: Nullable<bool>,
}
impl TryFrom<AccessibilityPatchInput> for compallm_core::preference::patch::AccessibilityPatch {
    type Error = crate::graphql::error::Error;
    fn try_from(value: AccessibilityPatchInput) -> Result<Self, Self::Error> {
        Ok(Self {
            underline_links: supplied(value.underline_links)?,
            reduce_motion: supplied(value.reduce_motion)?,
            pointer_cursors: supplied(value.pointer_cursors)?,
        })
    }
}
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MessageCompositionPatchInput {
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<bool>", extend("x-compallm-non-null" = true))]
    pub convert_emoticons: Nullable<bool>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<PreferenceKeyCombination>", extend("x-compallm-non-null" = true))]
    pub send_shortcut: Nullable<PreferenceKeyCombination>,
}
impl TryFrom<MessageCompositionPatchInput>
    for compallm_core::preference::patch::MessageCompositionPatch
{
    type Error = crate::graphql::error::Error;
    fn try_from(value: MessageCompositionPatchInput) -> Result<Self, Self::Error> {
        Ok(Self {
            convert_emoticons: supplied(value.convert_emoticons)?,
            send_shortcut: supplied(value.send_shortcut)?.map(Into::into),
        })
    }
}
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ThemePatchInput {
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<PreferenceAppearanceMode>", extend("x-compallm-non-null" = true))]
    pub mode: Nullable<PreferenceAppearanceMode>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<PalettePatchInput>", extend("x-compallm-non-null" = true))]
    pub palettes: Nullable<PalettePatchInput>,
}
impl TryFrom<ThemePatchInput> for compallm_core::preference::patch::ThemePatch {
    type Error = crate::graphql::error::Error;
    fn try_from(value: ThemePatchInput) -> Result<Self, Self::Error> {
        Ok(Self {
            mode: supplied(value.mode)?.map(Into::into),
            palettes: supplied(value.palettes)?
                .map(TryInto::try_into)
                .transpose()?,
        })
    }
}
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CodeAppearancePatchInput {
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<PreferenceCodeTheme>", extend("x-compallm-non-null" = true))]
    pub theme: Nullable<PreferenceCodeTheme>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<CodeFontPatchInput>", extend("x-compallm-non-null" = true))]
    pub font: Nullable<CodeFontPatchInput>,
}
impl TryFrom<CodeAppearancePatchInput> for compallm_core::preference::patch::CodeAppearancePatch {
    type Error = crate::graphql::error::Error;
    fn try_from(value: CodeAppearancePatchInput) -> Result<Self, Self::Error> {
        Ok(Self {
            theme: supplied(value.theme)?.map(Into::into),
            font: supplied(value.font)?.map(TryInto::try_into).transpose()?,
        })
    }
}
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
/// Order replaces ordering while preserving visibility; visibility patches are keyed by destination.
pub struct SidebarPatchInput {
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<Vec<PreferenceDestination>>", extend("x-compallm-non-null" = true))]
    pub order: Nullable<Vec<PreferenceDestination>>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<Vec<SidebarVisibilityPatchInput>>", extend("x-compallm-non-null" = true))]
    pub visibility: Nullable<Vec<SidebarVisibilityPatchInput>>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<PreferenceBadgeStyle>", extend("x-compallm-non-null" = true))]
    pub badge_style: Nullable<PreferenceBadgeStyle>,
}
impl TryFrom<SidebarPatchInput> for compallm_core::preference::patch::SidebarPatch {
    type Error = crate::graphql::error::Error;
    fn try_from(value: SidebarPatchInput) -> Result<Self, Self::Error> {
        Ok(Self {
            order: supplied(value.order)?
                .map(|values| values.into_iter().map(Into::into).collect()),
            visibility: supplied(value.visibility)?
                .map(|values| values.into_iter().map(Into::into).collect()),
            badge_style: supplied(value.badge_style)?.map(Into::into),
        })
    }
}
#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
/// Partial personal preferences. Omitted fields are preserved; explicit null is rejected.
pub struct UserPreferencesPatchInput {
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<NavigationPatchInput>", extend("x-compallm-non-null" = true))]
    pub navigation: Nullable<NavigationPatchInput>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<SidebarPatchInput>", extend("x-compallm-non-null" = true))]
    pub sidebar: Nullable<SidebarPatchInput>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<MemberDisplayPatchInput>", extend("x-compallm-non-null" = true))]
    pub member_display: Nullable<MemberDisplayPatchInput>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<CalendarPatchInput>", extend("x-compallm-non-null" = true))]
    pub calendar: Nullable<CalendarPatchInput>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<ThemePatchInput>", extend("x-compallm-non-null" = true))]
    pub theme: Nullable<ThemePatchInput>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<TypographyPatchInput>", extend("x-compallm-non-null" = true))]
    pub typography: Nullable<TypographyPatchInput>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<AccessibilityPatchInput>", extend("x-compallm-non-null" = true))]
    pub accessibility: Nullable<AccessibilityPatchInput>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<MessageCompositionPatchInput>", extend("x-compallm-non-null" = true))]
    pub message_composition: Nullable<MessageCompositionPatchInput>,
    #[serde(
        default,
        skip_serializing_if = "Nullable::is_implicit_null",
        serialize_with = "serialize_field",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Option<CodeAppearancePatchInput>", extend("x-compallm-non-null" = true))]
    pub code_appearance: Nullable<CodeAppearancePatchInput>,
}
impl TryFrom<UserPreferencesPatchInput> for compallm_core::preference::patch::UserPreferencesPatch {
    type Error = crate::graphql::error::Error;
    fn try_from(value: UserPreferencesPatchInput) -> Result<Self, Self::Error> {
        Ok(Self {
            navigation: supplied(value.navigation)?
                .map(TryInto::try_into)
                .transpose()?,
            sidebar: supplied(value.sidebar)?
                .map(TryInto::try_into)
                .transpose()?,
            member_display: supplied(value.member_display)?
                .map(TryInto::try_into)
                .transpose()?,
            calendar: supplied(value.calendar)?
                .map(TryInto::try_into)
                .transpose()?,
            theme: supplied(value.theme)?.map(TryInto::try_into).transpose()?,
            typography: supplied(value.typography)?
                .map(TryInto::try_into)
                .transpose()?,
            accessibility: supplied(value.accessibility)?
                .map(TryInto::try_into)
                .transpose()?,
            message_composition: supplied(value.message_composition)?
                .map(TryInto::try_into)
                .transpose()?,
            code_appearance: supplied(value.code_appearance)?
                .map(TryInto::try_into)
                .transpose()?,
        })
    }
}
#[derive(Clone, Copy, GraphQLEnum)]
pub enum UserPreferenceGroup {
    Navigation,
    Sidebar,
    MemberDisplay,
    Calendar,
    Theme,
    Typography,
    Accessibility,
    MessageComposition,
    CodeAppearance,
    All,
}
impl From<UserPreferenceGroup> for compallm_core::preference::patch::UserPreferenceGroup {
    fn from(value: UserPreferenceGroup) -> Self {
        match value {
            UserPreferenceGroup::Navigation => Self::Navigation,
            UserPreferenceGroup::Sidebar => Self::Sidebar,
            UserPreferenceGroup::MemberDisplay => Self::MemberDisplay,
            UserPreferenceGroup::Calendar => Self::Calendar,
            UserPreferenceGroup::Theme => Self::Theme,
            UserPreferenceGroup::Typography => Self::Typography,
            UserPreferenceGroup::Accessibility => Self::Accessibility,
            UserPreferenceGroup::MessageComposition => Self::MessageComposition,
            UserPreferenceGroup::CodeAppearance => Self::CodeAppearance,
            UserPreferenceGroup::All => Self::All,
        }
    }
}
