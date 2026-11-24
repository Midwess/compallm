import artifact from '$houdini/artifacts/AuthPasskeyAuthenticationFinish2.js'
import type { AuthPasskeyAuthenticationFinish2$result, AuthPasskeyAuthenticationFinish2$input, AuthPasskeyAuthenticationFinish2$optimistic } from '$houdini/artifacts/AuthPasskeyAuthenticationFinish2.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class AuthPasskeyAuthenticationFinish2Store extends MutationStore<AuthPasskeyAuthenticationFinish2$result, AuthPasskeyAuthenticationFinish2$input, AuthPasskeyAuthenticationFinish2$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
