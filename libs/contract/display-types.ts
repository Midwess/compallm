// Interim safe display/response types; form input types are generated from Rust.
import type {
  EmailInput,
  EmailCodeInput,
  ProviderFields,
  ProviderModelFields,
  ProviderHeaders,
  ProviderHeader,
  ProviderModels,
  ProviderInputRules,
  CreateProvidersRequest,
  ProviderCreateInput,
  ProviderModelWrite,
  ProviderModelConfiguration,
  TextModelProtocol,
  ProviderEffortMapping,
  ProviderModelCapability,
  ProviderCatalogRef,
  ReplaceProviderRequest,
  ProviderHeaderUpdate,
  PatchProviderRequest,
  SignupEmailRequest,
  SigninEmailRequest,
  ConfirmSignupOtpRequest,
  ConfirmSigninOtpRequest,
} from './form-types.ts'
/**
 * Public timing metadata; contains no token claims or credentials.
 */
export interface TokenLifetime {
  /**
   * Token expiry time, Unix seconds.
   * @minimum 0
   */
  expiresAt: number
  /**
   * Remaining whole seconds when the response is prepared. Zero means expired.
   * @minimum 0
   */
  expiresInSeconds: number
  /**
   * Token issuance time, Unix seconds.
   * @minimum 0
   */
  issuedAt: number
  /**
   * Minimum client wait before requesting another token, in seconds.
   * OTP responses return 30; access and refresh tokens return 0. Independent of expiry.
   * @minimum 0
   */
  throttleSeconds: number
}

/**
 * Both delivery modes return validity metadata; only token delivery includes credentials.
 */
export interface AuthTokensResponse {
  /** Access-token validity, present for both delivery modes. */
  accessLifetime: TokenLifetime
  /**
   * Access CWT, present only when delivery is token.
   * @nullable
   */
  accessToken?: string | null
  /** Refresh-token validity, present for both delivery modes. */
  refreshLifetime: TokenLifetime
  /**
   * Refresh CWT, present only when delivery is token.
   * @nullable
   */
  refreshToken?: string | null
}

/**
 * How each signup/signin step delivers access credentials. Never stored in CWT claims.
 */
export type TokenDelivery = (typeof TokenDelivery)[keyof typeof TokenDelivery]

export const TokenDelivery = {
  cookie: 'cookie',
  token: 'token',
} as const

/**
 * Header metadata only. A response never includes a header value.
 */
export interface ProviderHeaderMetadata {
  key: string
}

/**
 * Positive u64 as a decimal string.
 * @pattern ^[1-9][0-9]*$
 */
export type ProviderResourceId = string

export interface ProviderModelDetail {
  /** Complete text-model configuration, including explicit token prices. */
  configuration: ProviderModelConfiguration
  id: ProviderResourceId
}

/**
 * Safe management detail. API keys and header values are absent by construction.
 */
export interface ProviderDetailResponse {
  baseUrl: string
  headers: ProviderHeaderMetadata[]
  id: ProviderResourceId
  models: ProviderModelDetail[]
  name: string
}

export interface CreateProvidersResponse {
  providers: ProviderDetailResponse[]
}

/**
 * Identity and organization scope from the current signed access credential.
 */
export interface CurrentUserResponse {
  /**
   * Organization selected for this session.
   * @minimum 0
   */
  organizationId: number
  /** Organization name snapshot from the signed session; refreshed when tokens renew. */
  organizationName: string
  /**
   * Current login session identifier.
   * @minimum 0
   */
  sessionId: number
  /**
   * Authenticated user identifier.
   * @minimum 0
   */
  userId: number
}

export interface ErrorResponse {
  /**
   * Human-readable explanation; may include safe inline HTML and links.
   * Clients preserve message wording and sanitize markup before rendering.
   */
  message: string
}

/**
 * Catalog connection family hint; each configured model selects its own operation.
 */
export type ProviderApiProtocol = (typeof ProviderApiProtocol)[keyof typeof ProviderApiProtocol]

export const ProviderApiProtocol = {
  openai_compatible: 'openai_compatible',
  anthropic: 'anthropic',
  provider_specific: 'provider_specific',
} as const

/**
 * Safe connection summary. IDs are positive decimal strings, not JSON numbers.
 */
export interface ProviderSummary {
  baseUrl: string
  id: ProviderResourceId
  name: string
}

export interface ProviderListResponse {
  providers: ProviderSummary[]
}

/**
 * Prices may be unknown in registry data; missing rates never imply zero.
 */
export interface ProviderRegistryModel {
  /** @nullable */
  cachedPrice?: string | null
  capabilities: ProviderModelCapability[]
  /** @nullable */
  description?: string | null
  effortMapping?: ProviderEffortMapping | null
  id: string
  /** @nullable */
  inputPrice?: string | null
  /**
   * @minimum 0
   * @nullable
   */
  maxContextTokens?: number | null
  name: string
  /** @nullable */
  outputPrice?: string | null
  /** Text operations supported by this catalog model; an empty list is not selectable in this flow. */
  protocols: TextModelProtocol[]
}

/**
 * Provider identity and editable connection defaults from the registry.
 */
export interface ProviderRegistryEntry {
  /**
   * Null when an account-specific endpoint must be supplied during configuration.
   * @nullable
   */
  baseUrl?: string | null
  id: string
  name: string
  protocol: ProviderApiProtocol
}

export interface ProviderRegistryDetailResponse {
  models: ProviderRegistryModel[]
  provider: ProviderRegistryEntry
}

export interface ProviderRegistryListResponse {
  providers: ProviderRegistryEntry[]
}

/**
 * Verification challenge returned after email delivery is requested.
 */
export interface SignupEmailResponse {
  /** OTP challenge validity in both cookie and token delivery. */
  lifetime: TokenLifetime
  /**
   * Encrypted OTP access token, present only in token delivery. Expires after 10 minutes.
   * This is not a user access credential and must not be used to call protected APIs.
   * @nullable
   */
  token?: string | null
}
