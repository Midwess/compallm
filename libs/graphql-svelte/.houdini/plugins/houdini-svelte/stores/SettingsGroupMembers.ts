import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/SettingsGroupMembers.js'
import type { SettingsGroupMembers$result, SettingsGroupMembers$input } from '$houdini/artifacts/SettingsGroupMembers.js'

export class SettingsGroupMembersStore extends QueryStore<SettingsGroupMembers$result, SettingsGroupMembers$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "SettingsGroupMembersStore",
            variables: false,
        })
    }
}

export async function load_SettingsGroupMembers(params: QueryStoreFetchParams<SettingsGroupMembers$result, SettingsGroupMembers$input>): Promise<{SettingsGroupMembers: SettingsGroupMembersStore}>{
    const store = new SettingsGroupMembersStore()
    await store.fetch(params)
    return { SettingsGroupMembers: store }
}
