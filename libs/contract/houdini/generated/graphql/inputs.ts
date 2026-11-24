import type { AuthenticationPurpose$options, CredentialDelivery$options, FilterIs$options, HomeDestination$options, ImageMediaType$options, OnboardingDestination$options, OrderByDirection$options, PreferenceAppearanceMode$options, PreferenceBadgeStyle$options, PreferenceCodeTheme$options, PreferenceDarkPalette$options, PreferenceDestination$options, PreferenceFontFamily$options, PreferenceKeyCombination$options, PreferenceLightPalette$options, PreferenceNamingFormat$options, PreferencePixelFontSize$options, PreferenceRelativeFontSize$options, PreferenceSidebarCategory$options, PreferenceSidebarVisibility$options, PreferenceWeekday$options, ProviderModelCapability$options, TextModelProtocol$options } from './enums.js';

type ValueOf<T> = T[keyof T];

export type AccessibilityPatchInput = {
    pointerCursors?: boolean | null | undefined;
    reduceMotion?: boolean | null | undefined;
    underlineLinks?: boolean | null | undefined;
};

export type AnalyticUserProfilePutInput = {
    key: string;
    value: string;
};

export type CalendarPatchInput = {
    firstDay?: PreferenceWeekday$options | null | undefined;
};

export type ChatCreateInput = {
    message: string;
    modelId: string;
};

export type CodeAppearancePatchInput = {
    font?: CodeFontPatchInput | null | undefined;
    theme?: PreferenceCodeTheme$options | null | undefined;
};

export type CodeFontPatchInput = {
    family?: PreferenceFontFamily$options | null | undefined;
    size?: PreferencePixelFontSize$options | null | undefined;
};

export type CredentialCreateInput = {
    name: string;
};

export type EmailCodeInput = {
    code: string;
    delivery?: CredentialDelivery$options | null | undefined;
    purpose?: AuthenticationPurpose$options | null | undefined;
};

export type EmailInput = {
    delivery?: CredentialDelivery$options | null | undefined;
    email: string;
    invitation?: boolean | null | undefined;
    purpose?: AuthenticationPurpose$options | null | undefined;
};

export type GroupCreateInput = {
    description: string;
    name: string;
    roleIds: (string)[];
};

export type GroupUpdateInput = {
    addMemberIds?: (string)[] | null | undefined;
    addRoleIds?: (string)[] | null | undefined;
    description?: string | null | undefined;
    name?: string | null | undefined;
    removeMemberIds?: (string)[] | null | undefined;
    removeRoleIds?: (string)[] | null | undefined;
};

export type IDFilter = {
    eq?: string | number | null | undefined;
};

export type ImageUploadInput = {
    mediaType: ImageMediaType$options;
    size: number;
};

export type InvitationBatchCreateInput = {
    invitations: (InvitationCreateInput)[];
};

export type InvitationCreateInput = {
    email: string;
};

export type MemberDisplayPatchInput = {
    nameFormat?: PreferenceNamingFormat$options | null | undefined;
};

export type MemberFilterInput = {
    displayName?: StringFilter | null | undefined;
    email?: StringFilter | null | undefined;
    or?: (MemberMatchInput)[] | null | undefined;
    roleId?: IDFilter | null | undefined;
};

export type MemberMatchInput = {
    displayName?: StringFilter | null | undefined;
    email?: StringFilter | null | undefined;
};

export type MemberOrderInput = {
    displayName?: OrderByDirection$options | null | undefined;
    email?: OrderByDirection$options | null | undefined;
    id?: OrderByDirection$options | null | undefined;
};

export type MessageCompositionPatchInput = {
    convertEmoticons?: boolean | null | undefined;
    sendShortcut?: PreferenceKeyCombination$options | null | undefined;
};

export type ModelTemplateCreateInput = {
    configuration: ModelTemplateCustomizeInput;
};

export type ModelTemplateCustomizeInput = {
    bodyDefaults?: (ProviderBodyDefault)[] | null | undefined;
    cachedPrice?: ModelTemplatePriceInput | null | undefined;
    capabilities?: (ProviderModelCapability$options)[] | null | undefined;
    effortMapping?: ProviderEffortMapping | null | undefined;
    headers?: (ProviderHeader)[] | null | undefined;
    inputPrice?: ModelTemplatePriceInput | null | undefined;
    limits?: ModelTemplateLimitsInput | null | undefined;
    name?: string | null | undefined;
    outputPrice?: ModelTemplatePriceInput | null | undefined;
    path?: string | null | undefined;
    protocol?: TextModelProtocol$options | null | undefined;
};

