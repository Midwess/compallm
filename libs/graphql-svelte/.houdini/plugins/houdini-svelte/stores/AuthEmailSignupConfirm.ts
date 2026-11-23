import artifact from '$houdini/artifacts/AuthEmailSignupConfirm.js'
import type { AuthEmailSignupConfirm$result, AuthEmailSignupConfirm$input, AuthEmailSignupConfirm$optimistic } from '$houdini/artifacts/AuthEmailSignupConfirm.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class AuthEmailSignupConfirmStore extends MutationStore<AuthEmailSignupConfirm$result, AuthEmailSignupConfirm$input, AuthEmailSignupConfirm$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
