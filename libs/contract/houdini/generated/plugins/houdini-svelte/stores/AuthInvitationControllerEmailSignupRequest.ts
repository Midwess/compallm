import artifact from '$houdini/artifacts/AuthInvitationControllerEmailSignupRequest.js'
import type { AuthInvitationControllerEmailSignupRequest$result, AuthInvitationControllerEmailSignupRequest$input, AuthInvitationControllerEmailSignupRequest$optimistic } from '$houdini/artifacts/AuthInvitationControllerEmailSignupRequest.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class AuthInvitationControllerEmailSignupRequestStore extends MutationStore<AuthInvitationControllerEmailSignupRequest$result, AuthInvitationControllerEmailSignupRequest$input, AuthInvitationControllerEmailSignupRequest$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
