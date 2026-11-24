import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/SettingsOnboardingOrganizationOnboardingSettings.js'
import type { SettingsOnboardingOrganizationOnboardingSettings$result, SettingsOnboardingOrganizationOnboardingSettings$input } from '$houdini/artifacts/SettingsOnboardingOrganizationOnboardingSettings.js'

export class SettingsOnboardingOrganizationOnboardingSettingsStore extends QueryStore<SettingsOnboardingOrganizationOnboardingSettings$result, SettingsOnboardingOrganizationOnboardingSettings$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "SettingsOnboardingOrganizationOnboardingSettingsStore",
            variables: false,
        })
    }
}

export async function load_SettingsOnboardingOrganizationOnboardingSettings(params: QueryStoreFetchParams<SettingsOnboardingOrganizationOnboardingSettings$result, SettingsOnboardingOrganizationOnboardingSettings$input>): Promise<{SettingsOnboardingOrganizationOnboardingSettings: SettingsOnboardingOrganizationOnboardingSettingsStore}>{
    const store = new SettingsOnboardingOrganizationOnboardingSettingsStore()
    await store.fetch(params)
    return { SettingsOnboardingOrganizationOnboardingSettings: store }
}
