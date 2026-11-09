// Interim offline UI type snapshot. No network transport is implemented here.
/**
 * Public timing metadata; contains no token claims or credentials.
 */
export interface TokenLifetime {
  /**
     * Token expiry time, Unix seconds.
     * @minimum 0
     */
  expiresAt: number;
  /**
     * Remaining whole seconds when the response is prepared. Zero means expired.
     * @minimum 0
     */
  expiresInSeconds: number;
  /**
     * Token issuance time, Unix seconds.
     * @minimum 0
     */
  issuedAt: number;
  /**
     * Minimum client wait before requesting another token, in seconds.
     * OTP responses return 30; access and refresh tokens return 0. Independent of expiry.
     * @minimum 0
     */
  throttleSeconds: number;
}

/**
 * Both delivery modes return validity metadata; only token delivery includes credentials.
 */
export interface AuthTokensResponse {
  /** Access-token validity, present for both delivery modes. */
  accessLifetime: TokenLifetime;
  /**
     * Access CWT, present only when delivery is token.
     * @nullable
     */
  accessToken?: string | null;
  /** Refresh-token validity, present for both delivery modes. */
  refreshLifetime: TokenLifetime;
  /**
     * Refresh CWT, present only when delivery is token.
     * @nullable
     */
  refreshToken?: string | null;
}

/**
 * How each signup/signin step delivers access credentials. Never stored in CWT claims.
 */
export type TokenDelivery = typeof TokenDelivery[keyof typeof TokenDelivery];


export const TokenDelivery = {
  cookie: 'cookie',
  token: 'token',
} as const;

/**
 * Email challenge, verification code, and credential delivery for an existing user.
 */
export interface ConfirmSigninOtpRequest {
  /**
     * Six ASCII digits received by email; surrounding whitespace is trimmed. Valid for ten minutes.
     * @minLength 6
     * @maxLength 6
     * @pattern ^[0-9]*$
     */
  code: string;
  /** Cookie (default) sets HttpOnly cookies; token returns both CWTs in JSON. */
  delivery?: TokenDelivery;
}

/**
 * Email challenge and code used to confirm signup.
 */
export interface ConfirmSignupOtpRequest {
  /**
     * Six ASCII digits received by email; surrounding whitespace is trimmed. Submit before expiry.
     * @minLength 6
     * @maxLength 6
     * @pattern ^[0-9]*$
     */
  code: string;
  /**
     * Credential delivery: cookie (default) sets HttpOnly cookies and requires Origin;
     * token returns both CWTs in JSON without setting cookies.
     */
  delivery?: TokenDelivery;
}

export interface ProviderHeader {
  /**
     * @maxLength 128
     * @pattern ^[!#$%&'*+.^_`|~0-9A-Za-z-]+$
     */
  key: string;
  /**
     * Write-only header content; never returned by management reads.
     * @pattern ^[\t\x20-\x7e\x80-\xff]*$
     */
  value: string;
}

/**
 * Capabilities declared by a catalog or configured model; not an authorization grant.
 */
export type ProviderModelCapability = typeof ProviderModelCapability[keyof typeof ProviderModelCapability];


export const ProviderModelCapability = {
  text: 'text',
  vision: 'vision',
  pdf: 'pdf',
  audio: 'audio',
  video: 'video',
  image_generation: 'image_generation',
  audio_generation: 'audio_generation',
  search: 'search',
  reasoning: 'reasoning',
  tool_call: 'tool_call',
  forced_tool_choice: 'forced_tool_choice',
  structured_output: 'structured_output',
} as const;

/**
 * Opaque registry identity, independent of an edited upstream model name.
 */
export interface ProviderCatalogRef {
  model: string;
  provider: string;
}

/**
 * Empty object means no configured effort levels. Configured values cannot be empty.
 */
export interface ProviderEffortMapping {
  /**
     * @minLength 1
     * @maxLength 25
     * @pattern ^[A-Za-z0-9_-]+$
     */
  high?: string;
  /**
     * @minLength 1
     * @maxLength 25
     * @pattern ^[A-Za-z0-9_-]+$
     */
  low?: string;
  /**
     * @minLength 1
     * @maxLength 25
     * @pattern ^[A-Za-z0-9_-]+$
     */
  max?: string;
  /**
     * @minLength 1
     * @maxLength 25
     * @pattern ^[A-Za-z0-9_-]+$
     */
  medium?: string;
  /**
     * @minLength 1
     * @maxLength 25
     * @pattern ^[A-Za-z0-9_-]+$
     */
  xhigh?: string;
}

/**
 * Text operation determining the upstream request and response format.
 */
export type TextModelProtocol = typeof TextModelProtocol[keyof typeof TextModelProtocol];


export const TextModelProtocol = {
  openai_chat_completions: 'openai_chat_completions',
  openai_responses: 'openai_responses',
  anthropic_messages: 'anthropic_messages',
} as const;

