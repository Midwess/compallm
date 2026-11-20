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

export interface BooleanPayload {
  node: Scalars['Boolean']
  __typename: 'BooleanPayload'
}

export interface CalendarPreferences {
  firstDay: PreferenceWeekday
  __typename: 'CalendarPreferences'
}

export interface CatalogRequiredInput {
  allowedValues: Scalars['String'][] | null
  kind: TemplateInputKind
  name: Scalars['String']
  required: Scalars['Boolean']
  __typename: 'CatalogRequiredInput'
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

/** How the request delivers credentials; this is never stored in signed claims. */
export type CredentialDelivery = 'COOKIE' | 'TOKEN'

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

export interface ModelTemplate {
  billingModel: TemplateBillingModel
  cacheBilling: TemplateCacheBilling
  cachedPrice: Scalars['String'] | null
  capabilities: ProviderModelCapability[]
  capabilitySupport: TemplateCapabilitySupport[]
  description: Scalars['String'] | null
  displayName: Scalars['String']
  effortMapping: TemplateEffort[]
  fieldStatuses: TemplateFieldStatus[]
  id: Scalars['ID']
  inputPrice: Scalars['String'] | null
  maxContextTokens: Scalars['ContextTokens'] | null
  maxInputTokens: Scalars['ContextTokens'] | null
  maxOutputTokens: Scalars['ContextTokens'] | null
  modelInfoSources: TemplateModelInfoSource[]
  operations: TemplateOperation[]
  outputPrice: Scalars['String'] | null
  pricing: TemplatePricingConfig
  pricingMode: TemplatePricingMode
  providerTemplateId: Scalars['ID']
  readiness: TemplateReadiness
  resolutionPlan: TemplateResolution[]
  serviceKind: TemplateServiceKind
  sources: TemplateSource[]
  specificationMode: TemplateSpecificationMode
  unresolvedFields: TemplateUnresolved[]
  upstreamModelId: Scalars['String']
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

/** Catalog connection family hint; each configured model selects its own operation. */
export type ProviderApiProtocol = 'ANTHROPIC' | 'OPENAI_COMPATIBLE' | 'PROVIDER_SPECIFIC'

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

export interface ProviderTemplate {
  authentication: TemplateAuthentication[]
  baseUrl: Scalars['String'] | null
  description: Scalars['String'] | null
  endpoint: TemplateEndpoint
  fieldStatuses: TemplateFieldStatus[]
  headers: TemplateHeader[]
  id: Scalars['ID']
  logoPath: Scalars['String']
  modelInfoSources: TemplateModelInfoSource[]
  models: ModelTemplateConnection
  name: Scalars['String']
  protocol: ProviderApiProtocol
  readiness: TemplateReadiness
  sources: TemplateSource[]
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
  /** Find visible group metadata in the active organization. */
  group: GroupPayload | null
  /** Return the complete visible group metadata collection, ordered by ID. */
  groups: GroupConnection
  invitationContext: InvitationContextPayload
  invitations: InvitationConnection
  me: MePayload
  members: MemberConnection
  modelTemplate: ModelTemplatePayload | null
  onboardingContext: OnboardingContextPayload | null
  /** Return the active organization from verified user authority. */
  organization: OrganizationPayload
  organizationOnboardingSettings: OrganizationOnboardingSettingsPayload
  /** Return the system-owned organization policy action catalog. */
  organizationPolicyActions: PolicyActionConnection
  organizationProfile: OrganizationPayload | null
  passkeys: PasskeySummaryConnection
  providerTemplate: ProviderTemplatePayload | null
  providerTemplates: ProviderTemplateConnection
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

export interface TemplateAuthConfig {
  adapter: Scalars['String'] | null
  authorizationUrl: Scalars['String'] | null
  deviceAuthorizationUrl: Scalars['String'] | null
  flow: Scalars['String'] | null
  placements: TemplateCredentialPlacement[] | null
  reason: Scalars['String'] | null
  requiredInputs: CatalogRequiredInput[] | null
  scopes: Scalars['String'][] | null
  sourceUrls: Scalars['String'][] | null
  status: TemplateAuthStatus
  tokenUrl: Scalars['String'] | null
  __typename: 'TemplateAuthConfig'
}

export type TemplateAuthStatus = 'CONFIGURED' | 'UNKNOWN'

export interface TemplateAuthentication {
  configuration: TemplateAuthConfig
  method: TemplateAuthenticationMethod
  __typename: 'TemplateAuthentication'
}

export type TemplateAuthenticationMethod = 'API_KEY' | 'CUSTOM' | 'NONE' | 'OAUTH'

export type TemplateBillingModel = 'FREE' | 'METERED' | 'SUBSCRIPTION' | 'SUBSCRIPTION_WITH_OVERAGE' | 'UNKNOWN'

export type TemplateCacheBilling =
  'INCLUDED' | 'NOT_APPLICABLE' | 'PER_ROUTE' | 'SEPARATE_RATE' | 'STANDARD_INPUT_RATE' | 'UNKNOWN' | 'UNSUPPORTED'

export interface TemplateCapabilitySupport {
  capability: ProviderModelCapability
  supported: Scalars['Boolean'] | null
  __typename: 'TemplateCapabilitySupport'
}

export type TemplateCredentialLocation = 'HEADER' | 'QUERY'

export interface TemplateCredentialPlacement {
  adapterRequirements: Scalars['String'][]
  location: TemplateCredentialLocation
  name: Scalars['String']
  operations: Scalars['String'][]
  prefix: Scalars['String']
  __typename: 'TemplateCredentialPlacement'
}

export interface TemplateCreditConversion {
  currency: Scalars['String']
  usdPerCredit: Scalars['String']
  __typename: 'TemplateCreditConversion'
}

export interface TemplateEffort {
  level: TemplateEffortLevel
  value: Scalars['String']
  __typename: 'TemplateEffort'
}

export type TemplateEffortLevel = 'HIGH' | 'LOW' | 'MAX' | 'MEDIUM' | 'XHIGH'

export interface TemplateEndpoint {
  configuration: TemplateEndpointConfig
  mode: TemplateEndpointMode
  __typename: 'TemplateEndpoint'
}

export interface TemplateEndpointConfig {
  reason: Scalars['String'] | null
  requiredInputs: CatalogRequiredInput[] | null
  sourceIndex: Scalars['String'] | null
  sourceUrls: Scalars['String'][] | null
  __typename: 'TemplateEndpointConfig'
}

export type TemplateEndpointMode = 'ACCOUNT_SPECIFIC' | 'DISCOVERED' | 'FIXED' | 'UNKNOWN'

export interface TemplateEntitlement {
  overagePolicy: Scalars['String']
  period: Scalars['String']
  quota: Scalars['String']
  unit: Scalars['String']
  __typename: 'TemplateEntitlement'
}

export type TemplateEstimateConfidence = 'HIGH' | 'LOW' | 'MEDIUM'

export type TemplateFallbackBasis = 'RELATED_MODEL' | 'SAME_MODEL_OTHER_PROVIDER' | 'TARGET_PROVIDER_ESTIMATE'

export interface TemplateFallbackPricing {
  basis: TemplateFallbackBasis
  cachedPrice: Scalars['String'] | null
  confidence: TemplateEstimateConfidence
  currency: Scalars['String']
  inputPrice: Scalars['String'] | null
  notes: Scalars['String']
  outputPrice: Scalars['String'] | null
  referenceModel: Scalars['String']
  referenceProvider: Scalars['String']
  sourceUrls: Scalars['String'][]
  unitQuantity: Scalars['String']
  __typename: 'TemplateFallbackPricing'
}

export interface TemplateFieldLocation {
  pointer: Scalars['String']
  unit: TemplateSourceUnit
  __typename: 'TemplateFieldLocation'
}

export interface TemplateFieldStatus {
  conditionsJson: Scalars['String'] | null
  field: Scalars['String']
  reason: Scalars['String']
  sourceReference: Scalars['String'] | null
  sourceUnitsJson: Scalars['String'] | null
  status: TemplateFieldStatusKind
  __typename: 'TemplateFieldStatus'
}

export type TemplateFieldStatusKind =
  | 'ACCOUNT_SPECIFIC'
  | 'CLIENT_RESTRICTED'
  | 'CONDITIONAL'
  | 'CREDIT_BASED'
  | 'DEPRECATED'
  | 'MIGRATION_REQUIRED'
  | 'NOT_APPLICABLE'
  | 'NO_PUBLIC_ENDPOINT'
  | 'REQUIRES_AUTHENTICATION'
  | 'STANDARD_INPUT_RATE'
  | 'SUBSCRIPTION_BASED'
  | 'UNCONFIGURED'
  | 'UNKNOWN'
  | 'UNPUBLISHED'
  | 'UNSUPPORTED_PRECISION'
  | 'VARIABLE'

export interface TemplateFreeEvidence {
  reason: Scalars['String']
  restrictions: TemplateFreeEvidenceRestrictionsEntry[]
  sourceIndex: Scalars['String']
  __typename: 'TemplateFreeEvidence'
}

export interface TemplateFreeEvidenceRestrictionsEntry {
  key: Scalars['String']
  value: Scalars['String']
  __typename: 'TemplateFreeEvidenceRestrictionsEntry'
}

export interface TemplateHeader {
  key: Scalars['String']
  value: Scalars['String']
  __typename: 'TemplateHeader'
}

export type TemplateInputKind = 'OBJECT' | 'POSITIVE_INTEGER' | 'PRICE' | 'STRING' | 'URL'

export type TemplateMissingStatus = 'UNKNOWN' | 'UNPUBLISHED' | 'UNSUPPORTED'

export interface TemplateModelInfoSource {
  access: TemplateSourceAccess
  fields: Scalars['String'][]
  kind: TemplateSourceKind
  notes: Scalars['String']
  pagination: TemplatePagePagination | null
  reference: Scalars['String'] | null
  request: TemplateSourceRequest | null
  response: TemplateSourceResponse | null
  revision: Scalars['String'] | null
  status: TemplateSourceStatus
  url: Scalars['String'] | null
  __typename: 'TemplateModelInfoSource'
}

export interface TemplateOperation {
  operation: TextModelProtocol
  path: Scalars['String']
  __typename: 'TemplateOperation'
}

export interface TemplatePagePagination {
  pageParameter: Scalars['String']
  pageSize: Scalars['Int']
  pageSizeParameter: Scalars['String']
  totalPagesPointer: Scalars['String']
  __typename: 'TemplatePagePagination'
}

export interface TemplatePricingConfig {
  cacheEvidence: TemplateSourceEvidence | null
  conversion: TemplateCreditConversion | null
  costProxy: TemplateFallbackPricing | null
  currency: Scalars['String'] | null
  fallback: TemplateFallbackPricing | null
  freeEvidence: TemplateFreeEvidence | null
  nativeReference: Scalars['String'] | null
  rates: TemplateUnitRate[] | null
  reason: Scalars['String'] | null
  requiredInputs: CatalogRequiredInput[] | null
  routeOptions: TemplateRoutePricing[] | null
  routePolicy: TemplateRoutePolicy | null
  schedule: TemplatePricingSchedule | null
  sourceIndex: Scalars['String'] | null
  subscription: TemplateSubscription | null
  tiers: TemplateTokenTier[] | null
  unitQuantity: Scalars['String'] | null
  zeroPriceEvidence: TemplatePricingConfigZeroPriceEvidenceEntry[] | null
  __typename: 'TemplatePricingConfig'
}

export interface TemplatePricingConfigZeroPriceEvidenceEntry {
  key: Scalars['String']
  value: TemplateSourceEvidence
  __typename: 'TemplatePricingConfigZeroPriceEvidenceEntry'
}

export type TemplatePricingMode =
  'ACCOUNT_SPECIFIC' | 'API' | 'FIXED' | 'NOT_APPLICABLE' | 'SCHEDULED' | 'TIERED' | 'UNKNOWN' | 'UNPUBLISHED'

export interface TemplatePricingSchedule {
  slots: TemplateTimePriceSlot[]
  timezone: Scalars['String']
  __typename: 'TemplatePricingSchedule'
}

export type TemplateReadiness = 'INCOMPLETE' | 'NEEDS_CONFIGURATION' | 'NEEDS_FETCH' | 'READY'

export type TemplateReferenceKind = 'REPOSITORY' | 'WEB'

export type TemplateReferenceStatus = 'ESTIMATE' | 'IMPORTED' | 'VERIFIED'

export interface TemplateResolution {
  field: Scalars['String']
  method: TemplateResolutionMethod
  requiredInputs: CatalogRequiredInput[]
  sourceIndex: Scalars['String'] | null
  __typename: 'TemplateResolution'
}

export type TemplateResolutionMethod = 'ACCOUNT' | 'API'

export type TemplateResponseFormat = 'MODEL_LIST' | 'MODEL_OBJECT'

export type TemplateRetrievalMethod = 'GET'

export interface TemplateRoutePolicy {
  allowFallbacks: Scalars['Boolean']
  exactProviderSelection: Scalars['Boolean']
  __typename: 'TemplateRoutePolicy'
}

export interface TemplateRoutePricing {
  contextTokens: Scalars['String']
  implicitCaching: Scalars['Boolean'] | null
  maxInputTokens: Scalars['String'] | null
  maxOutputTokens: Scalars['String'] | null
  modelId: Scalars['String']
  provider: Scalars['String']
  rates: TemplateTokenRates
  sourceIndex: Scalars['String']
  tag: Scalars['String']
  tiers: TemplateTokenTier[]
  __typename: 'TemplateRoutePricing'
}

export type TemplateServiceKind = 'AUDIO' | 'EMBEDDING' | 'IMAGE' | 'RERANK' | 'TEXT' | 'UNKNOWN' | 'VIDEO'

export interface TemplateSource {
  fields: Scalars['String'][]
  kind: TemplateReferenceKind
  note: Scalars['String'] | null
  reference: Scalars['String']
  revision: Scalars['String'] | null
  status: TemplateReferenceStatus
  __typename: 'TemplateSource'
}

export type TemplateSourceAccess = 'MANUAL' | 'PROVIDER_CREDENTIAL' | 'PUBLIC' | 'UNAVAILABLE' | 'UNKNOWN'

export type TemplateSourceAuthentication = 'NONE' | 'PROVIDER_API_KEY' | 'PROVIDER_OAUTH' | 'UNKNOWN'

export interface TemplateSourceEvidence {
  reason: Scalars['String']
  sourceIndex: Scalars['String']
  __typename: 'TemplateSourceEvidence'
}

export type TemplateSourceKind = 'API' | 'DOCUMENTATION' | 'MANUAL' | 'REPOSITORY'

export interface TemplateSourceRequest {
  authentication: TemplateSourceAuthentication
  method: TemplateRetrievalMethod
  __typename: 'TemplateSourceRequest'
}

export interface TemplateSourceResponse {
  fields: TemplateSourceResponseFieldsEntry[]
  format: TemplateResponseFormat
  identityPointer: Scalars['String']
  identityValue: Scalars['String'] | null
  objectPointer: Scalars['String']
  __typename: 'TemplateSourceResponse'
}

export interface TemplateSourceResponseFieldsEntry {
  key: Scalars['String']
  value: TemplateFieldLocation
  __typename: 'TemplateSourceResponseFieldsEntry'
}

export type TemplateSourceStatus = 'DOCUMENTED' | 'OBSERVED' | 'REFERENCE' | 'UNAVAILABLE'

export type TemplateSourceUnit =
  'IDENTIFIER' | 'PROVIDER_NATIVE' | 'TEXT' | 'TOKENS' | 'USD_PER_MILLION_TOKENS' | 'USD_PER_TOKEN'

export type TemplateSpecificationMode = 'ACCOUNT_SPECIFIC' | 'API' | 'NOT_APPLICABLE' | 'STORED' | 'UNKNOWN'

export interface TemplateSubscription {
  entitlement: TemplateEntitlement | null
  fee: TemplateSubscriptionFee | null
  plan: Scalars['String'] | null
  planMode: TemplateSubscriptionPlanMode
  product: Scalars['String']
  requiredInputs: CatalogRequiredInput[] | null
  sourceUrls: Scalars['String'][]
  __typename: 'TemplateSubscription'
}

export interface TemplateSubscriptionFee {
  amount: Scalars['String']
  currency: Scalars['String']
  period: Scalars['String']
  __typename: 'TemplateSubscriptionFee'
}

export type TemplateSubscriptionPlanMode = 'ACCOUNT_SPECIFIC' | 'STORED'

export interface TemplateTimePriceSlot {
  fromHour: Scalars['Int']
  rates: TemplateTokenRates
  toHour: Scalars['Int']
  __typename: 'TemplateTimePriceSlot'
}

export interface TemplateTokenRates {
  cachedPrice: Scalars['String'] | null
  inputPrice: Scalars['String']
  outputPrice: Scalars['String']
  __typename: 'TemplateTokenRates'
}

export interface TemplateTokenTier {
  fromTokens: Scalars['String']
  rates: TemplateTokenRates
  toTokens: Scalars['String'] | null
  __typename: 'TemplateTokenTier'
}

export interface TemplateUnitRate {
  amount: Scalars['String']
  conditions: TemplateUnitRateConditionsEntry[]
  quantity: Scalars['String']
  __typename: 'TemplateUnitRate'
}

export interface TemplateUnitRateConditionsEntry {
  key: Scalars['String']
  value: Scalars['String']
  __typename: 'TemplateUnitRateConditionsEntry'
}

export interface TemplateUnresolved {
  field: Scalars['String']
  requirement: TemplateUnresolvedField
  __typename: 'TemplateUnresolved'
}

export interface TemplateUnresolvedField {
  reason: Scalars['String']
  status: TemplateMissingStatus
  __typename: 'TemplateUnresolvedField'
}

/** Text operation determining the upstream request and response format. */
export type TextModelProtocol = 'ANTHROPIC_MESSAGES' | 'OPENAI_CHAT_COMPLETIONS' | 'OPENAI_RESPONSES'

export interface ThemePreferences {
  mode: PreferenceAppearanceMode
  palettes: PalettePreferences
  __typename: 'ThemePreferences'
}

/** Exact Unix-second validity and client throttle values encoded as decimal strings. */
export interface TokenLifetime {
  expiresAt: Scalars['String']
  expiresInSeconds: Scalars['String']
  issuedAt: Scalars['String']
  throttleSeconds: Scalars['String']
  __typename: 'TokenLifetime'
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

export interface CatalogRequiredInputGenqlSelection {
  allowedValues?: boolean | number
  kind?: boolean | number
  name?: boolean | number
  required?: boolean | number
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

export interface CredentialCreateInput {
  name: Scalars['String']
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

export interface ModelTemplateGenqlSelection {
  billingModel?: boolean | number
  cacheBilling?: boolean | number
  cachedPrice?: boolean | number
  capabilities?: boolean | number
  capabilitySupport?: TemplateCapabilitySupportGenqlSelection
  description?: boolean | number
  displayName?: boolean | number
  effortMapping?: TemplateEffortGenqlSelection
  fieldStatuses?: TemplateFieldStatusGenqlSelection
  id?: boolean | number
  inputPrice?: boolean | number
  maxContextTokens?: boolean | number
  maxInputTokens?: boolean | number
  maxOutputTokens?: boolean | number
  modelInfoSources?: TemplateModelInfoSourceGenqlSelection
  operations?: TemplateOperationGenqlSelection
  outputPrice?: boolean | number
  pricing?: TemplatePricingConfigGenqlSelection
  pricingMode?: boolean | number
  providerTemplateId?: boolean | number
  readiness?: boolean | number
  resolutionPlan?: TemplateResolutionGenqlSelection
  serviceKind?: boolean | number
  sources?: TemplateSourceGenqlSelection
  specificationMode?: boolean | number
  unresolvedFields?: TemplateUnresolvedGenqlSelection
  upstreamModelId?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ModelTemplateConnectionGenqlSelection {
  edges?: ModelTemplateEdgeGenqlSelection
  pageInfo?: PageInfoGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ModelTemplateEdgeGenqlSelection {
  cursor?: boolean | number
  node?: ModelTemplateGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface ModelTemplatePayloadGenqlSelection {
  node?: ModelTemplateGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
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

export interface ProviderBodyDefault {
  key: Scalars['String']
  value: Scalars['String']
}

/** Opaque registry identity, independent of an edited upstream model name. */
export interface ProviderCatalogRef {
  model: Scalars['String']
  provider: Scalars['String']
}

/** New connection definition; credentials and header values are write-only. */
export interface ProviderCreateInput {
  apiKey: Scalars['String']
  baseUrl: Scalars['String']
  headers: ProviderHeader[]
  models: ProviderModelWrite[]
  name: Scalars['String']
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
  authentication?: TemplateAuthenticationGenqlSelection
  baseUrl?: boolean | number
  description?: boolean | number
  endpoint?: TemplateEndpointGenqlSelection
  fieldStatuses?: TemplateFieldStatusGenqlSelection
  headers?: TemplateHeaderGenqlSelection
  id?: boolean | number
  logoPath?: boolean | number
  modelInfoSources?: TemplateModelInfoSourceGenqlSelection
  models?: ModelTemplateConnectionGenqlSelection
  name?: boolean | number
  protocol?: boolean | number
  readiness?: boolean | number
  sources?: TemplateSourceGenqlSelection
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
  name?: StringFilter | null
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
  providerTemplate?: ProviderTemplatePayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  providerTemplates?: ProviderTemplateConnectionGenqlSelection & {
    __args?: {
      after?: Scalars['String'] | null
      before?: Scalars['String'] | null
      filter?: ProviderTemplateFilterInput | null
      first?: Scalars['Int'] | null
      last?: Scalars['Int'] | null
      offset?: Scalars['Int'] | null
      orderBy?: ProviderTemplateOrderInput[] | null
    }
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

export interface TemplateAuthConfigGenqlSelection {
  adapter?: boolean | number
  authorizationUrl?: boolean | number
  deviceAuthorizationUrl?: boolean | number
  flow?: boolean | number
  placements?: TemplateCredentialPlacementGenqlSelection
  reason?: boolean | number
  requiredInputs?: CatalogRequiredInputGenqlSelection
  scopes?: boolean | number
  sourceUrls?: boolean | number
  status?: boolean | number
  tokenUrl?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateAuthenticationGenqlSelection {
  configuration?: TemplateAuthConfigGenqlSelection
  method?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateCapabilitySupportGenqlSelection {
  capability?: boolean | number
  supported?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateCredentialPlacementGenqlSelection {
  adapterRequirements?: boolean | number
  location?: boolean | number
  name?: boolean | number
  operations?: boolean | number
  prefix?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateCreditConversionGenqlSelection {
  currency?: boolean | number
  usdPerCredit?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateEffortGenqlSelection {
  level?: boolean | number
  value?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateEndpointGenqlSelection {
  configuration?: TemplateEndpointConfigGenqlSelection
  mode?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateEndpointConfigGenqlSelection {
  reason?: boolean | number
  requiredInputs?: CatalogRequiredInputGenqlSelection
  sourceIndex?: boolean | number
  sourceUrls?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateEntitlementGenqlSelection {
  overagePolicy?: boolean | number
  period?: boolean | number
  quota?: boolean | number
  unit?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateFallbackPricingGenqlSelection {
  basis?: boolean | number
  cachedPrice?: boolean | number
  confidence?: boolean | number
  currency?: boolean | number
  inputPrice?: boolean | number
  notes?: boolean | number
  outputPrice?: boolean | number
  referenceModel?: boolean | number
  referenceProvider?: boolean | number
  sourceUrls?: boolean | number
  unitQuantity?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateFieldLocationGenqlSelection {
  pointer?: boolean | number
  unit?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateFieldStatusGenqlSelection {
  conditionsJson?: boolean | number
  field?: boolean | number
  reason?: boolean | number
  sourceReference?: boolean | number
  sourceUnitsJson?: boolean | number
  status?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateFreeEvidenceGenqlSelection {
  reason?: boolean | number
  restrictions?: TemplateFreeEvidenceRestrictionsEntryGenqlSelection
  sourceIndex?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateFreeEvidenceRestrictionsEntryGenqlSelection {
  key?: boolean | number
  value?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateHeaderGenqlSelection {
  key?: boolean | number
  value?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateModelInfoSourceGenqlSelection {
  access?: boolean | number
  fields?: boolean | number
  kind?: boolean | number
  notes?: boolean | number
  pagination?: TemplatePagePaginationGenqlSelection
  reference?: boolean | number
  request?: TemplateSourceRequestGenqlSelection
  response?: TemplateSourceResponseGenqlSelection
  revision?: boolean | number
  status?: boolean | number
  url?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateOperationGenqlSelection {
  operation?: boolean | number
  path?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplatePagePaginationGenqlSelection {
  pageParameter?: boolean | number
  pageSize?: boolean | number
  pageSizeParameter?: boolean | number
  totalPagesPointer?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplatePricingConfigGenqlSelection {
  cacheEvidence?: TemplateSourceEvidenceGenqlSelection
  conversion?: TemplateCreditConversionGenqlSelection
  costProxy?: TemplateFallbackPricingGenqlSelection
  currency?: boolean | number
  fallback?: TemplateFallbackPricingGenqlSelection
  freeEvidence?: TemplateFreeEvidenceGenqlSelection
  nativeReference?: boolean | number
  rates?: TemplateUnitRateGenqlSelection
  reason?: boolean | number
  requiredInputs?: CatalogRequiredInputGenqlSelection
  routeOptions?: TemplateRoutePricingGenqlSelection
  routePolicy?: TemplateRoutePolicyGenqlSelection
  schedule?: TemplatePricingScheduleGenqlSelection
  sourceIndex?: boolean | number
  subscription?: TemplateSubscriptionGenqlSelection
  tiers?: TemplateTokenTierGenqlSelection
  unitQuantity?: boolean | number
  zeroPriceEvidence?: TemplatePricingConfigZeroPriceEvidenceEntryGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplatePricingConfigZeroPriceEvidenceEntryGenqlSelection {
  key?: boolean | number
  value?: TemplateSourceEvidenceGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplatePricingScheduleGenqlSelection {
  slots?: TemplateTimePriceSlotGenqlSelection
  timezone?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateResolutionGenqlSelection {
  field?: boolean | number
  method?: boolean | number
  requiredInputs?: CatalogRequiredInputGenqlSelection
  sourceIndex?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateRoutePolicyGenqlSelection {
  allowFallbacks?: boolean | number
  exactProviderSelection?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateRoutePricingGenqlSelection {
  contextTokens?: boolean | number
  implicitCaching?: boolean | number
  maxInputTokens?: boolean | number
  maxOutputTokens?: boolean | number
  modelId?: boolean | number
  provider?: boolean | number
  rates?: TemplateTokenRatesGenqlSelection
  sourceIndex?: boolean | number
  tag?: boolean | number
  tiers?: TemplateTokenTierGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateSourceGenqlSelection {
  fields?: boolean | number
  kind?: boolean | number
  note?: boolean | number
  reference?: boolean | number
  revision?: boolean | number
  status?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateSourceEvidenceGenqlSelection {
  reason?: boolean | number
  sourceIndex?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateSourceRequestGenqlSelection {
  authentication?: boolean | number
  method?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateSourceResponseGenqlSelection {
  fields?: TemplateSourceResponseFieldsEntryGenqlSelection
  format?: boolean | number
  identityPointer?: boolean | number
  identityValue?: boolean | number
  objectPointer?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateSourceResponseFieldsEntryGenqlSelection {
  key?: boolean | number
  value?: TemplateFieldLocationGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateSubscriptionGenqlSelection {
  entitlement?: TemplateEntitlementGenqlSelection
  fee?: TemplateSubscriptionFeeGenqlSelection
  plan?: boolean | number
  planMode?: boolean | number
  product?: boolean | number
  requiredInputs?: CatalogRequiredInputGenqlSelection
  sourceUrls?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateSubscriptionFeeGenqlSelection {
  amount?: boolean | number
  currency?: boolean | number
  period?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateTimePriceSlotGenqlSelection {
  fromHour?: boolean | number
  rates?: TemplateTokenRatesGenqlSelection
  toHour?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateTokenRatesGenqlSelection {
  cachedPrice?: boolean | number
  inputPrice?: boolean | number
  outputPrice?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateTokenTierGenqlSelection {
  fromTokens?: boolean | number
  rates?: TemplateTokenRatesGenqlSelection
  toTokens?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateUnitRateGenqlSelection {
  amount?: boolean | number
  conditions?: TemplateUnitRateConditionsEntryGenqlSelection
  quantity?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateUnitRateConditionsEntryGenqlSelection {
  key?: boolean | number
  value?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateUnresolvedGenqlSelection {
  field?: boolean | number
  requirement?: TemplateUnresolvedFieldGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface TemplateUnresolvedFieldGenqlSelection {
  reason?: boolean | number
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

/** Exact Unix-second validity and client throttle values encoded as decimal strings. */
export interface TokenLifetimeGenqlSelection {
  expiresAt?: boolean | number
  expiresInSeconds?: boolean | number
  issuedAt?: boolean | number
  throttleSeconds?: boolean | number
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

const CatalogRequiredInput_possibleTypes: string[] = ['CatalogRequiredInput']
export const isCatalogRequiredInput = (obj?: { __typename?: any } | null): obj is CatalogRequiredInput => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isCatalogRequiredInput"')
  return CatalogRequiredInput_possibleTypes.includes(obj.__typename)
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

const TemplateAuthConfig_possibleTypes: string[] = ['TemplateAuthConfig']
export const isTemplateAuthConfig = (obj?: { __typename?: any } | null): obj is TemplateAuthConfig => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateAuthConfig"')
  return TemplateAuthConfig_possibleTypes.includes(obj.__typename)
}

const TemplateAuthentication_possibleTypes: string[] = ['TemplateAuthentication']
export const isTemplateAuthentication = (obj?: { __typename?: any } | null): obj is TemplateAuthentication => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateAuthentication"')
  return TemplateAuthentication_possibleTypes.includes(obj.__typename)
}

const TemplateCapabilitySupport_possibleTypes: string[] = ['TemplateCapabilitySupport']
export const isTemplateCapabilitySupport = (obj?: { __typename?: any } | null): obj is TemplateCapabilitySupport => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateCapabilitySupport"')
  return TemplateCapabilitySupport_possibleTypes.includes(obj.__typename)
}

const TemplateCredentialPlacement_possibleTypes: string[] = ['TemplateCredentialPlacement']
export const isTemplateCredentialPlacement = (
  obj?: { __typename?: any } | null,
): obj is TemplateCredentialPlacement => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateCredentialPlacement"')
  return TemplateCredentialPlacement_possibleTypes.includes(obj.__typename)
}

const TemplateCreditConversion_possibleTypes: string[] = ['TemplateCreditConversion']
export const isTemplateCreditConversion = (obj?: { __typename?: any } | null): obj is TemplateCreditConversion => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateCreditConversion"')
  return TemplateCreditConversion_possibleTypes.includes(obj.__typename)
}

const TemplateEffort_possibleTypes: string[] = ['TemplateEffort']
export const isTemplateEffort = (obj?: { __typename?: any } | null): obj is TemplateEffort => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateEffort"')
  return TemplateEffort_possibleTypes.includes(obj.__typename)
}

const TemplateEndpoint_possibleTypes: string[] = ['TemplateEndpoint']
export const isTemplateEndpoint = (obj?: { __typename?: any } | null): obj is TemplateEndpoint => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateEndpoint"')
  return TemplateEndpoint_possibleTypes.includes(obj.__typename)
}

const TemplateEndpointConfig_possibleTypes: string[] = ['TemplateEndpointConfig']
export const isTemplateEndpointConfig = (obj?: { __typename?: any } | null): obj is TemplateEndpointConfig => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateEndpointConfig"')
  return TemplateEndpointConfig_possibleTypes.includes(obj.__typename)
}

const TemplateEntitlement_possibleTypes: string[] = ['TemplateEntitlement']
export const isTemplateEntitlement = (obj?: { __typename?: any } | null): obj is TemplateEntitlement => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateEntitlement"')
  return TemplateEntitlement_possibleTypes.includes(obj.__typename)
}

const TemplateFallbackPricing_possibleTypes: string[] = ['TemplateFallbackPricing']
export const isTemplateFallbackPricing = (obj?: { __typename?: any } | null): obj is TemplateFallbackPricing => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateFallbackPricing"')
  return TemplateFallbackPricing_possibleTypes.includes(obj.__typename)
}

const TemplateFieldLocation_possibleTypes: string[] = ['TemplateFieldLocation']
export const isTemplateFieldLocation = (obj?: { __typename?: any } | null): obj is TemplateFieldLocation => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateFieldLocation"')
  return TemplateFieldLocation_possibleTypes.includes(obj.__typename)
}

const TemplateFieldStatus_possibleTypes: string[] = ['TemplateFieldStatus']
export const isTemplateFieldStatus = (obj?: { __typename?: any } | null): obj is TemplateFieldStatus => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateFieldStatus"')
  return TemplateFieldStatus_possibleTypes.includes(obj.__typename)
}

const TemplateFreeEvidence_possibleTypes: string[] = ['TemplateFreeEvidence']
export const isTemplateFreeEvidence = (obj?: { __typename?: any } | null): obj is TemplateFreeEvidence => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateFreeEvidence"')
  return TemplateFreeEvidence_possibleTypes.includes(obj.__typename)
}

const TemplateFreeEvidenceRestrictionsEntry_possibleTypes: string[] = ['TemplateFreeEvidenceRestrictionsEntry']
export const isTemplateFreeEvidenceRestrictionsEntry = (
  obj?: { __typename?: any } | null,
): obj is TemplateFreeEvidenceRestrictionsEntry => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateFreeEvidenceRestrictionsEntry"')
  return TemplateFreeEvidenceRestrictionsEntry_possibleTypes.includes(obj.__typename)
}

const TemplateHeader_possibleTypes: string[] = ['TemplateHeader']
export const isTemplateHeader = (obj?: { __typename?: any } | null): obj is TemplateHeader => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateHeader"')
  return TemplateHeader_possibleTypes.includes(obj.__typename)
}

const TemplateModelInfoSource_possibleTypes: string[] = ['TemplateModelInfoSource']
export const isTemplateModelInfoSource = (obj?: { __typename?: any } | null): obj is TemplateModelInfoSource => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateModelInfoSource"')
  return TemplateModelInfoSource_possibleTypes.includes(obj.__typename)
}

const TemplateOperation_possibleTypes: string[] = ['TemplateOperation']
export const isTemplateOperation = (obj?: { __typename?: any } | null): obj is TemplateOperation => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateOperation"')
  return TemplateOperation_possibleTypes.includes(obj.__typename)
}

const TemplatePagePagination_possibleTypes: string[] = ['TemplatePagePagination']
export const isTemplatePagePagination = (obj?: { __typename?: any } | null): obj is TemplatePagePagination => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplatePagePagination"')
  return TemplatePagePagination_possibleTypes.includes(obj.__typename)
}

const TemplatePricingConfig_possibleTypes: string[] = ['TemplatePricingConfig']
export const isTemplatePricingConfig = (obj?: { __typename?: any } | null): obj is TemplatePricingConfig => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplatePricingConfig"')
  return TemplatePricingConfig_possibleTypes.includes(obj.__typename)
}

const TemplatePricingConfigZeroPriceEvidenceEntry_possibleTypes: string[] = [
  'TemplatePricingConfigZeroPriceEvidenceEntry',
]
export const isTemplatePricingConfigZeroPriceEvidenceEntry = (
  obj?: { __typename?: any } | null,
): obj is TemplatePricingConfigZeroPriceEvidenceEntry => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplatePricingConfigZeroPriceEvidenceEntry"')
  return TemplatePricingConfigZeroPriceEvidenceEntry_possibleTypes.includes(obj.__typename)
}

const TemplatePricingSchedule_possibleTypes: string[] = ['TemplatePricingSchedule']
export const isTemplatePricingSchedule = (obj?: { __typename?: any } | null): obj is TemplatePricingSchedule => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplatePricingSchedule"')
  return TemplatePricingSchedule_possibleTypes.includes(obj.__typename)
}

const TemplateResolution_possibleTypes: string[] = ['TemplateResolution']
export const isTemplateResolution = (obj?: { __typename?: any } | null): obj is TemplateResolution => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateResolution"')
  return TemplateResolution_possibleTypes.includes(obj.__typename)
}

const TemplateRoutePolicy_possibleTypes: string[] = ['TemplateRoutePolicy']
export const isTemplateRoutePolicy = (obj?: { __typename?: any } | null): obj is TemplateRoutePolicy => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateRoutePolicy"')
  return TemplateRoutePolicy_possibleTypes.includes(obj.__typename)
}

const TemplateRoutePricing_possibleTypes: string[] = ['TemplateRoutePricing']
export const isTemplateRoutePricing = (obj?: { __typename?: any } | null): obj is TemplateRoutePricing => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateRoutePricing"')
  return TemplateRoutePricing_possibleTypes.includes(obj.__typename)
}

const TemplateSource_possibleTypes: string[] = ['TemplateSource']
export const isTemplateSource = (obj?: { __typename?: any } | null): obj is TemplateSource => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateSource"')
  return TemplateSource_possibleTypes.includes(obj.__typename)
}

const TemplateSourceEvidence_possibleTypes: string[] = ['TemplateSourceEvidence']
export const isTemplateSourceEvidence = (obj?: { __typename?: any } | null): obj is TemplateSourceEvidence => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateSourceEvidence"')
  return TemplateSourceEvidence_possibleTypes.includes(obj.__typename)
}

const TemplateSourceRequest_possibleTypes: string[] = ['TemplateSourceRequest']
export const isTemplateSourceRequest = (obj?: { __typename?: any } | null): obj is TemplateSourceRequest => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateSourceRequest"')
  return TemplateSourceRequest_possibleTypes.includes(obj.__typename)
}

const TemplateSourceResponse_possibleTypes: string[] = ['TemplateSourceResponse']
export const isTemplateSourceResponse = (obj?: { __typename?: any } | null): obj is TemplateSourceResponse => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateSourceResponse"')
  return TemplateSourceResponse_possibleTypes.includes(obj.__typename)
}

const TemplateSourceResponseFieldsEntry_possibleTypes: string[] = ['TemplateSourceResponseFieldsEntry']
export const isTemplateSourceResponseFieldsEntry = (
  obj?: { __typename?: any } | null,
): obj is TemplateSourceResponseFieldsEntry => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateSourceResponseFieldsEntry"')
  return TemplateSourceResponseFieldsEntry_possibleTypes.includes(obj.__typename)
}

const TemplateSubscription_possibleTypes: string[] = ['TemplateSubscription']
export const isTemplateSubscription = (obj?: { __typename?: any } | null): obj is TemplateSubscription => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateSubscription"')
  return TemplateSubscription_possibleTypes.includes(obj.__typename)
}

const TemplateSubscriptionFee_possibleTypes: string[] = ['TemplateSubscriptionFee']
export const isTemplateSubscriptionFee = (obj?: { __typename?: any } | null): obj is TemplateSubscriptionFee => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateSubscriptionFee"')
  return TemplateSubscriptionFee_possibleTypes.includes(obj.__typename)
}

const TemplateTimePriceSlot_possibleTypes: string[] = ['TemplateTimePriceSlot']
export const isTemplateTimePriceSlot = (obj?: { __typename?: any } | null): obj is TemplateTimePriceSlot => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateTimePriceSlot"')
  return TemplateTimePriceSlot_possibleTypes.includes(obj.__typename)
}

const TemplateTokenRates_possibleTypes: string[] = ['TemplateTokenRates']
export const isTemplateTokenRates = (obj?: { __typename?: any } | null): obj is TemplateTokenRates => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateTokenRates"')
  return TemplateTokenRates_possibleTypes.includes(obj.__typename)
}

const TemplateTokenTier_possibleTypes: string[] = ['TemplateTokenTier']
export const isTemplateTokenTier = (obj?: { __typename?: any } | null): obj is TemplateTokenTier => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateTokenTier"')
  return TemplateTokenTier_possibleTypes.includes(obj.__typename)
}

const TemplateUnitRate_possibleTypes: string[] = ['TemplateUnitRate']
export const isTemplateUnitRate = (obj?: { __typename?: any } | null): obj is TemplateUnitRate => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateUnitRate"')
  return TemplateUnitRate_possibleTypes.includes(obj.__typename)
}

const TemplateUnitRateConditionsEntry_possibleTypes: string[] = ['TemplateUnitRateConditionsEntry']
export const isTemplateUnitRateConditionsEntry = (
  obj?: { __typename?: any } | null,
): obj is TemplateUnitRateConditionsEntry => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateUnitRateConditionsEntry"')
  return TemplateUnitRateConditionsEntry_possibleTypes.includes(obj.__typename)
}

const TemplateUnresolved_possibleTypes: string[] = ['TemplateUnresolved']
export const isTemplateUnresolved = (obj?: { __typename?: any } | null): obj is TemplateUnresolved => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateUnresolved"')
  return TemplateUnresolved_possibleTypes.includes(obj.__typename)
}

const TemplateUnresolvedField_possibleTypes: string[] = ['TemplateUnresolvedField']
export const isTemplateUnresolvedField = (obj?: { __typename?: any } | null): obj is TemplateUnresolvedField => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTemplateUnresolvedField"')
  return TemplateUnresolvedField_possibleTypes.includes(obj.__typename)
}

const ThemePreferences_possibleTypes: string[] = ['ThemePreferences']
export const isThemePreferences = (obj?: { __typename?: any } | null): obj is ThemePreferences => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isThemePreferences"')
  return ThemePreferences_possibleTypes.includes(obj.__typename)
}

const TokenLifetime_possibleTypes: string[] = ['TokenLifetime']
export const isTokenLifetime = (obj?: { __typename?: any } | null): obj is TokenLifetime => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTokenLifetime"')
  return TokenLifetime_possibleTypes.includes(obj.__typename)
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

export const enumAuthenticationPurpose = {
  LOGIN: 'LOGIN' as const,
  PASSKEY_CHANGE: 'PASSKEY_CHANGE' as const,
}

export const enumCredentialDelivery = {
  COOKIE: 'COOKIE' as const,
  TOKEN: 'TOKEN' as const,
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

export const enumTemplateAuthStatus = {
  CONFIGURED: 'CONFIGURED' as const,
  UNKNOWN: 'UNKNOWN' as const,
}

export const enumTemplateAuthenticationMethod = {
  API_KEY: 'API_KEY' as const,
  CUSTOM: 'CUSTOM' as const,
  NONE: 'NONE' as const,
  OAUTH: 'OAUTH' as const,
}

export const enumTemplateBillingModel = {
  FREE: 'FREE' as const,
  METERED: 'METERED' as const,
  SUBSCRIPTION: 'SUBSCRIPTION' as const,
  SUBSCRIPTION_WITH_OVERAGE: 'SUBSCRIPTION_WITH_OVERAGE' as const,
  UNKNOWN: 'UNKNOWN' as const,
}

export const enumTemplateCacheBilling = {
  INCLUDED: 'INCLUDED' as const,
  NOT_APPLICABLE: 'NOT_APPLICABLE' as const,
  PER_ROUTE: 'PER_ROUTE' as const,
  SEPARATE_RATE: 'SEPARATE_RATE' as const,
  STANDARD_INPUT_RATE: 'STANDARD_INPUT_RATE' as const,
  UNKNOWN: 'UNKNOWN' as const,
  UNSUPPORTED: 'UNSUPPORTED' as const,
}

export const enumTemplateCredentialLocation = {
  HEADER: 'HEADER' as const,
  QUERY: 'QUERY' as const,
}

export const enumTemplateEffortLevel = {
  HIGH: 'HIGH' as const,
  LOW: 'LOW' as const,
  MAX: 'MAX' as const,
  MEDIUM: 'MEDIUM' as const,
  XHIGH: 'XHIGH' as const,
}

export const enumTemplateEndpointMode = {
  ACCOUNT_SPECIFIC: 'ACCOUNT_SPECIFIC' as const,
  DISCOVERED: 'DISCOVERED' as const,
  FIXED: 'FIXED' as const,
  UNKNOWN: 'UNKNOWN' as const,
}

export const enumTemplateEstimateConfidence = {
  HIGH: 'HIGH' as const,
  LOW: 'LOW' as const,
  MEDIUM: 'MEDIUM' as const,
}

export const enumTemplateFallbackBasis = {
  RELATED_MODEL: 'RELATED_MODEL' as const,
  SAME_MODEL_OTHER_PROVIDER: 'SAME_MODEL_OTHER_PROVIDER' as const,
  TARGET_PROVIDER_ESTIMATE: 'TARGET_PROVIDER_ESTIMATE' as const,
}

export const enumTemplateFieldStatusKind = {
  ACCOUNT_SPECIFIC: 'ACCOUNT_SPECIFIC' as const,
  CLIENT_RESTRICTED: 'CLIENT_RESTRICTED' as const,
  CONDITIONAL: 'CONDITIONAL' as const,
  CREDIT_BASED: 'CREDIT_BASED' as const,
  DEPRECATED: 'DEPRECATED' as const,
  MIGRATION_REQUIRED: 'MIGRATION_REQUIRED' as const,
  NOT_APPLICABLE: 'NOT_APPLICABLE' as const,
  NO_PUBLIC_ENDPOINT: 'NO_PUBLIC_ENDPOINT' as const,
  REQUIRES_AUTHENTICATION: 'REQUIRES_AUTHENTICATION' as const,
  STANDARD_INPUT_RATE: 'STANDARD_INPUT_RATE' as const,
  SUBSCRIPTION_BASED: 'SUBSCRIPTION_BASED' as const,
  UNCONFIGURED: 'UNCONFIGURED' as const,
  UNKNOWN: 'UNKNOWN' as const,
  UNPUBLISHED: 'UNPUBLISHED' as const,
  UNSUPPORTED_PRECISION: 'UNSUPPORTED_PRECISION' as const,
  VARIABLE: 'VARIABLE' as const,
}

export const enumTemplateInputKind = {
  OBJECT: 'OBJECT' as const,
  POSITIVE_INTEGER: 'POSITIVE_INTEGER' as const,
  PRICE: 'PRICE' as const,
  STRING: 'STRING' as const,
  URL: 'URL' as const,
}

export const enumTemplateMissingStatus = {
  UNKNOWN: 'UNKNOWN' as const,
  UNPUBLISHED: 'UNPUBLISHED' as const,
  UNSUPPORTED: 'UNSUPPORTED' as const,
}

export const enumTemplatePricingMode = {
  ACCOUNT_SPECIFIC: 'ACCOUNT_SPECIFIC' as const,
  API: 'API' as const,
  FIXED: 'FIXED' as const,
  NOT_APPLICABLE: 'NOT_APPLICABLE' as const,
  SCHEDULED: 'SCHEDULED' as const,
  TIERED: 'TIERED' as const,
  UNKNOWN: 'UNKNOWN' as const,
  UNPUBLISHED: 'UNPUBLISHED' as const,
}

export const enumTemplateReadiness = {
  INCOMPLETE: 'INCOMPLETE' as const,
  NEEDS_CONFIGURATION: 'NEEDS_CONFIGURATION' as const,
  NEEDS_FETCH: 'NEEDS_FETCH' as const,
  READY: 'READY' as const,
}

export const enumTemplateReferenceKind = {
  REPOSITORY: 'REPOSITORY' as const,
  WEB: 'WEB' as const,
}

export const enumTemplateReferenceStatus = {
  ESTIMATE: 'ESTIMATE' as const,
  IMPORTED: 'IMPORTED' as const,
  VERIFIED: 'VERIFIED' as const,
}

export const enumTemplateResolutionMethod = {
  ACCOUNT: 'ACCOUNT' as const,
  API: 'API' as const,
}

export const enumTemplateResponseFormat = {
  MODEL_LIST: 'MODEL_LIST' as const,
  MODEL_OBJECT: 'MODEL_OBJECT' as const,
}

export const enumTemplateRetrievalMethod = {
  GET: 'GET' as const,
}

export const enumTemplateServiceKind = {
  AUDIO: 'AUDIO' as const,
  EMBEDDING: 'EMBEDDING' as const,
  IMAGE: 'IMAGE' as const,
  RERANK: 'RERANK' as const,
  TEXT: 'TEXT' as const,
  UNKNOWN: 'UNKNOWN' as const,
  VIDEO: 'VIDEO' as const,
}

export const enumTemplateSourceAccess = {
  MANUAL: 'MANUAL' as const,
  PROVIDER_CREDENTIAL: 'PROVIDER_CREDENTIAL' as const,
  PUBLIC: 'PUBLIC' as const,
  UNAVAILABLE: 'UNAVAILABLE' as const,
  UNKNOWN: 'UNKNOWN' as const,
}

export const enumTemplateSourceAuthentication = {
  NONE: 'NONE' as const,
  PROVIDER_API_KEY: 'PROVIDER_API_KEY' as const,
  PROVIDER_OAUTH: 'PROVIDER_OAUTH' as const,
  UNKNOWN: 'UNKNOWN' as const,
}

export const enumTemplateSourceKind = {
  API: 'API' as const,
  DOCUMENTATION: 'DOCUMENTATION' as const,
  MANUAL: 'MANUAL' as const,
  REPOSITORY: 'REPOSITORY' as const,
}

export const enumTemplateSourceStatus = {
  DOCUMENTED: 'DOCUMENTED' as const,
  OBSERVED: 'OBSERVED' as const,
  REFERENCE: 'REFERENCE' as const,
  UNAVAILABLE: 'UNAVAILABLE' as const,
}

export const enumTemplateSourceUnit = {
  IDENTIFIER: 'IDENTIFIER' as const,
  PROVIDER_NATIVE: 'PROVIDER_NATIVE' as const,
  TEXT: 'TEXT' as const,
  TOKENS: 'TOKENS' as const,
  USD_PER_MILLION_TOKENS: 'USD_PER_MILLION_TOKENS' as const,
  USD_PER_TOKEN: 'USD_PER_TOKEN' as const,
}

export const enumTemplateSpecificationMode = {
  ACCOUNT_SPECIFIC: 'ACCOUNT_SPECIFIC' as const,
  API: 'API' as const,
  NOT_APPLICABLE: 'NOT_APPLICABLE' as const,
  STORED: 'STORED' as const,
  UNKNOWN: 'UNKNOWN' as const,
}

export const enumTemplateSubscriptionPlanMode = {
  ACCOUNT_SPECIFIC: 'ACCOUNT_SPECIFIC' as const,
  STORED: 'STORED' as const,
}

export const enumTextModelProtocol = {
  ANTHROPIC_MESSAGES: 'ANTHROPIC_MESSAGES' as const,
  OPENAI_CHAT_COMPLETIONS: 'OPENAI_CHAT_COMPLETIONS' as const,
  OPENAI_RESPONSES: 'OPENAI_RESPONSES' as const,
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
