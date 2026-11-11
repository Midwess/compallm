import {
  createSettingsStore,
  type SettingsController as Controller,
  type SettingsDestination,
} from '@compallm/storybook/pages/settings'
import type { UrlController, AuthController } from '@compallm/storybook'
import { notify } from '../notifications/toast'
import { codeExamples } from './code-examples'

export const settingsPages = [
  'general',
  'appearance',
  'messaging',
  'code',
  'profile',
  'security',
  'api-keys',
  'danger',
  'organization',
  'members',
  'usage-limits',
  'member-limits',
  'usage-explorer',
  'audit',
] as const satisfies readonly SettingsDestination[]

export class SettingsController implements Controller {
  readonly store = createSettingsStore({ readonly: true, codeExamples })
  private readonly url: Pick<UrlController, 'navigate'>
  private readonly auth: Pick<AuthController, 'signOut'>
  private readonly base: string

  constructor(
    organizationName: string,
    url: Pick<UrlController, 'navigate'>,
    auth: Pick<AuthController, 'signOut'>,
    page: SettingsDestination,
  ) {
    this.url = url
    this.auth = auth
    this.base = `/${encodeURIComponent(organizationName)}`
    this.store.setState({ location: { page } })
  }

  setPage = (page: SettingsDestination) => this.store.setState({ location: { page } })
  navigate = (page: SettingsDestination) => {
    if (settingsPages.some((entry) => entry === page)) this.url.navigate(`${this.base}/settings/${page}`)
    else this.unavailable()
  }
  setNavigationQuery = (navigationQuery: string) => this.store.setState({ navigationQuery })
  setQuery = (query: string) => this.store.setState({ query })
  setFilter = (key: string, value: string) => {
    this.store.setState((state) => ({ filters: { ...state.filters, [key]: value } }))
  }
  private unavailable = () => {
    notify.info('Settings changes are not available yet.')
  }
  setPreference = (key: string, value: string | boolean) => {
    if (!['codeTheme', 'codeFont', 'codeFontSize', 'codeLanguage'].includes(key)) return this.unavailable()
    this.store.setState((state) => ({ preferences: { ...state.preferences, [key]: value } }))
  }
  updateProfile = this.unavailable
  saveProfile = this.unavailable
  setAvatar = this.unavailable
  updateOrganization = this.unavailable
  openDialog = this.unavailable
  updateDialog = this.unavailable
  closeDialog = () => this.store.setState({ dialog: null })
  confirmDialog = this.unavailable
  updateRole = this.unavailable
  setPolicyMode = this.unavailable
  setPolicySource = this.unavailable
  togglePolicyAction = this.unavailable
  setPolicyActions = this.unavailable
  formatPolicy = this.unavailable
  saveRole = this.unavailable
  resetRole = this.unavailable
  updateGroup = this.unavailable
  toggleGroupMember = this.unavailable
  toggleGroupRole = this.unavailable
  saveGroup = this.unavailable
  resetGroup = this.unavailable
  toggleMemberRole = this.unavailable
  copy = this.unavailable
  signOut = () => {
    void this.auth.signOut()
  }
  leaveSettings = () => this.url.navigate(`${this.base}/chat`)
  dismissNotice = () => this.store.setState({ notice: null })
}
