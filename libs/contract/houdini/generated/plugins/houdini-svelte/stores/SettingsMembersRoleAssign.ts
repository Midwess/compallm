import artifact from '$houdini/artifacts/SettingsMembersRoleAssign.js'
import type { SettingsMembersRoleAssign$result, SettingsMembersRoleAssign$input, SettingsMembersRoleAssign$optimistic } from '$houdini/artifacts/SettingsMembersRoleAssign.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsMembersRoleAssignStore extends MutationStore<SettingsMembersRoleAssign$result, SettingsMembersRoleAssign$input, SettingsMembersRoleAssign$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
