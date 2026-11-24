import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/SettingsMembersMeAndOrganizationAndMembers.js'
import type { SettingsMembersMeAndOrganizationAndMembers$result, SettingsMembersMeAndOrganizationAndMembers$input } from '$houdini/artifacts/SettingsMembersMeAndOrganizationAndMembers.js'

export class SettingsMembersMeAndOrganizationAndMembersStore extends QueryStore<SettingsMembersMeAndOrganizationAndMembers$result, SettingsMembersMeAndOrganizationAndMembers$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "SettingsMembersMeAndOrganizationAndMembersStore",
            variables: true,
        })
    }
}

export async function load_SettingsMembersMeAndOrganizationAndMembers(params: QueryStoreFetchParams<SettingsMembersMeAndOrganizationAndMembers$result, SettingsMembersMeAndOrganizationAndMembers$input>): Promise<{SettingsMembersMeAndOrganizationAndMembers: SettingsMembersMeAndOrganizationAndMembersStore}>{
    const store = new SettingsMembersMeAndOrganizationAndMembersStore()
    await store.fetch(params)
    return { SettingsMembersMeAndOrganizationAndMembers: store }
}
