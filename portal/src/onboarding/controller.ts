import { createStore } from 'zustand/vanilla'
import type { OnboardingController as Contract, OnboardingState } from '@compallm/storybook/pages/onboarding'
import type { AuthController, UrlController, AuthUser, PermissionController } from '@compallm/storybook'
import type {
  FieldsSelection,
  OnboardingContext,
  MutationGenqlSelection,
  QueryGenqlSelection,
} from '@compallm/contract/graphql'
import {
  UserProfileUpdateInput,
  OrganizationProfileUpdateInput,
  AnalyticUserProfilePutInput,
  ImageUploadInput,
} from '@compallm/contract/validation'
import { graphql, ApiError } from '../graphql-client'
import { notify } from '../notifications/toast'
import { beginRequest } from '../request-activity'
import { userPreferenceController } from '../preference/user/controller'
import { requirePermissions } from '../permission/controller'
import { organizationRoot } from '../organization/paths'

export const onboardingSelection = {
  required: true,
  shouldPresent: true,
  flow: true,
  step: true,
  settings: {
    welcome: true,
    links: { label: true, href: true },
    destination: true,
    artworkColor: true,
    requireFullName: true,
    requireAvatar: true,
  },
} as const
const profileSelection = { displayName: true, username: true, email: true, avatarUrl: true } as const
type Context = FieldsSelection<OnboardingContext, typeof onboardingSelection>
export class OnboardingController implements Contract {
  readonly store = createStore<OnboardingState & { visible: boolean }>(() => ({
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
    pending: false,
  }))
  private scope = new AbortController()
  private read?: AbortController
  private uploaded?: File
  private current?: Context
  private manual = false
  private stopIdentity?: () => void
  private readonly auth: AuthController
  private readonly url: UrlController
  private readonly permissions?: PermissionController
  constructor(auth: AuthController, url: UrlController, permissions?: PermissionController) {
    this.auth = auth
    this.url = url
    this.permissions = permissions
  }
  private get user() {
    return this.auth.store.getState().currentUser!
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
    this.read?.abort()
    const read = new AbortController()
    this.read = read
    const signal = AbortSignal.any([this.scope.signal, read.signal])
    try {
      const selection = {
        onboardingContext: { node: onboardingSelection },
        userProfile: { __args: { id: this.user.userId }, node: profileSelection },
        analyticUserProfile: { __args: { key: 'work' }, node: { value: true } },
      } satisfies QueryGenqlSelection
      const result = await graphql(signal).query(selection)
      if (signal.aborted || this.read !== read) return
      const context = result.onboardingContext?.node
      if (!manual && !context?.shouldPresent) return
      const presented = await graphql(signal).mutation({
        onboardingPresent: { __args: { manual }, node: onboardingSelection },
      } satisfies MutationGenqlSelection)
      if (signal.aborted || this.read !== read) return
      this.manual = manual
      this.apply(presented.onboardingPresent.node)
      this.store.setState({
        pending: false,
        visible: true,
        completed: false,
        profile: result.userProfile?.node ?? this.store.getState().profile,
        workType: result.analyticUserProfile?.node.value ?? '',
        appearance: userPreferenceController.store.getState().saved?.theme.mode ?? 'SYSTEM',
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
  dismiss = () => this.store.setState({ visible: false })
  dismissError = () => this.store.setState({ error: null })
  private transition = async (back: boolean) => {
    if (this.store.getState().pending) return
    this.store.setState({ pending: true })
    try {
      if (this.manual && !this.current?.required) {
        const steps = ['introduction', 'ready'] as const
        const index = steps.indexOf(this.store.getState().step as (typeof steps)[number])
        this.store.setState({ step: steps[back ? Math.max(0, index - 1) : Math.min(1, index + 1)] })
        return
      }
      const selection = { onboardingAdvance: { __args: { back }, node: onboardingSelection } } satisfies MutationGenqlSelection
      const { onboardingAdvance } = await graphql(this.scope.signal).mutation(selection)
      if (!this.scope.signal.aborted) this.apply(onboardingAdvance.node)
    } catch (error) {
      if (!this.scope.signal.aborted) notify.error(error)
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
    this.store.setState({ pending: true })
    try {
      if (this.current?.required) {
        const result = await graphql(this.scope.signal).mutation({
          onboardingComplete: { node: onboardingSelection },
        } satisfies MutationGenqlSelection)
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
      this.store.setState({ visible: false, completed: true })
      this.url.navigate(`${organizationRoot(this.user)}/${destination.toLowerCase()}`)
    } catch (error) {
      if (!this.scope.signal.aborted) notify.error(error)
    } finally {
      if (!this.scope.signal.aborted) this.store.setState({ pending: false })
    }
  }
  private save = async (signal: AbortSignal, operation: (signal: AbortSignal) => Promise<void>) => {
    if (this.store.getState().pending) return false
    this.store.setState({ pending: true })
    const scope = this.scope
    const combined = AbortSignal.any([signal, scope.signal])
    try {
      await operation(combined)
      return !combined.aborted
    } catch (error) {
      if (!combined.aborted) notify.error(error)
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
        } = await graphql(combined).mutation({
          generateUrlAvatarUrl: { __args: { input }, node: { url: true, fields: { name: true, value: true } } },
        } satisfies MutationGenqlSelection)
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
      } = await graphql(combined).mutation({
        userProfileUpdate: { __args: { input }, node: profileSelection },
      } satisfies MutationGenqlSelection)
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
      } = await graphql(combined).mutation({
        organizationProfileUpdate: { __args: { input }, node: { name: true } },
      } satisfies MutationGenqlSelection)
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
      const result = await graphql(combined).mutation({
        analyticUserProfilePut: { __args: { input }, node: { value: true } },
      } satisfies MutationGenqlSelection)
      if (!combined.aborted) this.store.setState({ workType: result.analyticUserProfilePut.node.value })
    })
  saveAppearance: Contract['saveAppearance'] = (value, signal) =>
    this.save(signal, async (combined) => {
      await userPreferenceController.update({ theme: value }, combined)
      if (!combined.aborted) this.store.setState({ appearance: value })
    })
}
