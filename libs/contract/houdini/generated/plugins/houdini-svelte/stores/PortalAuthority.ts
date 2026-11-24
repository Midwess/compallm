import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/PortalAuthority.js'
import type { PortalAuthority$result, PortalAuthority$input } from '$houdini/artifacts/PortalAuthority.js'

export class PortalAuthorityStore extends QueryStore<PortalAuthority$result, PortalAuthority$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "PortalAuthorityStore",
            variables: false,
        })
    }
}

export async function load_PortalAuthority(params: QueryStoreFetchParams<PortalAuthority$result, PortalAuthority$input>): Promise<{PortalAuthority: PortalAuthorityStore}>{
    const store = new PortalAuthorityStore()
    await store.fetch(params)
    return { PortalAuthority: store }
}
