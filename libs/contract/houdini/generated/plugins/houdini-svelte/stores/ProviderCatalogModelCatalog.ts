import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/ProviderCatalogModelCatalog.js'
import type { ProviderCatalogModelCatalog$result, ProviderCatalogModelCatalog$input } from '$houdini/artifacts/ProviderCatalogModelCatalog.js'

export class ProviderCatalogModelCatalogStore extends QueryStore<ProviderCatalogModelCatalog$result, ProviderCatalogModelCatalog$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "ProviderCatalogModelCatalogStore",
            variables: true,
        })
    }
}

export async function load_ProviderCatalogModelCatalog(params: QueryStoreFetchParams<ProviderCatalogModelCatalog$result, ProviderCatalogModelCatalog$input>): Promise<{ProviderCatalogModelCatalog: ProviderCatalogModelCatalogStore}>{
    const store = new ProviderCatalogModelCatalogStore()
    await store.fetch(params)
    return { ProviderCatalogModelCatalog: store }
}
