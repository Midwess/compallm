import artifact from '$houdini/artifacts/SettingsSessionRevokeOthers.js'
import type { SettingsSessionRevokeOthers$result, SettingsSessionRevokeOthers$input, SettingsSessionRevokeOthers$optimistic } from '$houdini/artifacts/SettingsSessionRevokeOthers.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsSessionRevokeOthersStore extends MutationStore<SettingsSessionRevokeOthers$result, SettingsSessionRevokeOthers$input, SettingsSessionRevokeOthers$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
