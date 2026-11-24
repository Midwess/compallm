import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/ProviderCatalogEffectiveModelCatalog.js'
import type { ProviderCatalogEffectiveModelCatalog$result, ProviderCatalogEffectiveModelCatalog$input } from '$houdini/artifacts/ProviderCatalogEffectiveModelCatalog.js'

export class ProviderCatalogEffectiveModelCatalogStore extends QueryStore<ProviderCatalogEffectiveModelCatalog$result, ProviderCatalogEffectiveModelCatalog$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "ProviderCatalogEffectiveModelCatalogStore",
            variables: true,
        })
    }
}

export async function load_ProviderCatalogEffectiveModelCatalog(params: QueryStoreFetchParams<ProviderCatalogEffectiveModelCatalog$result, ProviderCatalogEffectiveModelCatalog$input>): Promise<{ProviderCatalogEffectiveModelCatalog: ProviderCatalogEffectiveModelCatalogStore}>{
    const store = new ProviderCatalogEffectiveModelCatalogStore()
    await store.fetch(params)
    return { ProviderCatalogEffectiveModelCatalog: store }
}
