import {
  createOrganizationStore,
  type OrganizationController as Controller,
  type OrganizationState,
} from '@compallm/storybook/pages/organization'
import type { AuthController, UrlController } from '@compallm/storybook'
import find from 'lodash-es/find'
import type { AuthUser } from '@compallm/storybook'
import { notify } from '../notifications/toast'
import type { PreferenceDestination } from '@compallm/contract/graphql'
import { organizationHome, organizationRoot } from './paths'

export class OrganizationController implements Controller {
  readonly store = createOrganizationStore()
  private readonly url: Pick<UrlController, 'navigate' | 'store'>
  private chatPath = ''
  private readonly auth: Pick<AuthController, 'signOut'>

  constructor(user: AuthUser, url: Pick<UrlController, 'navigate' | 'store'>, auth: Pick<AuthController, 'signOut'>) {
    this.url = url
    this.auth = auth
    this.setCurrentUser(user)
  }

  getCurrentOrganization = () => {
    const state = this.store.getState()
    return find(state.organizations, { id: state.organizationId }) ?? null
  }

  setCurrentUser = (user: AuthUser) => {
    const id = user.organizationId
    this.chatPath = organizationHome(user)
    const state = this.store.getState()
    if (state.organizationId === id && state.organizations[0]?.name === user.organizationName) return
    this.store.setState({
      organizationId: id,
      organizations: [{ id, name: user.organizationName }],
      ...(state.organizationId !== id ? { providers: [], models: [], conversations: [], selectedDestination: null } : {}),
    })
  }

  setSidebarLayout = (sidebarLayout: OrganizationState['sidebarLayout']) => {
    if (JSON.stringify(this.store.getState().sidebarLayout) === JSON.stringify(sidebarLayout)) return
    this.store.setState({
      sidebarLayout: sidebarLayout.map((section) => ({ ...section, items: [...section.items] })),
      sidebarOrder: sidebarLayout.flatMap((section) => section.items),
    })
  }

  setSidebarOrder = (sidebarOrder: OrganizationState['sidebarOrder']) => {
    const current = this.store.getState().sidebarOrder
    if (current.length === sidebarOrder.length && current.every((item, index) => item === sidebarOrder[index])) return
    this.store.setState({
      sidebarOrder: [...sidebarOrder],
      sidebarLayout: this.store.getState().sidebarLayout.map((section) => ({
        ...section,
        items: [...section.items].sort((a, b) => sidebarOrder.indexOf(a) - sidebarOrder.indexOf(b)),
      })),
    })
  }

  onOpenHome = (destination: PreferenceDestination) => {
    const organization = this.getCurrentOrganization()
    if (!organization) return
    const root = organizationRoot({ organizationName: organization.name })
    const pathname = this.url.store.getState().pathname.replace(/\/$/, '')
    if (pathname !== root) return
    const pages: Partial<Record<PreferenceDestination, string>> = { CHAT: 'chat', MODELS: 'models', PROVIDERS: 'providers' }
    const page = pages[destination]
    if (page) this.url.navigate(`${root}/${page}`, { replace: true })
  }

  onOpenChat = () => this.url.navigate(this.chatPath)
  onSelectOrganization = (id: string) => {
    if (id !== this.store.getState().organizationId) notify.info('Organization switching is not available yet.')
  }
  onSignOut = async () => {
    await this.auth.signOut()
  }
  onOpenSettings = () => {
    const organization = this.getCurrentOrganization()
    if (organization) this.url.navigate(`/${encodeURIComponent(organization.name)}/settings/general`)
  }
  onSearch = () => {
    notify.info('Workspace search is not available yet.')
  }
  onOpenDestination = () => {
    notify.info('This feature is not available yet.')
  }
  onCreateProvider = () => {
    const organization = this.getCurrentOrganization()
    if (organization) this.url.navigate(`/${encodeURIComponent(organization.name)}/providers/new/choose-provider`)
  }
  onCreateRouter = () => {
    notify.info('Router setup is not available yet.')
  }
  onCreateModel = () => {
    notify.info('Model setup is not available yet.')
  }
}
