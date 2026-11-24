import artifact from '$houdini/artifacts/SettingsMembersGroupUpdate.js'
import type { SettingsMembersGroupUpdate$result, SettingsMembersGroupUpdate$input, SettingsMembersGroupUpdate$optimistic } from '$houdini/artifacts/SettingsMembersGroupUpdate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsMembersGroupUpdateStore extends MutationStore<SettingsMembersGroupUpdate$result, SettingsMembersGroupUpdate$input, SettingsMembersGroupUpdate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
