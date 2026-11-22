// @ts-nocheck
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

export type Scalars = {
  Boolean: boolean
  ContextTokens: number
  Float: number
  ID: string
  Int: number
  String: string
}

export interface AccessibilityPreferences {
  pointerCursors: Scalars['Boolean']
  reduceMotion: Scalars['Boolean']
  underlineLinks: Scalars['Boolean']
  __typename: 'AccessibilityPreferences'
}

export interface AnalyticUserProfile {
  key: Scalars['String']
  value: Scalars['String']
  __typename: 'AnalyticUserProfile'
}

export interface AnalyticUserProfilePayload {
  node: AnalyticUserProfile
  __typename: 'AnalyticUserProfilePayload'
}

export interface AudioCapabilities {
  input: AudioSupport
  output: AudioSupport
  __typename: 'AudioCapabilities'
}

export interface AudioConfiguration {
  formats: Scalars['String'][] | null
  maxBytes: Scalars['Float'] | null
  maxDurationMs: Scalars['Float'] | null
  __typename: 'AudioConfiguration'
}

export interface AudioSupport {
  configuration: AudioConfiguration | null
  pricing: CapabilityPricing | null
  status: CapabilityStatus
  __typename: 'AudioSupport'
}

export interface AuthenticationConfiguration {
  configuration: AuthenticationParameters
  method: AuthenticationMethod
  __typename: 'AuthenticationConfiguration'
}

export type AuthenticationMethod = 'API_KEY' | 'CUSTOM' | 'NONE' | 'OAUTH'

export interface AuthenticationParameters {
  adapter: Scalars['String'] | null
  authorizationUrl: Scalars['String'] | null
  deviceAuthorizationUrl: Scalars['String'] | null
  flow: OAuthFlow | null
  parameters: ConfigurationEntry[]
  placement: CredentialPlacement | null
  requiredInputs: ConfigurationInput[]
  scopes: Scalars['String'][]
  tokenUrl: Scalars['String'] | null
  __typename: 'AuthenticationParameters'
}

export type AuthenticationPurpose = 'LOGIN' | 'PASSKEY_CHANGE'

export interface AvatarUpload {
  /** Expiration as Unix seconds, encoded as a string. */
  expiresAt: Scalars['String']
  fields: UploadFormField[]
  url: Scalars['String']
  __typename: 'AvatarUpload'
}

export interface AvatarUploadPayload {
  node: AvatarUpload
  __typename: 'AvatarUploadPayload'
}

export interface BillingAllowance {
  quota: Scalars['String'] | null
  resetPeriod: PriceBillingPeriod | null
  unit: PriceUnit
  __typename: 'BillingAllowance'
}

export interface BillingFee {
  amount: Scalars['String'] | null
  currency: Scalars['String'] | null
  period: PriceBillingPeriod | null
  __typename: 'BillingFee'
}

export type BillingModel = 'FREE' | 'METERED' | 'SUBSCRIPTION' | 'SUBSCRIPTION_WITH_OVERAGE' | 'UNKNOWN'

/** Public body default. Value is JSON text for the configuration editor. */
export interface BodyDefaultEntry {
  key: Scalars['String']
  value: Scalars['String']
  __typename: 'BodyDefaultEntry'
}

export interface BooleanPayload {
  node: Scalars['Boolean']
  __typename: 'BooleanPayload'
}

export type CacheBilling =
  'INCLUDED' | 'NOT_APPLICABLE' | 'PER_ROUTE' | 'SEPARATE_RATE' | 'STANDARD_INPUT_RATE' | 'UNKNOWN' | 'UNSUPPORTED'

export interface CalendarPreferences {
  firstDay: PreferenceWeekday
  __typename: 'CalendarPreferences'
}

export type CapabilityCacheMode = 'INCLUDED' | 'SEPARATE_RATE' | 'STANDARD_INPUT_RATE' | 'UNSUPPORTED'

export interface CapabilityCachePrice {
  amount: Scalars['String'] | null
  currency: Scalars['String'] | null
  mode: CapabilityCacheMode
  quantity: Scalars['String'] | null
  __typename: 'CapabilityCachePrice'
}

export type CapabilityPriceMode = 'FIXED' | 'SCHEDULED' | 'TIERED_TOKEN'

export interface CapabilityPricing {
  kind: CapabilityPricingKind
  usage: CapabilityUsage | null
  __typename: 'CapabilityPricing'
}

export type CapabilityPricingKind = 'FREE' | 'INCLUDED' | 'MODEL_BILLING' | 'USAGE'

export interface CapabilityRate {
  amount: Scalars['String'] | null
  cache: CapabilityCachePrice | null
  conditions: ConfigurationEntry[]
  credits: Scalars['String'] | null
  currency: Scalars['String'] | null
  direction: CreditUsageDirection | null
  quantity: Scalars['String'] | null
  unit: PriceUnit
  __typename: 'CapabilityRate'
}

export interface CapabilitySlot {
  fromHour: Scalars['Int']
  pricing: CapabilityUsage
  toHour: Scalars['Int']
  __typename: 'CapabilitySlot'
}

export type CapabilityStatus = 'SUPPORTED' | 'UNKNOWN' | 'UNSUPPORTED'

export interface CapabilityTier {
  fromTokens: Scalars['String']
  rate: CapabilityRate
  toTokens: Scalars['String'] | null
  __typename: 'CapabilityTier'
}

export interface CapabilityUsage {
  mode: CapabilityPriceMode
  rates: CapabilityRate[]
  slots: CapabilitySlot[]
  tierBasis: PriceTierBasis | null
  tiers: CapabilityTier[]
  timezone: Scalars['String'] | null
  __typename: 'CapabilityUsage'
}

export interface CodeAppearancePreferences {
  font: CodeFontPreferences
  theme: PreferenceCodeTheme
  __typename: 'CodeAppearancePreferences'
}

export interface CodeFontPreferences {
  family: PreferenceFontFamily
  size: PreferencePixelFontSize
  __typename: 'CodeFontPreferences'
}

export interface ConfigurationEntry {
  key: Scalars['String']
  value: Scalars['String']
  __typename: 'ConfigurationEntry'
}

export interface ConfigurationInput {
  allowedValues: Scalars['String'][] | null
  kind: ConfigurationInputKind
  name: Scalars['String']
  required: Scalars['Boolean']
  __typename: 'ConfigurationInput'
}

export type ConfigurationInputKind = 'OBJECT' | 'POSITIVE_INTEGER' | 'PRICE' | 'STRING' | 'URL'

export interface ConnectionConfiguration {
  authentication: AuthenticationConfiguration | null
  baseUrl: Scalars['String'] | null
  endpoint: EndpointRequirements
  path: Scalars['String'] | null
  query: ConfigurationEntry[]
  timeoutMs: Scalars['Float'] | null
  __typename: 'ConnectionConfiguration'
}

/** How the request delivers credentials; this is never stored in signed claims. */
export type CredentialDelivery = 'COOKIE' | 'TOKEN'

export type CredentialLocation = 'HEADER' | 'QUERY'

export interface CredentialPlacement {
  location: CredentialLocation
  name: Scalars['String']
  prefix: Scalars['String'] | null
  __typename: 'CredentialPlacement'
}

export type CreditUsageDirection = 'CACHE_READ' | 'INPUT' | 'OUTPUT'

/** Email admission succeeded; cookie delivery never exposes the OTP credential. */
export interface EmailChallenge {
  lifetime: TokenLifetime
  token: Scalars['String'] | null
  __typename: 'EmailChallenge'
}

export interface EmailChallengePayload {
  node: EmailChallenge
  __typename: 'EmailChallengePayload'
}

export interface EndpointConfiguration {
  requiredInputs: ConfigurationInput[]
  urlTemplate: Scalars['String'] | null
  __typename: 'EndpointConfiguration'
}

export type EndpointMode = 'ACCOUNT_SPECIFIC' | 'DISCOVERED' | 'FIXED' | 'UNKNOWN'

export interface EndpointRequirements {
  configuration: EndpointConfiguration
  mode: EndpointMode
  __typename: 'EndpointRequirements'
}

export interface FeatureSupport {
  pricing: CapabilityPricing | null
  status: CapabilityStatus
  __typename: 'FeatureSupport'
}

export type FilterIs = 'NOT_NULL' | 'NULL'

export interface Group {
  description: Scalars['String']
  id: Scalars['ID']
  memberIds: Scalars['ID'][]
  name: Scalars['String']
  roles: RoleConnection
  __typename: 'Group'
}

export interface GroupConnection {
  edges: GroupEdge[]
  pageInfo: PageInfo
  __typename: 'GroupConnection'
}

export interface GroupEdge {
  cursor: Scalars['String']
  node: Group
  __typename: 'GroupEdge'
}

export interface GroupPayload {
  node: Group
  __typename: 'GroupPayload'
}

export type HomeDestination = 'CHAT' | 'MODELS' | 'PROVIDERS'

export interface IDPayload {
  node: Scalars['ID']
  __typename: 'IDPayload'
}

export interface ImageGenerationConfiguration {
  formats: Scalars['String'][] | null
  maxImages: Scalars['Float'] | null
  __typename: 'ImageGenerationConfiguration'
}

export interface ImageGenerationSupport {
  configuration: ImageGenerationConfiguration | null
  pricing: CapabilityPricing | null
  status: CapabilityStatus
  __typename: 'ImageGenerationSupport'
}

export type ImageMediaType = 'JPEG' | 'PNG' | 'WEBP'

export interface Invitation {
  email: Scalars['String']
  expiresAt: Scalars['Float']
  id: Scalars['ID']
  inviterId: Scalars['ID']
  issuedAt: Scalars['Float']
  organizationId: Scalars['ID']
  status: Scalars['String']
  __typename: 'Invitation'
}

export interface InvitationConnection {
  edges: InvitationEdge[]
  pageInfo: PageInfo
  __typename: 'InvitationConnection'
}

export interface InvitationContext {
  email: Scalars['String']
  expiresAt: Scalars['Float']
  organizationId: Scalars['ID']
  organizationName: Scalars['String']
  signup: Scalars['Boolean']
  __typename: 'InvitationContext'
}

export interface InvitationContextPayload {
  node: InvitationContext
  __typename: 'InvitationContextPayload'
}

export interface InvitationEdge {
  cursor: Scalars['String']
  node: Invitation
  __typename: 'InvitationEdge'
}

export interface InvitationIssued {
  emailAdmitted: Scalars['Boolean']
  invitation: Invitation
  url: Scalars['String']
  __typename: 'InvitationIssued'
}

export interface InvitationIssuedConnection {
  edges: InvitationIssuedEdge[]
  pageInfo: PageInfo
  __typename: 'InvitationIssuedConnection'
}

export interface InvitationIssuedEdge {
  cursor: Scalars['String']
  node: InvitationIssued
  __typename: 'InvitationIssuedEdge'
}

export interface InvitationIssuedPayload {
  node: InvitationIssued
  __typename: 'InvitationIssuedPayload'
}

export interface InvitationPayload {
  node: Invitation
  __typename: 'InvitationPayload'
}

export interface Me {
  /** Cedar JSON entity facts for the verified principal and its role/group ancestry; contains no credentials. */
  cedarEntities: Scalars['String']
  /** Effective policies for this verified user only. UI decisions do not grant server authority. */
  cedarSource: Scalars['String']
  id: Scalars['ID']
  organizationId: Scalars['ID']
  organizationOwner: Scalars['Boolean']
  sessionId: Scalars['ID']
  __typename: 'Me'
}

export interface MePayload {
  node: Me
  __typename: 'MePayload'
}

export interface Member {
  available: Scalars['Boolean']
  displayName: Scalars['String']
  /** Direct and group-inherited role identities in the verified organization. */
  effectiveRoleIds: Scalars['ID'][]
  email: Scalars['String']
  /** Group identities in the verified organization; no policy source is exposed. */
  groupIds: Scalars['ID'][]
  groups: Group[]
  id: Scalars['ID']
  joinedAt: Scalars['Float']
  role: Role
  __typename: 'Member'
}

export interface MemberConnection {
  edges: MemberEdge[]
  pageInfo: PageInfo
  __typename: 'MemberConnection'
}

export interface MemberDisplayPreferences {
  nameFormat: PreferenceNamingFormat
  __typename: 'MemberDisplayPreferences'
}

export interface MemberEdge {
  cursor: Scalars['String']
  node: Member
  __typename: 'MemberEdge'
}

export interface MessageCompositionPreferences {
  convertEmoticons: Scalars['Boolean']
  sendShortcut: PreferenceKeyCombination
  __typename: 'MessageCompositionPreferences'
}

export interface ModelBilling {
  amountPerCredit: Scalars['String'] | null
  currency: Scalars['String'] | null
  description: Scalars['String'] | null
  entitlements: BillingAllowance[]
  fee: BillingFee | null
  kind: ModelBillingKind
  plan: Scalars['String'] | null
  product: Scalars['String'] | null
  requestRate: CapabilityRate | null
  __typename: 'ModelBilling'
}

export type ModelBillingKind = 'CREDIT' | 'FREE' | 'REQUEST' | 'SUBSCRIPTION'

export interface ModelConnection {
  configuration: ConnectionConfiguration
  protocol: UpstreamProtocol
  __typename: 'ModelConnection'
}

export interface ModelOperation {
  operation: TextModelProtocol
  path: Scalars['String']
  __typename: 'ModelOperation'
}

export type ModelServiceKind = 'AUDIO' | 'EMBEDDING' | 'IMAGE' | 'RERANK' | 'STT' | 'TEXT' | 'TTS' | 'VIDEO'

export interface ModelTemplate {
  audio: AudioCapabilities
  billing: ModelBilling | null
  billingModel: BillingModel
  bodyDefaults: BodyDefaultEntry[]
  cacheBilling: CacheBilling
  cachedPrice: Scalars['String'] | null
  capabilities: ProviderModelCapability[]
  connections: ModelConnection[]
  customFields: Scalars['String'][]
  description: Scalars['String'] | null
  displayName: Scalars['String']
  effortMapping: ThinkingEffort[]
  headers: ConfigurationEntry[]
  id: Scalars['ID']
  imageGeneration: ImageGenerationSupport
  inputPrice: Scalars['String'] | null
  maxContextTokens: Scalars['ContextTokens'] | null
  maxInputTokens: Scalars['ContextTokens'] | null
  maxOutputTokens: Scalars['ContextTokens'] | null
  name: Scalars['String']
  operations: ModelOperation[]
  origin: ModelTemplateOrigin
  outputPrice: Scalars['String'] | null
  pdf: PdfSupport
  pricingMode: PricingMode
  providerTemplateId: Scalars['ID']
  routeSelection: RouteSelection | null
  search: SearchSupport
  serviceKind: ModelServiceKind | null
  structuredOutput: StructuredOutputSupport
  text: TextCapabilities
  thinking: ThinkingSupport
  tools: ToolCapabilities
  upstreamModelId: Scalars['String']
  video: VideoSupport
  vision: VisionSupport
  __typename: 'ModelTemplate'
}

export interface ModelTemplateConnection {
  edges: ModelTemplateEdge[]
  pageInfo: PageInfo
  __typename: 'ModelTemplateConnection'
}

export interface ModelTemplateEdge {
  cursor: Scalars['String']
  node: ModelTemplate
  __typename: 'ModelTemplateEdge'
}

export type ModelTemplateOrigin = 'CUSTOM' | 'GLOBAL' | 'OVERRIDE'

export interface ModelTemplatePayload {
  node: ModelTemplate
  __typename: 'ModelTemplatePayload'
}

export interface Mutation {
  analyticUserProfilePut: AnalyticUserProfilePayload
  /** Confirm the verified signin challenge and create a scoped session. */
  emailSigninConfirm: SessionCredentialsPayload
  /** Request a signin email challenge for an existing active account. */
  emailSigninRequest: EmailChallengePayload
  /** Confirm the verified signup challenge and create the account and session. */
  emailSignupConfirm: SessionCredentialsPayload
  /** Request a signup email challenge; reject existing accounts before sending. */
  emailSignupRequest: EmailChallengePayload
  generateOrgAvatarUrl: AvatarUploadPayload
  /** Authorize a direct JPEG/PNG/WebP avatar upload up to 5 MiB for five minutes. */
  generateUrlAvatarUrl: AvatarUploadPayload
  /** Create a group and its initial role assignments in the verified active organization. */
  groupCreate: GroupPayload
  groupDelete: BooleanPayload
  groupUpdate: GroupPayload
  invitationCreate: InvitationIssuedPayload
  invitationCreateMany: InvitationIssuedConnection
  invitationRevoke: InvitationPayload
  modelTemplateCreate: ModelTemplatePayload
  modelTemplateCustomize: ModelTemplatePayload
  modelTemplateDelete: BooleanPayload
  modelTemplateReset: ModelTemplatePayload
  onboardingAdvance: OnboardingContextPayload
  onboardingComplete: OnboardingContextPayload
  onboardingPresent: OnboardingContextPayload
  organizationOnboardingSettingsUpdate: OrganizationOnboardingSettingsPayload
  organizationProfileUpdate: OrganizationPayload
  passkeyAuthenticationFinish: SessionCredentialsPayload
  passkeyAuthenticationStart: PasskeyCeremonyPayload
  passkeyRegistrationFinish: PasskeySummaryPayload
  passkeyRegistrationStart: PasskeyCeremonyPayload
  passkeyRemove: IDPayload
  passkeyUpdate: PasskeySummaryPayload
  providerCreate: ProviderPayload
  providerLogoUpload: ProviderLogoUploadPayload
  roleAssign: RolePayload
  /** Create a role in the verified active organization. */
  roleCreate: RolePayload
  roleDelete: BooleanPayload
  roleUpdate: RolePayload
  /** Refresh using explicit refresh Bearer authority; leave browser cookies unchanged. */
  sessionRefresh: SessionCredentialsPayload
  /** Revoke the current session and clear its cookies after success. */
  sessionRevoke: BooleanPayload
  /** Revoke one other session belonging to the authenticated account. */
  sessionRevokeOther: BooleanPayload
  /** Revoke all other account sessions, retaining this login. */
  sessionRevokeOthers: BooleanPayload
  /** Restore selected preference groups, or ALL alone, to Core defaults. */
  userPreferencesReset: UserPreferencesPayload
  /** Atomically patch the signed-in user's preferences, preserving omitted fields. */
  userPreferencesUpdate: UserPreferencesPayload
  /** Update the current user's display name and confirm an uploaded avatar. */
  userProfileUpdate: UserPayload
  __typename: 'Mutation'
}

