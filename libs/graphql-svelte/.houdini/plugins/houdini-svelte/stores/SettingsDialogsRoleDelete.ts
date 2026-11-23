import artifact from '$houdini/artifacts/SettingsDialogsRoleDelete.js'
import type { SettingsDialogsRoleDelete$result, SettingsDialogsRoleDelete$input, SettingsDialogsRoleDelete$optimistic } from '$houdini/artifacts/SettingsDialogsRoleDelete.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsDialogsRoleDeleteStore extends MutationStore<SettingsDialogsRoleDelete$result, SettingsDialogsRoleDelete$input, SettingsDialogsRoleDelete$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
