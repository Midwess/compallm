import artifact from '$houdini/artifacts/SettingsRoleRoleUpdate2.js'
import type { SettingsRoleRoleUpdate2$result, SettingsRoleRoleUpdate2$input, SettingsRoleRoleUpdate2$optimistic } from '$houdini/artifacts/SettingsRoleRoleUpdate2.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsRoleRoleUpdate2Store extends MutationStore<SettingsRoleRoleUpdate2$result, SettingsRoleRoleUpdate2$input, SettingsRoleRoleUpdate2$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
