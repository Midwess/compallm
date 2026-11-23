import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/SettingsRoleOrganizationPolicyActions.js'
import type { SettingsRoleOrganizationPolicyActions$result, SettingsRoleOrganizationPolicyActions$input } from '$houdini/artifacts/SettingsRoleOrganizationPolicyActions.js'

export class SettingsRoleOrganizationPolicyActionsStore extends QueryStore<SettingsRoleOrganizationPolicyActions$result, SettingsRoleOrganizationPolicyActions$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "SettingsRoleOrganizationPolicyActionsStore",
            variables: false,
        })
    }
}

export async function load_SettingsRoleOrganizationPolicyActions(params: QueryStoreFetchParams<SettingsRoleOrganizationPolicyActions$result, SettingsRoleOrganizationPolicyActions$input>): Promise<{SettingsRoleOrganizationPolicyActions: SettingsRoleOrganizationPolicyActionsStore}>{
    const store = new SettingsRoleOrganizationPolicyActionsStore()
    await store.fetch(params)
    return { SettingsRoleOrganizationPolicyActions: store }
}
