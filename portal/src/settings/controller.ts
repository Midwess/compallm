import {
  createSettingsStore,
  type SettingsController as Controller,
  type SettingsDestination,
} from '@compallm/storybook/pages/settings'
import type { UrlController, AuthController } from '@compallm/storybook'
import { notify } from '../notifications/toast'
import { codeExamples } from './code-examples'
import { RoleController } from './role/controller'
import { GroupController } from './group/controller'
import { graphql } from '../graphql-client'
import type { QueryGenqlSelection } from '@compallm/contract/graphql'
import { createSettingsPages } from './pages'

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
  'new-role',
  'new-group',
  'usage-limits',
  'member-limits',
  'usage-explorer',
  'audit',
] as const satisfies readonly SettingsDestination[]

export class SettingsController implements Controller {
  readonly store = createSettingsStore({ readonly: true, codeExamples })
  readonly role = new RoleController()
  readonly group = new GroupController()
  readonly pages = createSettingsPages(this.store)
  private readonly url: Pick<UrlController, 'navigate'>
  private readonly auth: Pick<AuthController, 'signOut'>
  private readonly base: string
  private request?: AbortController
  private generation = 0

  start = () => {
    const controller = new AbortController()
    this.request?.abort()
    this.request = controller
    const generation = ++this.generation
    this.store.setState({ roles: [], groups: [] })
    void this.loadPermissions(controller.signal, generation)
    return () => {
      controller.abort()
      if (this.request === controller) this.request = undefined
      this.generation += 1
    }
  }

  private async loadPermissions(signal: AbortSignal, generation: number) {
    try {
      const selection = {
        roles: { edges: { node: { id: true, name: true, description: true, color: true } } },
        groups: { edges: { node: { id: true, name: true, description: true } } },
      } satisfies QueryGenqlSelection
      const { roles, groups } = await graphql(signal).query(selection)
      if (signal.aborted || generation !== this.generation) return
      this.store.setState({
        roles: roles.edges.map(({ node: role }) => ({ ...role, source: '' })),
        groups: groups.edges.map(({ node: group }) => ({ ...group, roleIds: [], memberIds: [], color: '' })),
      })
    } catch (error) {
      if (!signal.aborted && generation === this.generation) notify.error(error)
    }
  }

  constructor(
    organizationName: string,
    url: Pick<UrlController, 'navigate'>,
    auth: Pick<AuthController, 'signOut'>,
    page: SettingsDestination,
  ) {
    this.url = url
    this.auth = auth
    this.base = `/${encodeURIComponent(organizationName)}`
    this.setPage(page)
  }

  setPage = (page: SettingsDestination) => {
    this.store.setState({ location: { page } })
    if (page === 'new-role') this.role.open()
    if (page === 'new-group') this.group.open()
  }
  navigate = (page: SettingsDestination) => {
    if (settingsPages.some((entry) => entry === page)) this.url.navigate(`${this.base}/settings/${page}`)
    else this.unavailable()
  }
  setNavigationQuery = (navigationQuery: string) => this.store.setState({ navigationQuery })
  private unavailable = () => {
    notify.info('Settings changes are not available yet.')
  }
  signOut = () => {
    void this.auth.signOut()
  }
  leaveSettings = () => this.url.navigate(`${this.base}/chat`)
  dismissNotice = () => this.store.setState({ notice: null })
}
