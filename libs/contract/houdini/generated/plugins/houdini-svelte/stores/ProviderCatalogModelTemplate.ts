import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/ProviderCatalogModelTemplate.js'
import type { ProviderCatalogModelTemplate$result, ProviderCatalogModelTemplate$input } from '$houdini/artifacts/ProviderCatalogModelTemplate.js'

export class ProviderCatalogModelTemplateStore extends QueryStore<ProviderCatalogModelTemplate$result, ProviderCatalogModelTemplate$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "ProviderCatalogModelTemplateStore",
            variables: true,
        })
    }
}

export async function load_ProviderCatalogModelTemplate(params: QueryStoreFetchParams<ProviderCatalogModelTemplate$result, ProviderCatalogModelTemplate$input>): Promise<{ProviderCatalogModelTemplate: ProviderCatalogModelTemplateStore}>{
    const store = new ProviderCatalogModelTemplateStore()
    await store.fetch(params)
    return { ProviderCatalogModelTemplate: store }
}
