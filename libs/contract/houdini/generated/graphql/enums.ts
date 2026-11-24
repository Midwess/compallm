type ValuesOf<T> = T[keyof T]

export const AuthenticationMethod = {
    "API_KEY": "API_KEY",
    "CUSTOM": "CUSTOM",
    "NONE": "NONE",
    "OAUTH": "OAUTH"
} as const;

export type AuthenticationMethod$options = ValuesOf<typeof AuthenticationMethod>

export const AuthenticationPurpose = {
    "LOGIN": "LOGIN",
    "PASSKEY_CHANGE": "PASSKEY_CHANGE"
} as const;

export type AuthenticationPurpose$options = ValuesOf<typeof AuthenticationPurpose>

export const BillingModel = {
    "FREE": "FREE",
    "METERED": "METERED",
    "SUBSCRIPTION": "SUBSCRIPTION",
    "SUBSCRIPTION_WITH_OVERAGE": "SUBSCRIPTION_WITH_OVERAGE",
    "UNKNOWN": "UNKNOWN"
} as const;

export type BillingModel$options = ValuesOf<typeof BillingModel>

export const CacheBilling = {
    "INCLUDED": "INCLUDED",
    "NOT_APPLICABLE": "NOT_APPLICABLE",
    "PER_ROUTE": "PER_ROUTE",
    "SEPARATE_RATE": "SEPARATE_RATE",
    "STANDARD_INPUT_RATE": "STANDARD_INPUT_RATE",
    "UNKNOWN": "UNKNOWN",
    "UNSUPPORTED": "UNSUPPORTED"
} as const;

export type CacheBilling$options = ValuesOf<typeof CacheBilling>

export const CapabilityCacheMode = {
    "INCLUDED": "INCLUDED",
    "SEPARATE_RATE": "SEPARATE_RATE",
    "STANDARD_INPUT_RATE": "STANDARD_INPUT_RATE",
    "UNSUPPORTED": "UNSUPPORTED"
} as const;

export type CapabilityCacheMode$options = ValuesOf<typeof CapabilityCacheMode>

export const CapabilityPriceMode = {
    "FIXED": "FIXED",
    "SCHEDULED": "SCHEDULED",
    "TIERED_TOKEN": "TIERED_TOKEN"
} as const;

export type CapabilityPriceMode$options = ValuesOf<typeof CapabilityPriceMode>

export const CapabilityPricingKind = {
    "FREE": "FREE",
    "INCLUDED": "INCLUDED",
    "MODEL_BILLING": "MODEL_BILLING",
    "USAGE": "USAGE"
} as const;

export type CapabilityPricingKind$options = ValuesOf<typeof CapabilityPricingKind>

export const CapabilityStatus = {
    "SUPPORTED": "SUPPORTED",
    "UNKNOWN": "UNKNOWN",
    "UNSUPPORTED": "UNSUPPORTED"
} as const;

export type CapabilityStatus$options = ValuesOf<typeof CapabilityStatus>

export const ConfigurationInputKind = {
    "OBJECT": "OBJECT",
    "POSITIVE_INTEGER": "POSITIVE_INTEGER",
    "PRICE": "PRICE",
    "STRING": "STRING",
    "URL": "URL"
} as const;

export type ConfigurationInputKind$options = ValuesOf<typeof ConfigurationInputKind>

/** How the request delivers credentials; this is never stored in signed claims. */
export const CredentialDelivery = {
    /**
     * Set HttpOnly cookies and return metadata without credential values.
    */
    "COOKIE": "COOKIE",
    /**
     * Return credentials explicitly without setting browser cookies.
    */
    "TOKEN": "TOKEN"
} as const;

export type CredentialDelivery$options = ValuesOf<typeof CredentialDelivery>

export const CredentialLocation = {
    "HEADER": "HEADER",
    "QUERY": "QUERY"
} as const;

export type CredentialLocation$options = ValuesOf<typeof CredentialLocation>

export const CreditUsageDirection = {
    "CACHE_READ": "CACHE_READ",
    "INPUT": "INPUT",
    "OUTPUT": "OUTPUT"
} as const;

export type CreditUsageDirection$options = ValuesOf<typeof CreditUsageDirection>

export const EndpointMode = {
    "ACCOUNT_SPECIFIC": "ACCOUNT_SPECIFIC",
    "DISCOVERED": "DISCOVERED",
    "FIXED": "FIXED",
    "UNKNOWN": "UNKNOWN"
} as const;

