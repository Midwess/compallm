import artifact from '$houdini/artifacts/SettingsMembersInvitationCreate.js'
import type { SettingsMembersInvitationCreate$result, SettingsMembersInvitationCreate$input, SettingsMembersInvitationCreate$optimistic } from '$houdini/artifacts/SettingsMembersInvitationCreate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsMembersInvitationCreateStore extends MutationStore<SettingsMembersInvitationCreate$result, SettingsMembersInvitationCreate$input, SettingsMembersInvitationCreate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
