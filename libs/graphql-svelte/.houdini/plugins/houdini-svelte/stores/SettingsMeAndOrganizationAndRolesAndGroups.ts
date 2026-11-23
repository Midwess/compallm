import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/SettingsMeAndOrganizationAndRolesAndGroups.js'
import type { SettingsMeAndOrganizationAndRolesAndGroups$result, SettingsMeAndOrganizationAndRolesAndGroups$input } from '$houdini/artifacts/SettingsMeAndOrganizationAndRolesAndGroups.js'

export class SettingsMeAndOrganizationAndRolesAndGroupsStore extends QueryStore<SettingsMeAndOrganizationAndRolesAndGroups$result, SettingsMeAndOrganizationAndRolesAndGroups$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "SettingsMeAndOrganizationAndRolesAndGroupsStore",
            variables: false,
        })
    }
}

export async function load_SettingsMeAndOrganizationAndRolesAndGroups(params: QueryStoreFetchParams<SettingsMeAndOrganizationAndRolesAndGroups$result, SettingsMeAndOrganizationAndRolesAndGroups$input>): Promise<{SettingsMeAndOrganizationAndRolesAndGroups: SettingsMeAndOrganizationAndRolesAndGroupsStore}>{
    const store = new SettingsMeAndOrganizationAndRolesAndGroupsStore()
    await store.fetch(params)
    return { SettingsMeAndOrganizationAndRolesAndGroups: store }
}
