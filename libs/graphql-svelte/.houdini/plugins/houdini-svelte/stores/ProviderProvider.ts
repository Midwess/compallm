import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/ProviderProvider.js'
import type { ProviderProvider$result, ProviderProvider$input } from '$houdini/artifacts/ProviderProvider.js'

export class ProviderProviderStore extends QueryStore<ProviderProvider$result, ProviderProvider$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "ProviderProviderStore",
            variables: true,
        })
    }
}

export async function load_ProviderProvider(params: QueryStoreFetchParams<ProviderProvider$result, ProviderProvider$input>): Promise<{ProviderProvider: ProviderProviderStore}>{
    const store = new ProviderProviderStore()
    await store.fetch(params)
    return { ProviderProvider: store }
}
