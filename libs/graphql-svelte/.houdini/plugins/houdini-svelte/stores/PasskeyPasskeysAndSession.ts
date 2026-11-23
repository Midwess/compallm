import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/PasskeyPasskeysAndSession.js'
import type { PasskeyPasskeysAndSession$result, PasskeyPasskeysAndSession$input } from '$houdini/artifacts/PasskeyPasskeysAndSession.js'

export class PasskeyPasskeysAndSessionStore extends QueryStore<PasskeyPasskeysAndSession$result, PasskeyPasskeysAndSession$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "PasskeyPasskeysAndSessionStore",
            variables: false,
        })
    }
}

export async function load_PasskeyPasskeysAndSession(params: QueryStoreFetchParams<PasskeyPasskeysAndSession$result, PasskeyPasskeysAndSession$input>): Promise<{PasskeyPasskeysAndSession: PasskeyPasskeysAndSessionStore}>{
    const store = new PasskeyPasskeysAndSessionStore()
    await store.fetch(params)
    return { PasskeyPasskeysAndSession: store }
}
