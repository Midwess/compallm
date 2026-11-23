import artifact from '$houdini/artifacts/SettingsGroupGroupUpdate.js'
import type { SettingsGroupGroupUpdate$result, SettingsGroupGroupUpdate$input, SettingsGroupGroupUpdate$optimistic } from '$houdini/artifacts/SettingsGroupGroupUpdate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsGroupGroupUpdateStore extends MutationStore<SettingsGroupGroupUpdate$result, SettingsGroupGroupUpdate$input, SettingsGroupGroupUpdate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
