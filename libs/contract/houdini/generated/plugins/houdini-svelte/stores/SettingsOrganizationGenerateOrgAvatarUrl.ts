import artifact from '$houdini/artifacts/SettingsOrganizationGenerateOrgAvatarUrl.js'
import type { SettingsOrganizationGenerateOrgAvatarUrl$result, SettingsOrganizationGenerateOrgAvatarUrl$input, SettingsOrganizationGenerateOrgAvatarUrl$optimistic } from '$houdini/artifacts/SettingsOrganizationGenerateOrgAvatarUrl.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsOrganizationGenerateOrgAvatarUrlStore extends MutationStore<SettingsOrganizationGenerateOrgAvatarUrl$result, SettingsOrganizationGenerateOrgAvatarUrl$input, SettingsOrganizationGenerateOrgAvatarUrl$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
