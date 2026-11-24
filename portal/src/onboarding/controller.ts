import type { AuthController } from '@compallm/storybook/pages/auth'
import {
  OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfileStore,
  OnboardingOnboardingPresentStore,
  OnboardingOnboardingAdvanceStore,
  OnboardingOnboardingCompleteStore,
  OnboardingGenerateUrlAvatarUrlStore,
  OnboardingUserProfileUpdateStore,
  OnboardingOrganizationProfileUpdateStore,
  OnboardingAnalyticUserProfilePutStore,
} from '@compallm/contract/houdini'
import { createStore } from '@compallm/storybook/state'
import type { OnboardingController as Contract, OnboardingState } from '@compallm/storybook/pages/onboarding'
import type { UrlController, AuthUser, PermissionController } from '@compallm/storybook'
type OnboardingContext = NonNullable<
  import('@compallm/contract/houdini').OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$result['onboardingContext']
>['node']
import {
  UserProfileUpdateInput,
  OrganizationProfileUpdateInput,
  AnalyticUserProfilePutInput,
  ImageUploadInput,
} from '@compallm/contract/validation'
import { graphqlClient, requireData, ApiError } from '../graphql-client'
import { notify, apiErrorMessage } from '../notifications/toast'
import { beginRequest } from '../request-activity'
import type { UserPreferenceController } from '../preference/user/controller'
import { requirePermissions } from '../permission/controller'
import { organizationRoot } from '../organization/paths'