export interface NavigationPreferences {
  home: PreferenceDestination
  __typename: 'NavigationPreferences'
}

export type OAuthFlow = 'AUTHORIZATION_CODE' | 'CLIENT_CREDENTIALS' | 'DEVICE_CODE'

export interface OnboardingContext {
  flow: OnboardingFlow
  hasBeenPresented: Scalars['Boolean']
  required: Scalars['Boolean']
  settings: OrganizationOnboardingSettings
  shouldPresent: Scalars['Boolean']
  step: OnboardingStep
  __typename: 'OnboardingContext'
}

export interface OnboardingContextPayload {
  node: OnboardingContext
  __typename: 'OnboardingContextPayload'
}

export type OnboardingDestination = 'CHAT' | 'MODELS' | 'PROVIDERS'

export type OnboardingFlow = 'INVITED_EXISTING' | 'INVITED_NEW' | 'PERSONAL'

export interface OnboardingLink {
  href: Scalars['String']
  label: Scalars['String']
  __typename: 'OnboardingLink'
}

export type OnboardingStep = 'INTRODUCTION' | 'PREFERENCES' | 'PROFILE' | 'READY' | 'WORK' | 'WORKSPACE'

export type OrderByDirection = 'AscNullsFirst' | 'AscNullsLast' | 'DescNullsFirst' | 'DescNullsLast'

export interface Organization {
  avatarUrl: Scalars['String'] | null
  canUpdateProfile: Scalars['Boolean']
  id: Scalars['ID']
  name: Scalars['String']
  ownerMemberIds: Scalars['ID'][]
  __typename: 'Organization'
}

export interface OrganizationOnboardingSettings {
  artworkColor: Scalars['String'] | null
  destination: OnboardingDestination
  links: OnboardingLink[]
  requireAvatar: Scalars['Boolean']
  requireFullName: Scalars['Boolean']
  welcome: Scalars['String']
  __typename: 'OrganizationOnboardingSettings'
}

export interface OrganizationOnboardingSettingsPayload {
  node: OrganizationOnboardingSettings
  __typename: 'OrganizationOnboardingSettingsPayload'
}

export interface OrganizationPayload {
  node: Organization
  __typename: 'OrganizationPayload'
}

export interface PageInfo {
  endCursor: Scalars['String'] | null
  hasNextPage: Scalars['Boolean']
  hasPreviousPage: Scalars['Boolean']
  startCursor: Scalars['String'] | null
  __typename: 'PageInfo'
}

export interface PalettePreferences {
  dark: PreferenceDarkPalette
  light: PreferenceLightPalette
  __typename: 'PalettePreferences'
}

export interface PasskeyCeremony {
  expiresAt: Scalars['String']
  options: PasskeyOptions
  token: Scalars['String']
  __typename: 'PasskeyCeremony'
}

export interface PasskeyCeremonyPayload {
  node: PasskeyCeremony
  __typename: 'PasskeyCeremonyPayload'
}

export interface PasskeyDescriptor {
  id: Scalars['String']
  transports: Scalars['String'][]
  __typename: 'PasskeyDescriptor'
}

export interface PasskeyExtensions {
  credProps: Scalars['Boolean'] | null
  credentialProtectionPolicy: Scalars['String'] | null
  enforceCredentialProtectionPolicy: Scalars['Boolean'] | null
  __typename: 'PasskeyExtensions'
}

export interface PasskeyOptions {
  algorithms: Scalars['Int'][]
  challenge: Scalars['String']
  credentials: PasskeyDescriptor[]
  extensions: PasskeyExtensions
  residentKey: Scalars['String'] | null
  rpId: Scalars['String']
  rpName: Scalars['String'] | null
  timeout: Scalars['Int']
  userDisplayName: Scalars['String'] | null
  userId: Scalars['String'] | null
  userName: Scalars['String'] | null
  userVerification: Scalars['String']
  __typename: 'PasskeyOptions'
}

export interface PasskeySummary {
  credentialId: Scalars['String']
  id: Scalars['ID']
  lastUsedAt: Scalars['String'] | null
  name: Scalars['String']
  registeredAt: Scalars['String']
  __typename: 'PasskeySummary'
}

export interface PasskeySummaryConnection {
  edges: PasskeySummaryEdge[]
  pageInfo: PageInfo
  __typename: 'PasskeySummaryConnection'
}

export interface PasskeySummaryEdge {
  cursor: Scalars['String']
  node: PasskeySummary
  __typename: 'PasskeySummaryEdge'
}

export interface PasskeySummaryPayload {
  node: PasskeySummary
  __typename: 'PasskeySummaryPayload'
}

export interface PdfConfiguration {
  maxFiles: Scalars['Float'] | null
  maxPages: Scalars['Float'] | null
  __typename: 'PdfConfiguration'
}

export interface PdfSupport {
  configuration: PdfConfiguration | null
  pricing: CapabilityPricing | null
  status: CapabilityStatus
  __typename: 'PdfSupport'
}

export interface PolicyAction {
  description: Scalars['String']
  displayName: Scalars['String']
  entityType: Scalars['String']
  name: Scalars['String']
  resourceType: Scalars['String']
  __typename: 'PolicyAction'
}

export interface PolicyActionConnection {
  edges: PolicyActionEdge[]
  pageInfo: PageInfo
  __typename: 'PolicyActionConnection'
}

export interface PolicyActionEdge {
  cursor: Scalars['String']
  node: PolicyAction
  __typename: 'PolicyActionEdge'
}

export type PreferenceAppearanceMode = 'DARK' | 'LIGHT' | 'SYSTEM'

export type PreferenceBadgeStyle = 'COUNT' | 'DOT'

export type PreferenceCodeTheme = 'FOLLOW_INTERFACE' | 'LINEAR_DARK' | 'LINEAR_LIGHT' | 'SYSTEM'

export type PreferenceDarkPalette = 'BASIC_DARK' | 'BLUEBERRY_DARK' | 'MARSHMALLOW_DARK'

export type PreferenceDestination =
  'API_KEYS' | 'CHAT' | 'CONVERSATIONS' | 'MEMBERS' | 'MODELS' | 'NOTEBOOKS' | 'PLUGINS' | 'PROVIDERS' | 'STATS'

export type PreferenceFontFamily = 'DEFAULT' | 'MONOSPACE'

export type PreferenceKeyCombination = 'ENTER' | 'MODIFIER_ENTER'

export type PreferenceLightPalette = 'BASIC_WHITE' | 'BLUEBERRY_WHITE' | 'MARSHMALLOW_WHITE'

export type PreferenceNamingFormat = 'FULL_NAME' | 'USERNAME'

export type PreferencePixelFontSize = 'PX12' | 'PX13' | 'PX14' | 'PX16'

export type PreferenceRelativeFontSize = 'DEFAULT' | 'LARGE' | 'LARGER' | 'SMALL' | 'SMALLER'

export type PreferenceSidebarCategory = 'CONVERSATIONS' | 'ORGANIZATION' | 'ROUTERS' | 'WORKSPACE'

export type PreferenceSidebarVisibility = 'ALWAYS' | 'HIDDEN' | 'WHEN_BADGED'

export type PreferenceWeekday = 'FRIDAY' | 'MONDAY' | 'SATURDAY' | 'SUNDAY' | 'THURSDAY' | 'TUESDAY' | 'WEDNESDAY'

export type PriceBillingPeriod = 'DAY' | 'HOUR' | 'MINUTE' | 'MONTH' | 'WEEK' | 'YEAR'

export type PriceTierBasis = 'INPUT_TOKENS' | 'TOTAL_TOKENS'

export type PriceUnit =
  'CHARACTERS' | 'CREDITS' | 'IMAGES' | 'REQUESTS' | 'SECONDS' | 'TOKENS' | 'UTF8_BYTES' | 'VIDEOS'

export type PricingMode =
  'ACCOUNT_SPECIFIC' | 'API' | 'FIXED' | 'NOT_APPLICABLE' | 'SCHEDULED' | 'TIERED' | 'UNKNOWN' | 'UNPUBLISHED'

export interface Provider {
  connection: ConnectionConfiguration
  description: Scalars['String'] | null
  /** Stable provider identity, serialized as a string. */
  id: Scalars['ID']
  logoUrl: Scalars['String'] | null
  models: ProviderModel[]
  /** Provider display name within its organization. */
  name: Scalars['String']
  /** Owning organization; a supplied identifier never grants access. */
  organizationId: Scalars['ID']
  __typename: 'Provider'
}

/** Catalog connection family hint; each configured model selects its own operation. */
export type ProviderApiProtocol = 'ANTHROPIC' | 'OPENAI_COMPATIBLE' | 'PROVIDER_SPECIFIC'

export interface ProviderConnection {
  edges: ProviderEdge[]
  pageInfo: PageInfo
  __typename: 'ProviderConnection'
}

export interface ProviderEdge {
  cursor: Scalars['String']
  node: Provider
  __typename: 'ProviderEdge'
}

export interface ProviderLogoUpload {
  id: Scalars['String']
  upload: AvatarUpload
  __typename: 'ProviderLogoUpload'
}

export interface ProviderLogoUploadPayload {
  node: ProviderLogoUpload
  __typename: 'ProviderLogoUploadPayload'
}

export interface ProviderModel {
  cachedPrice: Scalars['String'] | null
  capabilities: ProviderModelCapability[]
  connection: ConnectionConfiguration
  contextTokens: Scalars['ContextTokens'] | null
  /** Configured availability; this flag alone does not authorize invocation. */
  enabled: Scalars['Boolean']
  /** Stable configured-model identity, serialized as a string. */
  id: Scalars['ID']
  inputPrice: Scalars['String'] | null
  /** Configured upstream model name. */
  name: Scalars['String']
  /** Owning organization identity. */
  organizationId: Scalars['ID']
  outputPrice: Scalars['String'] | null
  protocol: UpstreamProtocol
  /** Owning provider identity. */
  providerId: Scalars['ID']
  text: TextCapabilities
  __typename: 'ProviderModel'
}

/** Capabilities declared by a catalog or configured model; not an authorization grant. */
export type ProviderModelCapability =
  | 'AUDIO'
  | 'AUDIO_GENERATION'
  | 'FORCED_TOOL_CHOICE'
  | 'IMAGE_GENERATION'
  | 'PDF'
  | 'REASONING'
  | 'SEARCH'
  | 'STRUCTURED_OUTPUT'
  | 'TEXT'
  | 'TOOL_CALL'
  | 'VIDEO'
  | 'VISION'

export interface ProviderPayload {
  node: Provider
  __typename: 'ProviderPayload'
}

export interface ProviderTemplate {
  authentication: AuthenticationConfiguration[]
  baseUrl: Scalars['String'] | null
  connection: ConnectionConfiguration
  description: Scalars['String'] | null
  endpoint: EndpointRequirements
  headers: ConfigurationEntry[]
  id: Scalars['ID']
  logoUrl: Scalars['String'] | null
  models: ModelTemplateConnection
  name: Scalars['String']
  __typename: 'ProviderTemplate'
}

export interface ProviderTemplateConnection {
  edges: ProviderTemplateEdge[]
  pageInfo: PageInfo
  __typename: 'ProviderTemplateConnection'
}

export interface ProviderTemplateEdge {
  cursor: Scalars['String']
  node: ProviderTemplate
  __typename: 'ProviderTemplateEdge'
}

export interface ProviderTemplatePayload {
  node: ProviderTemplate
  __typename: 'ProviderTemplatePayload'
}

export interface Query {
  analyticUserProfile: AnalyticUserProfilePayload | null
  effectiveModelTemplate: ModelTemplatePayload | null
  effectiveProviderTemplate: ProviderTemplatePayload | null
  effectiveProviderTemplates: ProviderTemplateConnection
  /** Find visible group metadata in the active organization. */
  group: GroupPayload | null
  /** Return the complete visible group metadata collection, ordered by ID. */
  groups: GroupConnection
  invitationContext: InvitationContextPayload
  invitations: InvitationConnection
  me: MePayload
  members: MemberConnection
  /** Public configuration defaults for one database-ready model, identified within its provider. */
  modelTemplate: ModelTemplatePayload | null
  onboardingContext: OnboardingContextPayload | null
  /** Return the active organization from verified user authority. */
  organization: OrganizationPayload
  organizationOnboardingSettings: OrganizationOnboardingSettingsPayload
  /** Return the system-owned organization policy action catalog. */
  organizationPolicyActions: PolicyActionConnection
  organizationProfile: OrganizationPayload | null
  passkeys: PasskeySummaryConnection
  provider: ProviderPayload | null
  /** Public provider defaults with a complete, unpaginated collection of database-ready models. */
  providerTemplate: ProviderTemplatePayload | null
  /** Public global provider catalog; defaults to 20 providers and caps pages at 100. */
  providerTemplates: ProviderTemplateConnection
  providers: ProviderConnection
  /** Find visible role metadata in the active organization. */
  role: RolePayload | null
  /** Return the complete visible role metadata collection, ordered by ID. */
  roles: RoleConnection
  /** Return the verified current session identity. */
  session: SessionPayload
  sessions: SessionConnection
  /** Return the signed-in user's personal preferences; missing rows use Core defaults without a write. */
  userPreferences: UserPreferencesPayload
  /** Read an active user profile using verified user identity. */
  userProfile: UserPayload | null
  __typename: 'Query'
}

export interface Role {
  /** Native Cedar source; template slots bind to this role and its organization. */
  cedarSource: Scalars['String'] | null
  color: Scalars['String']
  description: Scalars['String']
  id: Scalars['ID']
  name: Scalars['String']
  owner: Scalars['Boolean']
  __typename: 'Role'
}

export interface RoleConnection {
  edges: RoleEdge[]
  pageInfo: PageInfo
  __typename: 'RoleConnection'
}

export interface RoleEdge {
  cursor: Scalars['String']
  node: Role
  __typename: 'RoleEdge'
}

export interface RolePayload {
  node: Role
  __typename: 'RolePayload'
}

export interface RouteOption {
  billing: ModelBilling | null
  contextTokens: Scalars['ContextTokens'] | null
  id: Scalars['String']
  implicitCaching: Scalars['Boolean'] | null
  provider: Scalars['String']
  text: TextCapabilities
  upstreamModelId: Scalars['String']
  __typename: 'RouteOption'
}

export interface RouteSelection {
  allowFallbacks: Scalars['Boolean']
  exactProviderSelection: Scalars['Boolean']
  options: RouteOption[]
  selected: Scalars['String'] | null
  __typename: 'RouteSelection'
}

export interface SearchConfiguration {
  maxResults: Scalars['Float'] | null
  __typename: 'SearchConfiguration'
}

export interface SearchSupport {
  configuration: SearchConfiguration | null
  pricing: CapabilityPricing | null
  status: CapabilityStatus
  __typename: 'SearchSupport'
}

export interface Session {
  /** Public avatar for this user; clients show a fallback while absent or loading. */
  avatarUrl: Scalars['String'] | null
  expiresAt: Scalars['String']
  /** Login session identity, serialized as a string without numeric precision loss. */
  id: Scalars['ID']
  /** IP observed by the API connection at sign-in; it may belong to a proxy or VPN. */
  ipAddress: Scalars['String'] | null
  lastSeenAt: Scalars['String']
  /** Organization selected for this verified session. */
  organizationId: Scalars['ID']
  /** Current name of this login's organization, resolved only when selected. */
  organizationName: Scalars['String']
  passkey: PasskeySummary | null
  signedInAt: Scalars['String']
  /** User-Agent captured at sign-in, used only for session recognition. */
  userAgent: Scalars['String'] | null
  /** User owning this verified session. */
  userId: Scalars['ID']
  __typename: 'Session'
}

export interface SessionConnection {
  edges: SessionEdge[]
  pageInfo: PageInfo
  __typename: 'SessionConnection'
}

/** Timing is always returned; credential values exist only in explicit TOKEN delivery. */
export interface SessionCredentials {
  accessLifetime: TokenLifetime | null
  accessToken: Scalars['String'] | null
  passkeyChangeLifetime: TokenLifetime | null
  passkeyChangeToken: Scalars['String'] | null
  purpose: AuthenticationPurpose
  refreshLifetime: TokenLifetime | null
  refreshToken: Scalars['String'] | null
  __typename: 'SessionCredentials'
}

export interface SessionCredentialsPayload {
  node: SessionCredentials
  __typename: 'SessionCredentialsPayload'
}

export interface SessionEdge {
  cursor: Scalars['String']
  node: Session
  __typename: 'SessionEdge'
}

export interface SessionPayload {
  node: Session
  __typename: 'SessionPayload'
}

export interface SidebarEntry {
  item: PreferenceDestination
  visibility: PreferenceSidebarVisibility
  __typename: 'SidebarEntry'
}

export interface SidebarLayoutSection {
  category: PreferenceSidebarCategory
  items: PreferenceDestination[]
  __typename: 'SidebarLayoutSection'
}

export interface SidebarPreferences {
  badgeStyle: PreferenceBadgeStyle
  entries: SidebarEntry[]
  layout: SidebarLayoutSection[]
  __typename: 'SidebarPreferences'
}

export interface StructuredOutputConfiguration {
  jsonMode: FeatureSupport
  jsonSchema: FeatureSupport
  __typename: 'StructuredOutputConfiguration'
}

