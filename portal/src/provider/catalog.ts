import type {
  QueryGenqlSelection,
  ModelTemplateGenqlSelection,
  ConnectionConfigurationGenqlSelection,
} from '@compallm/contract/graphql'
import { graphql } from '../graphql-client'

const entry = { key: true, value: true } as const
const requiredInput = { name: true, kind: true, required: true, allowedValues: true } as const
const authentication = {
  method: true,
  configuration: {
    placement: { location: true, name: true, prefix: true },
    flow: true,
    authorizationUrl: true,
    tokenUrl: true,
    deviceAuthorizationUrl: true,
    scopes: true,
    requiredInputs: requiredInput,
    adapter: true,
    parameters: entry,
  },
} as const
const endpoint = { mode: true, configuration: { urlTemplate: true, requiredInputs: requiredInput } } as const
const connection = {
  baseUrl: true,
  path: true,
  timeoutMs: true,
  query: entry,
  authentication,
  endpoint,
} as const satisfies ConnectionConfigurationGenqlSelection
export const modelDetail = {
  id: true,
  providerTemplateId: true,
  origin: true,
  customFields: true,
  name: true,
  capabilities: true,
  maxContextTokens: true,
  inputPrice: true,
  outputPrice: true,
  cachedPrice: true,
  pricingMode: true,
  cacheBilling: true,
  connections: { protocol: true, configuration: connection },
  headers: entry,
  bodyDefaults: entry,
  text: {
    input: { configuration: { maxTokens: true } },
    output: {
      configuration: {
        maxTokens: true,
        thinking: { configuration: { effortMapping: { level: true, value: true } } },
      },
    },
  },
} as const satisfies ModelTemplateGenqlSelection
export const providerRow = { id: true, name: true, baseUrl: true, logoUrl: true } as const
const modelRow = {
  id: true,
  name: true,
  displayName: true,
  description: true,
  serviceKind: true,
  capabilities: true,
  maxContextTokens: true,
  maxInputTokens: true,
  maxOutputTokens: true,
  inputPrice: true,
  outputPrice: true,
  cachedPrice: true,
  pricingMode: true,
  billingModel: true,
  cacheBilling: true,
  operations: { operation: true, path: true },
} as const satisfies ModelTemplateGenqlSelection
export async function listProviders(query: string, after: string | undefined, signal: AbortSignal) {
  const selection = {
    effectiveProviderTemplates: {
      __args: { first: 20, after, search: query || undefined },
      edges: { node: providerRow },
      pageInfo: { endCursor: true, hasNextPage: true },
    },
  } satisfies QueryGenqlSelection
  return (await graphql(signal).query(selection)).effectiveProviderTemplates
}
export async function getProvider(id: string, signal: AbortSignal) {
  const selection = {
    effectiveProviderTemplate: {
      __args: { id },
      node: {
        ...providerRow,
        description: true,
        connection,
        headers: entry,
        authentication,
        models: { edges: { node: modelRow }, pageInfo: { hasNextPage: true } },
      },
    },
  } satisfies QueryGenqlSelection
  return (await graphql(signal).query(selection)).effectiveProviderTemplate?.node ?? null
}
export async function getModel(providerTemplateId: string, id: string, signal: AbortSignal) {
  const selection = {
    effectiveModelTemplate: { __args: { providerTemplateId, id }, node: modelDetail },
  } satisfies QueryGenqlSelection
  return (await graphql(signal).query(selection)).effectiveModelTemplate?.node ?? null
}
export type ProviderPage = Awaited<ReturnType<typeof listProviders>>
export type ProviderDetail = NonNullable<Awaited<ReturnType<typeof getProvider>>>
export type ModelDetail = NonNullable<Awaited<ReturnType<typeof getModel>>>

export async function getGlobalModel(providerTemplateId: string, id: string, signal: AbortSignal) {
  const selection = { modelTemplate: { __args: { providerTemplateId, id }, node: modelDetail } } satisfies QueryGenqlSelection
  return (await graphql(signal).query(selection)).modelTemplate?.node ?? null
}
