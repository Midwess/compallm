import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/SettingsProfileUserProfile.js'
import type { SettingsProfileUserProfile$result, SettingsProfileUserProfile$input } from '$houdini/artifacts/SettingsProfileUserProfile.js'

export class SettingsProfileUserProfileStore extends QueryStore<SettingsProfileUserProfile$result, SettingsProfileUserProfile$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "SettingsProfileUserProfileStore",
            variables: true,
        })
    }
}

export async function load_SettingsProfileUserProfile(params: QueryStoreFetchParams<SettingsProfileUserProfile$result, SettingsProfileUserProfile$input>): Promise<{SettingsProfileUserProfile: SettingsProfileUserProfileStore}>{
    const store = new SettingsProfileUserProfileStore()
    await store.fetch(params)
    return { SettingsProfileUserProfile: store }
}
