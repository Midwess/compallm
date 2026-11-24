import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/ProviderCatalogEffectiveProviderTemplates.js'
import type { ProviderCatalogEffectiveProviderTemplates$result, ProviderCatalogEffectiveProviderTemplates$input } from '$houdini/artifacts/ProviderCatalogEffectiveProviderTemplates.js'

export class ProviderCatalogEffectiveProviderTemplatesStore extends QueryStore<ProviderCatalogEffectiveProviderTemplates$result, ProviderCatalogEffectiveProviderTemplates$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "ProviderCatalogEffectiveProviderTemplatesStore",
            variables: false,
        })
    }
}

export async function load_ProviderCatalogEffectiveProviderTemplates(params: QueryStoreFetchParams<ProviderCatalogEffectiveProviderTemplates$result, ProviderCatalogEffectiveProviderTemplates$input>): Promise<{ProviderCatalogEffectiveProviderTemplates: ProviderCatalogEffectiveProviderTemplatesStore}>{
    const store = new ProviderCatalogEffectiveProviderTemplatesStore()
    await store.fetch(params)
    return { ProviderCatalogEffectiveProviderTemplates: store }
}
