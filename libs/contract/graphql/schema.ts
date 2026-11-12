// @ts-nocheck
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

export type Scalars = {
  Boolean: boolean
  ContextTokens: number
  ID: string
  String: string
}

export interface BooleanPayload {
  node: Scalars['Boolean']
  __typename: 'BooleanPayload'
}

export interface CreatedGroup {
  description: Scalars['String']
  id: Scalars['ID']
  name: Scalars['String']
  roleIds: Scalars['ID'][]
  __typename: 'CreatedGroup'
}

export interface CreatedGroupPayload {
  node: CreatedGroup
  __typename: 'CreatedGroupPayload'
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

export interface Group {
  description: Scalars['String']
  id: Scalars['ID']
  name: Scalars['String']
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

export interface Mutation {
  /** Confirm the verified signin challenge and create a scoped session. */
  emailSigninConfirm: SessionCredentialsPayload
  /** Request a signin email challenge for an existing active account. */
  emailSigninRequest: EmailChallengePayload
  /** Confirm the verified signup challenge and create the account and session. */
  emailSignupConfirm: SessionCredentialsPayload
  /** Request a signup email challenge; reject existing accounts before sending. */
  emailSignupRequest: EmailChallengePayload
  /** Create a group and its initial role assignments in the verified active organization. */
  groupCreate: CreatedGroupPayload
  /** Create a role in the verified active organization. */
  roleCreate: RolePayload
  /** Refresh using explicit refresh Bearer authority; leave browser cookies unchanged. */
  sessionRefresh: SessionCredentialsPayload
  /** Revoke the verified session and clear its browser cookies after success. */
  sessionRevoke: BooleanPayload
  __typename: 'Mutation'
}

export interface Organization {
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
  /** Return the active organization from verified user authority. */
  organization: OrganizationPayload
  /** Find visible role metadata in the active organization. */
  role: RolePayload | null
  /** Return the complete visible role metadata collection, ordered by ID. */
  roles: RoleConnection
  /** Return the verified current session identity. */
  session: SessionPayload
  __typename: 'Query'
}

export interface Role {
  color: Scalars['String']
  description: Scalars['String']
  id: Scalars['ID']
  name: Scalars['String']
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
  /** Login session identity, serialized as a string without numeric precision loss. */
  id: Scalars['ID']
  /** Organization selected for this verified session. */
  organizationId: Scalars['ID']
  /** Signed display snapshot; the name never establishes organization authority. */
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

/** Text operation determining the upstream request and response format. */
export type TextModelProtocol = 'ANTHROPIC_MESSAGES' | 'OPENAI_CHAT_COMPLETIONS' | 'OPENAI_RESPONSES'

/** Exact Unix-second validity and client throttle values encoded as decimal strings. */
export interface TokenLifetime {
  expiresAt: Scalars['String']
  expiresInSeconds: Scalars['String']
  issuedAt: Scalars['String']
  throttleSeconds: Scalars['String']
  __typename: 'TokenLifetime'
}

export interface BooleanPayloadGenqlSelection {
  node?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface CreatedGroupGenqlSelection {
  description?: boolean | number
  id?: boolean | number
  name?: boolean | number
  roleIds?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface CreatedGroupPayloadGenqlSelection {
  node?: CreatedGroupGenqlSelection
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
  name?: boolean | number
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

export interface MutationGenqlSelection {
  /** Confirm the verified signin challenge and create a scoped session. */
  emailSigninConfirm?: SessionCredentialsPayloadGenqlSelection & { __args: { input: EmailCodeInput } }
  /** Request a signin email challenge for an existing active account. */
  emailSigninRequest?: EmailChallengePayloadGenqlSelection & { __args: { input: EmailInput } }
  /** Confirm the verified signup challenge and create the account and session. */
  emailSignupConfirm?: SessionCredentialsPayloadGenqlSelection & { __args: { input: EmailCodeInput } }
  /** Request a signup email challenge; reject existing accounts before sending. */
  emailSignupRequest?: EmailChallengePayloadGenqlSelection & { __args: { input: EmailInput } }
  /** Create a group and its initial role assignments in the verified active organization. */
  groupCreate?: CreatedGroupPayloadGenqlSelection & { __args: { input: GroupCreateInput } }
  /** Create a role in the verified active organization. */
  roleCreate?: RolePayloadGenqlSelection & { __args: { input: RoleCreateInput } }
  /** Refresh using explicit refresh Bearer authority; leave browser cookies unchanged. */
  sessionRefresh?: SessionCredentialsPayloadGenqlSelection
  /** Revoke the verified session and clear its browser cookies after success. */
  sessionRevoke?: BooleanPayloadGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface OrganizationGenqlSelection {
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

export interface PageInfoGenqlSelection {
  endCursor?: boolean | number
  hasNextPage?: boolean | number
  hasPreviousPage?: boolean | number
  startCursor?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
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
  /** Return the active organization from verified user authority. */
  organization?: OrganizationPayloadGenqlSelection
  /** Find visible role metadata in the active organization. */
  role?: RolePayloadGenqlSelection & { __args: { id: Scalars['ID'] } }
  /** Return the complete visible role metadata collection, ordered by ID. */
  roles?: RoleConnectionGenqlSelection
  /** Return the verified current session identity. */
  session?: SessionPayloadGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface RoleGenqlSelection {
  color?: boolean | number
  description?: boolean | number
  id?: boolean | number
  name?: boolean | number
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

export interface SessionGenqlSelection {
  /** Login session identity, serialized as a string without numeric precision loss. */
  id?: boolean | number
  /** Organization selected for this verified session. */
  organizationId?: boolean | number
  /** Signed display snapshot; the name never establishes organization authority. */
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

/** Exact Unix-second validity and client throttle values encoded as decimal strings. */
export interface TokenLifetimeGenqlSelection {
  expiresAt?: boolean | number
  expiresInSeconds?: boolean | number
  issuedAt?: boolean | number
  throttleSeconds?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

const BooleanPayload_possibleTypes: string[] = ['BooleanPayload']
export const isBooleanPayload = (obj?: { __typename?: any } | null): obj is BooleanPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isBooleanPayload"')
  return BooleanPayload_possibleTypes.includes(obj.__typename)
}

const CreatedGroup_possibleTypes: string[] = ['CreatedGroup']
export const isCreatedGroup = (obj?: { __typename?: any } | null): obj is CreatedGroup => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isCreatedGroup"')
  return CreatedGroup_possibleTypes.includes(obj.__typename)
}

const CreatedGroupPayload_possibleTypes: string[] = ['CreatedGroupPayload']
export const isCreatedGroupPayload = (obj?: { __typename?: any } | null): obj is CreatedGroupPayload => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isCreatedGroupPayload"')
  return CreatedGroupPayload_possibleTypes.includes(obj.__typename)
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

const Mutation_possibleTypes: string[] = ['Mutation']
export const isMutation = (obj?: { __typename?: any } | null): obj is Mutation => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isMutation"')
  return Mutation_possibleTypes.includes(obj.__typename)
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

const TokenLifetime_possibleTypes: string[] = ['TokenLifetime']
export const isTokenLifetime = (obj?: { __typename?: any } | null): obj is TokenLifetime => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isTokenLifetime"')
  return TokenLifetime_possibleTypes.includes(obj.__typename)
}

export const enumCredentialDelivery = {
  COOKIE: 'COOKIE' as const,
  TOKEN: 'TOKEN' as const,
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
