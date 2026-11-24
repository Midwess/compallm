import artifact from '$houdini/artifacts/OnboardingGenerateUrlAvatarUrl.js'
import type { OnboardingGenerateUrlAvatarUrl$result, OnboardingGenerateUrlAvatarUrl$input, OnboardingGenerateUrlAvatarUrl$optimistic } from '$houdini/artifacts/OnboardingGenerateUrlAvatarUrl.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class OnboardingGenerateUrlAvatarUrlStore extends MutationStore<OnboardingGenerateUrlAvatarUrl$result, OnboardingGenerateUrlAvatarUrl$input, OnboardingGenerateUrlAvatarUrl$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
