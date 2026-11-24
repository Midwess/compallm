import type { SettingsRouteData } from '../load'
import {
  SettingsProfileUserProfileStore,
  SettingsProfileGenerateUrlAvatarUrlStore,
  SettingsProfileUserProfileUpdateStore,
} from '@compallm/contract/houdini'
import type { ProfileController as Contract, SettingsController } from '@compallm/storybook/pages/settings'
import type { AuthController } from '@compallm/storybook'

import { ImageUploadInput, UserProfileUpdateInput } from '@compallm/contract/validation'
import { ApiError, graphqlClient, requireData } from '../../graphql-client'
import { notify } from '../../notifications/toast'
import { beginRequest } from '../../request-activity'

export type ProfileAuth = Partial<Pick<AuthController, 'store'>>

export class ProfileController implements Contract {
  private scope = new AbortController()
  private read?: AbortController
  private userId?: string
  private active = false
  private preparation?: SettingsRouteData
  routeData?: () => SettingsRouteData | undefined
  private uploaded?: { file: File; userId: string }
  private readonly settings: SettingsController['store']
  private readonly auth: ProfileAuth

  constructor(settings: SettingsController['store'], auth: ProfileAuth) {
    this.settings = settings
    this.auth = auth
  }

  reload = () => {
    void this.load()
  }

  start = () => {
    if (!this.seeded) this.cancel()
    this.seeded = false
    this.active = true
    this.scope = new AbortController()
    const synchronize = () => {
      const state = this.auth.store?.getState()
      if (state?.status === 'checking' || state?.status === 'unknown') return
      const next = state?.currentUser?.userId
      if (next && next === this.userId && this.settings.getState().profileStatus === 'ready') return
      if (next === this.userId && this.read) return
      this.scope.abort()
      this.scope = new AbortController()
      this.uploaded = undefined
      this.userId = next
      this.settings.form.reset({
        ...this.settings.form.values,
        profile: { name: '', username: '', email: '', avatar: null },
        profileUpload: null,
      })
      this.settings.setState({ profileStatus: next ? 'loading' : 'denied', profileLoaded: false, profileSaving: false })
      if (next) void this.load()
    }
    const unsubscribe = this.auth.store?.subscribe(synchronize)
    synchronize()
    return () => {
      unsubscribe?.()
      this.cancel()
    }
  }

  cancel = () => {
    this.active = false
    this.scope.abort()
    this.read?.abort()
    this.read = undefined
    this.userId = undefined
    this.uploaded = undefined
    this.settings.form.stop()
    this.settings.form.set('profileUpload', null)
    this.settings.setState({ profileSaving: false })
  }

  load = async () => {
    if (!this.active || !this.userId) return
    if (this.settings.getState().profileStatus === 'ready' && this.settings.form.dirty) return
    const scope = this.scope
    const userId = this.userId
    this.read?.abort()
    const request = new AbortController()
    this.read = request
    this.settings.setState({ profileStatus: 'loading' })
    try {
      const prepared = this.routeData?.()
      const usable =
        prepared &&
        prepared !== this.preparation &&
        prepared.version === graphqlClient.readVersion &&
        prepared.userId === userId &&
        prepared.profile
      this.preparation = prepared
      const { userProfile } = usable
        ? prepared.profile!
        : await new SettingsProfileUserProfileStore()
            .fetch({
              variables: { userProfileId: userId },
              fetch: graphqlClient.fetch(AbortSignal.any([scope.signal, request.signal])),
              policy: 'NoCache',
              blocking: true,
            })
            .then(requireData)
      if (scope.signal.aborted || this.read !== request) return
      this.adopt(userProfile, userId)
    } catch (error) {
      if (scope.signal.aborted || request.signal.aborted || this.read !== request) return
      this.settings.setState({
        profileStatus: error instanceof ApiError && [401, 403].includes(error.status) ? 'denied' : 'error',
      })
      notify.apiError(error, () => {
        if (!scope.signal.aborted && this.active) void this.load()
      })
    } finally {
      if (this.read === request) this.read = undefined
    }
  }

