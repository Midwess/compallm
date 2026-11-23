import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/AuthSession2.js'
import type { AuthSession2$result, AuthSession2$input } from '$houdini/artifacts/AuthSession2.js'

export class AuthSession2Store extends QueryStore<AuthSession2$result, AuthSession2$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "AuthSession2Store",
            variables: false,
        })
    }
}

export async function load_AuthSession2(params: QueryStoreFetchParams<AuthSession2$result, AuthSession2$input>): Promise<{AuthSession2: AuthSession2Store}>{
    const store = new AuthSession2Store()
    await store.fetch(params)
    return { AuthSession2: store }
}