export interface StructuredOutputSupport {
  configuration: StructuredOutputConfiguration | null
  pricing: CapabilityPricing | null
  status: CapabilityStatus
  __typename: 'StructuredOutputSupport'
}

export interface TextCapabilities {
  input: TextInputSupport
  output: TextGenerationSupport
  __typename: 'TextCapabilities'
}

export interface TextGenerationConfiguration {
  maxTokens: Scalars['Float'] | null
  thinking: ThinkingSupport
  __typename: 'TextGenerationConfiguration'
}

export interface TextGenerationSupport {
  configuration: TextGenerationConfiguration | null
  pricing: CapabilityPricing | null
  status: CapabilityStatus
  __typename: 'TextGenerationSupport'
}

export interface TextInputConfiguration {
  maxTokens: Scalars['Float'] | null
  __typename: 'TextInputConfiguration'
}

export interface TextInputSupport {
  configuration: TextInputConfiguration | null
  pricing: CapabilityPricing | null
  status: CapabilityStatus
  __typename: 'TextInputSupport'
}

/** Text operation determining the upstream request and response format. */
export type TextModelProtocol = 'ANTHROPIC_MESSAGES' | 'OPENAI_CHAT_COMPLETIONS' | 'OPENAI_RESPONSES'

export interface ThemePreferences {
  mode: PreferenceAppearanceMode
  palettes: PalettePreferences
  __typename: 'ThemePreferences'
}

export interface ThinkingBudget {
  defaultTokens: Scalars['Float'] | null
  maxTokens: Scalars['Float']
  minTokens: Scalars['Float']
  __typename: 'ThinkingBudget'
}

export interface ThinkingConfiguration {
  budget: ThinkingBudget | null
  defaultEffort: ThinkingEffortLevel | null
  effortMapping: ThinkingEffort[]
  __typename: 'ThinkingConfiguration'
}

export interface ThinkingEffort {
  level: ThinkingEffortLevel
  value: Scalars['String']
  __typename: 'ThinkingEffort'
}

export type ThinkingEffortLevel = 'HIGH' | 'LOW' | 'MAX' | 'MEDIUM' | 'XHIGH'

export interface ThinkingSupport {
  configuration: ThinkingConfiguration | null
  pricing: CapabilityPricing | null
  status: CapabilityStatus
  __typename: 'ThinkingSupport'
}

/** Exact Unix-second validity and client throttle values encoded as decimal strings. */
export interface TokenLifetime {
  expiresAt: Scalars['String']
  expiresInSeconds: Scalars['String']
  issuedAt: Scalars['String']
  throttleSeconds: Scalars['String']
  __typename: 'TokenLifetime'
}

export interface ToolCallingConfiguration {
  maxTools: Scalars['Float'] | null
  __typename: 'ToolCallingConfiguration'
}

export interface ToolCallingSupport {
  configuration: ToolCallingConfiguration | null
  pricing: CapabilityPricing | null
  status: CapabilityStatus
  __typename: 'ToolCallingSupport'
}

export interface ToolCapabilities {
  calling: ToolCallingSupport
  forcedChoice: FeatureSupport
  parallelCalls: FeatureSupport
  __typename: 'ToolCapabilities'
}

export interface TypographyPreferences {
  family: PreferenceFontFamily
  size: PreferenceRelativeFontSize
  __typename: 'TypographyPreferences'
}

export interface UploadFormField {
  name: Scalars['String']
  value: Scalars['String']
  __typename: 'UploadFormField'
}

export type UpstreamProtocol = 'ANTHROPIC' | 'GEMINI' | 'MINIMAX' | 'OPENAI' | 'OPENAI_RESPONSES' | 'OPENROUTER' | 'XAI'

export interface User {
  /** Public avatar URL; the object may not exist yet and clients must show a fallback. */
  avatarUrl: Scalars['String'] | null
  /** Display name; this field does not establish identity or uniqueness. */
  displayName: Scalars['String']
  /** Email is readable by authenticated users and cannot be updated here. */
  email: Scalars['String']
  /** Stable user identity, serialized as a string. */
  id: Scalars['ID']
  username: Scalars['String']
  __typename: 'User'
}

export interface UserPayload {
  node: User
  __typename: 'UserPayload'
}

export type UserPreferenceGroup =
  | 'ACCESSIBILITY'
  | 'ALL'
  | 'CALENDAR'
  | 'CODE_APPEARANCE'
  | 'MEMBER_DISPLAY'
  | 'MESSAGE_COMPOSITION'
  | 'NAVIGATION'
  | 'SIDEBAR'
  | 'THEME'
  | 'TYPOGRAPHY'

export interface UserPreferences {
  accessibility: AccessibilityPreferences
  calendar: CalendarPreferences
  codeAppearance: CodeAppearancePreferences
  memberDisplay: MemberDisplayPreferences
  messageComposition: MessageCompositionPreferences
  navigation: NavigationPreferences
  sidebar: SidebarPreferences
  theme: ThemePreferences
  typography: TypographyPreferences
  __typename: 'UserPreferences'
}

export interface UserPreferencesPayload {
  node: UserPreferences
  __typename: 'UserPreferencesPayload'
}

export interface ValidationIssue {
  code: Scalars['String']
  message: Scalars['String']
  path: Scalars['String'][]
  __typename: 'ValidationIssue'
}

export interface VideoConfiguration {
  formats: Scalars['String'][] | null
  maxDurationMs: Scalars['Float'] | null
  maxFrames: Scalars['Float'] | null
  __typename: 'VideoConfiguration'
}

export interface VideoSupport {
  configuration: VideoConfiguration | null
  pricing: CapabilityPricing | null
  status: CapabilityStatus
  __typename: 'VideoSupport'
}

export interface VisionConfiguration {
  formats: Scalars['String'][] | null
  maxImages: Scalars['Float'] | null
  __typename: 'VisionConfiguration'
}

export interface VisionSupport {
  configuration: VisionConfiguration | null
  pricing: CapabilityPricing | null
  status: CapabilityStatus
  __typename: 'VisionSupport'
}

export interface AccessibilityPatchInput {
  pointerCursors?: Scalars['Boolean'] | null
  reduceMotion?: Scalars['Boolean'] | null
  underlineLinks?: Scalars['Boolean'] | null
}

