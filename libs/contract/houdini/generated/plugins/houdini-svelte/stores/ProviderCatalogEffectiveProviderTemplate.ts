import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/ProviderCatalogEffectiveProviderTemplate.js'
import type { ProviderCatalogEffectiveProviderTemplate$result, ProviderCatalogEffectiveProviderTemplate$input } from '$houdini/artifacts/ProviderCatalogEffectiveProviderTemplate.js'

export class ProviderCatalogEffectiveProviderTemplateStore extends QueryStore<ProviderCatalogEffectiveProviderTemplate$result, ProviderCatalogEffectiveProviderTemplate$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "ProviderCatalogEffectiveProviderTemplateStore",
            variables: true,
        })
    }
}

export async function load_ProviderCatalogEffectiveProviderTemplate(params: QueryStoreFetchParams<ProviderCatalogEffectiveProviderTemplate$result, ProviderCatalogEffectiveProviderTemplate$input>): Promise<{ProviderCatalogEffectiveProviderTemplate: ProviderCatalogEffectiveProviderTemplateStore}>{
    const store = new ProviderCatalogEffectiveProviderTemplateStore()
    await store.fetch(params)
    return { ProviderCatalogEffectiveProviderTemplate: store }
}
