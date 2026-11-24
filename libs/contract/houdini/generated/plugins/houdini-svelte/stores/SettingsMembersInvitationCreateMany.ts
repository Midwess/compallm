import artifact from '$houdini/artifacts/SettingsMembersInvitationCreateMany.js'
import type { SettingsMembersInvitationCreateMany$result, SettingsMembersInvitationCreateMany$input, SettingsMembersInvitationCreateMany$optimistic } from '$houdini/artifacts/SettingsMembersInvitationCreateMany.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsMembersInvitationCreateManyStore extends MutationStore<SettingsMembersInvitationCreateMany$result, SettingsMembersInvitationCreateMany$input, SettingsMembersInvitationCreateMany$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