export interface AccessibilityPreferencesGenqlSelection {
  pointerCursors?: boolean | number
  reduceMotion?: boolean | number
  underlineLinks?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface AnalyticUserProfileGenqlSelection {
  key?: boolean | number
  value?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface AnalyticUserProfilePayloadGenqlSelection {
  node?: AnalyticUserProfileGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface AnalyticUserProfilePutInput {
  key: Scalars['String']
  value: Scalars['String']
}

export interface AudioCapabilitiesGenqlSelection {
  input?: AudioSupportGenqlSelection
  output?: AudioSupportGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface AudioConfigurationGenqlSelection {
  formats?: boolean | number
  maxBytes?: boolean | number
  maxDurationMs?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface AudioSupportGenqlSelection {
  configuration?: AudioConfigurationGenqlSelection
  pricing?: CapabilityPricingGenqlSelection
  status?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface AuthenticationConfigurationGenqlSelection {
  configuration?: AuthenticationParametersGenqlSelection
  method?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface AuthenticationParametersGenqlSelection {
  adapter?: boolean | number
  authorizationUrl?: boolean | number
  deviceAuthorizationUrl?: boolean | number
  flow?: boolean | number
  parameters?: ConfigurationEntryGenqlSelection
  placement?: CredentialPlacementGenqlSelection
  requiredInputs?: ConfigurationInputGenqlSelection
  scopes?: boolean | number
  tokenUrl?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface AvatarUploadGenqlSelection {
  /** Expiration as Unix seconds, encoded as a string. */
  expiresAt?: boolean | number
  fields?: UploadFormFieldGenqlSelection
  url?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface AvatarUploadPayloadGenqlSelection {
  node?: AvatarUploadGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface BillingAllowanceGenqlSelection {
  quota?: boolean | number
  resetPeriod?: boolean | number
  unit?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface BillingFeeGenqlSelection {
  amount?: boolean | number
  currency?: boolean | number
  period?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

/** Public body default. Value is JSON text for the configuration editor. */
export interface BodyDefaultEntryGenqlSelection {
  key?: boolean | number
  value?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface BooleanPayloadGenqlSelection {
  node?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface CalendarPatchInput {
  firstDay?: PreferenceWeekday | null
}

export interface CalendarPreferencesGenqlSelection {
  firstDay?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface CapabilityCachePriceGenqlSelection {
  amount?: boolean | number
  currency?: boolean | number
  mode?: boolean | number
  quantity?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface CapabilityPricingGenqlSelection {
  kind?: boolean | number
  usage?: CapabilityUsageGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface CapabilityRateGenqlSelection {
  amount?: boolean | number
  cache?: CapabilityCachePriceGenqlSelection
  conditions?: ConfigurationEntryGenqlSelection
  credits?: boolean | number
  currency?: boolean | number
  direction?: boolean | number
  quantity?: boolean | number
  unit?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface CapabilitySlotGenqlSelection {
  fromHour?: boolean | number
  pricing?: CapabilityUsageGenqlSelection
  toHour?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface CapabilityTierGenqlSelection {
  fromTokens?: boolean | number
  rate?: CapabilityRateGenqlSelection
  toTokens?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface CapabilityUsageGenqlSelection {
  mode?: boolean | number
  rates?: CapabilityRateGenqlSelection
  slots?: CapabilitySlotGenqlSelection
  tierBasis?: boolean | number
  tiers?: CapabilityTierGenqlSelection
  timezone?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ChatCreateInput {
  message: Scalars['String']
  modelId: Scalars['String']
}

export interface CodeAppearancePatchInput {
  font?: CodeFontPatchInput | null
  theme?: PreferenceCodeTheme | null
}

export interface CodeAppearancePreferencesGenqlSelection {
  font?: CodeFontPreferencesGenqlSelection
  theme?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface CodeFontPatchInput {
  family?: PreferenceFontFamily | null
  size?: PreferencePixelFontSize | null
}

export interface CodeFontPreferencesGenqlSelection {
  family?: boolean | number
  size?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ConfigurationEntryGenqlSelection {
  key?: boolean | number
  value?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ConfigurationInputGenqlSelection {
  allowedValues?: boolean | number
  kind?: boolean | number
  name?: boolean | number
  required?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ConnectionConfigurationGenqlSelection {
  authentication?: AuthenticationConfigurationGenqlSelection
  baseUrl?: boolean | number
  endpoint?: EndpointRequirementsGenqlSelection
  path?: boolean | number
  query?: ConfigurationEntryGenqlSelection
  timeoutMs?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface CredentialCreateInput {
  name: Scalars['String']
}

export interface CredentialPlacementGenqlSelection {
  location?: boolean | number
  name?: boolean | number
  prefix?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

/** Email admission succeeded; cookie delivery never exposes the OTP credential. */
export interface EmailChallengeGenqlSelection {
  lifetime?: TokenLifetimeGenqlSelection
  token?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface EmailChallengePayloadGenqlSelection {
  node?: EmailChallengeGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

/**
 * Shared input for confirming a signup or signin email challenge.
 *
 * The verified encrypted challenge comes from request context, never this input.
 * The adapter calls `validify()` before Core verifies the code and challenge.
 */
export interface EmailCodeInput {
  /** Exactly six ASCII digits after trimming surrounding whitespace. */
  code: Scalars['String']
  /** Cookie delivery is the default; token delivery must be requested explicitly. */
  delivery?: CredentialDelivery | null
  purpose?: AuthenticationPurpose | null
}

/**
 * Shared input for requesting either a signup or a signin email challenge.
 *
 * Call `validify()` before Core execution. GraphQL parsing alone does not run
 * normalization, application validation, or email ownership checks.
 */
export interface EmailInput {
  /** Cookie delivery is the default; token delivery must be requested explicitly. */
  delivery?: CredentialDelivery | null
  /** Trimmed and ASCII-lowercased using the shared Rust email policy. */
  email: Scalars['String']
  invitation?: Scalars['Boolean'] | null
  purpose?: AuthenticationPurpose | null
}

export interface EndpointConfigurationGenqlSelection {
  requiredInputs?: ConfigurationInputGenqlSelection
  urlTemplate?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface EndpointRequirementsGenqlSelection {
  configuration?: EndpointConfigurationGenqlSelection
  mode?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface FeatureSupportGenqlSelection {
  pricing?: CapabilityPricingGenqlSelection
  status?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface GroupGenqlSelection {
  description?: boolean | number
  id?: boolean | number
  memberIds?: boolean | number
  name?: boolean | number
  roles?: RoleConnectionGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface GroupConnectionGenqlSelection {
  edges?: GroupEdgeGenqlSelection
  pageInfo?: PageInfoGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface GroupCreateInput {
  description: Scalars['String']
  name: Scalars['String']
  roleIds: Scalars['String'][]
}

export interface GroupEdgeGenqlSelection {
  cursor?: boolean | number
  node?: GroupGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface GroupPayloadGenqlSelection {
  node?: GroupGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface GroupUpdateInput {
  addMemberIds?: Scalars['String'][] | null
  addRoleIds?: Scalars['String'][] | null
  description?: Scalars['String'] | null
  name?: Scalars['String'] | null
  removeMemberIds?: Scalars['String'][] | null
  removeRoleIds?: Scalars['String'][] | null
}

export interface IDFilter {
  eq?: Scalars['ID'] | null
}

export interface IDPayloadGenqlSelection {
  node?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ImageGenerationConfigurationGenqlSelection {
  formats?: boolean | number
  maxImages?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ImageGenerationSupportGenqlSelection {
  configuration?: ImageGenerationConfigurationGenqlSelection
  pricing?: CapabilityPricingGenqlSelection
  status?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ImageUploadInput {
  mediaType: ImageMediaType
  size: Scalars['Int']
}

export interface InvitationGenqlSelection {
  email?: boolean | number
  expiresAt?: boolean | number
  id?: boolean | number
  inviterId?: boolean | number
  issuedAt?: boolean | number
  organizationId?: boolean | number
  status?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface InvitationBatchCreateInput {
  invitations: InvitationCreateInput[]
}

export interface InvitationConnectionGenqlSelection {
  edges?: InvitationEdgeGenqlSelection
  pageInfo?: PageInfoGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface InvitationContextGenqlSelection {
  email?: boolean | number
  expiresAt?: boolean | number
  organizationId?: boolean | number
  organizationName?: boolean | number
  signup?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface InvitationContextPayloadGenqlSelection {
  node?: InvitationContextGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface InvitationCreateInput {
  email: Scalars['String']
}

export interface InvitationEdgeGenqlSelection {
  cursor?: boolean | number
  node?: InvitationGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface InvitationIssuedGenqlSelection {
  emailAdmitted?: boolean | number
  invitation?: InvitationGenqlSelection
  url?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface InvitationIssuedConnectionGenqlSelection {
  edges?: InvitationIssuedEdgeGenqlSelection
  pageInfo?: PageInfoGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface InvitationIssuedEdgeGenqlSelection {
  cursor?: boolean | number
  node?: InvitationIssuedGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface InvitationIssuedPayloadGenqlSelection {
  node?: InvitationIssuedGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface InvitationPayloadGenqlSelection {
  node?: InvitationGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface MeGenqlSelection {
  /** Cedar JSON entity facts for the verified principal and its role/group ancestry; contains no credentials. */
  cedarEntities?: boolean | number
  /** Effective policies for this verified user only. UI decisions do not grant server authority. */
  cedarSource?: boolean | number
  id?: boolean | number
  organizationId?: boolean | number
  organizationOwner?: boolean | number
  sessionId?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface MePayloadGenqlSelection {
  node?: MeGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface MemberGenqlSelection {
  available?: boolean | number
  displayName?: boolean | number
  /** Direct and group-inherited role identities in the verified organization. */
  effectiveRoleIds?: boolean | number
  email?: boolean | number
  /** Group identities in the verified organization; no policy source is exposed. */
  groupIds?: boolean | number
  groups?: GroupGenqlSelection
  id?: boolean | number
  joinedAt?: boolean | number
  role?: RoleGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface MemberConnectionGenqlSelection {
  edges?: MemberEdgeGenqlSelection
  pageInfo?: PageInfoGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface MemberDisplayPatchInput {
  nameFormat?: PreferenceNamingFormat | null
}

export interface MemberDisplayPreferencesGenqlSelection {
  nameFormat?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface MemberEdgeGenqlSelection {
  cursor?: boolean | number
  node?: MemberGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface MemberFilterInput {
  displayName?: StringFilter | null
  email?: StringFilter | null
  or?: MemberMatchInput[] | null
  roleId?: IDFilter | null
}

export interface MemberMatchInput {
  displayName?: StringFilter | null
  email?: StringFilter | null
}

export interface MemberOrderInput {
  displayName?: OrderByDirection | null
  email?: OrderByDirection | null
  id?: OrderByDirection | null
}

export interface MessageCompositionPatchInput {
  convertEmoticons?: Scalars['Boolean'] | null
  sendShortcut?: PreferenceKeyCombination | null
}

export interface MessageCompositionPreferencesGenqlSelection {
  convertEmoticons?: boolean | number
  sendShortcut?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ModelBillingGenqlSelection {
  amountPerCredit?: boolean | number
  currency?: boolean | number
  description?: boolean | number
  entitlements?: BillingAllowanceGenqlSelection
  fee?: BillingFeeGenqlSelection
  kind?: boolean | number
  plan?: boolean | number
  product?: boolean | number
  requestRate?: CapabilityRateGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ModelConnectionGenqlSelection {
  configuration?: ConnectionConfigurationGenqlSelection
  protocol?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ModelOperationGenqlSelection {
  operation?: boolean | number
  path?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ModelTemplateGenqlSelection {
  audio?: AudioCapabilitiesGenqlSelection
  billing?: ModelBillingGenqlSelection
  billingModel?: boolean | number
  bodyDefaults?: BodyDefaultEntryGenqlSelection
  cacheBilling?: boolean | number
  cachedPrice?: boolean | number
  capabilities?: boolean | number
  connections?: ModelConnectionGenqlSelection
  customFields?: boolean | number
  description?: boolean | number
  displayName?: boolean | number
  effortMapping?: ThinkingEffortGenqlSelection
  headers?: ConfigurationEntryGenqlSelection
  id?: boolean | number
  imageGeneration?: ImageGenerationSupportGenqlSelection
  inputPrice?: boolean | number
  maxContextTokens?: boolean | number
  maxInputTokens?: boolean | number
  maxOutputTokens?: boolean | number
  name?: boolean | number
  operations?: ModelOperationGenqlSelection
  origin?: boolean | number
  outputPrice?: boolean | number
  pdf?: PdfSupportGenqlSelection
  pricingMode?: boolean | number
  providerTemplateId?: boolean | number
  routeSelection?: RouteSelectionGenqlSelection
  search?: SearchSupportGenqlSelection
  serviceKind?: boolean | number
  structuredOutput?: StructuredOutputSupportGenqlSelection
  text?: TextCapabilitiesGenqlSelection
  thinking?: ThinkingSupportGenqlSelection
  tools?: ToolCapabilitiesGenqlSelection
  upstreamModelId?: boolean | number
  video?: VideoSupportGenqlSelection
  vision?: VisionSupportGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ModelTemplateConnectionGenqlSelection {
  edges?: ModelTemplateEdgeGenqlSelection
  pageInfo?: PageInfoGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ModelTemplateCreateInput {
  configuration: ModelTemplateCustomizeInput
}

export interface ModelTemplateCustomizeInput {
  bodyDefaults?: ProviderBodyDefault[] | null
  cachedPrice?: ModelTemplatePriceInput | null
  capabilities?: ProviderModelCapability[] | null
  effortMapping?: ProviderEffortMapping | null
  headers?: ProviderHeader[] | null
  inputPrice?: ModelTemplatePriceInput | null
  limits?: ModelTemplateLimitsInput | null
  name?: Scalars['String'] | null
  outputPrice?: ModelTemplatePriceInput | null
  path?: Scalars['String'] | null
  protocol?: TextModelProtocol | null
}

export interface ModelTemplateEdgeGenqlSelection {
  cursor?: boolean | number
  node?: ModelTemplateGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ModelTemplateEffectiveLimitsInput {
  context?: Scalars['ContextTokens'] | null
  input?: Scalars['ContextTokens'] | null
  output?: Scalars['ContextTokens'] | null
}

export interface ModelTemplateFilterInput {
  displayName?: StringFilter | null
  id?: StringFilter | null
  name?: StringFilter | null
  or?: ModelTemplateMatchInput[] | null
}

export interface ModelTemplateLimitsInput {
  context?: Scalars['ContextTokens'] | null
  input?: Scalars['ContextTokens'] | null
  output?: Scalars['ContextTokens'] | null
}

export interface ModelTemplateMatchInput {
  displayName?: StringFilter | null
  id?: StringFilter | null
  name?: StringFilter | null
}

export interface ModelTemplatePayloadGenqlSelection {
  node?: ModelTemplateGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ModelTemplatePriceInput {
  enabled: Scalars['Boolean']
  value?: Scalars['String'] | null
}

export interface MutationGenqlSelection {
  analyticUserProfilePut?: AnalyticUserProfilePayloadGenqlSelection & { __args: { input: AnalyticUserProfilePutInput } }
  /** Confirm the verified signin challenge and create a scoped session. */
  emailSigninConfirm?: SessionCredentialsPayloadGenqlSelection & { __args: { input: EmailCodeInput } }
  /** Request a signin email challenge for an existing active account. */
  emailSigninRequest?: EmailChallengePayloadGenqlSelection & { __args: { input: EmailInput } }
  /** Confirm the verified signup challenge and create the account and session. */
  emailSignupConfirm?: SessionCredentialsPayloadGenqlSelection & { __args: { input: EmailCodeInput } }
  /** Request a signup email challenge; reject existing accounts before sending. */
  emailSignupRequest?: EmailChallengePayloadGenqlSelection & { __args: { input: EmailInput } }
  generateOrgAvatarUrl?: AvatarUploadPayloadGenqlSelection & { __args: { input: ImageUploadInput } }
  /** Authorize a direct JPEG/PNG/WebP avatar upload up to 5 MiB for five minutes. */
  generateUrlAvatarUrl?: AvatarUploadPayloadGenqlSelection & { __args: { input: ImageUploadInput } }
  /** Create a group and its initial role assignments in the verified active organization. */
  groupCreate?: GroupPayloadGenqlSelection & { __args: { input: GroupCreateInput } }
  groupDelete?: BooleanPayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  groupUpdate?: GroupPayloadGenqlSelection & { __args: { id: Scalars['ID']; input: GroupUpdateInput } }
  invitationCreate?: InvitationIssuedPayloadGenqlSelection & { __args: { input: InvitationCreateInput } }
  invitationCreateMany?: InvitationIssuedConnectionGenqlSelection & { __args: { input: InvitationBatchCreateInput } }
  invitationRevoke?: InvitationPayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  modelTemplateCreate?: ModelTemplatePayloadGenqlSelection & {
    __args: { input: ModelTemplateCreateInput; providerTemplateId: Scalars['ID'] }
  }
  modelTemplateCustomize?: ModelTemplatePayloadGenqlSelection & {
    __args: { id: Scalars['ID']; input: ModelTemplateCustomizeInput; providerTemplateId: Scalars['ID'] }
  }
  modelTemplateDelete?: BooleanPayloadGenqlSelection & {
    __args: { id: Scalars['ID']; providerTemplateId: Scalars['ID'] }
  }
  modelTemplateReset?: ModelTemplatePayloadGenqlSelection & {
    __args: { id: Scalars['ID']; providerTemplateId: Scalars['ID'] }
  }
  onboardingAdvance?: OnboardingContextPayloadGenqlSelection & { __args?: { back?: Scalars['Boolean'] } }
  onboardingComplete?: OnboardingContextPayloadGenqlSelection
  onboardingPresent?: OnboardingContextPayloadGenqlSelection & { __args?: { manual?: Scalars['Boolean'] } }
  organizationOnboardingSettingsUpdate?: OrganizationOnboardingSettingsPayloadGenqlSelection & {
    __args: { input: OrganizationOnboardingSettingsInput }
  }
  organizationProfileUpdate?: OrganizationPayloadGenqlSelection & { __args: { input: OrganizationProfileUpdateInput } }
  passkeyAuthenticationFinish?: SessionCredentialsPayloadGenqlSelection & { __args: { input: PasskeyFinishInput } }
  passkeyAuthenticationStart?: PasskeyCeremonyPayloadGenqlSelection & { __args?: { purpose?: AuthenticationPurpose } }
  passkeyRegistrationFinish?: PasskeySummaryPayloadGenqlSelection & {
    __args: { input: PasskeyFinishInput; name: PasskeyRegistrationInput }
  }
  passkeyRegistrationStart?: PasskeyCeremonyPayloadGenqlSelection
  passkeyRemove?: IDPayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  passkeyUpdate?: PasskeySummaryPayloadGenqlSelection & {
    __args: { id: Scalars['ID']; input: PasskeyRegistrationInput }
  }
  providerCreate?: ProviderPayloadGenqlSelection & { __args: { input: ProviderCreateFromTemplateInput } }
  providerLogoUpload?: ProviderLogoUploadPayloadGenqlSelection & { __args: { input: ImageUploadInput } }
  roleAssign?: RolePayloadGenqlSelection & { __args: { id: Scalars['ID']; memberId: Scalars['ID'] } }
  /** Create a role in the verified active organization. */
  roleCreate?: RolePayloadGenqlSelection & { __args: { input: RoleCreateInput } }
  roleDelete?: BooleanPayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  roleUpdate?: RolePayloadGenqlSelection & { __args: { id: Scalars['ID']; input: RoleUpdateInput } }
  /** Refresh using explicit refresh Bearer authority; leave browser cookies unchanged. */
  sessionRefresh?: SessionCredentialsPayloadGenqlSelection
  /** Revoke the current session and clear its cookies after success. */
  sessionRevoke?: BooleanPayloadGenqlSelection
  /** Revoke one other session belonging to the authenticated account. */
  sessionRevokeOther?: BooleanPayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  /** Revoke all other account sessions, retaining this login. */
  sessionRevokeOthers?: BooleanPayloadGenqlSelection
  /** Restore selected preference groups, or ALL alone, to Core defaults. */
  userPreferencesReset?: UserPreferencesPayloadGenqlSelection & { __args: { groups: UserPreferenceGroup[] } }
  /** Atomically patch the signed-in user's preferences, preserving omitted fields. */
  userPreferencesUpdate?: UserPreferencesPayloadGenqlSelection & { __args: { input: UserPreferencesPatchInput } }
  /** Update the current user's display name and confirm an uploaded avatar. */
  userProfileUpdate?: UserPayloadGenqlSelection & { __args: { input: UserProfileUpdateInput } }
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface NavigationPatchInput {
  home?: PreferenceDestination | null
}

export interface NavigationPreferencesGenqlSelection {
  home?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface OnboardingContextGenqlSelection {
  flow?: boolean | number
  hasBeenPresented?: boolean | number
  required?: boolean | number
  settings?: OrganizationOnboardingSettingsGenqlSelection
  shouldPresent?: boolean | number
  step?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface OnboardingContextPayloadGenqlSelection {
  node?: OnboardingContextGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface OnboardingLinkGenqlSelection {
  href?: boolean | number
  label?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface OnboardingLinkInput {
  href: Scalars['String']
  label: Scalars['String']
}

export interface OrganizationGenqlSelection {
  avatarUrl?: boolean | number
  canUpdateProfile?: boolean | number
  id?: boolean | number
  name?: boolean | number
  ownerMemberIds?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface OrganizationOnboardingSettingsGenqlSelection {
  artworkColor?: boolean | number
  destination?: boolean | number
  links?: OnboardingLinkGenqlSelection
  requireAvatar?: boolean | number
  requireFullName?: boolean | number
  welcome?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface OrganizationOnboardingSettingsInput {
  artworkColor?: Scalars['String'] | null
  destination: OnboardingDestination
  links: OnboardingLinkInput[]
  requireAvatar: Scalars['Boolean']
  requireFullName: Scalars['Boolean']
  welcome: Scalars['String']
}

export interface OrganizationOnboardingSettingsPayloadGenqlSelection {
  node?: OrganizationOnboardingSettingsGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface OrganizationPayloadGenqlSelection {
  node?: OrganizationGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface OrganizationProfileUpdateInput {
  avatarUploaded?: Scalars['Boolean'] | null
  name?: Scalars['String'] | null
}

export interface OrganizationSettingsUpdateInput {
  announcements?: Scalars['String'] | null
  fiscalMonth?: Scalars['String'] | null
  home?: HomeDestination | null
  name?: Scalars['String'] | null
  slug?: Scalars['String'] | null
  welcome?: Scalars['String'] | null
}

export interface PageInfoGenqlSelection {
  endCursor?: boolean | number
  hasNextPage?: boolean | number
  hasPreviousPage?: boolean | number
  startCursor?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface PalettePatchInput {
  dark?: PreferenceDarkPalette | null
  light?: PreferenceLightPalette | null
}

export interface PalettePreferencesGenqlSelection {
  dark?: boolean | number
  light?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface PasskeyCeremonyGenqlSelection {
  expiresAt?: boolean | number
  options?: PasskeyOptionsGenqlSelection
  token?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface PasskeyCeremonyPayloadGenqlSelection {
  node?: PasskeyCeremonyGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface PasskeyDescriptorGenqlSelection {
  id?: boolean | number
  transports?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface PasskeyExtensionsGenqlSelection {
  credProps?: boolean | number
  credentialProtectionPolicy?: boolean | number
  enforceCredentialProtectionPolicy?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface PasskeyFinishInput {
  attestationObject?: Scalars['String'] | null
  authenticatorData?: Scalars['String'] | null
  clientDataJson: Scalars['String']
  delivery?: CredentialDelivery | null
  id: Scalars['String']
  purpose?: AuthenticationPurpose | null
  rawId: Scalars['String']
  signature?: Scalars['String'] | null
  token: Scalars['String']
  transports?: Scalars['String'][] | null
  userHandle?: Scalars['String'] | null
}

export interface PasskeyOptionsGenqlSelection {
  algorithms?: boolean | number
  challenge?: boolean | number
  credentials?: PasskeyDescriptorGenqlSelection
  extensions?: PasskeyExtensionsGenqlSelection
  residentKey?: boolean | number
  rpId?: boolean | number
  rpName?: boolean | number
  timeout?: boolean | number
  userDisplayName?: boolean | number
  userId?: boolean | number
  userName?: boolean | number
  userVerification?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface PasskeyRegistrationInput {
  name: Scalars['String']
}

export interface PasskeySummaryGenqlSelection {
  credentialId?: boolean | number
  id?: boolean | number
  lastUsedAt?: boolean | number
  name?: boolean | number
  registeredAt?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface PasskeySummaryConnectionGenqlSelection {
  edges?: PasskeySummaryEdgeGenqlSelection
  pageInfo?: PageInfoGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface PasskeySummaryEdgeGenqlSelection {
  cursor?: boolean | number
  node?: PasskeySummaryGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface PasskeySummaryPayloadGenqlSelection {
  node?: PasskeySummaryGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface PdfConfigurationGenqlSelection {
  maxFiles?: boolean | number
  maxPages?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface PdfSupportGenqlSelection {
  configuration?: PdfConfigurationGenqlSelection
  pricing?: CapabilityPricingGenqlSelection
  status?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface PolicyActionGenqlSelection {
  description?: boolean | number
  displayName?: boolean | number
  entityType?: boolean | number
  name?: boolean | number
  resourceType?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface PolicyActionConnectionGenqlSelection {
  edges?: PolicyActionEdgeGenqlSelection
  pageInfo?: PageInfoGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface PolicyActionEdgeGenqlSelection {
  cursor?: boolean | number
  node?: PolicyActionGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ProfileUpdateInput {
  name?: Scalars['String'] | null
  username?: Scalars['String'] | null
}

export interface ProviderGenqlSelection {
  connection?: ConnectionConfigurationGenqlSelection
  description?: boolean | number
  /** Stable provider identity, serialized as a string. */
  id?: boolean | number
  logoUrl?: boolean | number
  models?: ProviderModelGenqlSelection
  /** Provider display name within its organization. */
  name?: boolean | number
  /** Owning organization; a supplied identifier never grants access. */
  organizationId?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ProviderBodyDefault {
  key: Scalars['String']
  value: Scalars['String']
}

/** Opaque registry identity, independent of an edited upstream model name. */
export interface ProviderCatalogRef {
  model: Scalars['String']
  provider: Scalars['String']
}

export interface ProviderConnectionGenqlSelection {
  edges?: ProviderEdgeGenqlSelection
  pageInfo?: PageInfoGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ProviderCreateFromTemplateInput {
  apiKey: Scalars['String']
  baseUrl: Scalars['String']
  headers: ProviderHeader[]
  logoUploadId?: Scalars['String'] | null
  logoUrl?: Scalars['String'] | null
  models: ProviderTemplateModelInput[]
  name: Scalars['String']
  providerTemplateId: Scalars['String']
}

/** New connection definition; credentials and header values are write-only. */
export interface ProviderCreateInput {
  apiKey: Scalars['String']
  baseUrl: Scalars['String']
  headers: ProviderHeader[]
  models: ProviderModelWrite[]
  name: Scalars['String']
}

export interface ProviderEdgeGenqlSelection {
  cursor?: boolean | number
  node?: ProviderGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

/** Empty object means no configured effort levels. Configured values cannot be empty. */
export interface ProviderEffortMapping {
  high?: Scalars['String'] | null
  low?: Scalars['String'] | null
  max?: Scalars['String'] | null
  medium?: Scalars['String'] | null
  xhigh?: Scalars['String'] | null
}

export interface ProviderHeader {
  key: Scalars['String']
  /** Write-only header content; never returned by management reads. */
  value: Scalars['String']
}

/** Omit value to retain an existing header secret; new names require a value. */
export interface ProviderHeaderUpdate {
  key: Scalars['String']
  /** Omit to preserve the saved value for this header name. New names require a value. */
  value?: Scalars['String'] | null
}

export interface ProviderLogoUploadGenqlSelection {
  id?: boolean | number
  upload?: AvatarUploadGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ProviderLogoUploadPayloadGenqlSelection {
  node?: ProviderLogoUploadGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ProviderModelGenqlSelection {
  cachedPrice?: boolean | number
  capabilities?: boolean | number
  connection?: ConnectionConfigurationGenqlSelection
  contextTokens?: boolean | number
  /** Configured availability; this flag alone does not authorize invocation. */
  enabled?: boolean | number
  /** Stable configured-model identity, serialized as a string. */
  id?: boolean | number
  inputPrice?: boolean | number
  /** Configured upstream model name. */
  name?: boolean | number
  /** Owning organization identity. */
  organizationId?: boolean | number
  outputPrice?: boolean | number
  protocol?: boolean | number
  /** Owning provider identity. */
  providerId?: boolean | number
  text?: TextCapabilitiesGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

/** Complete configured text-model values. Decimal prices are USD per million tokens. */
export interface ProviderModelConfiguration {
  bodyDefaults?: ProviderBodyDefault[]
  cachedPrice: Scalars['String']
  cachedPriceEnabled?: Scalars['Boolean']
  capabilities?: ProviderModelCapability[]
  catalogRef?: ProviderCatalogRef | null
  contextLimitEnabled?: Scalars['Boolean']
  effortMapping?: ProviderEffortMapping | null
  enabled?: Scalars['Boolean']
  headers?: ProviderHeader[]
  inputPrice: Scalars['String']
  inputPriceEnabled?: Scalars['Boolean']
  maxContextTokens: Scalars['ContextTokens']
  maxInputTokens?: Scalars['ContextTokens'] | null
  maxOutputTokens?: Scalars['ContextTokens'] | null
  name: Scalars['String']
  outputPrice: Scalars['String']
  outputPriceEnabled?: Scalars['Boolean']
  path: Scalars['String']
  protocol: TextModelProtocol
}

/** Persisted identity is omitted for new models and forbidden during provider creation. */
export interface ProviderModelWrite {
  /** Complete text-model configuration, including explicit token prices. */
  configuration: ProviderModelConfiguration
  /** Omit for a new model; supplied IDs must belong to this connection. */
  id?: Scalars['ID'] | null
}

/** Partial update. Omitted fields are unchanged; explicit null is rejected. */
export interface ProviderPatchInput {
  /** Omit to preserve the current credential. Empty strings and null are invalid. */
  apiKey?: Scalars['String'] | null
  baseUrl?: Scalars['String'] | null
  headers?: ProviderHeaderUpdate[] | null
  models?: ProviderModelWrite[] | null
  name?: Scalars['String'] | null
}

export interface ProviderPayloadGenqlSelection {
  node?: ProviderGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

/** Full non-secret replacement. Omitting apiKey preserves the saved credential. */
export interface ProviderReplaceInput {
  /** Omit to preserve the current credential. Empty strings and null are invalid. */
  apiKey?: Scalars['String'] | null
  baseUrl: Scalars['String']
  headers: ProviderHeaderUpdate[]
  models: ProviderModelWrite[]
  name: Scalars['String']
}

export interface ProviderTemplateGenqlSelection {
  authentication?: AuthenticationConfigurationGenqlSelection
  baseUrl?: boolean | number
  connection?: ConnectionConfigurationGenqlSelection
  description?: boolean | number
  endpoint?: EndpointRequirementsGenqlSelection
  headers?: ConfigurationEntryGenqlSelection
  id?: boolean | number
  logoUrl?: boolean | number
  models?: ModelTemplateConnectionGenqlSelection & { __args?: { filter?: ModelTemplateFilterInput | null } }
  name?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ProviderTemplateConnectionGenqlSelection {
  edges?: ProviderTemplateEdgeGenqlSelection
  pageInfo?: PageInfoGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ProviderTemplateEdgeGenqlSelection {
  cursor?: boolean | number
  node?: ProviderTemplateGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ProviderTemplateFilterInput {
  id?: StringFilter | null
  name?: StringFilter | null
  or?: ProviderTemplateMatchInput[] | null
}

export interface ProviderTemplateMatchInput {
  id?: StringFilter | null
  name?: StringFilter | null
}

export interface ProviderTemplateModelInput {
  id: Scalars['String']
  protocol: TextModelProtocol
}

export interface ProviderTemplateOrderInput {
  id?: OrderByDirection | null
  name?: OrderByDirection | null
}

export interface ProviderTemplatePayloadGenqlSelection {
  node?: ProviderTemplateGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

/** Atomic batch of new provider connections for the verified organization. */
export interface ProvidersCreateInput {
  providers: ProviderCreateInput[]
}

export interface QueryGenqlSelection {
  analyticUserProfile?: AnalyticUserProfilePayloadGenqlSelection & { __args: { key: Scalars['String'] } }
  effectiveModelTemplate?: ModelTemplatePayloadGenqlSelection & {
    __args: { id: Scalars['ID']; providerTemplateId: Scalars['ID'] }
  }
  effectiveProviderTemplate?: ProviderTemplatePayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  effectiveProviderTemplates?: ProviderTemplateConnectionGenqlSelection & {
    __args?: { after?: Scalars['String'] | null; first?: Scalars['Int'] | null; search?: Scalars['String'] | null }
  }
  /** Find visible group metadata in the active organization. */
  group?: GroupPayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  /** Return the complete visible group metadata collection, ordered by ID. */
  groups?: GroupConnectionGenqlSelection
  invitationContext?: InvitationContextPayloadGenqlSelection
  invitations?: InvitationConnectionGenqlSelection & {
    __args?: { after?: Scalars['String'] | null; first?: Scalars['Int'] | null }
  }
  me?: MePayloadGenqlSelection
  members?: MemberConnectionGenqlSelection & {
    __args?: {
      after?: Scalars['String'] | null
      before?: Scalars['String'] | null
      currentUserFirst?: Scalars['Boolean'] | null
      filter?: MemberFilterInput | null
      first?: Scalars['Int'] | null
      last?: Scalars['Int'] | null
      offset?: Scalars['Int'] | null
      orderBy?: MemberOrderInput[] | null
    }
  }
  /** Public configuration defaults for one database-ready model, identified within its provider. */
  modelTemplate?: ModelTemplatePayloadGenqlSelection & {
    __args: { id: Scalars['ID']; providerTemplateId: Scalars['ID'] }
  }
  onboardingContext?: OnboardingContextPayloadGenqlSelection
  /** Return the active organization from verified user authority. */
  organization?: OrganizationPayloadGenqlSelection
  organizationOnboardingSettings?: OrganizationOnboardingSettingsPayloadGenqlSelection
  /** Return the system-owned organization policy action catalog. */
  organizationPolicyActions?: PolicyActionConnectionGenqlSelection
  organizationProfile?: OrganizationPayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  passkeys?: PasskeySummaryConnectionGenqlSelection
  provider?: ProviderPayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  /** Public provider defaults with a complete, unpaginated collection of database-ready models. */
  providerTemplate?: ProviderTemplatePayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  /** Public global provider catalog; defaults to 20 providers and caps pages at 100. */
  providerTemplates?: ProviderTemplateConnectionGenqlSelection & {
    __args?: {
      after?: Scalars['String'] | null
      before?: Scalars['String'] | null
      filter?: ProviderTemplateFilterInput | null
      first?: Scalars['Int'] | null
      last?: Scalars['Int'] | null
      offset?: Scalars['Int'] | null
      orderBy?: ProviderTemplateOrderInput[] | null
      search?: Scalars['String'] | null
    }
  }
  providers?: ProviderConnectionGenqlSelection & {
    __args?: { after?: Scalars['String'] | null; first?: Scalars['Int'] | null }
  }
  /** Find visible role metadata in the active organization. */
  role?: RolePayloadGenqlSelection & { __args: { id: Scalars['ID']; includeUnseen?: Scalars['Boolean'] } }
  /** Return the complete visible role metadata collection, ordered by ID. */
  roles?: RoleConnectionGenqlSelection & { __args?: { includeUnseen?: Scalars['Boolean'] } }
  /** Return the verified current session identity. */
  session?: SessionPayloadGenqlSelection
  sessions?: SessionConnectionGenqlSelection & {
    __args?: { after?: Scalars['String'] | null; first?: Scalars['Int'] | null }
  }
  /** Return the signed-in user's personal preferences; missing rows use Core defaults without a write. */
  userPreferences?: UserPreferencesPayloadGenqlSelection
  /** Read an active user profile using verified user identity. */
  userProfile?: UserPayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ReportingScheduleInput {
  timezone: Scalars['String']
}

export interface RoleGenqlSelection {
  /** Native Cedar source; template slots bind to this role and its organization. */
  cedarSource?: boolean | number
  color?: boolean | number
  description?: boolean | number
  id?: boolean | number
  name?: boolean | number
  owner?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface RoleConnectionGenqlSelection {
  edges?: RoleEdgeGenqlSelection
  pageInfo?: PageInfoGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface RoleCreateInput {
  /** Comments and unnecessary whitespace are removed before the length limit is checked. */
  cedarSource: Scalars['String']
  description: Scalars['String']
  name: Scalars['String']
}

export interface RoleEdgeGenqlSelection {
  cursor?: boolean | number
  node?: RoleGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface RolePayloadGenqlSelection {
  node?: RoleGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface RoleUpdateInput {
  /** Comments and unnecessary whitespace are removed before the length limit is checked. */
  cedarSource?: Scalars['String'] | null
  description?: Scalars['String'] | null
  name?: Scalars['String'] | null
}

export interface RouteOptionGenqlSelection {
  billing?: ModelBillingGenqlSelection
  contextTokens?: boolean | number
  id?: boolean | number
  implicitCaching?: boolean | number
  provider?: boolean | number
  text?: TextCapabilitiesGenqlSelection
  upstreamModelId?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface RouteSelectionGenqlSelection {
  allowFallbacks?: boolean | number
  exactProviderSelection?: boolean | number
  options?: RouteOptionGenqlSelection
  selected?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface SearchConfigurationGenqlSelection {
  maxResults?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface SearchSupportGenqlSelection {
  configuration?: SearchConfigurationGenqlSelection
  pricing?: CapabilityPricingGenqlSelection
  status?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface SessionGenqlSelection {
  /** Public avatar for this user; clients show a fallback while absent or loading. */
  avatarUrl?: boolean | number
  expiresAt?: boolean | number
  /** Login session identity, serialized as a string without numeric precision loss. */
  id?: boolean | number
  /** IP observed by the API connection at sign-in; it may belong to a proxy or VPN. */
  ipAddress?: boolean | number
  lastSeenAt?: boolean | number
  /** Organization selected for this verified session. */
  organizationId?: boolean | number
  /** Current name of this login's organization, resolved only when selected. */
  organizationName?: boolean | number
  passkey?: PasskeySummaryGenqlSelection
  signedInAt?: boolean | number
  /** User-Agent captured at sign-in, used only for session recognition. */
  userAgent?: boolean | number
  /** User owning this verified session. */
  userId?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface SessionConnectionGenqlSelection {
  edges?: SessionEdgeGenqlSelection
  pageInfo?: PageInfoGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

/** Timing is always returned; credential values exist only in explicit TOKEN delivery. */
export interface SessionCredentialsGenqlSelection {
  accessLifetime?: TokenLifetimeGenqlSelection
  accessToken?: boolean | number
  passkeyChangeLifetime?: TokenLifetimeGenqlSelection
  passkeyChangeToken?: boolean | number
  purpose?: boolean | number
  refreshLifetime?: TokenLifetimeGenqlSelection
  refreshToken?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface SessionCredentialsPayloadGenqlSelection {
  node?: SessionCredentialsGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface SessionEdgeGenqlSelection {
  cursor?: boolean | number
  node?: SessionGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface SessionPayloadGenqlSelection {
  node?: SessionGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface SidebarEntryGenqlSelection {
  item?: boolean | number
  visibility?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface SidebarLayoutSectionGenqlSelection {
  category?: boolean | number
  items?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface SidebarLayoutSectionInput {
  category: PreferenceSidebarCategory
  items: PreferenceDestination[]
}

/** Order replaces ordering while preserving visibility; visibility patches are keyed by destination. */
export interface SidebarPatchInput {
  badgeStyle?: PreferenceBadgeStyle | null
  layout?: SidebarLayoutSectionInput[] | null
  order?: PreferenceDestination[] | null
  visibility?: SidebarVisibilityPatchInput[] | null
}

export interface SidebarPreferencesGenqlSelection {
  badgeStyle?: boolean | number
  entries?: SidebarEntryGenqlSelection
  layout?: SidebarLayoutSectionGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface SidebarVisibilityPatchInput {
  item: PreferenceDestination
  visibility: PreferenceSidebarVisibility
}

export interface SpendingLimitInput {
  amount?: Scalars['String'] | null
}

export interface StringFilter {
  eq?: Scalars['String'] | null
  gt?: Scalars['String'] | null
  gte?: Scalars['String'] | null
  ilike?: Scalars['String'] | null
  in?: Scalars['String'][] | null
  iregex?: Scalars['String'] | null
  is?: FilterIs | null
  like?: Scalars['String'] | null
  lt?: Scalars['String'] | null
  lte?: Scalars['String'] | null
  neq?: Scalars['String'] | null
  regex?: Scalars['String'] | null
  startsWith?: Scalars['String'] | null
}

export interface StructuredOutputConfigurationGenqlSelection {
  jsonMode?: FeatureSupportGenqlSelection
  jsonSchema?: FeatureSupportGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface StructuredOutputSupportGenqlSelection {
  configuration?: StructuredOutputConfigurationGenqlSelection
  pricing?: CapabilityPricingGenqlSelection
  status?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TextCapabilitiesGenqlSelection {
  input?: TextInputSupportGenqlSelection
  output?: TextGenerationSupportGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TextGenerationConfigurationGenqlSelection {
  maxTokens?: boolean | number
  thinking?: ThinkingSupportGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TextGenerationSupportGenqlSelection {
  configuration?: TextGenerationConfigurationGenqlSelection
  pricing?: CapabilityPricingGenqlSelection
  status?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TextInputConfigurationGenqlSelection {
  maxTokens?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TextInputSupportGenqlSelection {
  configuration?: TextInputConfigurationGenqlSelection
  pricing?: CapabilityPricingGenqlSelection
  status?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ThemePatchInput {
  mode?: PreferenceAppearanceMode | null
  palettes?: PalettePatchInput | null
}

export interface ThemePreferencesGenqlSelection {
  mode?: boolean | number
  palettes?: PalettePreferencesGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ThinkingBudgetGenqlSelection {
  defaultTokens?: boolean | number
  maxTokens?: boolean | number
  minTokens?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ThinkingConfigurationGenqlSelection {
  budget?: ThinkingBudgetGenqlSelection
  defaultEffort?: boolean | number
  effortMapping?: ThinkingEffortGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ThinkingEffortGenqlSelection {
  level?: boolean | number
  value?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ThinkingSupportGenqlSelection {
  configuration?: ThinkingConfigurationGenqlSelection
  pricing?: CapabilityPricingGenqlSelection
  status?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

/** Exact Unix-second validity and client throttle values encoded as decimal strings. */
export interface TokenLifetimeGenqlSelection {
  expiresAt?: boolean | number
  expiresInSeconds?: boolean | number
  issuedAt?: boolean | number
  throttleSeconds?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ToolCallingConfigurationGenqlSelection {
  maxTools?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ToolCallingSupportGenqlSelection {
  configuration?: ToolCallingConfigurationGenqlSelection
  pricing?: CapabilityPricingGenqlSelection
  status?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ToolCapabilitiesGenqlSelection {
  calling?: ToolCallingSupportGenqlSelection
  forcedChoice?: FeatureSupportGenqlSelection
  parallelCalls?: FeatureSupportGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TypographyPatchInput {
  family?: PreferenceFontFamily | null
  size?: PreferenceRelativeFontSize | null
}

export interface TypographyPreferencesGenqlSelection {
  family?: boolean | number
  size?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface UploadFormFieldGenqlSelection {
  name?: boolean | number
  value?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface UserGenqlSelection {
  /** Public avatar URL; the object may not exist yet and clients must show a fallback. */
  avatarUrl?: boolean | number
  /** Display name; this field does not establish identity or uniqueness. */
  displayName?: boolean | number
  /** Email is readable by authenticated users and cannot be updated here. */
  email?: boolean | number
  /** Stable user identity, serialized as a string. */
  id?: boolean | number
  username?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface UserPayloadGenqlSelection {
  node?: UserGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface UserPreferencesGenqlSelection {
  accessibility?: AccessibilityPreferencesGenqlSelection
  calendar?: CalendarPreferencesGenqlSelection
  codeAppearance?: CodeAppearancePreferencesGenqlSelection
  memberDisplay?: MemberDisplayPreferencesGenqlSelection
  messageComposition?: MessageCompositionPreferencesGenqlSelection
  navigation?: NavigationPreferencesGenqlSelection
  sidebar?: SidebarPreferencesGenqlSelection
  theme?: ThemePreferencesGenqlSelection
  typography?: TypographyPreferencesGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

/** Partial personal preferences. Omitted fields are preserved; explicit null is rejected. */
export interface UserPreferencesPatchInput {
  accessibility?: AccessibilityPatchInput | null
  calendar?: CalendarPatchInput | null
  codeAppearance?: CodeAppearancePatchInput | null
  memberDisplay?: MemberDisplayPatchInput | null
  messageComposition?: MessageCompositionPatchInput | null
  navigation?: NavigationPatchInput | null
  sidebar?: SidebarPatchInput | null
  theme?: ThemePatchInput | null
  typography?: TypographyPatchInput | null
}

export interface UserPreferencesPayloadGenqlSelection {
  node?: UserPreferencesGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface UserProfileUpdateInput {
  avatarUploaded?: Scalars['Boolean'] | null
  displayName?: Scalars['String'] | null
  username?: Scalars['String'] | null
}

export interface ValidationIssueGenqlSelection {
  code?: boolean | number
  message?: boolean | number
  path?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface VideoConfigurationGenqlSelection {
  formats?: boolean | number
  maxDurationMs?: boolean | number
  maxFrames?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface VideoSupportGenqlSelection {
  configuration?: VideoConfigurationGenqlSelection
  pricing?: CapabilityPricingGenqlSelection
  status?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface VisionConfigurationGenqlSelection {
  formats?: boolean | number
  maxImages?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface VisionSupportGenqlSelection {
  configuration?: VisionConfigurationGenqlSelection
  pricing?: CapabilityPricingGenqlSelection
  status?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

const AccessibilityPreferences_possibleTypes: string[] = ['AccessibilityPreferences']
export const isAccessibilityPreferences = (obj?: { __typename?: any } | null): obj is AccessibilityPreferences => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isAccessibilityPreferences"')
  return AccessibilityPreferences_possibleTypes.includes(obj.__typename)
}

const AnalyticUserProfile_possibleTypes: string[] = ['AnalyticUserProfile']
export const isAnalyticUserProfile = (obj?: { __typename?: any } | null): obj is AnalyticUserProfile => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isAnalyticUserProfile"')
  return AnalyticUserProfile_possibleTypes.includes(obj.__typename)
}

const AnalyticUserProfilePayload_possibleTypes: string[] = ['AnalyticUserProfilePayload']
export const isAnalyticUserProfilePayload = (obj?: { __typename?: any } | null): obj is AnalyticUserProfilePayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isAnalyticUserProfilePayload"')
  return AnalyticUserProfilePayload_possibleTypes.includes(obj.__typename)
}

const AudioCapabilities_possibleTypes: string[] = ['AudioCapabilities']
export const isAudioCapabilities = (obj?: { __typename?: any } | null): obj is AudioCapabilities => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isAudioCapabilities"')
  return AudioCapabilities_possibleTypes.includes(obj.__typename)
}

const AudioConfiguration_possibleTypes: string[] = ['AudioConfiguration']
export const isAudioConfiguration = (obj?: { __typename?: any } | null): obj is AudioConfiguration => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isAudioConfiguration"')
  return AudioConfiguration_possibleTypes.includes(obj.__typename)
}

const AudioSupport_possibleTypes: string[] = ['AudioSupport']
export const isAudioSupport = (obj?: { __typename?: any } | null): obj is AudioSupport => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isAudioSupport"')
  return AudioSupport_possibleTypes.includes(obj.__typename)
}

const AuthenticationConfiguration_possibleTypes: string[] = ['AuthenticationConfiguration']
export const isAuthenticationConfiguration = (
  obj?: { __typename?: any } | null,
): obj is AuthenticationConfiguration => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isAuthenticationConfiguration"')
  return AuthenticationConfiguration_possibleTypes.includes(obj.__typename)
}

const AuthenticationParameters_possibleTypes: string[] = ['AuthenticationParameters']
export const isAuthenticationParameters = (obj?: { __typename?: any } | null): obj is AuthenticationParameters => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isAuthenticationParameters"')
  return AuthenticationParameters_possibleTypes.includes(obj.__typename)
}

const AvatarUpload_possibleTypes: string[] = ['AvatarUpload']
export const isAvatarUpload = (obj?: { __typename?: any } | null): obj is AvatarUpload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isAvatarUpload"')
  return AvatarUpload_possibleTypes.includes(obj.__typename)
}

const AvatarUploadPayload_possibleTypes: string[] = ['AvatarUploadPayload']
export const isAvatarUploadPayload = (obj?: { __typename?: any } | null): obj is AvatarUploadPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isAvatarUploadPayload"')
  return AvatarUploadPayload_possibleTypes.includes(obj.__typename)
}

const BillingAllowance_possibleTypes: string[] = ['BillingAllowance']
export const isBillingAllowance = (obj?: { __typename?: any } | null): obj is BillingAllowance => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isBillingAllowance"')
  return BillingAllowance_possibleTypes.includes(obj.__typename)
}

const BillingFee_possibleTypes: string[] = ['BillingFee']
export const isBillingFee = (obj?: { __typename?: any } | null): obj is BillingFee => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isBillingFee"')
  return BillingFee_possibleTypes.includes(obj.__typename)
}

const BodyDefaultEntry_possibleTypes: string[] = ['BodyDefaultEntry']
export const isBodyDefaultEntry = (obj?: { __typename?: any } | null): obj is BodyDefaultEntry => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isBodyDefaultEntry"')
  return BodyDefaultEntry_possibleTypes.includes(obj.__typename)
}

const BooleanPayload_possibleTypes: string[] = ['BooleanPayload']
export const isBooleanPayload = (obj?: { __typename?: any } | null): obj is BooleanPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isBooleanPayload"')
  return BooleanPayload_possibleTypes.includes(obj.__typename)
}

const CalendarPreferences_possibleTypes: string[] = ['CalendarPreferences']
export const isCalendarPreferences = (obj?: { __typename?: any } | null): obj is CalendarPreferences => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isCalendarPreferences"')
  return CalendarPreferences_possibleTypes.includes(obj.__typename)
}

const CapabilityCachePrice_possibleTypes: string[] = ['CapabilityCachePrice']
export const isCapabilityCachePrice = (obj?: { __typename?: any } | null): obj is CapabilityCachePrice => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isCapabilityCachePrice"')
  return CapabilityCachePrice_possibleTypes.includes(obj.__typename)
}

const CapabilityPricing_possibleTypes: string[] = ['CapabilityPricing']
export const isCapabilityPricing = (obj?: { __typename?: any } | null): obj is CapabilityPricing => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isCapabilityPricing"')
  return CapabilityPricing_possibleTypes.includes(obj.__typename)
}

const CapabilityRate_possibleTypes: string[] = ['CapabilityRate']
export const isCapabilityRate = (obj?: { __typename?: any } | null): obj is CapabilityRate => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isCapabilityRate"')
  return CapabilityRate_possibleTypes.includes(obj.__typename)
}

const CapabilitySlot_possibleTypes: string[] = ['CapabilitySlot']
export const isCapabilitySlot = (obj?: { __typename?: any } | null): obj is CapabilitySlot => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isCapabilitySlot"')
  return CapabilitySlot_possibleTypes.includes(obj.__typename)
}

const CapabilityTier_possibleTypes: string[] = ['CapabilityTier']
export const isCapabilityTier = (obj?: { __typename?: any } | null): obj is CapabilityTier => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isCapabilityTier"')
  return CapabilityTier_possibleTypes.includes(obj.__typename)
}

const CapabilityUsage_possibleTypes: string[] = ['CapabilityUsage']
export const isCapabilityUsage = (obj?: { __typename?: any } | null): obj is CapabilityUsage => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isCapabilityUsage"')
  return CapabilityUsage_possibleTypes.includes(obj.__typename)
}

const CodeAppearancePreferences_possibleTypes: string[] = ['CodeAppearancePreferences']
export const isCodeAppearancePreferences = (obj?: { __typename?: any } | null): obj is CodeAppearancePreferences => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isCodeAppearancePreferences"')
  return CodeAppearancePreferences_possibleTypes.includes(obj.__typename)
}

const CodeFontPreferences_possibleTypes: string[] = ['CodeFontPreferences']
export const isCodeFontPreferences = (obj?: { __typename?: any } | null): obj is CodeFontPreferences => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isCodeFontPreferences"')
  return CodeFontPreferences_possibleTypes.includes(obj.__typename)
}

const ConfigurationEntry_possibleTypes: string[] = ['ConfigurationEntry']
export const isConfigurationEntry = (obj?: { __typename?: any } | null): obj is ConfigurationEntry => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isConfigurationEntry"')
  return ConfigurationEntry_possibleTypes.includes(obj.__typename)
}

const ConfigurationInput_possibleTypes: string[] = ['ConfigurationInput']
export const isConfigurationInput = (obj?: { __typename?: any } | null): obj is ConfigurationInput => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isConfigurationInput"')
  return ConfigurationInput_possibleTypes.includes(obj.__typename)
}

const ConnectionConfiguration_possibleTypes: string[] = ['ConnectionConfiguration']
export const isConnectionConfiguration = (obj?: { __typename?: any } | null): obj is ConnectionConfiguration => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isConnectionConfiguration"')
  return ConnectionConfiguration_possibleTypes.includes(obj.__typename)
}

const CredentialPlacement_possibleTypes: string[] = ['CredentialPlacement']
export const isCredentialPlacement = (obj?: { __typename?: any } | null): obj is CredentialPlacement => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isCredentialPlacement"')
  return CredentialPlacement_possibleTypes.includes(obj.__typename)
}

const EmailChallenge_possibleTypes: string[] = ['EmailChallenge']
export const isEmailChallenge = (obj?: { __typename?: any } | null): obj is EmailChallenge => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isEmailChallenge"')
  return EmailChallenge_possibleTypes.includes(obj.__typename)
}

const EmailChallengePayload_possibleTypes: string[] = ['EmailChallengePayload']
export const isEmailChallengePayload = (obj?: { __typename?: any } | null): obj is EmailChallengePayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isEmailChallengePayload"')
  return EmailChallengePayload_possibleTypes.includes(obj.__typename)
}

const EndpointConfiguration_possibleTypes: string[] = ['EndpointConfiguration']
export const isEndpointConfiguration = (obj?: { __typename?: any } | null): obj is EndpointConfiguration => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isEndpointConfiguration"')
  return EndpointConfiguration_possibleTypes.includes(obj.__typename)
}

const EndpointRequirements_possibleTypes: string[] = ['EndpointRequirements']
export const isEndpointRequirements = (obj?: { __typename?: any } | null): obj is EndpointRequirements => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isEndpointRequirements"')
  return EndpointRequirements_possibleTypes.includes(obj.__typename)
}

const FeatureSupport_possibleTypes: string[] = ['FeatureSupport']
export const isFeatureSupport = (obj?: { __typename?: any } | null): obj is FeatureSupport => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isFeatureSupport"')
  return FeatureSupport_possibleTypes.includes(obj.__typename)
}

const Group_possibleTypes: string[] = ['Group']
export const isGroup = (obj?: { __typename?: any } | null): obj is Group => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isGroup"')
  return Group_possibleTypes.includes(obj.__typename)
}

const GroupConnection_possibleTypes: string[] = ['GroupConnection']
export const isGroupConnection = (obj?: { __typename?: any } | null): obj is GroupConnection => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isGroupConnection"')
  return GroupConnection_possibleTypes.includes(obj.__typename)
}

const GroupEdge_possibleTypes: string[] = ['GroupEdge']
export const isGroupEdge = (obj?: { __typename?: any } | null): obj is GroupEdge => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isGroupEdge"')
  return GroupEdge_possibleTypes.includes(obj.__typename)
}

const GroupPayload_possibleTypes: string[] = ['GroupPayload']
export const isGroupPayload = (obj?: { __typename?: any } | null): obj is GroupPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isGroupPayload"')
  return GroupPayload_possibleTypes.includes(obj.__typename)
}

const IDPayload_possibleTypes: string[] = ['IDPayload']
export const isIDPayload = (obj?: { __typename?: any } | null): obj is IDPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isIDPayload"')
  return IDPayload_possibleTypes.includes(obj.__typename)
}

const ImageGenerationConfiguration_possibleTypes: string[] = ['ImageGenerationConfiguration']
export const isImageGenerationConfiguration = (
  obj?: { __typename?: any } | null,
): obj is ImageGenerationConfiguration => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isImageGenerationConfiguration"')
  return ImageGenerationConfiguration_possibleTypes.includes(obj.__typename)
}

const ImageGenerationSupport_possibleTypes: string[] = ['ImageGenerationSupport']
export const isImageGenerationSupport = (obj?: { __typename?: any } | null): obj is ImageGenerationSupport => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isImageGenerationSupport"')
  return ImageGenerationSupport_possibleTypes.includes(obj.__typename)
}

const Invitation_possibleTypes: string[] = ['Invitation']
export const isInvitation = (obj?: { __typename?: any } | null): obj is Invitation => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isInvitation"')
  return Invitation_possibleTypes.includes(obj.__typename)
}

const InvitationConnection_possibleTypes: string[] = ['InvitationConnection']
export const isInvitationConnection = (obj?: { __typename?: any } | null): obj is InvitationConnection => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isInvitationConnection"')
  return InvitationConnection_possibleTypes.includes(obj.__typename)
}

const InvitationContext_possibleTypes: string[] = ['InvitationContext']
export const isInvitationContext = (obj?: { __typename?: any } | null): obj is InvitationContext => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isInvitationContext"')
  return InvitationContext_possibleTypes.includes(obj.__typename)
}

const InvitationContextPayload_possibleTypes: string[] = ['InvitationContextPayload']
export const isInvitationContextPayload = (obj?: { __typename?: any } | null): obj is InvitationContextPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isInvitationContextPayload"')
  return InvitationContextPayload_possibleTypes.includes(obj.__typename)
}

const InvitationEdge_possibleTypes: string[] = ['InvitationEdge']
export const isInvitationEdge = (obj?: { __typename?: any } | null): obj is InvitationEdge => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isInvitationEdge"')
  return InvitationEdge_possibleTypes.includes(obj.__typename)
}

const InvitationIssued_possibleTypes: string[] = ['InvitationIssued']
export const isInvitationIssued = (obj?: { __typename?: any } | null): obj is InvitationIssued => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isInvitationIssued"')
  return InvitationIssued_possibleTypes.includes(obj.__typename)
}

const InvitationIssuedConnection_possibleTypes: string[] = ['InvitationIssuedConnection']
export const isInvitationIssuedConnection = (obj?: { __typename?: any } | null): obj is InvitationIssuedConnection => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isInvitationIssuedConnection"')
  return InvitationIssuedConnection_possibleTypes.includes(obj.__typename)
}

const InvitationIssuedEdge_possibleTypes: string[] = ['InvitationIssuedEdge']
export const isInvitationIssuedEdge = (obj?: { __typename?: any } | null): obj is InvitationIssuedEdge => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isInvitationIssuedEdge"')
  return InvitationIssuedEdge_possibleTypes.includes(obj.__typename)
}

const InvitationIssuedPayload_possibleTypes: string[] = ['InvitationIssuedPayload']
export const isInvitationIssuedPayload = (obj?: { __typename?: any } | null): obj is InvitationIssuedPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isInvitationIssuedPayload"')
  return InvitationIssuedPayload_possibleTypes.includes(obj.__typename)
}

const InvitationPayload_possibleTypes: string[] = ['InvitationPayload']
export const isInvitationPayload = (obj?: { __typename?: any } | null): obj is InvitationPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isInvitationPayload"')
  return InvitationPayload_possibleTypes.includes(obj.__typename)
}

const Me_possibleTypes: string[] = ['Me']
export const isMe = (obj?: { __typename?: any } | null): obj is Me => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isMe"')
  return Me_possibleTypes.includes(obj.__typename)
}

const MePayload_possibleTypes: string[] = ['MePayload']
export const isMePayload = (obj?: { __typename?: any } | null): obj is MePayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isMePayload"')
  return MePayload_possibleTypes.includes(obj.__typename)
}

const Member_possibleTypes: string[] = ['Member']
export const isMember = (obj?: { __typename?: any } | null): obj is Member => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isMember"')
  return Member_possibleTypes.includes(obj.__typename)
}

const MemberConnection_possibleTypes: string[] = ['MemberConnection']
export const isMemberConnection = (obj?: { __typename?: any } | null): obj is MemberConnection => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isMemberConnection"')
  return MemberConnection_possibleTypes.includes(obj.__typename)
}

const MemberDisplayPreferences_possibleTypes: string[] = ['MemberDisplayPreferences']
export const isMemberDisplayPreferences = (obj?: { __typename?: any } | null): obj is MemberDisplayPreferences => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isMemberDisplayPreferences"')
  return MemberDisplayPreferences_possibleTypes.includes(obj.__typename)
}

const MemberEdge_possibleTypes: string[] = ['MemberEdge']
export const isMemberEdge = (obj?: { __typename?: any } | null): obj is MemberEdge => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isMemberEdge"')
  return MemberEdge_possibleTypes.includes(obj.__typename)
}

const MessageCompositionPreferences_possibleTypes: string[] = ['MessageCompositionPreferences']
export const isMessageCompositionPreferences = (
  obj?: { __typename?: any } | null,
): obj is MessageCompositionPreferences => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isMessageCompositionPreferences"')
  return MessageCompositionPreferences_possibleTypes.includes(obj.__typename)
}

const ModelBilling_possibleTypes: string[] = ['ModelBilling']
export const isModelBilling = (obj?: { __typename?: any } | null): obj is ModelBilling => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isModelBilling"')
  return ModelBilling_possibleTypes.includes(obj.__typename)
}

const ModelConnection_possibleTypes: string[] = ['ModelConnection']
export const isModelConnection = (obj?: { __typename?: any } | null): obj is ModelConnection => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isModelConnection"')
  return ModelConnection_possibleTypes.includes(obj.__typename)
}

const ModelOperation_possibleTypes: string[] = ['ModelOperation']
export const isModelOperation = (obj?: { __typename?: any } | null): obj is ModelOperation => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isModelOperation"')
  return ModelOperation_possibleTypes.includes(obj.__typename)
}

const ModelTemplate_possibleTypes: string[] = ['ModelTemplate']
export const isModelTemplate = (obj?: { __typename?: any } | null): obj is ModelTemplate => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isModelTemplate"')
  return ModelTemplate_possibleTypes.includes(obj.__typename)
}

const ModelTemplateConnection_possibleTypes: string[] = ['ModelTemplateConnection']
export const isModelTemplateConnection = (obj?: { __typename?: any } | null): obj is ModelTemplateConnection => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isModelTemplateConnection"')
  return ModelTemplateConnection_possibleTypes.includes(obj.__typename)
}

const ModelTemplateEdge_possibleTypes: string[] = ['ModelTemplateEdge']
export const isModelTemplateEdge = (obj?: { __typename?: any } | null): obj is ModelTemplateEdge => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isModelTemplateEdge"')
  return ModelTemplateEdge_possibleTypes.includes(obj.__typename)
}

const ModelTemplatePayload_possibleTypes: string[] = ['ModelTemplatePayload']
export const isModelTemplatePayload = (obj?: { __typename?: any } | null): obj is ModelTemplatePayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isModelTemplatePayload"')
  return ModelTemplatePayload_possibleTypes.includes(obj.__typename)
}

const Mutation_possibleTypes: string[] = ['Mutation']
export const isMutation = (obj?: { __typename?: any } | null): obj is Mutation => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isMutation"')
  return Mutation_possibleTypes.includes(obj.__typename)
}

const NavigationPreferences_possibleTypes: string[] = ['NavigationPreferences']
export const isNavigationPreferences = (obj?: { __typename?: any } | null): obj is NavigationPreferences => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isNavigationPreferences"')
  return NavigationPreferences_possibleTypes.includes(obj.__typename)
}

const OnboardingContext_possibleTypes: string[] = ['OnboardingContext']
export const isOnboardingContext = (obj?: { __typename?: any } | null): obj is OnboardingContext => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isOnboardingContext"')
  return OnboardingContext_possibleTypes.includes(obj.__typename)
}

const OnboardingContextPayload_possibleTypes: string[] = ['OnboardingContextPayload']
export const isOnboardingContextPayload = (obj?: { __typename?: any } | null): obj is OnboardingContextPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isOnboardingContextPayload"')
  return OnboardingContextPayload_possibleTypes.includes(obj.__typename)
}

const OnboardingLink_possibleTypes: string[] = ['OnboardingLink']
export const isOnboardingLink = (obj?: { __typename?: any } | null): obj is OnboardingLink => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isOnboardingLink"')
  return OnboardingLink_possibleTypes.includes(obj.__typename)
}

const Organization_possibleTypes: string[] = ['Organization']
export const isOrganization = (obj?: { __typename?: any } | null): obj is Organization => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isOrganization"')
  return Organization_possibleTypes.includes(obj.__typename)
}

const OrganizationOnboardingSettings_possibleTypes: string[] = ['OrganizationOnboardingSettings']
export const isOrganizationOnboardingSettings = (
  obj?: { __typename?: any } | null,
): obj is OrganizationOnboardingSettings => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isOrganizationOnboardingSettings"')
  return OrganizationOnboardingSettings_possibleTypes.includes(obj.__typename)
}

const OrganizationOnboardingSettingsPayload_possibleTypes: string[] = ['OrganizationOnboardingSettingsPayload']
export const isOrganizationOnboardingSettingsPayload = (
  obj?: { __typename?: any } | null,
): obj is OrganizationOnboardingSettingsPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isOrganizationOnboardingSettingsPayload"')
  return OrganizationOnboardingSettingsPayload_possibleTypes.includes(obj.__typename)
}

const OrganizationPayload_possibleTypes: string[] = ['OrganizationPayload']
export const isOrganizationPayload = (obj?: { __typename?: any } | null): obj is OrganizationPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isOrganizationPayload"')
  return OrganizationPayload_possibleTypes.includes(obj.__typename)
}

const PageInfo_possibleTypes: string[] = ['PageInfo']
export const isPageInfo = (obj?: { __typename?: any } | null): obj is PageInfo => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPageInfo"')
  return PageInfo_possibleTypes.includes(obj.__typename)
}

const PalettePreferences_possibleTypes: string[] = ['PalettePreferences']
export const isPalettePreferences = (obj?: { __typename?: any } | null): obj is PalettePreferences => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPalettePreferences"')
  return PalettePreferences_possibleTypes.includes(obj.__typename)
}

const PasskeyCeremony_possibleTypes: string[] = ['PasskeyCeremony']
export const isPasskeyCeremony = (obj?: { __typename?: any } | null): obj is PasskeyCeremony => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPasskeyCeremony"')
  return PasskeyCeremony_possibleTypes.includes(obj.__typename)
}

const PasskeyCeremonyPayload_possibleTypes: string[] = ['PasskeyCeremonyPayload']
export const isPasskeyCeremonyPayload = (obj?: { __typename?: any } | null): obj is PasskeyCeremonyPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPasskeyCeremonyPayload"')
  return PasskeyCeremonyPayload_possibleTypes.includes(obj.__typename)
}

const PasskeyDescriptor_possibleTypes: string[] = ['PasskeyDescriptor']
export const isPasskeyDescriptor = (obj?: { __typename?: any } | null): obj is PasskeyDescriptor => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPasskeyDescriptor"')
  return PasskeyDescriptor_possibleTypes.includes(obj.__typename)
}

const PasskeyExtensions_possibleTypes: string[] = ['PasskeyExtensions']
export const isPasskeyExtensions = (obj?: { __typename?: any } | null): obj is PasskeyExtensions => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPasskeyExtensions"')
  return PasskeyExtensions_possibleTypes.includes(obj.__typename)
}

const PasskeyOptions_possibleTypes: string[] = ['PasskeyOptions']
export const isPasskeyOptions = (obj?: { __typename?: any } | null): obj is PasskeyOptions => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPasskeyOptions"')
  return PasskeyOptions_possibleTypes.includes(obj.__typename)
}

const PasskeySummary_possibleTypes: string[] = ['PasskeySummary']
export const isPasskeySummary = (obj?: { __typename?: any } | null): obj is PasskeySummary => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPasskeySummary"')
  return PasskeySummary_possibleTypes.includes(obj.__typename)
}

const PasskeySummaryConnection_possibleTypes: string[] = ['PasskeySummaryConnection']
export const isPasskeySummaryConnection = (obj?: { __typename?: any } | null): obj is PasskeySummaryConnection => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPasskeySummaryConnection"')
  return PasskeySummaryConnection_possibleTypes.includes(obj.__typename)
}

const PasskeySummaryEdge_possibleTypes: string[] = ['PasskeySummaryEdge']
export const isPasskeySummaryEdge = (obj?: { __typename?: any } | null): obj is PasskeySummaryEdge => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPasskeySummaryEdge"')
  return PasskeySummaryEdge_possibleTypes.includes(obj.__typename)
}

const PasskeySummaryPayload_possibleTypes: string[] = ['PasskeySummaryPayload']
export const isPasskeySummaryPayload = (obj?: { __typename?: any } | null): obj is PasskeySummaryPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPasskeySummaryPayload"')
  return PasskeySummaryPayload_possibleTypes.includes(obj.__typename)
}

const PdfConfiguration_possibleTypes: string[] = ['PdfConfiguration']
export const isPdfConfiguration = (obj?: { __typename?: any } | null): obj is PdfConfiguration => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPdfConfiguration"')
  return PdfConfiguration_possibleTypes.includes(obj.__typename)
}

const PdfSupport_possibleTypes: string[] = ['PdfSupport']
export const isPdfSupport = (obj?: { __typename?: any } | null): obj is PdfSupport => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPdfSupport"')
  return PdfSupport_possibleTypes.includes(obj.__typename)
}

const PolicyAction_possibleTypes: string[] = ['PolicyAction']
export const isPolicyAction = (obj?: { __typename?: any } | null): obj is PolicyAction => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPolicyAction"')
  return PolicyAction_possibleTypes.includes(obj.__typename)
}

const PolicyActionConnection_possibleTypes: string[] = ['PolicyActionConnection']
export const isPolicyActionConnection = (obj?: { __typename?: any } | null): obj is PolicyActionConnection => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPolicyActionConnection"')
  return PolicyActionConnection_possibleTypes.includes(obj.__typename)
}

const PolicyActionEdge_possibleTypes: string[] = ['PolicyActionEdge']
export const isPolicyActionEdge = (obj?: { __typename?: any } | null): obj is PolicyActionEdge => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isPolicyActionEdge"')
  return PolicyActionEdge_possibleTypes.includes(obj.__typename)
}

const Provider_possibleTypes: string[] = ['Provider']
export const isProvider = (obj?: { __typename?: any } | null): obj is Provider => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isProvider"')
  return Provider_possibleTypes.includes(obj.__typename)
}

const ProviderConnection_possibleTypes: string[] = ['ProviderConnection']
export const isProviderConnection = (obj?: { __typename?: any } | null): obj is ProviderConnection => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isProviderConnection"')
  return ProviderConnection_possibleTypes.includes(obj.__typename)
}

const ProviderEdge_possibleTypes: string[] = ['ProviderEdge']
export const isProviderEdge = (obj?: { __typename?: any } | null): obj is ProviderEdge => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isProviderEdge"')
  return ProviderEdge_possibleTypes.includes(obj.__typename)
}

const ProviderLogoUpload_possibleTypes: string[] = ['ProviderLogoUpload']
export const isProviderLogoUpload = (obj?: { __typename?: any } | null): obj is ProviderLogoUpload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isProviderLogoUpload"')
  return ProviderLogoUpload_possibleTypes.includes(obj.__typename)
}

const ProviderLogoUploadPayload_possibleTypes: string[] = ['ProviderLogoUploadPayload']
export const isProviderLogoUploadPayload = (obj?: { __typename?: any } | null): obj is ProviderLogoUploadPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isProviderLogoUploadPayload"')
  return ProviderLogoUploadPayload_possibleTypes.includes(obj.__typename)
}

const ProviderModel_possibleTypes: string[] = ['ProviderModel']
export const isProviderModel = (obj?: { __typename?: any } | null): obj is ProviderModel => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isProviderModel"')
  return ProviderModel_possibleTypes.includes(obj.__typename)
}

const ProviderPayload_possibleTypes: string[] = ['ProviderPayload']
export const isProviderPayload = (obj?: { __typename?: any } | null): obj is ProviderPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isProviderPayload"')
  return ProviderPayload_possibleTypes.includes(obj.__typename)
}

const ProviderTemplate_possibleTypes: string[] = ['ProviderTemplate']
export const isProviderTemplate = (obj?: { __typename?: any } | null): obj is ProviderTemplate => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isProviderTemplate"')
  return ProviderTemplate_possibleTypes.includes(obj.__typename)
}

const ProviderTemplateConnection_possibleTypes: string[] = ['ProviderTemplateConnection']
export const isProviderTemplateConnection = (obj?: { __typename?: any } | null): obj is ProviderTemplateConnection => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isProviderTemplateConnection"')
  return ProviderTemplateConnection_possibleTypes.includes(obj.__typename)
}

const ProviderTemplateEdge_possibleTypes: string[] = ['ProviderTemplateEdge']
export const isProviderTemplateEdge = (obj?: { __typename?: any } | null): obj is ProviderTemplateEdge => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isProviderTemplateEdge"')
  return ProviderTemplateEdge_possibleTypes.includes(obj.__typename)
}

const ProviderTemplatePayload_possibleTypes: string[] = ['ProviderTemplatePayload']
export const isProviderTemplatePayload = (obj?: { __typename?: any } | null): obj is ProviderTemplatePayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isProviderTemplatePayload"')
  return ProviderTemplatePayload_possibleTypes.includes(obj.__typename)
}

const Query_possibleTypes: string[] = ['Query']
export const isQuery = (obj?: { __typename?: any } | null): obj is Query => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isQuery"')
  return Query_possibleTypes.includes(obj.__typename)
}

const Role_possibleTypes: string[] = ['Role']
export const isRole = (obj?: { __typename?: any } | null): obj is Role => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isRole"')
  return Role_possibleTypes.includes(obj.__typename)
}

const RoleConnection_possibleTypes: string[] = ['RoleConnection']
export const isRoleConnection = (obj?: { __typename?: any } | null): obj is RoleConnection => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isRoleConnection"')
  return RoleConnection_possibleTypes.includes(obj.__typename)
}

const RoleEdge_possibleTypes: string[] = ['RoleEdge']
export const isRoleEdge = (obj?: { __typename?: any } | null): obj is RoleEdge => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isRoleEdge"')
  return RoleEdge_possibleTypes.includes(obj.__typename)
}

const RolePayload_possibleTypes: string[] = ['RolePayload']
export const isRolePayload = (obj?: { __typename?: any } | null): obj is RolePayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isRolePayload"')
  return RolePayload_possibleTypes.includes(obj.__typename)
}

const RouteOption_possibleTypes: string[] = ['RouteOption']
export const isRouteOption = (obj?: { __typename?: any } | null): obj is RouteOption => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isRouteOption"')
  return RouteOption_possibleTypes.includes(obj.__typename)
}

const RouteSelection_possibleTypes: string[] = ['RouteSelection']
export const isRouteSelection = (obj?: { __typename?: any } | null): obj is RouteSelection => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isRouteSelection"')
  return RouteSelection_possibleTypes.includes(obj.__typename)
}

const SearchConfiguration_possibleTypes: string[] = ['SearchConfiguration']
export const isSearchConfiguration = (obj?: { __typename?: any } | null): obj is SearchConfiguration => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isSearchConfiguration"')
  return SearchConfiguration_possibleTypes.includes(obj.__typename)
}

const SearchSupport_possibleTypes: string[] = ['SearchSupport']
export const isSearchSupport = (obj?: { __typename?: any } | null): obj is SearchSupport => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isSearchSupport"')
  return SearchSupport_possibleTypes.includes(obj.__typename)
}

const Session_possibleTypes: string[] = ['Session']
export const isSession = (obj?: { __typename?: any } | null): obj is Session => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isSession"')
  return Session_possibleTypes.includes(obj.__typename)
}

const SessionConnection_possibleTypes: string[] = ['SessionConnection']
export const isSessionConnection = (obj?: { __typename?: any } | null): obj is SessionConnection => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isSessionConnection"')
  return SessionConnection_possibleTypes.includes(obj.__typename)
}

const SessionCredentials_possibleTypes: string[] = ['SessionCredentials']
export const isSessionCredentials = (obj?: { __typename?: any } | null): obj is SessionCredentials => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isSessionCredentials"')
  return SessionCredentials_possibleTypes.includes(obj.__typename)
}

const SessionCredentialsPayload_possibleTypes: string[] = ['SessionCredentialsPayload']
export const isSessionCredentialsPayload = (obj?: { __typename?: any } | null): obj is SessionCredentialsPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isSessionCredentialsPayload"')
  return SessionCredentialsPayload_possibleTypes.includes(obj.__typename)
}

const SessionEdge_possibleTypes: string[] = ['SessionEdge']
export const isSessionEdge = (obj?: { __typename?: any } | null): obj is SessionEdge => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isSessionEdge"')
  return SessionEdge_possibleTypes.includes(obj.__typename)
}

const SessionPayload_possibleTypes: string[] = ['SessionPayload']
export const isSessionPayload = (obj?: { __typename?: any } | null): obj is SessionPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isSessionPayload"')
  return SessionPayload_possibleTypes.includes(obj.__typename)
}

const SidebarEntry_possibleTypes: string[] = ['SidebarEntry']
export const isSidebarEntry = (obj?: { __typename?: any } | null): obj is SidebarEntry => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isSidebarEntry"')
  return SidebarEntry_possibleTypes.includes(obj.__typename)
}

const SidebarLayoutSection_possibleTypes: string[] = ['SidebarLayoutSection']
export const isSidebarLayoutSection = (obj?: { __typename?: any } | null): obj is SidebarLayoutSection => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isSidebarLayoutSection"')
  return SidebarLayoutSection_possibleTypes.includes(obj.__typename)
}

const SidebarPreferences_possibleTypes: string[] = ['SidebarPreferences']
export const isSidebarPreferences = (obj?: { __typename?: any } | null): obj is SidebarPreferences => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isSidebarPreferences"')
  return SidebarPreferences_possibleTypes.includes(obj.__typename)
}

const StructuredOutputConfiguration_possibleTypes: string[] = ['StructuredOutputConfiguration']
export const isStructuredOutputConfiguration = (
  obj?: { __typename?: any } | null,
): obj is StructuredOutputConfiguration => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isStructuredOutputConfiguration"')
  return StructuredOutputConfiguration_possibleTypes.includes(obj.__typename)
}

const StructuredOutputSupport_possibleTypes: string[] = ['StructuredOutputSupport']
export const isStructuredOutputSupport = (obj?: { __typename?: any } | null): obj is StructuredOutputSupport => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isStructuredOutputSupport"')
  return StructuredOutputSupport_possibleTypes.includes(obj.__typename)
}

const TextCapabilities_possibleTypes: string[] = ['TextCapabilities']
export const isTextCapabilities = (obj?: { __typename?: any } | null): obj is TextCapabilities => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTextCapabilities"')
  return TextCapabilities_possibleTypes.includes(obj.__typename)
}

const TextGenerationConfiguration_possibleTypes: string[] = ['TextGenerationConfiguration']
export const isTextGenerationConfiguration = (
  obj?: { __typename?: any } | null,
): obj is TextGenerationConfiguration => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTextGenerationConfiguration"')
  return TextGenerationConfiguration_possibleTypes.includes(obj.__typename)
}

const TextGenerationSupport_possibleTypes: string[] = ['TextGenerationSupport']
export const isTextGenerationSupport = (obj?: { __typename?: any } | null): obj is TextGenerationSupport => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTextGenerationSupport"')
  return TextGenerationSupport_possibleTypes.includes(obj.__typename)
}

const TextInputConfiguration_possibleTypes: string[] = ['TextInputConfiguration']
export const isTextInputConfiguration = (obj?: { __typename?: any } | null): obj is TextInputConfiguration => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTextInputConfiguration"')
  return TextInputConfiguration_possibleTypes.includes(obj.__typename)
}

const TextInputSupport_possibleTypes: string[] = ['TextInputSupport']
export const isTextInputSupport = (obj?: { __typename?: any } | null): obj is TextInputSupport => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTextInputSupport"')
  return TextInputSupport_possibleTypes.includes(obj.__typename)
}

const ThemePreferences_possibleTypes: string[] = ['ThemePreferences']
export const isThemePreferences = (obj?: { __typename?: any } | null): obj is ThemePreferences => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isThemePreferences"')
  return ThemePreferences_possibleTypes.includes(obj.__typename)
}

const ThinkingBudget_possibleTypes: string[] = ['ThinkingBudget']
export const isThinkingBudget = (obj?: { __typename?: any } | null): obj is ThinkingBudget => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isThinkingBudget"')
  return ThinkingBudget_possibleTypes.includes(obj.__typename)
}

const ThinkingConfiguration_possibleTypes: string[] = ['ThinkingConfiguration']
export const isThinkingConfiguration = (obj?: { __typename?: any } | null): obj is ThinkingConfiguration => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isThinkingConfiguration"')
  return ThinkingConfiguration_possibleTypes.includes(obj.__typename)
}

const ThinkingEffort_possibleTypes: string[] = ['ThinkingEffort']
export const isThinkingEffort = (obj?: { __typename?: any } | null): obj is ThinkingEffort => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isThinkingEffort"')
  return ThinkingEffort_possibleTypes.includes(obj.__typename)
}

const ThinkingSupport_possibleTypes: string[] = ['ThinkingSupport']
export const isThinkingSupport = (obj?: { __typename?: any } | null): obj is ThinkingSupport => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isThinkingSupport"')
  return ThinkingSupport_possibleTypes.includes(obj.__typename)
}

const TokenLifetime_possibleTypes: string[] = ['TokenLifetime']
export const isTokenLifetime = (obj?: { __typename?: any } | null): obj is TokenLifetime => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTokenLifetime"')
  return TokenLifetime_possibleTypes.includes(obj.__typename)
}

const ToolCallingConfiguration_possibleTypes: string[] = ['ToolCallingConfiguration']
export const isToolCallingConfiguration = (obj?: { __typename?: any } | null): obj is ToolCallingConfiguration => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isToolCallingConfiguration"')
  return ToolCallingConfiguration_possibleTypes.includes(obj.__typename)
}

const ToolCallingSupport_possibleTypes: string[] = ['ToolCallingSupport']
export const isToolCallingSupport = (obj?: { __typename?: any } | null): obj is ToolCallingSupport => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isToolCallingSupport"')
  return ToolCallingSupport_possibleTypes.includes(obj.__typename)
}

const ToolCapabilities_possibleTypes: string[] = ['ToolCapabilities']
export const isToolCapabilities = (obj?: { __typename?: any } | null): obj is ToolCapabilities => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isToolCapabilities"')
  return ToolCapabilities_possibleTypes.includes(obj.__typename)
}

const TypographyPreferences_possibleTypes: string[] = ['TypographyPreferences']
export const isTypographyPreferences = (obj?: { __typename?: any } | null): obj is TypographyPreferences => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTypographyPreferences"')
  return TypographyPreferences_possibleTypes.includes(obj.__typename)
}

const UploadFormField_possibleTypes: string[] = ['UploadFormField']
export const isUploadFormField = (obj?: { __typename?: any } | null): obj is UploadFormField => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isUploadFormField"')
  return UploadFormField_possibleTypes.includes(obj.__typename)
}

const User_possibleTypes: string[] = ['User']
export const isUser = (obj?: { __typename?: any } | null): obj is User => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isUser"')
  return User_possibleTypes.includes(obj.__typename)
}

const UserPayload_possibleTypes: string[] = ['UserPayload']
export const isUserPayload = (obj?: { __typename?: any } | null): obj is UserPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isUserPayload"')
  return UserPayload_possibleTypes.includes(obj.__typename)
}

const UserPreferences_possibleTypes: string[] = ['UserPreferences']
export const isUserPreferences = (obj?: { __typename?: any } | null): obj is UserPreferences => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isUserPreferences"')
  return UserPreferences_possibleTypes.includes(obj.__typename)
}

const UserPreferencesPayload_possibleTypes: string[] = ['UserPreferencesPayload']
export const isUserPreferencesPayload = (obj?: { __typename?: any } | null): obj is UserPreferencesPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isUserPreferencesPayload"')
  return UserPreferencesPayload_possibleTypes.includes(obj.__typename)
}

const ValidationIssue_possibleTypes: string[] = ['ValidationIssue']
export const isValidationIssue = (obj?: { __typename?: any } | null): obj is ValidationIssue => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isValidationIssue"')
  return ValidationIssue_possibleTypes.includes(obj.__typename)
}

const VideoConfiguration_possibleTypes: string[] = ['VideoConfiguration']
export const isVideoConfiguration = (obj?: { __typename?: any } | null): obj is VideoConfiguration => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isVideoConfiguration"')
  return VideoConfiguration_possibleTypes.includes(obj.__typename)
}

const VideoSupport_possibleTypes: string[] = ['VideoSupport']
export const isVideoSupport = (obj?: { __typename?: any } | null): obj is VideoSupport => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isVideoSupport"')
  return VideoSupport_possibleTypes.includes(obj.__typename)
}

const VisionConfiguration_possibleTypes: string[] = ['VisionConfiguration']
export const isVisionConfiguration = (obj?: { __typename?: any } | null): obj is VisionConfiguration => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isVisionConfiguration"')
  return VisionConfiguration_possibleTypes.includes(obj.__typename)
}

const VisionSupport_possibleTypes: string[] = ['VisionSupport']
export const isVisionSupport = (obj?: { __typename?: any } | null): obj is VisionSupport => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isVisionSupport"')
  return VisionSupport_possibleTypes.includes(obj.__typename)
}

