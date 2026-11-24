import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/AuthInvitationControllerInvitationContext.js'
import type { AuthInvitationControllerInvitationContext$result, AuthInvitationControllerInvitationContext$input } from '$houdini/artifacts/AuthInvitationControllerInvitationContext.js'

export class AuthInvitationControllerInvitationContextStore extends QueryStore<AuthInvitationControllerInvitationContext$result, AuthInvitationControllerInvitationContext$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "AuthInvitationControllerInvitationContextStore",
            variables: false,
        })
    }
}

export async function load_AuthInvitationControllerInvitationContext(params: QueryStoreFetchParams<AuthInvitationControllerInvitationContext$result, AuthInvitationControllerInvitationContext$input>): Promise<{AuthInvitationControllerInvitationContext: AuthInvitationControllerInvitationContextStore}>{
    const store = new AuthInvitationControllerInvitationContextStore()
    await store.fetch(params)
    return { AuthInvitationControllerInvitationContext: store }
}
