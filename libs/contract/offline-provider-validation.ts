// Interim provider preview snapshot. Not regenerated or synchronized with Rust.
// Replace with actual GraphQL input contracts when provider operations are defined.

import { z } from 'zod'

import {
  inputString,
  stringInput,
  integerInput,
  headerObject,
  headersObject,
  modelsObject,
  headerUpdateObject,
  modelWritesArray,
  createProviderObject,
  createProvidersObject,
  providerPatchObject,
} from './runtime.ts'

export const ProviderHeaderConstraints = {
  key: { maxLength: 128, pattern: "^[!#$%&'*+.^_`|~0-9A-Za-z-]+$" },
  value: { pattern: '^[\\t\\x20-\\x7e\\x80-\\xff]*$' },
} as const

export const ProviderHeaderInput = {
  key: stringInput(ProviderHeaderConstraints['key']),
  value: stringInput(ProviderHeaderConstraints['value']),
}

export const ProviderHeader = z
  .object({
    key: inputString({ maxLength: 128, pattern: "^[!#$%&'*+.^_`|~0-9A-Za-z-]+$" }),
    value: inputString({ pattern: '^[\\t\\x20-\\x7e\\x80-\\xff]*$' }).describe(
      'Write-only header content; never returned by management reads.',
    ),
  })
  .strict()
  .refine(headerObject, { message: 'Invalid input' })

export type ProviderHeader = z.input<typeof ProviderHeader>

export type ProviderHeaderOutput = z.output<typeof ProviderHeader>

/** Text operation determining the upstream request and response format. */
export const TextModelProtocol = z
  .enum(['openai_chat_completions', 'openai_responses', 'anthropic_messages'])
  .describe('Text operation determining the upstream request and response format.')

export type TextModelProtocol = z.input<typeof TextModelProtocol>

export const ProviderEffortMappingConstraints = {
  low: { minLength: 1, maxLength: 25, pattern: '^[A-Za-z0-9_-]+$' },
  medium: { minLength: 1, maxLength: 25, pattern: '^[A-Za-z0-9_-]+$' },
  high: { minLength: 1, maxLength: 25, pattern: '^[A-Za-z0-9_-]+$' },
  xhigh: { minLength: 1, maxLength: 25, pattern: '^[A-Za-z0-9_-]+$' },
  max: { minLength: 1, maxLength: 25, pattern: '^[A-Za-z0-9_-]+$' },
} as const

export const ProviderEffortMappingInput = {
  low: stringInput(ProviderEffortMappingConstraints['low']),
  medium: stringInput(ProviderEffortMappingConstraints['medium']),
  high: stringInput(ProviderEffortMappingConstraints['high']),
  xhigh: stringInput(ProviderEffortMappingConstraints['xhigh']),
  max: stringInput(ProviderEffortMappingConstraints['max']),
}

/** Empty object means no configured effort levels. Configured values cannot be empty. */
export const ProviderEffortMapping = z
  .object({
    low: inputString({ minLength: 1, maxLength: 25, pattern: '^[A-Za-z0-9_-]+$' }).optional(),
    medium: inputString({ minLength: 1, maxLength: 25, pattern: '^[A-Za-z0-9_-]+$' }).optional(),
    high: inputString({ minLength: 1, maxLength: 25, pattern: '^[A-Za-z0-9_-]+$' }).optional(),
    xhigh: inputString({ minLength: 1, maxLength: 25, pattern: '^[A-Za-z0-9_-]+$' }).optional(),
    max: inputString({ minLength: 1, maxLength: 25, pattern: '^[A-Za-z0-9_-]+$' }).optional(),
  })
  .strict()
  .describe('Empty object means no configured effort levels. Configured values cannot be empty.')

export type ProviderEffortMapping = z.input<typeof ProviderEffortMapping>

export type ProviderEffortMappingOutput = z.output<typeof ProviderEffortMapping>

