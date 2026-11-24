import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/ProviderCatalogEffectiveProviderCatalogs.js'
import type { ProviderCatalogEffectiveProviderCatalogs$result, ProviderCatalogEffectiveProviderCatalogs$input } from '$houdini/artifacts/ProviderCatalogEffectiveProviderCatalogs.js'

export class ProviderCatalogEffectiveProviderCatalogsStore extends QueryStore<ProviderCatalogEffectiveProviderCatalogs$result, ProviderCatalogEffectiveProviderCatalogs$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "ProviderCatalogEffectiveProviderCatalogsStore",
            variables: false,
        })
    }
}

export async function load_ProviderCatalogEffectiveProviderCatalogs(params: QueryStoreFetchParams<ProviderCatalogEffectiveProviderCatalogs$result, ProviderCatalogEffectiveProviderCatalogs$input>): Promise<{ProviderCatalogEffectiveProviderCatalogs: ProviderCatalogEffectiveProviderCatalogsStore}>{
    const store = new ProviderCatalogEffectiveProviderCatalogsStore()
    await store.fetch(params)
    return { ProviderCatalogEffectiveProviderCatalogs: store }
}
