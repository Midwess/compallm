// Generated from Rust. Do not edit; run cargo run -p compallm-api -- generate.

import type { z } from 'zod'
import * as schemas from './validation.ts'
export interface ProviderHeader {
  key: string
  /** Write-only header content; never returned by management reads. */
  value: string
}
/** Text operation determining the upstream request and response format. */
export type TextModelProtocol = 'openai_chat_completions' | 'openai_responses' | 'anthropic_messages'
export const TextModelProtocol = schemas.TextModelProtocol.enum
/** Empty object means no configured effort levels. Configured values cannot be empty. */
export interface ProviderEffortMapping {
  low?: string
  medium?: string
  high?: string
  xhigh?: string
  max?: string
}
/** Capabilities declared by a catalog or configured model; not an authorization grant. */
export type ProviderModelCapability =
  | 'text'
  | 'vision'
  | 'pdf'
  | 'audio'
  | 'video'
  | 'image_generation'
  | 'audio_generation'
  | 'search'
  | 'reasoning'
  | 'tool_call'
  | 'forced_tool_choice'
  | 'structured_output'
export const ProviderModelCapability = schemas.ProviderModelCapability.enum
/** Opaque registry identity, independent of an edited upstream model name. */
export interface ProviderCatalogRef {
  provider: string
  model: string
}
/** Complete configured text-model values. Decimal prices are USD per million tokens. */
export interface ProviderModelConfiguration {
  name: string
  protocol: TextModelProtocol
  path: string
  maxContextTokens: number
  enabled?: boolean
  effortMapping?: ProviderEffortMapping
  capabilities?: Array<ProviderModelCapability>
  catalogRef?: ProviderCatalogRef
  inputPrice: string
  outputPrice: string
  cachedPrice: string
}
/** Persisted identity is omitted for new models and forbidden during provider creation. */
export interface ProviderModelWrite {
  /** Omit for a new model; supplied IDs must belong to this connection. */
  id?: string
  /** Complete text-model configuration, including explicit token prices. */
  configuration: ProviderModelConfiguration
}
/** New connection definition; credentials and header values are write-only. */
export interface ProviderCreateInput {
  name: string
  baseUrl: string
  apiKey: string
  headers: Array<ProviderHeader>
  models: Array<ProviderModelWrite>
}
/** Batch form payload. A single preview form submits one provider in this array. */
export interface CreateProvidersRequest {
  providers: Array<ProviderCreateInput>
}
/** How the request delivers credentials; this is never stored in signed claims. */
export type CredentialDelivery = 'COOKIE' | 'TOKEN'
export const CredentialDelivery = schemas.CredentialDelivery.enum
/** Shared input for confirming a signup or signin email challenge.

The verified encrypted challenge comes from request context, never this input.
The adapter calls `validify()` before Core verifies the code and challenge. */
export interface EmailCodeInput {
  /** Exactly six ASCII digits after trimming surrounding whitespace. */
  code: string
  /** Cookie delivery is the default; token delivery must be requested explicitly. */
  delivery?: CredentialDelivery
}
/** Shared input for requesting either a signup or a signin email challenge.

Call `validify()` before Core execution. GraphQL parsing alone does not run
normalization, application validation, or email ownership checks. */
export interface EmailInput {
  /** Trimmed and ASCII-lowercased using the shared Rust email policy. */
  email: string
  /** Cookie delivery is the default; token delivery must be requested explicitly. */
  delivery?: CredentialDelivery
}
/** Omit value to retain an existing header secret; new names require a value. */
export interface ProviderHeaderUpdate {
  key: string
  /** Omit to preserve the saved value for this header name. New names require a value. */
  value?: string
}
/** Partial update. Omitted fields are unchanged; explicit null is rejected. */
export interface PatchProviderRequest {
  name?: string
  baseUrl?: string
  /** Omit to preserve the current credential. Empty strings and null are invalid. */
  apiKey?: string
  headers?: Array<ProviderHeaderUpdate>
  models?: Array<ProviderModelWrite>
}
export interface ProviderFields {
  name: string
  baseUrl: string
  apiKey: string
}
export interface ProviderHeaders {
  entries: Array<ProviderHeader>
}
/** Independently exported rules for optional/draft values and collection adapters. */
export interface ProviderInputRules {
  effortValue: string
  authorization: string
  positiveInteger: string
}
export interface ProviderModelFields {
  name: string
  path: string
  maxContextTokens: number
  inputPrice: string
  outputPrice: string
  cachedPrice: string
}
export interface ProviderModels {
  entries: Array<ProviderModelFields>
}
/** Full non-secret replacement. Omitting apiKey preserves the saved credential. */
export interface ReplaceProviderRequest {
  name: string
  baseUrl: string
  /** Omit to preserve the current credential. Empty strings and null are invalid. */
  apiKey?: string
  headers: Array<ProviderHeaderUpdate>
  models: Array<ProviderModelWrite>
}
export type SignupEmailRequest = z.input<typeof schemas.SignupEmailRequest>
export type SigninEmailRequest = z.input<typeof schemas.SigninEmailRequest>
export type ConfirmSignupOtpRequest = z.input<typeof schemas.ConfirmSignupOtpRequest>
export type ConfirmSigninOtpRequest = z.input<typeof schemas.ConfirmSigninOtpRequest>