/** Capabilities declared by a catalog or configured model; not an authorization grant. */
export const ProviderModelCapability = z
  .enum([
    'text',
    'vision',
    'pdf',
    'audio',
    'video',
    'image_generation',
    'audio_generation',
    'search',
    'reasoning',
    'tool_call',
    'forced_tool_choice',
    'structured_output',
  ])
  .describe('Capabilities declared by a catalog or configured model; not an authorization grant.')

export type ProviderModelCapability = z.input<typeof ProviderModelCapability>

export type ProviderModelCapabilityOutput = z.output<typeof ProviderModelCapability>

export const ProviderCatalogRefConstraints = {} as const

export const ProviderCatalogRefInput = {}

/** Opaque registry identity, independent of an edited upstream model name. */
export const ProviderCatalogRef = z
  .object({ provider: inputString({}), model: inputString({}) })
  .strict()
  .describe('Opaque registry identity, independent of an edited upstream model name.')

export type ProviderCatalogRef = z.input<typeof ProviderCatalogRef>

export type ProviderCatalogRefOutput = z.output<typeof ProviderCatalogRef>

export const ProviderModelConfigurationConstraints = {
  name: { minLength: 1, maxLength: 40, pattern: '^[A-Za-z0-9_-]+$' },
  path: { maxLength: 40, pattern: '^/(?:[A-Za-z0-9_-][A-Za-z0-9_/-]*)?$' },
  inputPrice: { pattern: '^0*[0-9]{1,12}(?:\\.[0-9]{1,8})?$', inputPattern: '^0*[0-9]{0,12}(?:\\.[0-9]{0,8})?$' },
  outputPrice: { pattern: '^0*[0-9]{1,12}(?:\\.[0-9]{1,8})?$', inputPattern: '^0*[0-9]{0,12}(?:\\.[0-9]{0,8})?$' },
  cachedPrice: { pattern: '^0*[0-9]{1,12}(?:\\.[0-9]{1,8})?$', inputPattern: '^0*[0-9]{0,12}(?:\\.[0-9]{0,8})?$' },
} as const

export const ProviderModelConfigurationInput = {
  name: stringInput(ProviderModelConfigurationConstraints['name']),
  path: stringInput(ProviderModelConfigurationConstraints['path']),
  maxContextTokens: integerInput({ minimum: 1, maximum: 4294967295 }),
  inputPrice: stringInput(ProviderModelConfigurationConstraints['inputPrice']),
  outputPrice: stringInput(ProviderModelConfigurationConstraints['outputPrice']),
  cachedPrice: stringInput(ProviderModelConfigurationConstraints['cachedPrice']),
}

/** Complete configured text-model values. Decimal prices are USD per million tokens. */
export const ProviderModelConfiguration = z
  .object({
    name: inputString({ minLength: 1, maxLength: 40, pattern: '^[A-Za-z0-9_-]+$' }),
    protocol: TextModelProtocol,
    path: inputString({ maxLength: 40, pattern: '^/(?:[A-Za-z0-9_-][A-Za-z0-9_/-]*)?$' }),
    maxContextTokens: z.int().min(1).max(4294967295),
    enabled: z.boolean().default(true),
    effortMapping: ProviderEffortMapping.default({}),
    capabilities: z.array(ProviderModelCapability).default([]),
    catalogRef: ProviderCatalogRef.optional(),
    inputPrice: inputString({
      pattern: '^0*[0-9]{1,12}(?:\\.[0-9]{1,8})?$',
      inputPattern: '^0*[0-9]{0,12}(?:\\.[0-9]{0,8})?$',
    }),
    outputPrice: inputString({
      pattern: '^0*[0-9]{1,12}(?:\\.[0-9]{1,8})?$',
      inputPattern: '^0*[0-9]{0,12}(?:\\.[0-9]{0,8})?$',
    }),
    cachedPrice: inputString({
      pattern: '^0*[0-9]{1,12}(?:\\.[0-9]{1,8})?$',
      inputPattern: '^0*[0-9]{0,12}(?:\\.[0-9]{0,8})?$',
    }),
  })
  .strict()
  .describe('Complete configured text-model values. Decimal prices are USD per million tokens.')

