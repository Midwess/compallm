import artifact from '$houdini/artifacts/SettingsRoleCreate.js'
import type { SettingsRoleCreate$result, SettingsRoleCreate$input, SettingsRoleCreate$optimistic } from '$houdini/artifacts/SettingsRoleCreate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsRoleCreateStore extends MutationStore<SettingsRoleCreate$result, SettingsRoleCreate$input, SettingsRoleCreate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
