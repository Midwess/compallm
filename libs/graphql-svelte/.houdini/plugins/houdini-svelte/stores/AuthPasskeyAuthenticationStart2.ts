import artifact from '$houdini/artifacts/AuthPasskeyAuthenticationStart2.js'
import type { AuthPasskeyAuthenticationStart2$result, AuthPasskeyAuthenticationStart2$input, AuthPasskeyAuthenticationStart2$optimistic } from '$houdini/artifacts/AuthPasskeyAuthenticationStart2.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class AuthPasskeyAuthenticationStart2Store extends MutationStore<AuthPasskeyAuthenticationStart2$result, AuthPasskeyAuthenticationStart2$input, AuthPasskeyAuthenticationStart2$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
