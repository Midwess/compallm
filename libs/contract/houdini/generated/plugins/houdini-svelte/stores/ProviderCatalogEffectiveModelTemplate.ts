import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/ProviderCatalogEffectiveModelTemplate.js'
import type { ProviderCatalogEffectiveModelTemplate$result, ProviderCatalogEffectiveModelTemplate$input } from '$houdini/artifacts/ProviderCatalogEffectiveModelTemplate.js'

export class ProviderCatalogEffectiveModelTemplateStore extends QueryStore<ProviderCatalogEffectiveModelTemplate$result, ProviderCatalogEffectiveModelTemplate$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "ProviderCatalogEffectiveModelTemplateStore",
            variables: true,
        })
    }
}

export async function load_ProviderCatalogEffectiveModelTemplate(params: QueryStoreFetchParams<ProviderCatalogEffectiveModelTemplate$result, ProviderCatalogEffectiveModelTemplate$input>): Promise<{ProviderCatalogEffectiveModelTemplate: ProviderCatalogEffectiveModelTemplateStore}>{
    const store = new ProviderCatalogEffectiveModelTemplateStore()
    await store.fetch(params)
    return { ProviderCatalogEffectiveModelTemplate: store }
}
