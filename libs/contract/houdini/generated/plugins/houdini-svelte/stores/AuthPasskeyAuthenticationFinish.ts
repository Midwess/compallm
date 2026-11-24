import artifact from '$houdini/artifacts/AuthPasskeyAuthenticationFinish.js'
import type { AuthPasskeyAuthenticationFinish$result, AuthPasskeyAuthenticationFinish$input, AuthPasskeyAuthenticationFinish$optimistic } from '$houdini/artifacts/AuthPasskeyAuthenticationFinish.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class AuthPasskeyAuthenticationFinishStore extends MutationStore<AuthPasskeyAuthenticationFinish$result, AuthPasskeyAuthenticationFinish$input, AuthPasskeyAuthenticationFinish$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
