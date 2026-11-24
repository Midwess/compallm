import artifact from '$houdini/artifacts/AuthEmailSigninConfirm.js'
import type { AuthEmailSigninConfirm$result, AuthEmailSigninConfirm$input, AuthEmailSigninConfirm$optimistic } from '$houdini/artifacts/AuthEmailSigninConfirm.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class AuthEmailSigninConfirmStore extends MutationStore<AuthEmailSigninConfirm$result, AuthEmailSigninConfirm$input, AuthEmailSigninConfirm$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
