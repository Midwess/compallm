import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/AuthSession.js'
import type { AuthSession$result, AuthSession$input } from '$houdini/artifacts/AuthSession.js'

export class AuthSessionStore extends QueryStore<AuthSession$result, AuthSession$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "AuthSessionStore",
            variables: false,
        })
    }
}

export async function load_AuthSession(params: QueryStoreFetchParams<AuthSession$result, AuthSession$input>): Promise<{AuthSession: AuthSessionStore}>{
    const store = new AuthSessionStore()
    await store.fetch(params)
    return { AuthSession: store }
}