/**
 * Complete configured text-model values. Decimal prices are USD per million tokens.
 */
export interface ProviderModelConfiguration {
  /** @pattern ^0*[0-9]{1,12}(?:\.[0-9]{1,8})?$ */
  cachedPrice: string;
  capabilities?: ProviderModelCapability[];
  catalogRef?: ProviderCatalogRef;
  effortMapping?: ProviderEffortMapping;
  enabled?: boolean;
  /** @pattern ^0*[0-9]{1,12}(?:\.[0-9]{1,8})?$ */
  inputPrice: string;
  /**
     * @minimum 1
     * @maximum 4294967295
     */
  maxContextTokens: number;
  /**
     * @minLength 1
     * @maxLength 40
     * @pattern ^[A-Za-z0-9_-]+$
     */
  name: string;
  /** @pattern ^0*[0-9]{1,12}(?:\.[0-9]{1,8})?$ */
  outputPrice: string;
  /**
     * @maxLength 40
     * @pattern ^/(?:[A-Za-z0-9_-][A-Za-z0-9_/-]*)?$
     */
  path: string;
  protocol: TextModelProtocol;
}

/**
 * Persisted identity is omitted for new models and forbidden during provider creation.
 */
export interface ProviderModelWrite {
  /** Complete text-model configuration, including explicit token prices. */
  configuration: ProviderModelConfiguration;
  /**
     * Omit for a new model; supplied IDs must belong to this connection.
     * @pattern ^[1-9][0-9]*$
     */
  id?: string;
}

/**
 * New connection definition; credentials and header values are write-only.
 */
export interface ProviderCreateInput {
  /**
     * @minLength 1
     * @maxLength 512
     * @pattern ^[^\s]+$
     */
  apiKey: string;
  /** @maxLength 512 */
  baseUrl: string;
  headers: ProviderHeader[];
  /** @minItems 1 */
  models: ProviderModelWrite[];
  /**
     * @minLength 1
     * @maxLength 120
     */
  name: string;
}

/**
 * Atomic batch creation. A single form submits one provider in this array.
 */
export interface CreateProvidersRequest {
  providers: ProviderCreateInput[];
}

/**
 * Header metadata only. A response never includes a header value.
 */
export interface ProviderHeaderMetadata {
  key: string;
}

/**
 * Positive u64 as a decimal string.
 * @pattern ^[1-9][0-9]*$
 */
export type ProviderResourceId = string;

export interface ProviderModelDetail {
  /** Complete text-model configuration, including explicit token prices. */
  configuration: ProviderModelConfiguration;
  id: ProviderResourceId;
}

/**
 * Safe management detail. API keys and header values are absent by construction.
 */
export interface ProviderDetailResponse {
  baseUrl: string;
  headers: ProviderHeaderMetadata[];
  id: ProviderResourceId;
  models: ProviderModelDetail[];
  name: string;
}

export interface CreateProvidersResponse {
  providers: ProviderDetailResponse[];
}

/**
 * Identity and organization scope from the current signed access credential.
 */
export interface CurrentUserResponse {
  /**
     * Organization selected for this session.
     * @minimum 0
     */
  organizationId: number;
  /** Organization name snapshot from the signed session; refreshed when tokens renew. */
  organizationName: string;
  /**
     * Current login session identifier.
     * @minimum 0
     */
  sessionId: number;
  /**
     * Authenticated user identifier.
     * @minimum 0
     */
  userId: number;
}

export interface ErrorResponse {
  /**
     * Human-readable explanation; may include safe inline HTML and links.
     * Clients preserve message wording and sanitize markup before rendering.
     */
  message: string;
}

/**
 * Omit value to retain an existing header secret; new names require a value.
 */
export interface ProviderHeaderUpdate {
  /**
     * @maxLength 128
     * @pattern ^[!#$%&'*+.^_`|~0-9A-Za-z-]+$
     */
  key: string;
  /**
     * Omit to preserve the saved value for this header name. New names require a value.
     * @pattern ^[\t\x20-\x7e\x80-\xff]*$
     */
  value?: string;
}

/**
 * Partial update. Omitted fields are unchanged; explicit null is rejected.
 */
export interface PatchProviderRequest {
  /**
     * Omit to preserve the current credential. Empty strings and null are invalid.
     * @minLength 1
     * @maxLength 512
     * @pattern ^[^\s]+$
     */
  apiKey?: string;
  /** @maxLength 512 */
  baseUrl?: string;
  headers?: ProviderHeaderUpdate[];
  /** @minItems 1 */
  models?: ProviderModelWrite[];
  /**
     * @minLength 1
     * @maxLength 120
     */
  name?: string;
}

/**
 * Catalog connection family hint; each configured model selects its own operation.
 */
export type ProviderApiProtocol = typeof ProviderApiProtocol[keyof typeof ProviderApiProtocol];


export const ProviderApiProtocol = {
  openai_compatible: 'openai_compatible',
  anthropic: 'anthropic',
  provider_specific: 'provider_specific',
} as const;

