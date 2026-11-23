import artifact from '$houdini/artifacts/OnboardingOnboardingPresent.js'
import type { OnboardingOnboardingPresent$result, OnboardingOnboardingPresent$input, OnboardingOnboardingPresent$optimistic } from '$houdini/artifacts/OnboardingOnboardingPresent.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class OnboardingOnboardingPresentStore extends MutationStore<OnboardingOnboardingPresent$result, OnboardingOnboardingPresent$input, OnboardingOnboardingPresent$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
