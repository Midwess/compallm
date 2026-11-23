import artifact from '$houdini/artifacts/AuthEmailSigninConfirm2.js'
import type { AuthEmailSigninConfirm2$result, AuthEmailSigninConfirm2$input, AuthEmailSigninConfirm2$optimistic } from '$houdini/artifacts/AuthEmailSigninConfirm2.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class AuthEmailSigninConfirm2Store extends MutationStore<AuthEmailSigninConfirm2$result, AuthEmailSigninConfirm2$input, AuthEmailSigninConfirm2$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
