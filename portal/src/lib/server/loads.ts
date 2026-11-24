import { redirect } from '@sveltejs/kit'
import { settingsPages } from '../../settings/destinations'
import type { RequestEvent } from '@sveltejs/kit'
import { readProviderUrl } from '../../provider/url-state'

export async function providerLoad(event: RequestEvent, savedId?: string) {
  const api = event.locals.portal
  const user = await api.requireUser(event.params.organization)
  const destination = readProviderUrl({ pathname: event.url.pathname, search: event.url.search, hash: '' })
  const [list, provider, model, global, saved] = await Promise.all([
    !savedId && destination.step === 'choose'
      ? api
          .query('ProviderCatalogEffectiveProviderTemplates', {
            effectiveProviderTemplatesSearch: destination.query || undefined,
          })
          .then((r) => r.effectiveProviderTemplates)
      : undefined,
    !savedId && destination.provider
      ? api
          .query('ProviderCatalogEffectiveProviderTemplate', { effectiveProviderTemplateId: destination.provider })
          .then((r) => r.effectiveProviderTemplate?.node ?? null)
      : undefined,
    !savedId && destination.provider && destination.model && !destination.model.startsWith('draft:')
      ? api
          .query('ProviderCatalogEffectiveModelTemplate', {
            effectiveModelTemplateProviderTemplateId: destination.provider,
            effectiveModelTemplateId: destination.model,
          })
          .then((r) => r.effectiveModelTemplate?.node ?? null)
      : undefined,
    !savedId &&
    destination.provider &&
    destination.model &&
    !destination.model.startsWith('custom:') &&
    !destination.model.startsWith('draft:')
      ? api
          .query('ProviderCatalogModelTemplate', {
            modelTemplateProviderTemplateId: destination.provider,
            modelTemplateId: destination.model,
          })
          .then((r) => r.modelTemplate?.node ?? null)
      : undefined,
    savedId ? api.query('ProviderProvider', { providerId: savedId }) : undefined,
  ])
  return {
    user,
    providerLocation: event.url.pathname + event.url.search,
    providerRead: {
      version: 0,
      preparationVersion: api.preparationVersion,
      organizationId: user.organizationId,
      destination,
      savedId,
      list,
      provider,
      model,
      global,
      saved,
    },
  }
}

export async function settingsLoad(event: RequestEvent) {
  const api = event.locals.portal
  const user = await api.requireUser(event.params.organization)
  const destination = event.params.settingsPage
  if (!(event.params.id ? ['role', 'group'].includes(destination ?? '') : settingsPages.some((page) => page === destination)))
    redirect(303, '/' + encodeURIComponent(user.organizationName) + '/settings/general')
  const {
    me: { node },
  } = await api.query('PortalAuthority')
  const can = (action: string) =>
    node.organizationPermissions.some((permission) => permission.action === action && permission.allowed)
  const selection = {
    includeMembersEdgesNode: can('role.read'),
    includeMembersEdgesNode2: can('group.read'),
    includeMembersEdgesNode3: can('role.read') && can('group.read'),
  }
  const [profile, organization, onboarding, members, sessions, passkeys, group, role, policyActions] = await Promise.all([
    destination === 'profile' ? api.query('SettingsProfileUserProfile', { userProfileId: user.userId }) : undefined,
    destination === 'organization'
      ? api.query('SettingsOrganizationOrganizationProfile', { organizationProfileId: user.organizationId })
      : undefined,
    destination === 'organization-onboarding' && can('organization.settings.read')
      ? api.query('SettingsOnboardingOrganizationOnboardingSettings')
      : undefined,
    destination === 'members' && can('member.read')
      ? Promise.all([
          api.query('SettingsMembersMeAndOrganizationAndMembers', {
            membersFilter: {},
            membersOrderBy: [{ displayName: 'AscNullsLast' }],
            ...selection,
          }),
          api.query('SettingsMembersInvitations'),
        ]).then(([result, invitations]) => ({ result, invitations, selection }))
      : undefined,
    ['access', 'security'].includes(destination ?? '') ? api.query('SettingsSessions') : undefined,
    destination === 'security' ? api.query('PasskeyPasskeysAndSession') : undefined,
    destination === 'group' && event.params.id && can('group.read')
      ? api.query('SettingsGroupGroupAndRolesAndMembers', {
          groupId: event.params.id,
          includeGroupNode: can('role.read'),
          includeGroupNodeMemberIds: can('member.read'),
        })
      : undefined,
    destination === 'role' && event.params.id && can('role.read')
      ? api
          .query('SettingsRoleRole', { roleId: event.params.id, includeRoleNodeCedarSource: false })
          .then((result) =>
            result.role?.node.canReadPolicy
              ? api.query('SettingsRoleRole', { roleId: event.params.id!, includeRoleNodeCedarSource: true })
              : result,
          )
      : undefined,
    destination === 'role' && can('role.read') ? api.query('SettingsRoleOrganizationPolicyActions') : undefined,
  ])
  return {
    user,
    settingsRead: {
      version: 0,
      preparationVersion: api.preparationVersion,
      userId: user.userId,
      organizationId: user.organizationId,
      profile,
      organization,
      onboarding,
      members,
      sessions,
      passkeys,
      group,
      role,
      policyActions,
    },
  }
}