export interface ProviderFields {
  /**
     * @minLength 1
     * @maxLength 512
     * @pattern ^[^\s]+$
     */
  apiKey: string;
  /** @maxLength 512 */
  baseUrl: string;
  /**
     * @minLength 1
     * @maxLength 120
     */
  name: string;
}

export interface ProviderHeaders {
  entries: ProviderHeader[];
}

/**
 * Independently exported rules for optional/draft values and collection adapters.
 */
export interface ProviderInputRules {
  authorization: string;
  /**
     * @minLength 1
     * @maxLength 25
     * @pattern ^[A-Za-z0-9_-]+$
     */
  effortValue: string;
  /** @pattern ^[1-9][0-9]*$ */
  positiveInteger: string;
}

/**
 * Safe connection summary. IDs are positive decimal strings, not JSON numbers.
 */
export interface ProviderSummary {
  baseUrl: string;
  id: ProviderResourceId;
  name: string;
}

export interface ProviderListResponse {
  providers: ProviderSummary[];
}

export interface ProviderModelFields {
  /** @pattern ^0*[0-9]{1,12}(?:\.[0-9]{1,8})?$ */
  cachedPrice: string;
  /** @pattern ^0*[0-9]{1,12}(?:\.[0-9]{1,8})?$ */
  inputPrice: string;
  /**
     * @minimum 1
     * @maximum 4294967295
     */
  maxContextTokens: number;
  /**
     * @minLength 1
     * @maxLength 40
     * @pattern ^[A-Za-z0-9_-]+$
     */
  name: string;
  /** @pattern ^0*[0-9]{1,12}(?:\.[0-9]{1,8})?$ */
  outputPrice: string;
  /**
     * @maxLength 40
     * @pattern ^/(?:[A-Za-z0-9_-][A-Za-z0-9_/-]*)?$
     */
  path: string;
}

export interface ProviderModels {
  entries: ProviderModelFields[];
}

/**
 * Prices may be unknown in registry data; missing rates never imply zero.
 */
export interface ProviderRegistryModel {
  /** @nullable */
  cachedPrice?: string | null;
  capabilities: ProviderModelCapability[];
  /** @nullable */
  description?: string | null;
  effortMapping?: ProviderEffortMapping | null;
  id: string;
  /** @nullable */
  inputPrice?: string | null;
  /**
     * @minimum 0
     * @nullable
     */
  maxContextTokens?: number | null;
  name: string;
  /** @nullable */
  outputPrice?: string | null;
  /** Text operations supported by this catalog model; an empty list is not selectable in this flow. */
  protocols: TextModelProtocol[];
}

/**
 * Provider identity and editable connection defaults from the registry.
 */
export interface ProviderRegistryEntry {
  /**
     * Null when an account-specific endpoint must be supplied during configuration.
     * @nullable
     */
  baseUrl?: string | null;
  id: string;
  name: string;
  protocol: ProviderApiProtocol;
}

export interface ProviderRegistryDetailResponse {
  models: ProviderRegistryModel[];
  provider: ProviderRegistryEntry;
}

export interface ProviderRegistryListResponse {
  providers: ProviderRegistryEntry[];
}

/**
 * Full non-secret replacement. Omitting apiKey preserves the saved credential.
 */
export interface ReplaceProviderRequest {
  /**
     * Omit to preserve the current credential. Empty strings and null are invalid.
     * @minLength 1
     * @maxLength 512
     * @pattern ^[^\s]+$
     */
  apiKey?: string;
  /** @maxLength 512 */
  baseUrl: string;
  headers: ProviderHeaderUpdate[];
  /** @minItems 1 */
  models: ProviderModelWrite[];
  /**
     * @minLength 1
     * @maxLength 120
     */
  name: string;
}

/**
 * Email address to receive a signin verification code.
 */
export interface SigninEmailRequest {
  /** Cookie (default) sets the OTP access cookie and returns lifetime metadata; token additionally returns the credential. */
  delivery?: TokenDelivery;
  /** Trimmed and ASCII-lowercased before email validation. Account existence is checked only at confirmation. */
  email: string;
}

/**
 * Email address to receive a signup verification code.
 */
export interface SignupEmailRequest {
  /** Cookie (default) sets the OTP access cookie and returns lifetime metadata; token additionally returns the credential. */
  delivery?: TokenDelivery;
  /** Trimmed and ASCII-lowercased before email validation. Any valid domain is accepted. */
  email: string;
}

/**
 * Verification challenge returned after email delivery is requested.
 */
export interface SignupEmailResponse {
  /** OTP challenge validity in both cookie and token delivery. */
  lifetime: TokenLifetime;
  /**
     * Encrypted OTP access token, present only in token delivery. Expires after 10 minutes.
     * This is not a user access credential and must not be used to call protected APIs.
     * @nullable
     */
  token?: string | null;
}