export const enumAuthenticationMethod = {
  API_KEY: 'API_KEY' as const,
  CUSTOM: 'CUSTOM' as const,
  NONE: 'NONE' as const,
  OAUTH: 'OAUTH' as const,
}

export const enumAuthenticationPurpose = {
  LOGIN: 'LOGIN' as const,
  PASSKEY_CHANGE: 'PASSKEY_CHANGE' as const,
}

export const enumBillingModel = {
  FREE: 'FREE' as const,
  METERED: 'METERED' as const,
  SUBSCRIPTION: 'SUBSCRIPTION' as const,
  SUBSCRIPTION_WITH_OVERAGE: 'SUBSCRIPTION_WITH_OVERAGE' as const,
  UNKNOWN: 'UNKNOWN' as const,
}

export const enumCacheBilling = {
  INCLUDED: 'INCLUDED' as const,
  NOT_APPLICABLE: 'NOT_APPLICABLE' as const,
  PER_ROUTE: 'PER_ROUTE' as const,
  SEPARATE_RATE: 'SEPARATE_RATE' as const,
  STANDARD_INPUT_RATE: 'STANDARD_INPUT_RATE' as const,
  UNKNOWN: 'UNKNOWN' as const,
  UNSUPPORTED: 'UNSUPPORTED' as const,
}

