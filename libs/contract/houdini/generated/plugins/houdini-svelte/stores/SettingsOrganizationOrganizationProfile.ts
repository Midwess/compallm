import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/SettingsOrganizationOrganizationProfile.js'
import type { SettingsOrganizationOrganizationProfile$result, SettingsOrganizationOrganizationProfile$input } from '$houdini/artifacts/SettingsOrganizationOrganizationProfile.js'

export class SettingsOrganizationOrganizationProfileStore extends QueryStore<SettingsOrganizationOrganizationProfile$result, SettingsOrganizationOrganizationProfile$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "SettingsOrganizationOrganizationProfileStore",
            variables: true,
        })
    }
}

export async function load_SettingsOrganizationOrganizationProfile(params: QueryStoreFetchParams<SettingsOrganizationOrganizationProfile$result, SettingsOrganizationOrganizationProfile$input>): Promise<{SettingsOrganizationOrganizationProfile: SettingsOrganizationOrganizationProfileStore}>{
    const store = new SettingsOrganizationOrganizationProfileStore()
    await store.fetch(params)
    return { SettingsOrganizationOrganizationProfile: store }
}
