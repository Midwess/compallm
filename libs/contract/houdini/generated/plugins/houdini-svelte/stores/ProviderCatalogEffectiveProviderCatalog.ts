import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/ProviderCatalogEffectiveProviderCatalog.js'
import type { ProviderCatalogEffectiveProviderCatalog$result, ProviderCatalogEffectiveProviderCatalog$input } from '$houdini/artifacts/ProviderCatalogEffectiveProviderCatalog.js'

export class ProviderCatalogEffectiveProviderCatalogStore extends QueryStore<ProviderCatalogEffectiveProviderCatalog$result, ProviderCatalogEffectiveProviderCatalog$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "ProviderCatalogEffectiveProviderCatalogStore",
            variables: true,
        })
    }
}

export async function load_ProviderCatalogEffectiveProviderCatalog(params: QueryStoreFetchParams<ProviderCatalogEffectiveProviderCatalog$result, ProviderCatalogEffectiveProviderCatalog$input>): Promise<{ProviderCatalogEffectiveProviderCatalog: ProviderCatalogEffectiveProviderCatalogStore}>{
    const store = new ProviderCatalogEffectiveProviderCatalogStore()
    await store.fetch(params)
    return { ProviderCatalogEffectiveProviderCatalog: store }
}
