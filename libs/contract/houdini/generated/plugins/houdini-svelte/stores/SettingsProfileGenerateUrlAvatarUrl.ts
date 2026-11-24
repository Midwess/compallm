import artifact from '$houdini/artifacts/SettingsProfileGenerateUrlAvatarUrl.js'
import type { SettingsProfileGenerateUrlAvatarUrl$result, SettingsProfileGenerateUrlAvatarUrl$input, SettingsProfileGenerateUrlAvatarUrl$optimistic } from '$houdini/artifacts/SettingsProfileGenerateUrlAvatarUrl.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsProfileGenerateUrlAvatarUrlStore extends MutationStore<SettingsProfileGenerateUrlAvatarUrl$result, SettingsProfileGenerateUrlAvatarUrl$input, SettingsProfileGenerateUrlAvatarUrl$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
