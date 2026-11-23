import artifact from '$houdini/artifacts/SettingsOnboardingOrganizationOnboardingSettingsUpdate.js'
import type { SettingsOnboardingOrganizationOnboardingSettingsUpdate$result, SettingsOnboardingOrganizationOnboardingSettingsUpdate$input, SettingsOnboardingOrganizationOnboardingSettingsUpdate$optimistic } from '$houdini/artifacts/SettingsOnboardingOrganizationOnboardingSettingsUpdate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class SettingsOnboardingOrganizationOnboardingSettingsUpdateStore extends MutationStore<SettingsOnboardingOrganizationOnboardingSettingsUpdate$result, SettingsOnboardingOrganizationOnboardingSettingsUpdate$input, SettingsOnboardingOrganizationOnboardingSettingsUpdate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