export type EndpointMode$options = ValuesOf<typeof EndpointMode>

export const FilterIs = {
    "NOT_NULL": "NOT_NULL",
    "NULL": "NULL"
} as const;

export type FilterIs$options = ValuesOf<typeof FilterIs>

export const HomeDestination = {
    "CHAT": "CHAT",
    "MODELS": "MODELS",
    "PROVIDERS": "PROVIDERS"
} as const;

export type HomeDestination$options = ValuesOf<typeof HomeDestination>

export const ImageMediaType = {
    "JPEG": "JPEG",
    "PNG": "PNG",
    "WEBP": "WEBP"
} as const;

export type ImageMediaType$options = ValuesOf<typeof ImageMediaType>

export const ModelBillingKind = {
    "CREDIT": "CREDIT",
    "FREE": "FREE",
    "REQUEST": "REQUEST",
    "SUBSCRIPTION": "SUBSCRIPTION"
} as const;

export type ModelBillingKind$options = ValuesOf<typeof ModelBillingKind>

export const ModelServiceKind = {
    "AUDIO": "AUDIO",
    "EMBEDDING": "EMBEDDING",
    "IMAGE": "IMAGE",
    "RERANK": "RERANK",
    "STT": "STT",
    "TEXT": "TEXT",
    "TTS": "TTS",
    "VIDEO": "VIDEO"
} as const;

export type ModelServiceKind$options = ValuesOf<typeof ModelServiceKind>

export const ModelTemplateOrigin = {
    "CUSTOM": "CUSTOM",
    "GLOBAL": "GLOBAL",
    "OVERRIDE": "OVERRIDE"
} as const;

export type ModelTemplateOrigin$options = ValuesOf<typeof ModelTemplateOrigin>

export const OAuthFlow = {
    "AUTHORIZATION_CODE": "AUTHORIZATION_CODE",
    "CLIENT_CREDENTIALS": "CLIENT_CREDENTIALS",
    "DEVICE_CODE": "DEVICE_CODE"
} as const;

export type OAuthFlow$options = ValuesOf<typeof OAuthFlow>

export const OnboardingDestination = {
    "CHAT": "CHAT",
    "MODELS": "MODELS",
    "PROVIDERS": "PROVIDERS"
} as const;

export type OnboardingDestination$options = ValuesOf<typeof OnboardingDestination>

export const OnboardingFlow = {
    "INVITED_EXISTING": "INVITED_EXISTING",
    "INVITED_NEW": "INVITED_NEW",
    "PERSONAL": "PERSONAL"
} as const;

export type OnboardingFlow$options = ValuesOf<typeof OnboardingFlow>

export const OnboardingStep = {
    "INTRODUCTION": "INTRODUCTION",
    "PREFERENCES": "PREFERENCES",
    "PROFILE": "PROFILE",
    "READY": "READY",
    "WORK": "WORK",
    "WORKSPACE": "WORKSPACE"
} as const;

export type OnboardingStep$options = ValuesOf<typeof OnboardingStep>

export const OrderByDirection = {
    "AscNullsFirst": "AscNullsFirst",
    "AscNullsLast": "AscNullsLast",
    "DescNullsFirst": "DescNullsFirst",
    "DescNullsLast": "DescNullsLast"
} as const;

export type OrderByDirection$options = ValuesOf<typeof OrderByDirection>

export const PreferenceAppearanceMode = {
    "DARK": "DARK",
    "LIGHT": "LIGHT",
    "SYSTEM": "SYSTEM"
} as const;

export type PreferenceAppearanceMode$options = ValuesOf<typeof PreferenceAppearanceMode>

export const PreferenceBadgeStyle = {
    "COUNT": "COUNT",
    "DOT": "DOT"
} as const;

export type PreferenceBadgeStyle$options = ValuesOf<typeof PreferenceBadgeStyle>

export const PreferenceCodeTheme = {
    "FOLLOW_INTERFACE": "FOLLOW_INTERFACE",
    "LINEAR_DARK": "LINEAR_DARK",
    "LINEAR_LIGHT": "LINEAR_LIGHT",
    "SYSTEM": "SYSTEM"
} as const;

export type PreferenceCodeTheme$options = ValuesOf<typeof PreferenceCodeTheme>

