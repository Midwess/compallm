<script lang="ts">
  import { onMount, untrack, type Snippet } from 'svelte'
  import { page } from '$app/state'
  import { OrganizationPage } from '@compallm/storybook/pages/organization'
  import { OnboardingDialog } from '@compallm/storybook/pages/onboarding'
  import type { AuthUser } from '@compallm/storybook/pages/auth'
  import { setOrganizationController, setPermissionController, setOnboardingController } from '@compallm/storybook/context'
  import { getPortalControllers, setWorkspaceControllers } from './context'
  import { OrganizationController } from './organization/controller'
  import { cedarWorker } from './workers/cedar-client'
  import { OnboardingController } from './onboarding/controller'
  let { user, children }: { user: AuthUser; children?: Snippet } = $props()
  const { auth, url, preferences, permissions } = getPortalControllers()
  const organization = new OrganizationController(
    untrack(() => user),
    url,
    auth,
  )
  const onboarding = new OnboardingController(auth, url, permissions, preferences)
  setOrganizationController(organization)
  setPermissionController(permissions)
  setOnboardingController(onboarding)
  setWorkspaceControllers({ organization, permissions, onboarding })
  untrack(() => {
    if (page.data.user?.id === user.id && page.data.authority) permissions.seed(user, page.data.authority.me.node)
    if (page.data.user?.id === user.id && page.data.preferences) preferences.seed(page.data.preferences.userPreferences.node)
    if (page.data.onboardingRead) onboarding.seed(page.data.onboardingRead)
    if (page.data.providers) organization.seed(page.data.providers)
    const saved = preferences.store.getState().saved
    if (saved) {
      organization.setSidebarOrder(saved.sidebar.entries.map(({ item }) => item))
      organization.setSidebarLayout(saved.sidebar.layout)
    }
  })
  let settings = $derived(page.url.pathname.includes('/settings'))
  let visible = $derived(onboarding.store.getState().visible)
  let firstPresentation = $derived(onboarding.store.getState().firstPresentation)
  onMount(() => {
    void cedarWorker.start().catch(() => undefined)
    permissions.start(user)
    const stopOrganization = organization.start()
    const stopOnboarding = onboarding.start()
    const stopPreferences = preferences.connectOrganization(organization, url)
    return () => {
      stopPreferences()
      stopOnboarding()
      stopOrganization()
      permissions.stop()
      cedarWorker.stop()
    }
  })
  $effect(() => {
    const current = user
    untrack(() => organization.setCurrentUser(current))
  })
</script>

{#if settings}{@render children?.()}{:else}<OrganizationPage>{@render children?.()}</OrganizationPage>{/if}
<OnboardingDialog
  open={visible}
  {firstPresentation}
  onOpenChange={(open) => {
    if (!open) onboarding.dismiss()
  }}
/>
