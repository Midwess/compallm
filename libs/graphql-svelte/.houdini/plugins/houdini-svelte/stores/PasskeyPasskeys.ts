import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/PasskeyPasskeys.js'
import type { PasskeyPasskeys$result, PasskeyPasskeys$input } from '$houdini/artifacts/PasskeyPasskeys.js'

export class PasskeyPasskeysStore extends QueryStore<PasskeyPasskeys$result, PasskeyPasskeys$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "PasskeyPasskeysStore",
            variables: false,
        })
    }
}

export async function load_PasskeyPasskeys(params: QueryStoreFetchParams<PasskeyPasskeys$result, PasskeyPasskeys$input>): Promise<{PasskeyPasskeys: PasskeyPasskeysStore}>{
    const store = new PasskeyPasskeysStore()
    await store.fetch(params)
    return { PasskeyPasskeys: store }
}