type Context = OnboardingContext
export class OnboardingController implements Contract {
  readonly store = createStore<OnboardingState & { visible: boolean; firstPresentation: boolean }>(() => ({
    step: 'profile',
    invited: false,
    existingUser: false,
    profile: { displayName: '', username: '', email: '', avatarUrl: null },
    workType: '',
    appearance: 'SYSTEM',
    organization: { name: '', welcome: '', links: [], destination: 'CHAT' },
    completed: false,
    error: null,
    embedded: true,
    visible: false,
    firstPresentation: false,
    pending: false,
  }))
  private scope = new AbortController()
  private read?: AbortController
  private uploaded?: File
  private current?: Context
  private reminder?: ReturnType<typeof setTimeout>
  private manual = false
  private stopIdentity?: () => void
  private readonly auth: AuthController
  private readonly url: UrlController
  private readonly permissions?: PermissionController
  constructor(
    auth: AuthController,
    url: UrlController,
    permissions?: PermissionController,
    private readonly preferences?: UserPreferenceController,
  ) {
    this.auth = auth
    this.url = url
    this.permissions = permissions
  }
  private get user() {
    return this.auth.store.getState().currentUser!
  }
  private prepared?: import('@compallm/contract/houdini').OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$result
  seed(result: NonNullable<typeof this.prepared>) {
    this.prepared = result
  }
  start = () => {
    this.scope = new AbortController()
    this.stopIdentity = this.auth.store.subscribe((state, before) => {
      if (state.currentUser?.organizationName !== before.currentUser?.organizationName && state.currentUser)
        this.store.setState((s) => ({ organization: { ...s.organization, name: state.currentUser!.organizationName } }))
    })
    void this.load(false)
    return this.stop
  }
  stop = () => {
    clearTimeout(this.reminder)
    this.scope.abort()
    this.read?.abort()
    this.stopIdentity?.()
    this.uploaded = undefined
  }
  private apply = (value: Context) => {
    this.current = value
    this.store.setState((s) => ({
      step: value.step.toLowerCase() as OnboardingState['step'],
      invited: value.flow !== 'PERSONAL',
      existingUser: value.flow === 'INVITED_EXISTING',
      artworkColor: value.settings.artworkColor,
      requireFullName: value.flow === 'INVITED_NEW' && value.settings.requireFullName,
      requireAvatar: value.flow === 'INVITED_NEW' && value.settings.requireAvatar,
      organization: { ...s.organization, ...value.settings },
    }))
  }
  load = async (manual: boolean) => {
    clearTimeout(this.reminder)
    this.read?.abort()
    const read = new AbortController()
    this.read = read
    const signal = AbortSignal.any([this.scope.signal, read.signal])
    const reminderKey = this.reminderKey
    try {
      const prepared = this.prepared
      this.prepared = undefined
      const result: import('@compallm/contract/houdini').OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$result =
        prepared ??
        (await new OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfileStore()
          .fetch({
            variables: { userProfileId: this.user.userId },
            fetch: graphqlClient.fetch(signal),
            policy: 'NoCache',
            blocking: true,
          })
          .then(requireData))
      if (signal.aborted || this.read !== read) return
      const context = result.onboardingContext?.node
      if (!context?.required) {
        this.clearReminder(reminderKey)
        if (!manual) {
          this.store.setState({ visible: false, completed: true })
          return
        }
      }
      if (!manual && this.deferReminder(reminderKey)) return
      const presented = await new OnboardingOnboardingPresentStore()
        .mutate({ onboardingPresentManual: manual }, { fetch: graphqlClient.fetch(signal) })
        .then(requireData)
      if (signal.aborted || this.read !== read) return
      this.manual = manual
      this.apply(presented.onboardingPresent.node)
      this.store.setState({
        pending: false,
        firstPresentation:
          !manual && Boolean(context?.required) && context?.flow !== 'INVITED_EXISTING' && !context?.hasBeenPresented,
        visible: true,
        completed: false,
        profile: result.userProfile?.node ?? this.store.getState().profile,
        workType: result.analyticUserProfile?.node.value ?? '',
        appearance: this.preferences?.store.getState().saved?.theme.mode ?? 'SYSTEM',
        organization: { ...this.store.getState().organization, name: this.user.organizationName },
      })
    } catch (error) {
      if (!signal.aborted)
        notify.apiError(error, () => {
          void this.load(manual)
        })
    }
  }
  reopen = () => {
    void this.load(true)
  }
  private get reminderKey() {
    return `compallm:onboarding:defer:${this.user.userId}:${this.user.organizationId}`
  }
  private clearReminder(key: string) {
    clearTimeout(this.reminder)
    try {
      globalThis.localStorage?.removeItem(key)
    } catch {
      // Browser storage may be unavailable; server completion remains authoritative.
    }
  }
  private deferReminder(key: string) {
    let until = 0
    try {
      until = Number(globalThis.localStorage?.getItem(key))
    } catch {
      return false
    }
    if (!Number.isFinite(until) || until <= Date.now()) {
      this.clearReminder(key)
      return false
    }
    clearTimeout(this.reminder)
    this.reminder = setTimeout(() => void this.load(false), Math.min(until - Date.now(), 12 * 60 * 60 * 1000))
    return true
  }
  dismiss = () => {
    this.store.setState({ visible: false })
    if (!this.current?.required) return
    const key = this.reminderKey
    try {
      globalThis.localStorage?.setItem(key, String(Date.now() + 12 * 60 * 60 * 1000))
    } catch {
      return
    }
    this.deferReminder(key)
  }
  dismissError = () => this.store.setState({ error: null })
  private transition = async (back: boolean) => {
    if (this.store.getState().pending) return
    this.store.setState({ pending: true, error: null })
    try {
      if (this.manual && !this.current?.required) {
        const steps = ['introduction', 'ready'] as const
        const index = steps.indexOf(this.store.getState().step as (typeof steps)[number])
        this.store.setState({ step: steps[back ? Math.max(0, index - 1) : Math.min(1, index + 1)] })
        return
      }

      const { onboardingAdvance } = await new OnboardingOnboardingAdvanceStore()
        .mutate({ onboardingAdvanceBack: back }, { fetch: graphqlClient.fetch(this.scope.signal) })
        .then(requireData)
      if (!this.scope.signal.aborted) this.apply(onboardingAdvance.node)
    } catch (error) {
      if (!this.scope.signal.aborted) this.store.setState({ error: apiErrorMessage(error) })
    } finally {
      if (!this.scope.signal.aborted) this.store.setState({ pending: false })
    }
  }
  next = () => this.transition(false)
  back = () => this.transition(true)
  skip = () => {
    void this.next()
  }
  finish = async () => {
    if (this.store.getState().pending) return
    this.store.setState({ pending: true, error: null })
    try {
      if (this.current?.required) {
        const result = await new OnboardingOnboardingCompleteStore()
          .mutate(undefined, { fetch: graphqlClient.fetch(this.scope.signal) })
          .then(requireData)
        if (this.scope.signal.aborted) return
        this.apply(result.onboardingComplete.node)
      }
      let destination = this.current?.settings.destination ?? 'CHAT'
      if (destination !== 'CHAT') {
        try {
          await requirePermissions(this.permissions, [
            { action: destination === 'MODELS' ? 'model.read' : 'provider.read', resourceType: 'Organization' },
          ])
        } catch {
          destination = 'CHAT'
        }
      }
      if (this.scope.signal.aborted) return
      this.clearReminder(this.reminderKey)
      this.store.setState({ visible: false, completed: true })
      this.url.navigate(`${organizationRoot(this.user)}/${destination.toLowerCase()}`)
    } catch (error) {
      if (!this.scope.signal.aborted) this.store.setState({ error: apiErrorMessage(error) })
    } finally {
      if (!this.scope.signal.aborted) this.store.setState({ pending: false })
    }
  }
  private save = async (signal: AbortSignal, operation: (signal: AbortSignal) => Promise<void>) => {
    if (this.store.getState().pending) return false
    this.store.setState({ pending: true, error: null })
    const scope = this.scope
    const combined = AbortSignal.any([signal, scope.signal])
    try {
      await operation(combined)
      return !combined.aborted
    } catch (error) {
      if (!combined.aborted) this.store.setState({ error: apiErrorMessage(error) })
      return false
    } finally {
      if (!scope.signal.aborted && this.scope === scope) this.store.setState({ pending: false })
    }
  }
  saveProfile: Contract['saveProfile'] = async (value, file, signal) => {
    const success = await this.save(signal, async (combined) => {
      if (file && this.uploaded !== file) {
        const input = ImageUploadInput.parse({
          size: file.size,
          mediaType: ({ 'image/jpeg': 'JPEG', 'image/png': 'PNG', 'image/webp': 'WEBP' } as Record<string, string>)[file.type],
        })
        const {
          generateUrlAvatarUrl: { node },
        } = await new OnboardingGenerateUrlAvatarUrlStore()
          .mutate({ generateUrlAvatarUrlInput: input }, { fetch: graphqlClient.fetch(combined) })
          .then(requireData)
        const body = new FormData()
        node.fields.forEach((f) => body.append(f.name, f.value))
        body.append('file', file)
        const finish = beginRequest()
        try {
          const response = await fetch(node.url, { method: 'POST', body, signal: combined, credentials: 'omit' })
          if (!response.ok) throw new ApiError('Avatar upload failed.', response.status, 'UPLOAD_FAILED')
        } finally {
          finish()
        }
        this.uploaded = file
      }
      const input = UserProfileUpdateInput.parse({ ...value, avatarUploaded: Boolean(file) })
      const {
        userProfileUpdate: { node },
      } = await new OnboardingUserProfileUpdateStore()
        .mutate({ userProfileUpdateInput: input }, { fetch: graphqlClient.fetch(combined) })
        .then(requireData)
      if (combined.aborted) return
      this.store.setState({ profile: node })
      const user = this.user
      this.auth.store.setState({ currentUser: { ...user, avatarUrl: node.avatarUrl } })
    })
    return success ? this.store.getState().profile : false
  }
  saveWorkspaceName: Contract['saveWorkspaceName'] = (name, signal) =>
    this.save(signal, async (combined) => {
      if (name === this.store.getState().organization.name) return
      const input = OrganizationProfileUpdateInput.parse({ name })
      const {
        organizationProfileUpdate: { node },
      } = await new OnboardingOrganizationProfileUpdateStore()
        .mutate({ organizationProfileUpdateInput: input }, { fetch: graphqlClient.fetch(combined) })
        .then(requireData)
      if (combined.aborted) return
      const user: AuthUser = this.user
      const root = organizationRoot(user)
      const location = this.url.store.getState()
      this.auth.store.setState({ currentUser: { ...user, organizationName: node.name } })
      this.store.setState((s) => ({ organization: { ...s.organization, name: node.name } }))
      if (location.pathname === root || location.pathname.startsWith(root + '/'))
        this.url.navigate(
          organizationRoot({ organizationName: node.name }) +
            location.pathname.slice(root.length) +
            location.search +
            location.hash,
          { replace: true },
        )
    })
  saveWorkType: Contract['saveWorkType'] = (value, signal) =>
    this.save(signal, async (combined) => {
      const input = AnalyticUserProfilePutInput.parse({ key: 'work', value })
      const result = await new OnboardingAnalyticUserProfilePutStore()
        .mutate({ analyticUserProfilePutInput: input }, { fetch: graphqlClient.fetch(combined) })
        .then(requireData)
      if (!combined.aborted) this.store.setState({ workType: result.analyticUserProfilePut.node.value })
    })
  saveAppearance: Contract['saveAppearance'] = (value, signal) =>
    this.save(signal, async (combined) => {
      await this.preferences?.update({ theme: value }, combined)
      if (!combined.aborted) this.store.setState({ appearance: value })
    })
}