export const enumCapabilityCacheMode = {
  INCLUDED: 'INCLUDED' as const,
  SEPARATE_RATE: 'SEPARATE_RATE' as const,
  STANDARD_INPUT_RATE: 'STANDARD_INPUT_RATE' as const,
  UNSUPPORTED: 'UNSUPPORTED' as const,
}

export const enumCapabilityPriceMode = {
  FIXED: 'FIXED' as const,
  SCHEDULED: 'SCHEDULED' as const,
  TIERED_TOKEN: 'TIERED_TOKEN' as const,
}

export const enumCapabilityPricingKind = {
  FREE: 'FREE' as const,
  INCLUDED: 'INCLUDED' as const,
  MODEL_BILLING: 'MODEL_BILLING' as const,
  USAGE: 'USAGE' as const,
}

export const enumCapabilityStatus = {
  SUPPORTED: 'SUPPORTED' as const,
  UNKNOWN: 'UNKNOWN' as const,
  UNSUPPORTED: 'UNSUPPORTED' as const,
}

export const enumConfigurationInputKind = {
  OBJECT: 'OBJECT' as const,
  POSITIVE_INTEGER: 'POSITIVE_INTEGER' as const,
  PRICE: 'PRICE' as const,
  STRING: 'STRING' as const,
  URL: 'URL' as const,
}

