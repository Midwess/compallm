import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/PreferenceUserUserPreferences.js'
import type { PreferenceUserUserPreferences$result, PreferenceUserUserPreferences$input } from '$houdini/artifacts/PreferenceUserUserPreferences.js'

export class PreferenceUserUserPreferencesStore extends QueryStore<PreferenceUserUserPreferences$result, PreferenceUserUserPreferences$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "PreferenceUserUserPreferencesStore",
            variables: false,
        })
    }
}

export async function load_PreferenceUserUserPreferences(params: QueryStoreFetchParams<PreferenceUserUserPreferences$result, PreferenceUserUserPreferences$input>): Promise<{PreferenceUserUserPreferences: PreferenceUserUserPreferencesStore}>{
    const store = new PreferenceUserUserPreferencesStore()
    await store.fetch(params)
    return { PreferenceUserUserPreferences: store }
}