export type ModelTemplateEffectiveLimitsInput = {
    context?: number | null | undefined;
    input?: number | null | undefined;
    output?: number | null | undefined;
};

export type ModelTemplateFilterInput = {
    displayName?: StringFilter | null | undefined;
    id?: StringFilter | null | undefined;
    name?: StringFilter | null | undefined;
    or?: (ModelTemplateMatchInput)[] | null | undefined;
};

export type ModelTemplateLimitsInput = {
    context?: number | null | undefined;
    input?: number | null | undefined;
    output?: number | null | undefined;
};

export type ModelTemplateMatchInput = {
    displayName?: StringFilter | null | undefined;
    id?: StringFilter | null | undefined;
    name?: StringFilter | null | undefined;
};

export type ModelTemplatePriceInput = {
    enabled: boolean;
    value?: string | null | undefined;
};

export type NavigationPatchInput = {
    home?: PreferenceDestination$options | null | undefined;
};

export type OnboardingLinkInput = {
    href: string;
    label: string;
};

export type OrganizationOnboardingSettingsInput = {
    artworkColor?: string | null | undefined;
    destination: OnboardingDestination$options;
    links: (OnboardingLinkInput)[];
    requireAvatar: boolean;
    requireFullName: boolean;
    welcome: string;
};

export type OrganizationProfileUpdateInput = {
    avatarUploaded?: boolean | null | undefined;
    name?: string | null | undefined;
};

export type OrganizationSettingsUpdateInput = {
    announcements?: string | null | undefined;
    fiscalMonth?: string | null | undefined;
    home?: HomeDestination$options | null | undefined;
    name?: string | null | undefined;
    slug?: string | null | undefined;
    welcome?: string | null | undefined;
};

export type PalettePatchInput = {
    dark?: PreferenceDarkPalette$options | null | undefined;
    light?: PreferenceLightPalette$options | null | undefined;
};

export type PasskeyFinishInput = {
    attestationObject?: string | null | undefined;
    authenticatorData?: string | null | undefined;
    clientDataJson: string;
    delivery?: CredentialDelivery$options | null | undefined;
    id: string;
    purpose?: AuthenticationPurpose$options | null | undefined;
    rawId: string;
    signature?: string | null | undefined;
    token: string;
    transports?: (string)[] | null | undefined;
    userHandle?: string | null | undefined;
};

export type PasskeyRegistrationInput = {
    name: string;
};

export type ProfileUpdateInput = {
    name?: string | null | undefined;
    username?: string | null | undefined;
};

export type ProviderBodyDefault = {
    key: string;
    value: string;
};

export type ProviderCatalogRef = {
    model: string;
    provider: string;
};

export type ProviderCreateFromTemplateInput = {
    apiKey: string;
    baseUrl: string;
    headers: (ProviderHeader)[];
    logoUploadId?: string | null | undefined;
    logoUrl?: string | null | undefined;
    models: (ProviderTemplateModelInput)[];
    name: string;
    providerTemplateId: string;
};

export type ProviderCreateInput = {
    apiKey: string;
    baseUrl: string;
    headers: (ProviderHeader)[];
    models: (ProviderModelWrite)[];
    name: string;
};

export type ProviderEffortMapping = {
    high?: string | null | undefined;
    low?: string | null | undefined;
    max?: string | null | undefined;
    medium?: string | null | undefined;
    xhigh?: string | null | undefined;
};

export type ProviderHeader = {
    key: string;
    value: string;
};

export type ProviderHeaderUpdate = {
    key: string;
    value?: string | null | undefined;
};

export type ProviderModelConfiguration = {
    bodyDefaults: (ProviderBodyDefault)[];
    cachedPrice: string;
    cachedPriceEnabled: boolean;
    capabilities: (ProviderModelCapability$options)[];
    catalogRef?: ProviderCatalogRef | null | undefined;
    contextLimitEnabled: boolean;
    effortMapping?: ProviderEffortMapping | null | undefined;
    enabled: boolean;
    headers: (ProviderHeader)[];
    inputPrice: string;
    inputPriceEnabled: boolean;
    maxContextTokens: number;
    maxInputTokens?: number | null | undefined;
    maxOutputTokens?: number | null | undefined;
    name: string;
    outputPrice: string;
    outputPriceEnabled: boolean;
    path: string;
    protocol: TextModelProtocol$options;
};

