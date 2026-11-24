import { SettingsMembersMeAndOrganizationAndMembersStore, SettingsMembersInvitationsStore } from '@compallm/contract/houdini'
import type { SettingsMembersMeAndOrganizationAndMembers$input } from '@compallm/contract/houdini'
import { permissionKey, type PermissionController } from '@compallm/storybook'
import { requirePermissions } from '../../permission/controller'
import { graphqlClient, requireData, type RequestActivity } from '../../graphql-client'

export async function memberSelection(permissions?: PermissionController) {
  await requirePermissions(permissions, [{ action: 'member.read', resourceType: 'Organization' }])
  const checks = (['group.read', 'role.read'] as const).map((action) => ({ action, resourceType: 'Organization' as const }))
  await permissions?.evaluate(checks)
  const allowed = (index: number) =>
    !permissions || permissions.store.getState().entries[permissionKey(checks[index])]?.status === 'allowed'
  return {
    includeMembersEdgesNode: allowed(1),
    includeMembersEdgesNode2: allowed(0),
    includeMembersEdgesNode3: allowed(0) && allowed(1),
  }
}
export function readMembers(
  variables: SettingsMembersMeAndOrganizationAndMembers$input,
  signal: AbortSignal,
  activity?: RequestActivity,
) {
  return new SettingsMembersMeAndOrganizationAndMembersStore()
    .fetch({
      variables,
      fetch: graphqlClient.fetch(signal),
      metadata: { requestActivity: activity },
      policy: 'NoCache',
      blocking: true,
    })
    .then(requireData)
}
export function readInvitations(after: string | undefined, signal: AbortSignal, activity?: RequestActivity) {
  return new SettingsMembersInvitationsStore()
    .fetch({
      variables: { invitationsAfter: after },
      fetch: graphqlClient.fetch(signal),
      metadata: { requestActivity: activity },
      policy: 'NoCache',
      blocking: true,
    })
    .then(requireData)
}
