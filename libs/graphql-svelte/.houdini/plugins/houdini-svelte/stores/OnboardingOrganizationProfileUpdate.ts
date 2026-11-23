import artifact from '$houdini/artifacts/OnboardingOrganizationProfileUpdate.js'
import type { OnboardingOrganizationProfileUpdate$result, OnboardingOrganizationProfileUpdate$input, OnboardingOrganizationProfileUpdate$optimistic } from '$houdini/artifacts/OnboardingOrganizationProfileUpdate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class OnboardingOrganizationProfileUpdateStore extends MutationStore<OnboardingOrganizationProfileUpdate$result, OnboardingOrganizationProfileUpdate$input, OnboardingOrganizationProfileUpdate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
