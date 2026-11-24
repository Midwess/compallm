import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/SettingsGroupGroupAndRolesAndMembers.js'
import type { SettingsGroupGroupAndRolesAndMembers$result, SettingsGroupGroupAndRolesAndMembers$input } from '$houdini/artifacts/SettingsGroupGroupAndRolesAndMembers.js'

export class SettingsGroupGroupAndRolesAndMembersStore extends QueryStore<SettingsGroupGroupAndRolesAndMembers$result, SettingsGroupGroupAndRolesAndMembers$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "SettingsGroupGroupAndRolesAndMembersStore",
            variables: true,
        })
    }
}

export async function load_SettingsGroupGroupAndRolesAndMembers(params: QueryStoreFetchParams<SettingsGroupGroupAndRolesAndMembers$result, SettingsGroupGroupAndRolesAndMembers$input>): Promise<{SettingsGroupGroupAndRolesAndMembers: SettingsGroupGroupAndRolesAndMembersStore}>{
    const store = new SettingsGroupGroupAndRolesAndMembersStore()
    await store.fetch(params)
    return { SettingsGroupGroupAndRolesAndMembers: store }
}