export type ProviderModelWrite = {
    configuration: ProviderModelConfiguration;
    id?: string | number | null | undefined;
};

export type ProviderPatchInput = {
    apiKey?: string | null | undefined;
    baseUrl?: string | null | undefined;
    headers?: (ProviderHeaderUpdate)[] | null | undefined;
    models?: (ProviderModelWrite)[] | null | undefined;
    name?: string | null | undefined;
};

export type ProviderReplaceInput = {
    apiKey?: string | null | undefined;
    baseUrl: string;
    headers: (ProviderHeaderUpdate)[];
    models: (ProviderModelWrite)[];
    name: string;
};

export type ProviderTemplateFilterInput = {
    id?: StringFilter | null | undefined;
    name?: StringFilter | null | undefined;
    or?: (ProviderTemplateMatchInput)[] | null | undefined;
};

export type ProviderTemplateMatchInput = {
    id?: StringFilter | null | undefined;
    name?: StringFilter | null | undefined;
};

export type ProviderTemplateModelInput = {
    id: string;
    protocol: TextModelProtocol$options;
};

export type ProviderTemplateOrderInput = {
    id?: OrderByDirection$options | null | undefined;
    name?: OrderByDirection$options | null | undefined;
};

export type ProvidersCreateInput = {
    providers: (ProviderCreateInput)[];
};

export type ReportingScheduleInput = {
    timezone: string;
};

export type RoleCreateInput = {
    cedarSource: string;
    description: string;
    name: string;
};

export type RoleUpdateInput = {
    cedarSource?: string | null | undefined;
    description?: string | null | undefined;
    name?: string | null | undefined;
};

export type SidebarLayoutSectionInput = {
    category: PreferenceSidebarCategory$options;
    items: (PreferenceDestination$options)[];
};

export type SidebarPatchInput = {
    badgeStyle?: PreferenceBadgeStyle$options | null | undefined;
    layout?: (SidebarLayoutSectionInput)[] | null | undefined;
    order?: (PreferenceDestination$options)[] | null | undefined;
    visibility?: (SidebarVisibilityPatchInput)[] | null | undefined;
};

export type SidebarVisibilityPatchInput = {
    item: PreferenceDestination$options;
    visibility: PreferenceSidebarVisibility$options;
};

export type SpendingLimitInput = {
    amount?: string | null | undefined;
};

export type StringFilter = {
    eq?: string | null | undefined;
    gt?: string | null | undefined;
    gte?: string | null | undefined;
    ilike?: string | null | undefined;
    in?: (string)[] | null | undefined;
    iregex?: string | null | undefined;
    is?: FilterIs$options | null | undefined;
    like?: string | null | undefined;
    lt?: string | null | undefined;
    lte?: string | null | undefined;
    neq?: string | null | undefined;
    regex?: string | null | undefined;
    startsWith?: string | null | undefined;
};

export type ThemePatchInput = {
    mode?: PreferenceAppearanceMode$options | null | undefined;
    palettes?: PalettePatchInput | null | undefined;
};

export type TypographyPatchInput = {
    family?: PreferenceFontFamily$options | null | undefined;
    size?: PreferenceRelativeFontSize$options | null | undefined;
};

export type UserPreferencesPatchInput = {
    accessibility?: AccessibilityPatchInput | null | undefined;
    calendar?: CalendarPatchInput | null | undefined;
    codeAppearance?: CodeAppearancePatchInput | null | undefined;
    memberDisplay?: MemberDisplayPatchInput | null | undefined;
    messageComposition?: MessageCompositionPatchInput | null | undefined;
    navigation?: NavigationPatchInput | null | undefined;
    sidebar?: SidebarPatchInput | null | undefined;
    theme?: ThemePatchInput | null | undefined;
    typography?: TypographyPatchInput | null | undefined;
};

export type UserProfileUpdateInput = {
    avatarUploaded?: boolean | null | undefined;
    displayName?: string | null | undefined;
    username?: string | null | undefined;
};
