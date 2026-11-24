import type { LayoutServerLoad } from './$types'
export const load: LayoutServerLoad = async ({ locals, params }) => {
  await locals.portal.requireUser(params.organization)
  const {
    me: { node },
  } = await locals.portal.query('PortalAuthority')
  const allowed = (action: string) =>
    node.organizationPermissions.some((permission) => permission.action === action && permission.allowed)
  return {
    settingsNavigation: await locals.portal.query('SettingsMeAndOrganizationAndRolesAndGroups', {
      includeFields: allowed('member.read'),
      includeFields2: allowed('role.read'),
      includeFields3: allowed('group.read'),
    }),
  }
}
