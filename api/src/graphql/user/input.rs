use compallm_validation_derive::ExportValidation;
use juniper::{GraphQLEnum, GraphQLInputObject};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validify::Validify;

#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProfileUpdateInput {
    #[modify(trim)]
    #[validate(length(min = 1))]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub name: Option<String>,
    #[modify(trim)]
    #[validate(length(min = 1))]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub username: Option<String>,
}

fn present<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(deserializer).map(Some)
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HomeDestination {
    Chat,
    Models,
    Providers,
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MemberDisplay {
    FullName,
    Username,
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InterfaceFontSize {
    Smaller,
    Small,
    Default,
    Large,
    Larger,
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppearanceMode {
    SystemPreference,
    Light,
    Dark,
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LightPalette {
    Light,
    PureLight,
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DarkPalette {
    Dark,
    MagicBlue,
    ClassicDark,
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SendShortcut {
    ModifierEnter,
    Enter,
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CodeTheme {
    LinearDark,
    LinearLight,
    SystemPreference,
    FollowInterface,
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CodeFont {
    Default,
    Monospace,
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CodeFontSize {
    Px12,
    Px13,
    Px14,
    Px16,
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SidebarVisibility {
    AlwaysShow,
    OnlyWhenBadged,
    Hidden,
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SidebarBadgeStyle {
    Dot,
    Count,
}

#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PreferencesUpdateInput {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub home: Option<HomeDestination>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub display_names: Option<MemberDisplay>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub first_day: Option<Weekday>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub font_size: Option<InterfaceFontSize>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub underline_links: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub reduce_motion: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub pointer_cursors: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub theme: Option<AppearanceMode>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub light_theme: Option<LightPalette>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub dark_theme: Option<DarkPalette>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub emoticons: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub send_shortcut: Option<SendShortcut>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub code_theme: Option<CodeTheme>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub code_font: Option<CodeFont>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub code_font_size: Option<CodeFontSize>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub sidebar_order: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub sidebar_badge_style: Option<SidebarBadgeStyle>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub sidebar_providers: Option<SidebarVisibility>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub sidebar_models: Option<SidebarVisibility>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub sidebar_notebooks: Option<SidebarVisibility>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(extend("x-compallm-non-null" = true))]
    pub sidebar_plugins: Option<SidebarVisibility>,
}

#[derive(Clone, Copy, Serialize, Deserialize, GraphQLEnum, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ImageMediaType {
    Jpeg,
    Png,
    Webp,
}

#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ImageUploadInput {
    pub media_type: ImageMediaType,
    #[validate(range(min = 1.0, max = 5242880.0))]
    pub size: i32,
}

#[derive(Serialize, Deserialize, GraphQLInputObject, Validify, ExportValidation)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CredentialCreateInput {
    #[modify(trim)]
    #[validate(length(min = 1))]
    pub name: String,
}
