import artifact from '$houdini/artifacts/SettingsProfileUserProfileUpdate.js'
import type { SettingsProfileUserProfileUpdate$result, SettingsProfileUserProfileUpdate$input, SettingsProfileUserProfileUpdate$optimistic } from '$houdini/artifacts/SettingsProfileUserProfileUpdate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsProfileUserProfileUpdateStore extends MutationStore<SettingsProfileUserProfileUpdate$result, SettingsProfileUserProfileUpdate$input, SettingsProfileUserProfileUpdate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
