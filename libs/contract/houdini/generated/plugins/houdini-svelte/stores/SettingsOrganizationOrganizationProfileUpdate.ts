import artifact from '$houdini/artifacts/SettingsOrganizationOrganizationProfileUpdate.js'
import type { SettingsOrganizationOrganizationProfileUpdate$result, SettingsOrganizationOrganizationProfileUpdate$input, SettingsOrganizationOrganizationProfileUpdate$optimistic } from '$houdini/artifacts/SettingsOrganizationOrganizationProfileUpdate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsOrganizationOrganizationProfileUpdateStore extends MutationStore<SettingsOrganizationOrganizationProfileUpdate$result, SettingsOrganizationOrganizationProfileUpdate$input, SettingsOrganizationOrganizationProfileUpdate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
