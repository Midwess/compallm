import artifact from '$houdini/artifacts/OnboardingAnalyticUserProfilePut.js'
import type { OnboardingAnalyticUserProfilePut$result, OnboardingAnalyticUserProfilePut$input, OnboardingAnalyticUserProfilePut$optimistic } from '$houdini/artifacts/OnboardingAnalyticUserProfilePut.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class OnboardingAnalyticUserProfilePutStore extends MutationStore<OnboardingAnalyticUserProfilePut$result, OnboardingAnalyticUserProfilePut$input, OnboardingAnalyticUserProfilePut$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
