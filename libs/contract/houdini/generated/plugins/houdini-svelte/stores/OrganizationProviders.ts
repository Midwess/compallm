import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/OrganizationProviders.js'
import type { OrganizationProviders$result, OrganizationProviders$input } from '$houdini/artifacts/OrganizationProviders.js'

export class OrganizationProvidersStore extends QueryStore<OrganizationProviders$result, OrganizationProviders$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "OrganizationProvidersStore",
            variables: false,
        })
    }
}

export async function load_OrganizationProviders(params: QueryStoreFetchParams<OrganizationProviders$result, OrganizationProviders$input>): Promise<{OrganizationProviders: OrganizationProvidersStore}>{
    const store = new OrganizationProvidersStore()
    await store.fetch(params)
    return { OrganizationProviders: store }
}
