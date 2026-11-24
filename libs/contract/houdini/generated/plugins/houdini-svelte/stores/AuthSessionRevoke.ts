import artifact from '$houdini/artifacts/AuthSessionRevoke.js'
import type { AuthSessionRevoke$result, AuthSessionRevoke$input, AuthSessionRevoke$optimistic } from '$houdini/artifacts/AuthSessionRevoke.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class AuthSessionRevokeStore extends MutationStore<AuthSessionRevoke$result, AuthSessionRevoke$input, AuthSessionRevoke$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
