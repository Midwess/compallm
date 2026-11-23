import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/PermissionMe.js'
import type { PermissionMe$result, PermissionMe$input } from '$houdini/artifacts/PermissionMe.js'

export class PermissionMeStore extends QueryStore<PermissionMe$result, PermissionMe$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "PermissionMeStore",
            variables: false,
        })
    }
}

export async function load_PermissionMe(params: QueryStoreFetchParams<PermissionMe$result, PermissionMe$input>): Promise<{PermissionMe: PermissionMeStore}>{
    const store = new PermissionMeStore()
    await store.fetch(params)
    return { PermissionMe: store }
}