export type ProviderModelConfiguration = z.input<typeof ProviderModelConfiguration>

export type ProviderModelConfigurationOutput = z.output<typeof ProviderModelConfiguration>

export const ProviderModelWriteConstraints = { id: { pattern: '^[1-9][0-9]*$', format: 'resourceId' } } as const

export const ProviderModelWriteInput = { id: stringInput(ProviderModelWriteConstraints['id']) }

/** Persisted identity is omitted for new models and forbidden during provider creation. */
export const ProviderModelWrite = z
  .object({
    id: inputString({ pattern: '^[1-9][0-9]*$', format: 'resourceId' })
      .describe('Omit for a new model; supplied IDs must belong to this connection.')
      .optional(),
    configuration: ProviderModelConfiguration.describe(
      'Complete text-model configuration, including explicit token prices.',
    ),
  })
  .strict()
  .describe('Persisted identity is omitted for new models and forbidden during provider creation.')

export type ProviderModelWrite = z.input<typeof ProviderModelWrite>

export type ProviderModelWriteOutput = z.output<typeof ProviderModelWrite>

export const ProviderCreateInputConstraints = {
  name: { minLength: 1, maxLength: 120, format: 'nonblank', trim: true },
  baseUrl: { maxLength: 512, format: 'baseUrl' },
  apiKey: { minLength: 1, maxLength: 512, pattern: '^[^\\s]+$' },
} as const

export const ProviderCreateInputInput = {
  name: stringInput(ProviderCreateInputConstraints['name']),
  baseUrl: stringInput(ProviderCreateInputConstraints['baseUrl']),
  apiKey: stringInput(ProviderCreateInputConstraints['apiKey']),
}

/** New connection definition; credentials and header values are write-only. */
export const ProviderCreateInput = z
  .object({
    name: inputString({ minLength: 1, maxLength: 120, format: 'nonblank', trim: true }),
    baseUrl: inputString({ maxLength: 512, format: 'baseUrl' }),
    apiKey: inputString({ minLength: 1, maxLength: 512, pattern: '^[^\\s]+$' }),
    headers: z.array(ProviderHeader).refine((entries) => headersObject({ entries }), { message: 'Invalid input' }),
    models: z.array(ProviderModelWrite).min(1).refine(modelWritesArray, { message: 'Invalid input' }),
  })
  .strict()
  .refine(createProviderObject, { message: 'Invalid input' })
  .describe('New connection definition; credentials and header values are write-only.')

export type ProviderCreateInput = z.input<typeof ProviderCreateInput>

export type ProviderCreateInputOutput = z.output<typeof ProviderCreateInput>

/** Batch form payload. A single preview form submits one provider in this array. */
export const CreateProvidersRequest = z
  .object({ providers: z.array(ProviderCreateInput) })
  .strict()
  .refine(createProvidersObject, { message: 'Invalid input' })
  .describe('Batch form payload. A single preview form submits one provider in this array.')

export type CreateProvidersRequest = z.input<typeof CreateProvidersRequest>

export const ProviderHeaderUpdateConstraints = {
  key: { maxLength: 128, pattern: "^[!#$%&'*+.^_`|~0-9A-Za-z-]+$" },
  value: { pattern: '^[\\t\\x20-\\x7e\\x80-\\xff]*$' },
} as const

export const ProviderHeaderUpdateInput = {
  key: stringInput(ProviderHeaderUpdateConstraints['key']),
  value: stringInput(ProviderHeaderUpdateConstraints['value']),
}

/** Omit value to retain an existing header secret; new names require a value. */
export const ProviderHeaderUpdate = z
  .object({
    key: inputString({ maxLength: 128, pattern: "^[!#$%&'*+.^_`|~0-9A-Za-z-]+$" }),
    value: inputString({ pattern: '^[\\t\\x20-\\x7e\\x80-\\xff]*$' })
      .describe('Omit to preserve the saved value for this header name. New names require a value.')
      .optional(),
  })
  .strict()
  .refine(headerUpdateObject, { message: 'Invalid input' })
  .describe('Omit value to retain an existing header secret; new names require a value.')

