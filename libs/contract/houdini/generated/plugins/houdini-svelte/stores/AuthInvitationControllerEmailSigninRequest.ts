import artifact from '$houdini/artifacts/AuthInvitationControllerEmailSigninRequest.js'
import type { AuthInvitationControllerEmailSigninRequest$result, AuthInvitationControllerEmailSigninRequest$input, AuthInvitationControllerEmailSigninRequest$optimistic } from '$houdini/artifacts/AuthInvitationControllerEmailSigninRequest.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class AuthInvitationControllerEmailSigninRequestStore extends MutationStore<AuthInvitationControllerEmailSigninRequest$result, AuthInvitationControllerEmailSigninRequest$input, AuthInvitationControllerEmailSigninRequest$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
