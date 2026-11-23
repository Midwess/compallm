import artifact from '$houdini/artifacts/AuthEmailSignupRequest.js'
import type { AuthEmailSignupRequest$result, AuthEmailSignupRequest$input, AuthEmailSignupRequest$optimistic } from '$houdini/artifacts/AuthEmailSignupRequest.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class AuthEmailSignupRequestStore extends MutationStore<AuthEmailSignupRequest$result, AuthEmailSignupRequest$input, AuthEmailSignupRequest$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
