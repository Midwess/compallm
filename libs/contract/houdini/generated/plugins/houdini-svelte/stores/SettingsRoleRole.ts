import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/SettingsRoleRole.js'
import type { SettingsRoleRole$result, SettingsRoleRole$input } from '$houdini/artifacts/SettingsRoleRole.js'

export class SettingsRoleRoleStore extends QueryStore<SettingsRoleRole$result, SettingsRoleRole$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "SettingsRoleRoleStore",
            variables: true,
        })
    }
}

export async function load_SettingsRoleRole(params: QueryStoreFetchParams<SettingsRoleRole$result, SettingsRoleRole$input>): Promise<{SettingsRoleRole: SettingsRoleRoleStore}>{
    const store = new SettingsRoleRoleStore()
    await store.fetch(params)
    return { SettingsRoleRole: store }
}
