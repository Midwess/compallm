import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/SettingsMembersInvitations.js'
import type { SettingsMembersInvitations$result, SettingsMembersInvitations$input } from '$houdini/artifacts/SettingsMembersInvitations.js'

export class SettingsMembersInvitationsStore extends QueryStore<SettingsMembersInvitations$result, SettingsMembersInvitations$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "SettingsMembersInvitationsStore",
            variables: false,
        })
    }
}

export async function load_SettingsMembersInvitations(params: QueryStoreFetchParams<SettingsMembersInvitations$result, SettingsMembersInvitations$input>): Promise<{SettingsMembersInvitations: SettingsMembersInvitationsStore}>{
    const store = new SettingsMembersInvitationsStore()
    await store.fetch(params)
    return { SettingsMembersInvitations: store }
}
