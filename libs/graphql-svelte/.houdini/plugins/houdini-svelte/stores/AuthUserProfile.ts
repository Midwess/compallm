import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/AuthUserProfile.js'
import type { AuthUserProfile$result, AuthUserProfile$input } from '$houdini/artifacts/AuthUserProfile.js'

export class AuthUserProfileStore extends QueryStore<AuthUserProfile$result, AuthUserProfile$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "AuthUserProfileStore",
            variables: true,
        })
    }
}

export async function load_AuthUserProfile(params: QueryStoreFetchParams<AuthUserProfile$result, AuthUserProfile$input>): Promise<{AuthUserProfile: AuthUserProfileStore}>{
    const store = new AuthUserProfileStore()
    await store.fetch(params)
    return { AuthUserProfile: store }
}