export const PreferenceDarkPalette = {
    "BASIC_DARK": "BASIC_DARK",
    "BLUEBERRY_DARK": "BLUEBERRY_DARK",
    "MARSHMALLOW_DARK": "MARSHMALLOW_DARK"
} as const;

export type PreferenceDarkPalette$options = ValuesOf<typeof PreferenceDarkPalette>

export const PreferenceDestination = {
    "API_KEYS": "API_KEYS",
    "CHAT": "CHAT",
    "CONVERSATIONS": "CONVERSATIONS",
    "MEMBERS": "MEMBERS",
    "MODELS": "MODELS",
    "NOTEBOOKS": "NOTEBOOKS",
    "PLUGINS": "PLUGINS",
    "PROVIDERS": "PROVIDERS",
    "STATS": "STATS"
} as const;

export type PreferenceDestination$options = ValuesOf<typeof PreferenceDestination>

export const PreferenceFontFamily = {
    "DEFAULT": "DEFAULT",
    "MONOSPACE": "MONOSPACE"
} as const;

export type PreferenceFontFamily$options = ValuesOf<typeof PreferenceFontFamily>

export const PreferenceKeyCombination = {
    "ENTER": "ENTER",
    "MODIFIER_ENTER": "MODIFIER_ENTER"
} as const;

export type PreferenceKeyCombination$options = ValuesOf<typeof PreferenceKeyCombination>

export const PreferenceLightPalette = {
    "BASIC_WHITE": "BASIC_WHITE",
    "BLUEBERRY_WHITE": "BLUEBERRY_WHITE",
    "MARSHMALLOW_WHITE": "MARSHMALLOW_WHITE"
} as const;

export type PreferenceLightPalette$options = ValuesOf<typeof PreferenceLightPalette>

export const PreferenceNamingFormat = {
    "FULL_NAME": "FULL_NAME",
    "USERNAME": "USERNAME"
} as const;

export type PreferenceNamingFormat$options = ValuesOf<typeof PreferenceNamingFormat>

export const PreferencePixelFontSize = {
    "PX12": "PX12",
    "PX13": "PX13",
    "PX14": "PX14",
    "PX16": "PX16"
} as const;

export type PreferencePixelFontSize$options = ValuesOf<typeof PreferencePixelFontSize>

export const PreferenceRelativeFontSize = {
    "DEFAULT": "DEFAULT",
    "LARGE": "LARGE",
    "LARGER": "LARGER",
    "SMALL": "SMALL",
    "SMALLER": "SMALLER"
} as const;

export type PreferenceRelativeFontSize$options = ValuesOf<typeof PreferenceRelativeFontSize>

export const PreferenceSidebarCategory = {
    "CONVERSATIONS": "CONVERSATIONS",
    "ORGANIZATION": "ORGANIZATION",
    "ROUTERS": "ROUTERS",
    "WORKSPACE": "WORKSPACE"
} as const;

export type PreferenceSidebarCategory$options = ValuesOf<typeof PreferenceSidebarCategory>

export const PreferenceSidebarVisibility = {
    "ALWAYS": "ALWAYS",
    "HIDDEN": "HIDDEN",
    "WHEN_BADGED": "WHEN_BADGED"
} as const;

export type PreferenceSidebarVisibility$options = ValuesOf<typeof PreferenceSidebarVisibility>

export const PreferenceWeekday = {
    "FRIDAY": "FRIDAY",
    "MONDAY": "MONDAY",
    "SATURDAY": "SATURDAY",
    "SUNDAY": "SUNDAY",
    "THURSDAY": "THURSDAY",
    "TUESDAY": "TUESDAY",
    "WEDNESDAY": "WEDNESDAY"
} as const;

export type PreferenceWeekday$options = ValuesOf<typeof PreferenceWeekday>

export const PriceBillingPeriod = {
    "DAY": "DAY",
    "HOUR": "HOUR",
    "MINUTE": "MINUTE",
    "MONTH": "MONTH",
    "WEEK": "WEEK",
    "YEAR": "YEAR"
} as const;

export type PriceBillingPeriod$options = ValuesOf<typeof PriceBillingPeriod>

export const PriceTierBasis = {
    "INPUT_TOKENS": "INPUT_TOKENS",
    "TOTAL_TOKENS": "TOTAL_TOKENS"
} as const;

