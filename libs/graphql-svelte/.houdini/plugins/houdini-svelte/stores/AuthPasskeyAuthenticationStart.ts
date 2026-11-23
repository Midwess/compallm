import artifact from '$houdini/artifacts/AuthPasskeyAuthenticationStart.js'
import type { AuthPasskeyAuthenticationStart$result, AuthPasskeyAuthenticationStart$input, AuthPasskeyAuthenticationStart$optimistic } from '$houdini/artifacts/AuthPasskeyAuthenticationStart.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class AuthPasskeyAuthenticationStartStore extends MutationStore<AuthPasskeyAuthenticationStart$result, AuthPasskeyAuthenticationStart$input, AuthPasskeyAuthenticationStart$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
