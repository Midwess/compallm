import artifact from '$houdini/artifacts/SettingsSessionRevokeOther.js'
import type { SettingsSessionRevokeOther$result, SettingsSessionRevokeOther$input, SettingsSessionRevokeOther$optimistic } from '$houdini/artifacts/SettingsSessionRevokeOther.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsSessionRevokeOtherStore extends MutationStore<SettingsSessionRevokeOther$result, SettingsSessionRevokeOther$input, SettingsSessionRevokeOther$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
