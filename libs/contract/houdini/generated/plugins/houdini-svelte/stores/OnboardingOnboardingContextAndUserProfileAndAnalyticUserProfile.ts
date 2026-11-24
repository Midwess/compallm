import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile.js'
import type { OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$result, OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$input } from '$houdini/artifacts/OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile.js'

export class OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfileStore extends QueryStore<OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$result, OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfileStore",
            variables: true,
        })
    }
}

export async function load_OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile(params: QueryStoreFetchParams<OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$result, OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$input>): Promise<{OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile: OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfileStore}>{
    const store = new OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfileStore()
    await store.fetch(params)
    return { OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile: store }
}
