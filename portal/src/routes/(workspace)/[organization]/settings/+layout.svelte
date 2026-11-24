<script lang="ts">
  let { children } = $props()
  import { onMount, untrack } from 'svelte'
  import { graphqlClient } from '../../../../graphql-client'
  import { page } from '$app/state'
  import { SettingsPage } from '@compallm/storybook/pages/settings'
  import { setSettingsController } from '@compallm/storybook/context'
  import { SettingsController, settingsPages } from '../../../../settings/controller'
  import { getPortalControllers, getWorkspaceControllers } from '../../../../context'
  import type { SettingsDestination } from '@compallm/storybook/pages/settings'
  const { auth, url, preferences, passkeys } = getPortalControllers()
  const { organization, permissions, onboarding } = getWorkspaceControllers()
  const controller = new SettingsController(
    organization.getCurrentOrganization()!.name,
    url,
    auth,
    'general',
    preferences,
    permissions,
    () => onboarding.reopen(),
  )
  controller.pages.members.routeData = () => page.data.settingsRead
  controller.pages.profile.routeData = () => page.data.settingsRead
  controller.pages.organization.routeData = () => page.data.settingsRead
  controller.pages.onboarding.routeData = () => page.data.settingsRead
  function seedPage(data: typeof page.data.settingsRead, id?: string) {
    if (!data || data.version !== graphqlClient.readVersion) return
    const prepared = { ...data, version: graphqlClient.readVersion }
    if (!controller.store.form.dirty && !controller.store.form.pending) {
      controller.pages.profile.seed(prepared)
      controller.pages.organization.seed(prepared)
    }
    if (!controller.pages.onboarding.store.form.dirty) controller.pages.onboarding.seed(prepared)
    controller.pages.members.seed(prepared)
    if (data.sessions) controller.seedSessions(data.sessions)
    if (data.passkeys) passkeys.seed(data.passkeys)
    if (data.group && id && !controller.group.store.form.dirty) controller.group.seed(data.group, id)
    if (data.role && !controller.role.store.form.dirty) controller.role.seed(data.role, data.policyActions)
  }
  let adopted = untrack(() => page.data.settingsRead)
  untrack(() => {
    const parts = page.url.pathname.split('/').slice(3)
    controller.setPage(parts[0] as SettingsDestination, parts[1] && decodeURIComponent(parts[1]))
    if (page.data.settingsNavigation) controller.seedNavigation(page.data.settingsNavigation)
    seedPage(page.data.settingsRead, parts[1] && decodeURIComponent(parts[1]))
  })
  setSettingsController(controller)
  onMount(() => controller.start())
  $effect(() => {
    const parts = page.url.pathname.split('/').slice(3)
    const value = parts[0],
      id = parts[1] && decodeURIComponent(parts[1])
    const valid = ((value === 'role' || value === 'group') && id) || (settingsPages.some((entry) => entry === value) && !id)
    if (!valid) {
      void url.navigate(`/${encodeURIComponent(organization.getCurrentOrganization()!.name)}/settings/general`, {
        replace: true,
      })
      return
    }
    const data = page.data.settingsRead
    untrack(() => {
      if (data !== adopted) {
        adopted = data
        seedPage(data, id)
      }
      controller.setPage(value as SettingsDestination, id)
    })
    if (value === 'security') return untrack(() => passkeys.startView())
  })
</script>

<SettingsPage />

{@render children?.()}