export type ProviderHeaderUpdate = z.input<typeof ProviderHeaderUpdate>

export type ProviderHeaderUpdateOutput = z.output<typeof ProviderHeaderUpdate>

/** Partial update. Omitted fields are unchanged; explicit null is rejected. */
export const PatchProviderRequest = z
  .object({
    name: inputString({ minLength: 1, maxLength: 120, format: 'nonblank', trim: true }).optional(),
    baseUrl: inputString({ maxLength: 512, format: 'baseUrl' }).optional(),
    apiKey: inputString({ minLength: 1, maxLength: 512, pattern: '^[^\\s]+$' })
      .describe('Omit to preserve the current credential. Empty strings and null are invalid.')
      .optional(),
    headers: z
      .array(ProviderHeaderUpdate)
      .refine((entries) => headersObject({ entries }), { message: 'Invalid input' })
      .optional(),
    models: z.array(ProviderModelWrite).min(1).refine(modelWritesArray, { message: 'Invalid input' }).optional(),
  })
  .strict()
  .refine(providerPatchObject, { message: 'Invalid input' })
  .describe('Partial update. Omitted fields are unchanged; explicit null is rejected.')

export type PatchProviderRequest = z.input<typeof PatchProviderRequest>

export const ProviderFieldsConstraints = {
  name: { minLength: 1, maxLength: 120, format: 'nonblank' },
  baseUrl: { maxLength: 512, format: 'baseUrl' },
  apiKey: { minLength: 1, maxLength: 512, pattern: '^[^\\s]+$' },
} as const

export const ProviderFieldsInput = {
  name: stringInput(ProviderFieldsConstraints['name']),
  baseUrl: stringInput(ProviderFieldsConstraints['baseUrl']),
  apiKey: stringInput(ProviderFieldsConstraints['apiKey']),
}

export const ProviderFields = z
  .object({
    name: inputString({ minLength: 1, maxLength: 120, format: 'nonblank' }),
    baseUrl: inputString({ maxLength: 512, format: 'baseUrl' }),
    apiKey: inputString({ minLength: 1, maxLength: 512, pattern: '^[^\\s]+$' }),
  })
  .strict()

export type ProviderFields = z.input<typeof ProviderFields>

export type ProviderFieldsOutput = z.output<typeof ProviderFields>

export const ProviderHeadersConstraints = {} as const

export const ProviderHeadersInput = {}

export const ProviderHeaders = z
  .object({ entries: z.array(ProviderHeader) })
  .strict()
  .refine(headersObject, { message: 'Invalid input' })

export type ProviderHeaders = z.input<typeof ProviderHeaders>

export type ProviderHeadersOutput = z.output<typeof ProviderHeaders>

export const ProviderInputRulesConstraints = {
  effortValue: { minLength: 1, maxLength: 25, pattern: '^[A-Za-z0-9_-]+$' },
  authorization: { format: 'authorization' },
  positiveInteger: { pattern: '^[1-9][0-9]*$' },
} as const

export const ProviderInputRulesInput = {
  effortValue: stringInput(ProviderInputRulesConstraints['effortValue']),
  authorization: stringInput(ProviderInputRulesConstraints['authorization']),
  positiveInteger: stringInput(ProviderInputRulesConstraints['positiveInteger']),
}

/** Independently exported rules for optional/draft values and collection adapters. */
export const ProviderInputRules = z
  .object({
    effortValue: inputString({ minLength: 1, maxLength: 25, pattern: '^[A-Za-z0-9_-]+$' }),
    authorization: inputString({ format: 'authorization' }),
    positiveInteger: inputString({ pattern: '^[1-9][0-9]*$' }),
  })
  .strict()
  .describe('Independently exported rules for optional/draft values and collection adapters.')

export type ProviderInputRules = z.input<typeof ProviderInputRules>

export type ProviderInputRulesOutput = z.output<typeof ProviderInputRules>