  seed(data: SettingsRouteData) {
    if (!data.profile) return
    this.userId = data.userId
    this.preparation = data
    this.adopt(data.profile.userProfile, data.userId)
    this.seeded = true
  }
  private seeded = false
  private adopt(userProfile: NonNullable<SettingsRouteData['profile']>['userProfile'], userId: string) {
    if (!userProfile) {
      this.settings.setState({ profileStatus: 'missing' })
      return
    }
    if (userProfile.node.id !== userId) throw new ApiError('Profile identity did not match.', 409, 'PROFILE_MISMATCH')
    const profile = {
      ...this.settings.getState().profile,
      name: userProfile.node.displayName,
      username: userProfile.node.username,
      email: userProfile.node.email,
      avatar: userProfile.node.avatarUrl,
    }
    if (!this.settings.form.dirty) this.settings.form.reset({ ...this.settings.form.values, profile, profileUpload: null })
    this.settings.setState({ profileStatus: 'ready', profileLoaded: true })
  }
  saveProfile = async (value: { name?: string; username?: string }, file: File | null, signal: AbortSignal) => {
    const scope = this.scope
    const userId = this.userId
    if (!this.active || !userId || this.settings.getState().profileStatus !== 'ready') return false as const
    const combined = AbortSignal.any([scope.signal, signal])
    const current = () => !combined.aborted && this.active && this.userId === userId
    const input = UserProfileUpdateInput.parse({
      ...(value.name === undefined ? {} : { displayName: value.name }),
      ...(value.username === undefined ? {} : { username: value.username }),
      avatarUploaded: Boolean(file),
    })
    this.settings.setState({ profileSaving: true })
    try {
      if (file && (this.uploaded?.file !== file || this.uploaded.userId !== userId)) {
        const uploadInput = ImageUploadInput.parse({
          size: file.size,
          mediaType: ({ 'image/jpeg': 'JPEG', 'image/png': 'PNG', 'image/webp': 'WEBP' } as Record<string, string>)[file.type],
        })

        const {
          generateUrlAvatarUrl: { node },
        } = await new SettingsProfileGenerateUrlAvatarUrlStore()
          .mutate({ generateUrlAvatarUrlInput: uploadInput }, { fetch: graphqlClient.fetch(combined) })
          .then(requireData)
        if (!current()) return false as const
        const form = new FormData()
        node.fields.forEach(({ name, value }) => form.append(name, value))
        form.append('file', file)
        const finish = beginRequest()
        try {
          const response = await fetch(node.url, { method: 'POST', body: form, credentials: 'omit', signal: combined })
          if (!response.ok)
            throw new ApiError('Avatar upload failed. Your profile has not been saved.', response.status, 'UPLOAD_FAILED')
        } finally {
          finish()
        }
        if (!current()) return false as const
        this.uploaded = { file, userId }
      }

      const {
        userProfileUpdate: { node },
      } = await new SettingsProfileUserProfileUpdateStore()
        .mutate({ userProfileUpdateInput: input }, { fetch: graphqlClient.fetch(combined) })
        .then(requireData)
      if (!current()) return false as const
      if (node.id !== userId) throw new ApiError('Profile identity did not match.', 409, 'PROFILE_MISMATCH')
      this.settings.form.set('profile.avatar', node.avatarUrl)
      this.settings.form.accept({ profile: { ...this.settings.form.baseline.profile!, avatar: node.avatarUrl } })
      const auth = this.auth.store
      const identity = auth?.getState().currentUser
      if (auth && identity?.userId === userId) auth.setState({ currentUser: { ...identity, avatarUrl: node.avatarUrl } })
      this.uploaded = undefined
      return { name: node.displayName, username: node.username, email: node.email, avatar: node.avatarUrl }
    } catch (error) {
      if (!current()) return false as const
      throw error
    } finally {
      if (!scope.signal.aborted && this.scope === scope) this.settings.setState({ profileSaving: false })
    }
  }
}