export const enumCredentialDelivery = {
  COOKIE: 'COOKIE' as const,
  TOKEN: 'TOKEN' as const,
}

export const enumCredentialLocation = {
  HEADER: 'HEADER' as const,
  QUERY: 'QUERY' as const,
}

export const enumCreditUsageDirection = {
  CACHE_READ: 'CACHE_READ' as const,
  INPUT: 'INPUT' as const,
  OUTPUT: 'OUTPUT' as const,
}

export const enumEndpointMode = {
  ACCOUNT_SPECIFIC: 'ACCOUNT_SPECIFIC' as const,
  DISCOVERED: 'DISCOVERED' as const,
  FIXED: 'FIXED' as const,
  UNKNOWN: 'UNKNOWN' as const,
}

export const enumFilterIs = {
  NOT_NULL: 'NOT_NULL' as const,
  NULL: 'NULL' as const,
}

export const enumHomeDestination = {
  CHAT: 'CHAT' as const,
  MODELS: 'MODELS' as const,
  PROVIDERS: 'PROVIDERS' as const,
}

export const enumImageMediaType = {
  JPEG: 'JPEG' as const,
  PNG: 'PNG' as const,
  WEBP: 'WEBP' as const,
}

export const enumModelBillingKind = {
  CREDIT: 'CREDIT' as const,
  FREE: 'FREE' as const,
  REQUEST: 'REQUEST' as const,
  SUBSCRIPTION: 'SUBSCRIPTION' as const,
}

