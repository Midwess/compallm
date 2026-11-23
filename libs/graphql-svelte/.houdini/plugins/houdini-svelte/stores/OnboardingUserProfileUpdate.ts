import artifact from '$houdini/artifacts/OnboardingUserProfileUpdate.js'
import type { OnboardingUserProfileUpdate$result, OnboardingUserProfileUpdate$input, OnboardingUserProfileUpdate$optimistic } from '$houdini/artifacts/OnboardingUserProfileUpdate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class OnboardingUserProfileUpdateStore extends MutationStore<OnboardingUserProfileUpdate$result, OnboardingUserProfileUpdate$input, OnboardingUserProfileUpdate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
