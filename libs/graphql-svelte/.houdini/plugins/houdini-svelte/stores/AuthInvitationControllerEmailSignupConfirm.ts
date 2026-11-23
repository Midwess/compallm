import artifact from '$houdini/artifacts/AuthInvitationControllerEmailSignupConfirm.js'
import type { AuthInvitationControllerEmailSignupConfirm$result, AuthInvitationControllerEmailSignupConfirm$input, AuthInvitationControllerEmailSignupConfirm$optimistic } from '$houdini/artifacts/AuthInvitationControllerEmailSignupConfirm.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class AuthInvitationControllerEmailSignupConfirmStore extends MutationStore<AuthInvitationControllerEmailSignupConfirm$result, AuthInvitationControllerEmailSignupConfirm$input, AuthInvitationControllerEmailSignupConfirm$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