export const enumModelServiceKind = {
  AUDIO: 'AUDIO' as const,
  EMBEDDING: 'EMBEDDING' as const,
  IMAGE: 'IMAGE' as const,
  RERANK: 'RERANK' as const,
  STT: 'STT' as const,
  TEXT: 'TEXT' as const,
  TTS: 'TTS' as const,
  VIDEO: 'VIDEO' as const,
}

export const enumModelTemplateOrigin = {
  CUSTOM: 'CUSTOM' as const,
  GLOBAL: 'GLOBAL' as const,
  OVERRIDE: 'OVERRIDE' as const,
}

export const enumOAuthFlow = {
  AUTHORIZATION_CODE: 'AUTHORIZATION_CODE' as const,
  CLIENT_CREDENTIALS: 'CLIENT_CREDENTIALS' as const,
  DEVICE_CODE: 'DEVICE_CODE' as const,
}

export const enumOnboardingDestination = {
  CHAT: 'CHAT' as const,
  MODELS: 'MODELS' as const,
  PROVIDERS: 'PROVIDERS' as const,
}

export const enumOnboardingFlow = {
  INVITED_EXISTING: 'INVITED_EXISTING' as const,
  INVITED_NEW: 'INVITED_NEW' as const,
  PERSONAL: 'PERSONAL' as const,
}

export const enumOnboardingStep = {
  INTRODUCTION: 'INTRODUCTION' as const,
  PREFERENCES: 'PREFERENCES' as const,
  PROFILE: 'PROFILE' as const,
  READY: 'READY' as const,
  WORK: 'WORK' as const,
  WORKSPACE: 'WORKSPACE' as const,
}

export const enumOrderByDirection = {
  AscNullsFirst: 'AscNullsFirst' as const,
  AscNullsLast: 'AscNullsLast' as const,
  DescNullsFirst: 'DescNullsFirst' as const,
  DescNullsLast: 'DescNullsLast' as const,
}

export const enumPreferenceAppearanceMode = {
  DARK: 'DARK' as const,
  LIGHT: 'LIGHT' as const,
  SYSTEM: 'SYSTEM' as const,
}

export const enumPreferenceBadgeStyle = {
  COUNT: 'COUNT' as const,
  DOT: 'DOT' as const,
}

export const enumPreferenceCodeTheme = {
  FOLLOW_INTERFACE: 'FOLLOW_INTERFACE' as const,
  LINEAR_DARK: 'LINEAR_DARK' as const,
  LINEAR_LIGHT: 'LINEAR_LIGHT' as const,
  SYSTEM: 'SYSTEM' as const,
}

export const enumPreferenceDarkPalette = {
  BASIC_DARK: 'BASIC_DARK' as const,
  BLUEBERRY_DARK: 'BLUEBERRY_DARK' as const,
  MARSHMALLOW_DARK: 'MARSHMALLOW_DARK' as const,
}

export const enumPreferenceDestination = {
  API_KEYS: 'API_KEYS' as const,
  CHAT: 'CHAT' as const,
  CONVERSATIONS: 'CONVERSATIONS' as const,
  MEMBERS: 'MEMBERS' as const,
  MODELS: 'MODELS' as const,
  NOTEBOOKS: 'NOTEBOOKS' as const,
  PLUGINS: 'PLUGINS' as const,
  PROVIDERS: 'PROVIDERS' as const,
  STATS: 'STATS' as const,
}

export const enumPreferenceFontFamily = {
  DEFAULT: 'DEFAULT' as const,
  MONOSPACE: 'MONOSPACE' as const,
}

export const enumPreferenceKeyCombination = {
  ENTER: 'ENTER' as const,
  MODIFIER_ENTER: 'MODIFIER_ENTER' as const,
}

export const enumPreferenceLightPalette = {
  BASIC_WHITE: 'BASIC_WHITE' as const,
  BLUEBERRY_WHITE: 'BLUEBERRY_WHITE' as const,
  MARSHMALLOW_WHITE: 'MARSHMALLOW_WHITE' as const,
}

export const enumPreferenceNamingFormat = {
  FULL_NAME: 'FULL_NAME' as const,
  USERNAME: 'USERNAME' as const,
}

export const enumPreferencePixelFontSize = {
  PX12: 'PX12' as const,
  PX13: 'PX13' as const,
  PX14: 'PX14' as const,
  PX16: 'PX16' as const,
}

export const enumPreferenceRelativeFontSize = {
  DEFAULT: 'DEFAULT' as const,
  LARGE: 'LARGE' as const,
  LARGER: 'LARGER' as const,
  SMALL: 'SMALL' as const,
  SMALLER: 'SMALLER' as const,
}

export const enumPreferenceSidebarCategory = {
  CONVERSATIONS: 'CONVERSATIONS' as const,
  ORGANIZATION: 'ORGANIZATION' as const,
  ROUTERS: 'ROUTERS' as const,
  WORKSPACE: 'WORKSPACE' as const,
}

export const enumPreferenceSidebarVisibility = {
  ALWAYS: 'ALWAYS' as const,
  HIDDEN: 'HIDDEN' as const,
  WHEN_BADGED: 'WHEN_BADGED' as const,
}

export const enumPreferenceWeekday = {
  FRIDAY: 'FRIDAY' as const,
  MONDAY: 'MONDAY' as const,
  SATURDAY: 'SATURDAY' as const,
  SUNDAY: 'SUNDAY' as const,
  THURSDAY: 'THURSDAY' as const,
  TUESDAY: 'TUESDAY' as const,
  WEDNESDAY: 'WEDNESDAY' as const,
}

export const enumPriceBillingPeriod = {
  DAY: 'DAY' as const,
  HOUR: 'HOUR' as const,
  MINUTE: 'MINUTE' as const,
  MONTH: 'MONTH' as const,
  WEEK: 'WEEK' as const,
  YEAR: 'YEAR' as const,
}

export const enumPriceTierBasis = {
  INPUT_TOKENS: 'INPUT_TOKENS' as const,
  TOTAL_TOKENS: 'TOTAL_TOKENS' as const,
}

export const enumPriceUnit = {
  CHARACTERS: 'CHARACTERS' as const,
  CREDITS: 'CREDITS' as const,
  IMAGES: 'IMAGES' as const,
  REQUESTS: 'REQUESTS' as const,
  SECONDS: 'SECONDS' as const,
  TOKENS: 'TOKENS' as const,
  UTF8_BYTES: 'UTF8_BYTES' as const,
  VIDEOS: 'VIDEOS' as const,
}

export const enumPricingMode = {
  ACCOUNT_SPECIFIC: 'ACCOUNT_SPECIFIC' as const,
  API: 'API' as const,
  FIXED: 'FIXED' as const,
  NOT_APPLICABLE: 'NOT_APPLICABLE' as const,
  SCHEDULED: 'SCHEDULED' as const,
  TIERED: 'TIERED' as const,
  UNKNOWN: 'UNKNOWN' as const,
  UNPUBLISHED: 'UNPUBLISHED' as const,
}

export const enumProviderApiProtocol = {
  ANTHROPIC: 'ANTHROPIC' as const,
  OPENAI_COMPATIBLE: 'OPENAI_COMPATIBLE' as const,
  PROVIDER_SPECIFIC: 'PROVIDER_SPECIFIC' as const,
}

export const enumProviderModelCapability = {
  AUDIO: 'AUDIO' as const,
  AUDIO_GENERATION: 'AUDIO_GENERATION' as const,
  FORCED_TOOL_CHOICE: 'FORCED_TOOL_CHOICE' as const,
  IMAGE_GENERATION: 'IMAGE_GENERATION' as const,
  PDF: 'PDF' as const,
  REASONING: 'REASONING' as const,
  SEARCH: 'SEARCH' as const,
  STRUCTURED_OUTPUT: 'STRUCTURED_OUTPUT' as const,
  TEXT: 'TEXT' as const,
  TOOL_CALL: 'TOOL_CALL' as const,
  VIDEO: 'VIDEO' as const,
  VISION: 'VISION' as const,
}

export const enumTextModelProtocol = {
  ANTHROPIC_MESSAGES: 'ANTHROPIC_MESSAGES' as const,
  OPENAI_CHAT_COMPLETIONS: 'OPENAI_CHAT_COMPLETIONS' as const,
  OPENAI_RESPONSES: 'OPENAI_RESPONSES' as const,
}

export const enumThinkingEffortLevel = {
  HIGH: 'HIGH' as const,
  LOW: 'LOW' as const,
  MAX: 'MAX' as const,
  MEDIUM: 'MEDIUM' as const,
  XHIGH: 'XHIGH' as const,
}

export const enumUpstreamProtocol = {
  ANTHROPIC: 'ANTHROPIC' as const,
  GEMINI: 'GEMINI' as const,
  MINIMAX: 'MINIMAX' as const,
  OPENAI: 'OPENAI' as const,
  OPENAI_RESPONSES: 'OPENAI_RESPONSES' as const,
  OPENROUTER: 'OPENROUTER' as const,
  XAI: 'XAI' as const,
}

export const enumUserPreferenceGroup = {
  ACCESSIBILITY: 'ACCESSIBILITY' as const,
  ALL: 'ALL' as const,
  CALENDAR: 'CALENDAR' as const,
  CODE_APPEARANCE: 'CODE_APPEARANCE' as const,
  MEMBER_DISPLAY: 'MEMBER_DISPLAY' as const,
  MESSAGE_COMPOSITION: 'MESSAGE_COMPOSITION' as const,
  NAVIGATION: 'NAVIGATION' as const,
  SIDEBAR: 'SIDEBAR' as const,
  THEME: 'THEME' as const,
  TYPOGRAPHY: 'TYPOGRAPHY' as const,
}
