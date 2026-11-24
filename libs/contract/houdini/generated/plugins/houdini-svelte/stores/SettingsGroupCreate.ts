import artifact from '$houdini/artifacts/SettingsGroupCreate.js'
import type { SettingsGroupCreate$result, SettingsGroupCreate$input, SettingsGroupCreate$optimistic } from '$houdini/artifacts/SettingsGroupCreate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsGroupCreateStore extends MutationStore<SettingsGroupCreate$result, SettingsGroupCreate$input, SettingsGroupCreate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
