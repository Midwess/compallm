<script lang="ts">
  import { loadedPreferenceDraft } from '../../../src/pages/settings'
  import { SettingsPage } from '../../../src/pages/settings'
  import type {
    SettingsState,
    SettingsDestination,
    RoleDraft,
    GroupDraft,
    UserPreferenceState,
    UserPreferenceValues,
  } from '../../../src/pages/settings'
  import { DemoAuthController } from '../demo-auth-controller'
  import type { AuthUser } from '../../../src/pages/auth/types'
  import { DemoSettingsController } from './demo-controller'

  import { untrack } from 'svelte'

  import { setAuthController, setSettingsController, setUserPreferenceController } from '../../../src/context'

  let {
    page = 'general',
    id,
    initial = {},
    failure = false,
    roleLoadingMs = 0,
    groupLoadingMs = 0,
    pagedGroup = false,
    preferenceStatus,
    savedPreferences,
    session,
  }: {
    page?: SettingsDestination
    id?: string
    initial?: Partial<SettingsState> & {
      roleDraft?: RoleDraft | null
      groupDraft?: GroupDraft | null
    }
    failure?: boolean | 'conflict'
    roleLoadingMs?: number
    groupLoadingMs?: number
    pagedGroup?: boolean
    preferenceStatus?: UserPreferenceState['status']
    savedPreferences?: UserPreferenceValues
    session?: Partial<AuthUser>
  } = $props()

  let auth = $state(
    untrack(() => {
      const auth = new DemoAuthController({ mode: 'signin' })
      auth.store.setState({
        status: 'authenticated',
        currentUser: {
          id: 'session',
          userId: 'alex',
          organizationId: 'workspace',
          organizationName: 'Workspace',
          avatarUrl: null,
          ipAddress: null,
          userAgent: null,
          signedInAt: '1794838000',
          lastSeenAt: '1794838000',
          ...session,
        },
      })
      return auth
    }),
  )
  let controller = $state(
    untrack(() => {
      const controller = new DemoSettingsController({ location: { page, id }, ...initial }, failure)
      const user = controller.pages.preferences.user
      if (savedPreferences) {
        user.store.setState({ saved: savedPreferences })
        user.store.form.reset({ draft: loadedPreferenceDraft(savedPreferences) })
      }
      if (preferenceStatus)
        user.store.setState({
          status: preferenceStatus,
          ...(preferenceStatus !== 'ready' && !savedPreferences ? { saved: null } : {}),
        })
      return controller
    }),
  )
  $effect(() => {
    void [controller, initial.error]
    return untrack(() => {
      controller.start()
      if (initial.error) controller.apiError({ info: { message: initial.error } })
      return controller.stop
    })
  })
  $effect(() => {
    void [controller, roleLoadingMs]
    return untrack(() => {
      if (!roleLoadingMs) return
      const draft = controller.pages.role.store.getState().draft
      controller.pages.role.store.setState({ loading: true, draft: null })
      const timer = setTimeout(() => {
        controller.pages.role.store.setState({ loading: false, draft })
      }, roleLoadingMs)
      return () => clearTimeout(timer)
    })
  })
  $effect(() => {
    void [controller, groupLoadingMs]
    return untrack(() => {
      if (!groupLoadingMs) return
      const draft = controller.group.store.getState().draft
      controller.group.store.setState({ loading: true, draft: null })
      const timer = setTimeout(() => controller.group.store.setState({ loading: false, draft }), groupLoadingMs)
      return () => clearTimeout(timer)
    })
  })
  $effect(() => {
    void [controller, pagedGroup]
    return untrack(() => {
      if (!pagedGroup) return
      const members = controller.store.getState().members
      controller.group.store.setState({
        members: members.slice(0, 1).map((member) => ({ ...member, available: true })),
        hasNextPage: members.length > 1,
        memberCount: controller.group.store.getState().draft?.memberIds.length,
      })
    })
  })
  setAuthController(auth)
  setSettingsController(controller)
  setUserPreferenceController(controller.pages.preferences.user)
</script>

<SettingsPage></SettingsPage>
