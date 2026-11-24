import artifact from '$houdini/artifacts/SettingsRoleRoleUpdate.js'
import type { SettingsRoleRoleUpdate$result, SettingsRoleRoleUpdate$input, SettingsRoleRoleUpdate$optimistic } from '$houdini/artifacts/SettingsRoleRoleUpdate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsRoleRoleUpdateStore extends MutationStore<SettingsRoleRoleUpdate$result, SettingsRoleRoleUpdate$input, SettingsRoleRoleUpdate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
