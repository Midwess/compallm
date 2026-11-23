import artifact from '$houdini/artifacts/OnboardingOnboardingComplete.js'
import type { OnboardingOnboardingComplete$result, OnboardingOnboardingComplete$input, OnboardingOnboardingComplete$optimistic } from '$houdini/artifacts/OnboardingOnboardingComplete.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class OnboardingOnboardingCompleteStore extends MutationStore<OnboardingOnboardingComplete$result, OnboardingOnboardingComplete$input, OnboardingOnboardingComplete$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
