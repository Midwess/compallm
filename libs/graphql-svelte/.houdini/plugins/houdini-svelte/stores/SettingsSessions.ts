import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/SettingsSessions.js'
import type { SettingsSessions$result, SettingsSessions$input } from '$houdini/artifacts/SettingsSessions.js'

export class SettingsSessionsStore extends QueryStore<SettingsSessions$result, SettingsSessions$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "SettingsSessionsStore",
            variables: false,
        })
    }
}

export async function load_SettingsSessions(params: QueryStoreFetchParams<SettingsSessions$result, SettingsSessions$input>): Promise<{SettingsSessions: SettingsSessionsStore}>{
    const store = new SettingsSessionsStore()
    await store.fetch(params)
    return { SettingsSessions: store }
}
