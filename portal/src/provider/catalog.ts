import {
  ProviderCatalogEffectiveProviderCatalogsStore,
  ProviderCatalogEffectiveProviderCatalogStore,
  ProviderCatalogEffectiveModelCatalogStore,
  ProviderCatalogModelCatalogStore,
} from '@compallm/contract/houdini'

import { graphqlClient, requireData, type RequestActivity } from '../graphql-client'

export async function listProviders(query: string, after: string | undefined, signal: AbortSignal, activity?: RequestActivity) {
  return (
    await new ProviderCatalogEffectiveProviderCatalogsStore()
      .fetch({
        variables: { effectiveProviderCatalogsAfter: after, effectiveProviderCatalogsSearch: query || undefined },
        fetch: graphqlClient.fetch(signal),
        metadata: { requestActivity: activity },
        policy: 'NoCache',
        blocking: true,
      })
      .then(requireData)
  ).effectiveProviderCatalogs
}
export async function getProvider(id: string, signal: AbortSignal, activity?: RequestActivity) {
  return (
    (
      await new ProviderCatalogEffectiveProviderCatalogStore()
        .fetch({
          variables: { effectiveProviderCatalogId: id },
          fetch: graphqlClient.fetch(signal),
          metadata: { requestActivity: activity },
          policy: 'NoCache',
          blocking: true,
        })
        .then(requireData)
    ).effectiveProviderCatalog?.node ?? null
  )
}
export async function getModel(providerCatalogId: string, id: string, signal: AbortSignal, activity?: RequestActivity) {
  return (
    (
      await new ProviderCatalogEffectiveModelCatalogStore()
        .fetch({
          variables: { effectiveModelCatalogProviderCatalogId: providerCatalogId, effectiveModelCatalogId: id },
          fetch: graphqlClient.fetch(signal),
          metadata: { requestActivity: activity },
          policy: 'NoCache',
          blocking: true,
        })
        .then(requireData)
    ).effectiveModelCatalog?.node ?? null
  )
}
export type ProviderPage = Awaited<ReturnType<typeof listProviders>>
export type ProviderDetail = NonNullable<Awaited<ReturnType<typeof getProvider>>>
export type ModelDetail = NonNullable<Awaited<ReturnType<typeof getModel>>>

export async function getGlobalModel(providerCatalogId: string, id: string, signal: AbortSignal, activity?: RequestActivity) {
  return (
    (
      await new ProviderCatalogModelCatalogStore()
        .fetch({
          variables: { modelCatalogProviderCatalogId: providerCatalogId, modelCatalogId: id },
          fetch: graphqlClient.fetch(signal),
          metadata: { requestActivity: activity },
          policy: 'NoCache',
          blocking: true,
        })
        .then(requireData)
    ).modelCatalog?.node ?? null
  )
}
