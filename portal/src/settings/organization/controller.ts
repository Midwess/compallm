import { requirePermissions } from '../../permission/controller'
import type { PermissionController } from '@compallm/storybook'
import type { OrganizationController as Contract, SettingsController } from '@compallm/storybook/pages/settings'
import type { AuthController } from '@compallm/storybook'
import type { MutationGenqlSelection, QueryGenqlSelection } from '@compallm/contract/graphql'
import { ImageUploadInput, OrganizationProfileUpdateInput } from '@compallm/contract/validation'
import { ApiError, graphql } from '../../graphql-client'
import { notify } from '../../notifications/toast'
import { beginRequest } from '../../request-activity'

export type OrganizationAuth = Partial<Pick<AuthController, 'store'>>

const organizationSelection = { id: true, name: true, avatarUrl: true, canUpdateProfile: true } as const

export class OrganizationController implements Contract {
  private scope = new AbortController()
  private read?: AbortController
  private organizationId?: string
  private identity?: string
  private active = false
  private uploaded?: { file: File; organizationId: string }
  private readonly permissions?: PermissionController
  private readonly settings: SettingsController['store']
  private readonly auth: OrganizationAuth
  private readonly renamed: (name: string) => void

  constructor(
    settings: SettingsController['store'],
    auth: OrganizationAuth,
    renamed: (name: string) => void,
    permissions?: PermissionController,
  ) {
    this.permissions = permissions
    this.settings = settings
    this.auth = auth
    this.renamed = renamed
  }

  updateOrganization = () => false as const
  setAvatar = () => false as const

  reload = () => {
    void this.load()
  }

  start = () => {
    this.cancel()
    this.active = true
    this.scope = new AbortController()
    const synchronize = () => {
      const state = this.auth.store?.getState()
      if (state?.status === 'checking' || state?.status === 'unknown') return
      const next = state?.currentUser?.organizationId
      const identity = state?.currentUser ? `${state.currentUser.userId}:${next}` : undefined
      if (next && identity === this.identity && this.settings.getState().organizationProfileStatus === 'ready') return
      if (identity === this.identity && this.read) return
      this.scope.abort()
      this.scope = new AbortController()
      this.uploaded = undefined
      this.organizationId = next
      this.identity = identity
      this.settings.form.reset({
        ...this.settings.form.values,
        organization: { ...this.settings.getState().organization, name: '', logo: null },
        organizationUpload: null,
      })
      this.settings.setState({ organizationProfileStatus: next ? 'loading' : 'denied', organizationProfileSaving: false })
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
    this.organizationId = undefined
    this.identity = undefined
    this.uploaded = undefined
    this.settings.form.stop()
    this.settings.form.set('organizationUpload', null)
    this.settings.setState({ organizationProfileSaving: false })
  }

  load = async () => {
    if (!this.active || !this.organizationId) return
    if (this.settings.getState().organizationProfileStatus === 'ready' && this.settings.form.dirty) return
    const scope = this.scope
    const organizationId = this.organizationId
    this.read?.abort()
    const request = new AbortController()
    this.read = request
    this.settings.setState({ organizationProfileStatus: 'loading' })
    try {
      const selection = {
        organizationProfile: { __args: { id: organizationId }, node: organizationSelection },
      } satisfies QueryGenqlSelection
      const { organizationProfile } = await graphql(AbortSignal.any([scope.signal, request.signal])).query(selection)
      if (scope.signal.aborted || this.read !== request) return
      if (!organizationProfile) {
        this.settings.setState({ organizationProfileStatus: 'missing' })
        return
      }
      if (organizationProfile.node.id !== organizationId)
        throw new ApiError('Organization identity did not match.', 409, 'ORGANIZATION_MISMATCH')
      const organization = {
        ...this.settings.getState().organization,
        name: organizationProfile.node.name,
        logo: organizationProfile.node.avatarUrl,
      }
      this.settings.form.reset({ ...this.settings.form.values, organization, organizationUpload: null })
      this.settings.setState({
        organizationProfileStatus: 'ready',
        organizationCanUpdateProfile: organizationProfile.node.canUpdateProfile,
        organizationSavedName: organizationProfile.node.name,
      })
    } catch (error) {
      if (scope.signal.aborted || request.signal.aborted || this.read !== request) return
      this.settings.setState({
        organizationProfileStatus: error instanceof ApiError && [401, 403].includes(error.status) ? 'denied' : 'error',
      })
      notify.apiError(error, () => {
        if (!scope.signal.aborted && this.active) void this.load()
      })
    } finally {
      if (this.read === request) this.read = undefined
    }
  }

  saveProfile = async (value: { name?: string }, file: File | null, signal: AbortSignal) => {
    const scope = this.scope
    const organizationId = this.organizationId
    if (
      !this.active ||
      !organizationId ||
      this.settings.getState().organizationProfileStatus !== 'ready' ||
      !this.settings.getState().organizationCanUpdateProfile
    )
      return false as const
    const combined = AbortSignal.any([scope.signal, signal])
    const current = () => !combined.aborted && this.active && this.organizationId === organizationId
    const input = OrganizationProfileUpdateInput.parse({
      ...(value.name === undefined ? {} : { name: value.name }),
      avatarUploaded: Boolean(file),
    })
    this.settings.setState({ organizationProfileSaving: true })
    try {
      if (this.permissions)
        await requirePermissions(this.permissions, [{ action: 'organization.settings.update', resourceType: 'Organization' }])
      if (!current()) return false as const
      if (file && (this.uploaded?.file !== file || this.uploaded.organizationId !== organizationId)) {
        const uploadInput = ImageUploadInput.parse({
          size: file.size,
          mediaType: ({ 'image/jpeg': 'JPEG', 'image/png': 'PNG', 'image/webp': 'WEBP' } as Record<string, string>)[file.type],
        })
        const selection = {
          generateOrgAvatarUrl: {
            __args: { input: uploadInput },
            node: { url: true, fields: { name: true, value: true }, expiresAt: true },
          },
        } satisfies MutationGenqlSelection
        const {
          generateOrgAvatarUrl: { node },
        } = await graphql(combined).mutation(selection)
        if (!current()) return false as const
        const form = new FormData()
        node.fields.forEach(({ name, value }) => form.append(name, value))
        form.append('file', file)
        const finish = beginRequest()
        try {
          const response = await fetch(node.url, { method: 'POST', body: form, credentials: 'omit', signal: combined })
          if (!response.ok)
            throw new ApiError(
              'Organization avatar upload failed. Your profile has not been saved.',
              response.status,
              'UPLOAD_FAILED',
            )
        } finally {
          finish()
        }
        if (!current()) return false as const
        this.uploaded = { file, organizationId }
      }
      const selection = {
        organizationProfileUpdate: { __args: { input }, node: organizationSelection },
      } satisfies MutationGenqlSelection
      const {
        organizationProfileUpdate: { node },
      } = await graphql(combined).mutation(selection)
      if (!current()) return false as const
      if (node.id !== organizationId) throw new ApiError('Organization identity did not match.', 409, 'ORGANIZATION_MISMATCH')
      this.settings.form.set('organization.logo', node.avatarUrl)
      this.settings.form.accept({ organization: { ...this.settings.form.baseline.organization!, logo: node.avatarUrl } })
      this.settings.setState({ organizationSavedName: node.name })
      this.renamed(node.name)
      this.uploaded = undefined
      return { name: node.name, logo: node.avatarUrl }
    } catch (error) {
      if (!current()) return false as const
      throw error
    } finally {
      if (!scope.signal.aborted && this.scope === scope) this.settings.setState({ organizationProfileSaving: false })
    }
  }
}
