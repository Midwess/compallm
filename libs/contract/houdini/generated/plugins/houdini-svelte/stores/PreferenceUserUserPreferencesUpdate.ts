import artifact from '$houdini/artifacts/PreferenceUserUserPreferencesUpdate.js'
import type { PreferenceUserUserPreferencesUpdate$result, PreferenceUserUserPreferencesUpdate$input, PreferenceUserUserPreferencesUpdate$optimistic } from '$houdini/artifacts/PreferenceUserUserPreferencesUpdate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class PreferenceUserUserPreferencesUpdateStore extends MutationStore<PreferenceUserUserPreferencesUpdate$result, PreferenceUserUserPreferencesUpdate$input, PreferenceUserUserPreferencesUpdate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