export type PriceTierBasis$options = ValuesOf<typeof PriceTierBasis>

export const PriceUnit = {
    "CHARACTERS": "CHARACTERS",
    "CREDITS": "CREDITS",
    "IMAGES": "IMAGES",
    "REQUESTS": "REQUESTS",
    "SECONDS": "SECONDS",
    "TOKENS": "TOKENS",
    "UTF8_BYTES": "UTF8_BYTES",
    "VIDEOS": "VIDEOS"
} as const;

export type PriceUnit$options = ValuesOf<typeof PriceUnit>

export const PricingMode = {
    "ACCOUNT_SPECIFIC": "ACCOUNT_SPECIFIC",
    "API": "API",
    "FIXED": "FIXED",
    "NOT_APPLICABLE": "NOT_APPLICABLE",
    "SCHEDULED": "SCHEDULED",
    "TIERED": "TIERED",
    "UNKNOWN": "UNKNOWN",
    "UNPUBLISHED": "UNPUBLISHED"
} as const;

export type PricingMode$options = ValuesOf<typeof PricingMode>

/** Catalog connection family hint; each configured model selects its own operation. */
export const ProviderApiProtocol = {
    "ANTHROPIC": "ANTHROPIC",
    "OPENAI_COMPATIBLE": "OPENAI_COMPATIBLE",
    "PROVIDER_SPECIFIC": "PROVIDER_SPECIFIC"
} as const;

export type ProviderApiProtocol$options = ValuesOf<typeof ProviderApiProtocol>

/** Capabilities declared by a catalog or configured model; not an authorization grant. */
export const ProviderModelCapability = {
    "AUDIO": "AUDIO",
    "AUDIO_GENERATION": "AUDIO_GENERATION",
    "FORCED_TOOL_CHOICE": "FORCED_TOOL_CHOICE",
    "IMAGE_GENERATION": "IMAGE_GENERATION",
    "PDF": "PDF",
    "REASONING": "REASONING",
    "SEARCH": "SEARCH",
    "STRUCTURED_OUTPUT": "STRUCTURED_OUTPUT",
    "TEXT": "TEXT",
    "TOOL_CALL": "TOOL_CALL",
    "VIDEO": "VIDEO",
    "VISION": "VISION"
} as const;

export type ProviderModelCapability$options = ValuesOf<typeof ProviderModelCapability>

/** Text operation determining the upstream request and response format. */
export const TextModelProtocol = {
    "ANTHROPIC_MESSAGES": "ANTHROPIC_MESSAGES",
    "OPENAI_CHAT_COMPLETIONS": "OPENAI_CHAT_COMPLETIONS",
    "OPENAI_RESPONSES": "OPENAI_RESPONSES"
} as const;

export type TextModelProtocol$options = ValuesOf<typeof TextModelProtocol>

export const ThinkingEffortLevel = {
    "HIGH": "HIGH",
    "LOW": "LOW",
    "MAX": "MAX",
    "MEDIUM": "MEDIUM",
    "XHIGH": "XHIGH"
} as const;

export type ThinkingEffortLevel$options = ValuesOf<typeof ThinkingEffortLevel>

export const UpstreamProtocol = {
    "ANTHROPIC": "ANTHROPIC",
    "GEMINI": "GEMINI",
    "MINIMAX": "MINIMAX",
    "OPENAI": "OPENAI",
    "OPENAI_RESPONSES": "OPENAI_RESPONSES",
    "OPENROUTER": "OPENROUTER",
    "XAI": "XAI"
} as const;

export type UpstreamProtocol$options = ValuesOf<typeof UpstreamProtocol>

export const UserPreferenceGroup = {
    "ACCESSIBILITY": "ACCESSIBILITY",
    "ALL": "ALL",
    "CALENDAR": "CALENDAR",
    "CODE_APPEARANCE": "CODE_APPEARANCE",
    "MEMBER_DISPLAY": "MEMBER_DISPLAY",
    "MESSAGE_COMPOSITION": "MESSAGE_COMPOSITION",
    "NAVIGATION": "NAVIGATION",
    "SIDEBAR": "SIDEBAR",
    "THEME": "THEME",
    "TYPOGRAPHY": "TYPOGRAPHY"
} as const;

export type UserPreferenceGroup$options = ValuesOf<typeof UserPreferenceGroup>
