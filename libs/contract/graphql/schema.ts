// @ts-nocheck
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

export type Scalars = {
  Boolean: boolean
  ContextTokens: number
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
  memberCount: Scalars['String']
  memberOptions: GroupMemberOptionConnection
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

export interface GroupMemberOption {
  included: Scalars['Boolean']
  member: Member
  __typename: 'GroupMemberOption'
}

export interface GroupMemberOptionConnection {
  edges: GroupMemberOptionEdge[]
  pageInfo: PageInfo
  __typename: 'GroupMemberOptionConnection'
}

export interface GroupMemberOptionEdge {
  cursor: Scalars['String']
  node: GroupMemberOption
  __typename: 'GroupMemberOptionEdge'
}

export interface GroupPayload {
  node: Group
  __typename: 'GroupPayload'
}

export type HomeDestination = 'CHAT' | 'MODELS' | 'PROVIDERS'

export type ImageMediaType = 'JPEG' | 'PNG' | 'WEBP'

export interface Member {
  available: Scalars['Boolean']
  displayName: Scalars['String']
  email: Scalars['String']
  id: Scalars['ID']
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

export interface Mutation {
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
  organizationProfileUpdate: OrganizationPayload
  /** Create a role in the verified active organization. */
  roleCreate: RolePayload
  roleDelete: BooleanPayload
  roleUpdate: RolePayload
  /** Refresh using explicit refresh Bearer authority; leave browser cookies unchanged. */
  sessionRefresh: SessionCredentialsPayload
  /** Revoke the verified session and clear its browser cookies after success. */
  sessionRevoke: BooleanPayload
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

export type OrderByDirection = 'AscNullsFirst' | 'AscNullsLast' | 'DescNullsFirst' | 'DescNullsLast'

export interface Organization {
  avatarUrl: Scalars['String'] | null
  canUpdateProfile: Scalars['Boolean']
  id: Scalars['ID']
  name: Scalars['String']
  __typename: 'Organization'
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

/** Public catalog defaults; selecting a template creates no configured provider. */
export interface ProviderTemplate {
  /** Unknown and account-specific addresses remain null. */
  baseUrl: Scalars['String'] | null
  /** Global opaque catalog identity, independent of organization resource IDs. */
  id: Scalars['ID']
  name: Scalars['String']
  /** Family hint only; configured models select their own text operation. */
  protocol: ProviderApiProtocol
  __typename: 'ProviderTemplate'
}

export interface Query {
  /** Find visible group metadata in the active organization. */
  group: GroupPayload | null
  /** Return the complete visible group metadata collection, ordered by ID. */
  groups: GroupConnection
  members: MemberConnection
  /** Return the active organization from verified user authority. */
  organization: OrganizationPayload
  /** Return the system-owned organization policy action catalog. */
  organizationPolicyActions: PolicyActionConnection
  organizationProfile: OrganizationPayload | null
  /** Find visible role metadata in the active organization. */
  role: RolePayload | null
  /** Return the complete visible role metadata collection, ordered by ID. */
  roles: RoleConnection
  /** Return the verified current session identity. */
  session: SessionPayload
  /** Return the signed-in user's personal preferences; missing rows use Core defaults without a write. */
  userPreferences: UserPreferencesPayload
  /** Public profile for an active user. Email and account state are excluded. */
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
  /** Login session identity, serialized as a string without numeric precision loss. */
  id: Scalars['ID']
  /** Organization selected for this verified session. */
  organizationId: Scalars['ID']
  /** Current organization name. */
  organizationName: Scalars['String']
  /** User owning this verified session. */
  userId: Scalars['ID']
  __typename: 'Session'
}

/** Timing is always returned; credential values exist only in explicit TOKEN delivery. */
export interface SessionCredentials {
  accessLifetime: TokenLifetime
  accessToken: Scalars['String'] | null
  refreshLifetime: TokenLifetime
  refreshToken: Scalars['String'] | null
  __typename: 'SessionCredentials'
}

export interface SessionCredentialsPayload {
  node: SessionCredentials
  __typename: 'SessionCredentialsPayload'
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
  /** Email is readable only by the verified account owner and cannot be updated here. */
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
}

export interface GroupGenqlSelection {
  description?: boolean | number
  id?: boolean | number
  memberCount?: boolean | number
  memberOptions?: GroupMemberOptionConnectionGenqlSelection & {
    __args?: {
      after?: Scalars['String'] | null
      before?: Scalars['String'] | null
      filter?: MemberFilterInput | null
      first?: Scalars['Int'] | null
      last?: Scalars['Int'] | null
      offset?: Scalars['Int'] | null
      orderBy?: MemberOrderInput[] | null
    }
  }
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

export interface GroupMemberOptionGenqlSelection {
  included?: boolean | number
  member?: MemberGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface GroupMemberOptionConnectionGenqlSelection {
  edges?: GroupMemberOptionEdgeGenqlSelection
  pageInfo?: PageInfoGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface GroupMemberOptionEdgeGenqlSelection {
  cursor?: boolean | number
  node?: GroupMemberOptionGenqlSelection
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

export interface ImageUploadInput {
  mediaType: ImageMediaType
  size: Scalars['Int']
}

export interface MemberGenqlSelection {
  available?: boolean | number
  displayName?: boolean | number
  email?: boolean | number
  id?: boolean | number
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

export interface MutationGenqlSelection {
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
  organizationProfileUpdate?: OrganizationPayloadGenqlSelection & { __args: { input: OrganizationProfileUpdateInput } }
  /** Create a role in the verified active organization. */
  roleCreate?: RolePayloadGenqlSelection & { __args: { input: RoleCreateInput } }
  roleDelete?: BooleanPayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  roleUpdate?: RolePayloadGenqlSelection & { __args: { id: Scalars['ID']; input: RoleUpdateInput } }
  /** Refresh using explicit refresh Bearer authority; leave browser cookies unchanged. */
  sessionRefresh?: SessionCredentialsPayloadGenqlSelection
  /** Revoke the verified session and clear its browser cookies after success. */
  sessionRevoke?: BooleanPayloadGenqlSelection
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

export interface OrganizationGenqlSelection {
  avatarUrl?: boolean | number
  canUpdateProfile?: boolean | number
  id?: boolean | number
  name?: boolean | number
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
  cachedPrice: Scalars['String']
  capabilities?: ProviderModelCapability[]
  catalogRef?: ProviderCatalogRef | null
  effortMapping?: ProviderEffortMapping | null
  enabled?: Scalars['Boolean']
  inputPrice: Scalars['String']
  maxContextTokens: Scalars['ContextTokens']
  name: Scalars['String']
  outputPrice: Scalars['String']
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

/** Public catalog defaults; selecting a template creates no configured provider. */
export interface ProviderTemplateGenqlSelection {
  /** Unknown and account-specific addresses remain null. */
  baseUrl?: boolean | number
  /** Global opaque catalog identity, independent of organization resource IDs. */
  id?: boolean | number
  name?: boolean | number
  /** Family hint only; configured models select their own text operation. */
  protocol?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

/** Atomic batch of new provider connections for the verified organization. */
export interface ProvidersCreateInput {
  providers: ProviderCreateInput[]
}

export interface QueryGenqlSelection {
  /** Find visible group metadata in the active organization. */
  group?: GroupPayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  /** Return the complete visible group metadata collection, ordered by ID. */
  groups?: GroupConnectionGenqlSelection
  members?: MemberConnectionGenqlSelection & {
    __args?: {
      after?: Scalars['String'] | null
      before?: Scalars['String'] | null
      filter?: MemberFilterInput | null
      first?: Scalars['Int'] | null
      last?: Scalars['Int'] | null
      offset?: Scalars['Int'] | null
      orderBy?: MemberOrderInput[] | null
    }
  }
  /** Return the active organization from verified user authority. */
  organization?: OrganizationPayloadGenqlSelection
  /** Return the system-owned organization policy action catalog. */
  organizationPolicyActions?: PolicyActionConnectionGenqlSelection
  organizationProfile?: OrganizationPayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  /** Find visible role metadata in the active organization. */
  role?: RolePayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  /** Return the complete visible role metadata collection, ordered by ID. */
  roles?: RoleConnectionGenqlSelection
  /** Return the verified current session identity. */
  session?: SessionPayloadGenqlSelection
  /** Return the signed-in user's personal preferences; missing rows use Core defaults without a write. */
  userPreferences?: UserPreferencesPayloadGenqlSelection
  /** Public profile for an active user. Email and account state are excluded. */
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
  /** Login session identity, serialized as a string without numeric precision loss. */
  id?: boolean | number
  /** Organization selected for this verified session. */
  organizationId?: boolean | number
  /** Current organization name. */
  organizationName?: boolean | number
  /** User owning this verified session. */
  userId?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

/** Timing is always returned; credential values exist only in explicit TOKEN delivery. */
export interface SessionCredentialsGenqlSelection {
  accessLifetime?: TokenLifetimeGenqlSelection
  accessToken?: boolean | number
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
  /** Email is readable only by the verified account owner and cannot be updated here. */
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

const GroupMemberOption_possibleTypes: string[] = ['GroupMemberOption']
export const isGroupMemberOption = (obj?: { __typename?: any } | null): obj is GroupMemberOption => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isGroupMemberOption"')
  return GroupMemberOption_possibleTypes.includes(obj.__typename)
}

const GroupMemberOptionConnection_possibleTypes: string[] = ['GroupMemberOptionConnection']
export const isGroupMemberOptionConnection = (
  obj?: { __typename?: any } | null,
): obj is GroupMemberOptionConnection => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isGroupMemberOptionConnection"')
  return GroupMemberOptionConnection_possibleTypes.includes(obj.__typename)
}

const GroupMemberOptionEdge_possibleTypes: string[] = ['GroupMemberOptionEdge']
export const isGroupMemberOptionEdge = (obj?: { __typename?: any } | null): obj is GroupMemberOptionEdge => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isGroupMemberOptionEdge"')
  return GroupMemberOptionEdge_possibleTypes.includes(obj.__typename)
}

const GroupPayload_possibleTypes: string[] = ['GroupPayload']
export const isGroupPayload = (obj?: { __typename?: any } | null): obj is GroupPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isGroupPayload"')
  return GroupPayload_possibleTypes.includes(obj.__typename)
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

const Organization_possibleTypes: string[] = ['Organization']
export const isOrganization = (obj?: { __typename?: any } | null): obj is Organization => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isOrganization"')
  return Organization_possibleTypes.includes(obj.__typename)
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
