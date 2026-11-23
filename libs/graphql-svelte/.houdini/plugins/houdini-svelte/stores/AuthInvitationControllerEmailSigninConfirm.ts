import artifact from '$houdini/artifacts/AuthInvitationControllerEmailSigninConfirm.js'
import type { AuthInvitationControllerEmailSigninConfirm$result, AuthInvitationControllerEmailSigninConfirm$input, AuthInvitationControllerEmailSigninConfirm$optimistic } from '$houdini/artifacts/AuthInvitationControllerEmailSigninConfirm.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class AuthInvitationControllerEmailSigninConfirmStore extends MutationStore<AuthInvitationControllerEmailSigninConfirm$result, AuthInvitationControllerEmailSigninConfirm$input, AuthInvitationControllerEmailSigninConfirm$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
