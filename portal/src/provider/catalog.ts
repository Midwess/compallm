import {
  ProviderCatalogEffectiveProviderTemplatesStore,
  ProviderCatalogEffectiveProviderTemplateStore,
  ProviderCatalogEffectiveModelTemplateStore,
  ProviderCatalogModelTemplateStore,
} from '@compallm/contract/houdini'

import { graphqlClient, requireData, type RequestActivity } from '../graphql-client'

export async function listProviders(query: string, after: string | undefined, signal: AbortSignal, activity?: RequestActivity) {
  return (
    await new ProviderCatalogEffectiveProviderTemplatesStore()
      .fetch({
        variables: { effectiveProviderTemplatesAfter: after, effectiveProviderTemplatesSearch: query || undefined },
        fetch: graphqlClient.fetch(signal),
        metadata: { requestActivity: activity },
        policy: 'NoCache',
        blocking: true,
      })
      .then(requireData)
  ).effectiveProviderTemplates
}
export async function getProvider(id: string, signal: AbortSignal, activity?: RequestActivity) {
  return (
    (
      await new ProviderCatalogEffectiveProviderTemplateStore()
        .fetch({
          variables: { effectiveProviderTemplateId: id },
          fetch: graphqlClient.fetch(signal),
          metadata: { requestActivity: activity },
          policy: 'NoCache',
          blocking: true,
        })
        .then(requireData)
    ).effectiveProviderTemplate?.node ?? null
  )
}
export async function getModel(providerTemplateId: string, id: string, signal: AbortSignal, activity?: RequestActivity) {
  return (
    (
      await new ProviderCatalogEffectiveModelTemplateStore()
        .fetch({
          variables: { effectiveModelTemplateProviderTemplateId: providerTemplateId, effectiveModelTemplateId: id },
          fetch: graphqlClient.fetch(signal),
          metadata: { requestActivity: activity },
          policy: 'NoCache',
          blocking: true,
        })
        .then(requireData)
    ).effectiveModelTemplate?.node ?? null
  )
}
export type ProviderPage = Awaited<ReturnType<typeof listProviders>>
export type ProviderDetail = NonNullable<Awaited<ReturnType<typeof getProvider>>>
export type ModelDetail = NonNullable<Awaited<ReturnType<typeof getModel>>>

export async function getGlobalModel(providerTemplateId: string, id: string, signal: AbortSignal, activity?: RequestActivity) {
  return (
    (
      await new ProviderCatalogModelTemplateStore()
        .fetch({
          variables: { modelTemplateProviderTemplateId: providerTemplateId, modelTemplateId: id },
          fetch: graphqlClient.fetch(signal),
          metadata: { requestActivity: activity },
          policy: 'NoCache',
          blocking: true,
        })
        .then(requireData)
    ).modelTemplate?.node ?? null
  )
}
