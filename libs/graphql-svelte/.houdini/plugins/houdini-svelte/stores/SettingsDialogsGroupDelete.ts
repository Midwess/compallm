import artifact from '$houdini/artifacts/SettingsDialogsGroupDelete.js'
import type { SettingsDialogsGroupDelete$result, SettingsDialogsGroupDelete$input, SettingsDialogsGroupDelete$optimistic } from '$houdini/artifacts/SettingsDialogsGroupDelete.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsDialogsGroupDeleteStore extends MutationStore<SettingsDialogsGroupDelete$result, SettingsDialogsGroupDelete$input, SettingsDialogsGroupDelete$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
