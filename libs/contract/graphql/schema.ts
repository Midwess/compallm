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

/** How the request delivers credentials; this is never stored in signed claims. */
export type CredentialDelivery = 'COOKIE' | 'TOKEN'

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
  /**
   * Read the selected login's verified identity and organization-name snapshot.
   *
   * Requires active, current user access. No credentials, policy source, or
   * freshness internals are exposed; no user/organization hydration is performed.
   */
  session: Session
  __typename: 'Query'
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

/** Text operation determining the upstream request and response format. */
export type TextModelProtocol = 'ANTHROPIC_MESSAGES' | 'OPENAI_CHAT_COMPLETIONS' | 'OPENAI_RESPONSES'

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
  delivery?: CredentialDelivery
}

/**
 * Shared input for requesting either a signup or a signin email challenge.
 *
 * Call `validify()` before Core execution. GraphQL parsing alone does not run
 * normalization, application validation, or email ownership checks.
 */
export interface EmailInput {
  /** Cookie delivery is the default; token delivery must be requested explicitly. */
  delivery?: CredentialDelivery
  /** Trimmed and ASCII-lowercased using the shared Rust email policy. */
  email: Scalars['String']
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
  /**
   * Read the selected login's verified identity and organization-name snapshot.
   *
   * Requires active, current user access. No credentials, policy source, or
   * freshness internals are exposed; no user/organization hydration is performed.
   */
  session?: SessionGenqlSelection
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

const Session_possibleTypes: string[] = ['Session']
export const isSession = (obj?: { __typename?: any } | null): obj is Session => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isSession"')
  return Session_possibleTypes.includes(obj.__typename)
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
