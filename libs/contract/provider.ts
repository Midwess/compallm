import { z } from 'zod'
import type { ProviderHeader as GraphqlHeader } from './graphql.ts'
import {
  ProviderCreateInput,
  ProviderCreateInputInput,
  ProviderCreateInputConstraints,
  ProviderModelConfiguration,
  ProviderModelConfigurationInput,
  ProviderModelConfigurationConstraints,
  ProviderHeader,
  ProviderHeaderInput,
  ProviderHeaderConstraints,
  ProviderEffortMapping,
  ProviderEffortMappingInput,
  ProviderEffortMappingConstraints,
} from './validation.ts'
import { headersObject, modelsObject, authorizationInput, headerObject } from './runtime.ts'

// UI projections reuse canonical generated fields; no API shapes or rules are
// declared here. Draft-only text parsing stays outside the wire contract.
export const ProviderFields = z
  .object({
    name: ProviderCreateInput.shape.name,
    baseUrl: ProviderCreateInput.shape.baseUrl,
    apiKey: ProviderCreateInput.shape.apiKey,
  })
  .strict()
export const ProviderFieldsInput = ProviderCreateInputInput
export const ProviderFieldsConstraints = ProviderCreateInputConstraints
export const ProviderModelFields = ProviderModelConfiguration.pick({
  name: true,
  path: true,
  maxContextTokens: true,
  inputPrice: true,
  outputPrice: true,
  cachedPrice: true,
})
export const ProviderModelFieldsInput = ProviderModelConfigurationInput
export const ProviderModelFieldsConstraints = ProviderModelConfigurationConstraints
export const ProviderModels = z
  .object({ entries: z.array(ProviderModelFields) })
  .strict()
  .refine(modelsObject)
export const ProviderHeaders = z
  .object({ entries: z.array(ProviderHeader) })
  .strict()
  .refine(headersObject)
export { ProviderHeader, ProviderHeaderInput, ProviderHeaderConstraints }

export function contextTokensFromText(value: string): number {
  const number = Number(value)
  return String(number) === value && ProviderModelConfigurationInput.maxContextTokens(value) ? number : Number.NaN
}
export const ProviderInputRules = z.object({
  effortValue: ProviderEffortMapping.shape.low.unwrap(),
  authorization: ProviderHeader.shape.value.refine((value) => headerObject({ key: 'Authorization', value })),
  positiveInteger: z
    .string()
    .refine((value) => ProviderModelFields.shape.maxContextTokens.safeParse(contextTokensFromText(value)).success),
})
export const ProviderInputRulesInput = {
  effortValue: ProviderEffortMappingInput.low,
  authorization: authorizationInput,
}
export const ProviderInputRulesConstraints = {
  effortValue: ProviderEffortMappingConstraints.low,
}

export type HeaderInput = Pick<GraphqlHeader, 'key' | 'value'>

export function headersAreValid(headers: HeaderInput[]): boolean {
  return ProviderHeaders.safeParse({
    entries: headers.map(({ key, value }) => ({ key, value })),
  }).success
}

// Drafts may contain an empty key or an incomplete Authorization value.
// Final collection validation remains required before submission.
export function acceptsHeaderInput(entry: HeaderInput): boolean {
  return (
    ProviderHeaderInput.key(entry.key) &&
    ProviderHeaderInput.value(entry.value) &&
    (entry.key.toLowerCase() !== 'authorization' || authorizationInput(entry.value))
  )
}