export const ProviderModelFieldsConstraints = {
  name: { minLength: 1, maxLength: 40, pattern: '^[A-Za-z0-9_-]+$' },
  path: { maxLength: 40, pattern: '^/(?:[A-Za-z0-9_-][A-Za-z0-9_/-]*)?$' },
  inputPrice: { pattern: '^0*[0-9]{1,12}(?:\\.[0-9]{1,8})?$', inputPattern: '^0*[0-9]{0,12}(?:\\.[0-9]{0,8})?$' },
  outputPrice: { pattern: '^0*[0-9]{1,12}(?:\\.[0-9]{1,8})?$', inputPattern: '^0*[0-9]{0,12}(?:\\.[0-9]{0,8})?$' },
  cachedPrice: { pattern: '^0*[0-9]{1,12}(?:\\.[0-9]{1,8})?$', inputPattern: '^0*[0-9]{0,12}(?:\\.[0-9]{0,8})?$' },
} as const

export const ProviderModelFieldsInput = {
  name: stringInput(ProviderModelFieldsConstraints['name']),
  path: stringInput(ProviderModelFieldsConstraints['path']),
  maxContextTokens: integerInput({ minimum: 1, maximum: 4294967295 }),
  inputPrice: stringInput(ProviderModelFieldsConstraints['inputPrice']),
  outputPrice: stringInput(ProviderModelFieldsConstraints['outputPrice']),
  cachedPrice: stringInput(ProviderModelFieldsConstraints['cachedPrice']),
}

export const ProviderModelFields = z
  .object({
    name: inputString({ minLength: 1, maxLength: 40, pattern: '^[A-Za-z0-9_-]+$' }),
    path: inputString({ maxLength: 40, pattern: '^/(?:[A-Za-z0-9_-][A-Za-z0-9_/-]*)?$' }),
    maxContextTokens: z.int().min(1).max(4294967295),
    inputPrice: inputString({
      pattern: '^0*[0-9]{1,12}(?:\\.[0-9]{1,8})?$',
      inputPattern: '^0*[0-9]{0,12}(?:\\.[0-9]{0,8})?$',
    }),
    outputPrice: inputString({
      pattern: '^0*[0-9]{1,12}(?:\\.[0-9]{1,8})?$',
      inputPattern: '^0*[0-9]{0,12}(?:\\.[0-9]{0,8})?$',
    }),
    cachedPrice: inputString({
      pattern: '^0*[0-9]{1,12}(?:\\.[0-9]{1,8})?$',
      inputPattern: '^0*[0-9]{0,12}(?:\\.[0-9]{0,8})?$',
    }),
  })
  .strict()

export type ProviderModelFields = z.input<typeof ProviderModelFields>

export type ProviderModelFieldsOutput = z.output<typeof ProviderModelFields>

export const ProviderModelsConstraints = {} as const

export const ProviderModelsInput = {}

export const ProviderModels = z
  .object({ entries: z.array(ProviderModelFields) })
  .strict()
  .refine(modelsObject, { message: 'Invalid input' })

export type ProviderModels = z.input<typeof ProviderModels>

export type ProviderModelsOutput = z.output<typeof ProviderModels>

/** Full non-secret replacement. Omitting apiKey preserves the saved credential. */
export const ReplaceProviderRequest = z
  .object({
    name: inputString({ minLength: 1, maxLength: 120, format: 'nonblank', trim: true }),
    baseUrl: inputString({ maxLength: 512, format: 'baseUrl' }),
    apiKey: inputString({ minLength: 1, maxLength: 512, pattern: '^[^\\s]+$' })
      .describe('Omit to preserve the current credential. Empty strings and null are invalid.')
      .optional(),
    headers: z
      .array(ProviderHeaderUpdate)
      .refine((entries) => headersObject({ entries }), { message: 'Invalid input' }),
    models: z.array(ProviderModelWrite).min(1).refine(modelWritesArray, { message: 'Invalid input' }),
  })
  .strict()
  .describe('Full non-secret replacement. Omitting apiKey preserves the saved credential.')

export type ReplaceProviderRequest = z.input<typeof ReplaceProviderRequest>
