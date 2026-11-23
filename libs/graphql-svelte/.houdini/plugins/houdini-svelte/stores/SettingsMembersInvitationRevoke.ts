import artifact from '$houdini/artifacts/SettingsMembersInvitationRevoke.js'
import type { SettingsMembersInvitationRevoke$result, SettingsMembersInvitationRevoke$input, SettingsMembersInvitationRevoke$optimistic } from '$houdini/artifacts/SettingsMembersInvitationRevoke.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsMembersInvitationRevokeStore extends MutationStore<SettingsMembersInvitationRevoke$result, SettingsMembersInvitationRevoke$input, SettingsMembersInvitationRevoke$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
