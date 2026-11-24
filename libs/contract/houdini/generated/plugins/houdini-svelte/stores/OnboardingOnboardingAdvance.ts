import artifact from '$houdini/artifacts/OnboardingOnboardingAdvance.js'
import type { OnboardingOnboardingAdvance$result, OnboardingOnboardingAdvance$input, OnboardingOnboardingAdvance$optimistic } from '$houdini/artifacts/OnboardingOnboardingAdvance.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class OnboardingOnboardingAdvanceStore extends MutationStore<OnboardingOnboardingAdvance$result, OnboardingOnboardingAdvance$input, OnboardingOnboardingAdvance$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
