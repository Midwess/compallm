import artifact from '$houdini/artifacts/AuthEmailSigninRequest.js'
import type { AuthEmailSigninRequest$result, AuthEmailSigninRequest$input, AuthEmailSigninRequest$optimistic } from '$houdini/artifacts/AuthEmailSigninRequest.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class AuthEmailSigninRequestStore extends MutationStore<AuthEmailSigninRequest$result, AuthEmailSigninRequest$input, AuthEmailSigninRequest$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
