import type { AuthController } from '@compallm/storybook/pages/auth'
import { OrganizationProvidersStore } from '@compallm/contract/houdini'
import {
  createOrganizationStore,
  type OrganizationController as Controller,
  type OrganizationState,
} from '@compallm/storybook/pages/organization'
import type { UrlController } from '@compallm/storybook'
import find from 'lodash-es/find.js'
import type { AuthUser } from '@compallm/storybook'
import { notify } from '../notifications/toast'
import type { PreferenceDestination$options as PreferenceDestination } from '@compallm/contract/houdini'

import { graphqlClient, requireData, ApiError } from '../graphql-client'

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

  private providerRequest?: AbortController
  private providerCursor?: string
  private seeded = false
  seed(result: import('@compallm/contract/houdini').OrganizationProviders$result) {
    const organization = this.getCurrentOrganization()!
    this.seeded = true
    this.providerCursor = result.providers.pageInfo.endCursor ?? undefined
    this.store.setState({
      providers: result.providers.edges.map(({ node }) => ({
        ...node,
        href: `/${encodeURIComponent(organization.name)}/providers/${node.id}`,
      })),
      providersHasMore: result.providers.pageInfo.hasNextPage,
      providersLoading: false,
    })
  }
  start = () => {
    if (!this.seeded) void this.loadProviders()
    return this.stop
  }
  stop = () => {
    this.providerRequest?.abort()
  }
  onLoadMoreProviders = () => {
    if (this.store.getState().providersHasMore && !this.store.getState().providersLoading) void this.loadProviders(true)
  }
  onRetryProviders = () => {
    void this.loadProviders(!!this.providerCursor)
  }
  private async loadProviders(append = false) {
    this.providerRequest?.abort()
    const request = new AbortController(),
      organization = this.getCurrentOrganization()
    if (!organization) return
    this.providerRequest = request
    this.store.setState({ providersLoading: true, providersLoadError: false })
    try {
      const after = append ? this.providerCursor : undefined
      {
        const result = (
          await new OrganizationProvidersStore()
            .fetch({
              variables: { providersAfter: after },
              fetch: graphqlClient.fetch(request.signal),
              policy: 'NoCache',
              blocking: true,
            })
            .then(requireData)
        ).providers
        if (request.signal.aborted || this.getCurrentOrganization()?.id !== organization.id) return
        this.store.setState((state) => {
          const rows = new Map((append ? state.providers : []).map((value) => [value.id, value]))
          for (const { node } of result.edges)
            if (!rows.has(node.id))
              rows.set(node.id, { ...node, href: `/${encodeURIComponent(organization.name)}/providers/${node.id}` })
          return { providers: [...rows.values()], providersHasMore: result.pageInfo.hasNextPage }
        })
        this.providerCursor = result.pageInfo.endCursor ?? undefined
      }
    } catch (error) {
      if (!request.signal.aborted && (!(error instanceof ApiError) || error.status !== 403))
        this.store.setState({ providersLoadError: true })
    } finally {
      if (this.providerRequest === request) this.store.setState({ providersLoading: false })
    }
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
      links: { ...state.links, members: `${organizationRoot(user)}/settings/members` },
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
  onOpenDestination = (id: string) => {
    const state = this.store.getState()
    const destination = [...state.providers, ...state.models].find((entry) => entry.id === id)
    if (id === 'members') this.url.navigate(state.links.members)
    else if (destination) this.url.navigate(destination.href)
    else notify.info('This feature is not available yet.')
  }
  onProviderCreated = (provider: { id: string; name: string }) => {
    const organization = this.getCurrentOrganization()
    if (!organization) return
    const href = `/${encodeURIComponent(organization.name)}/providers/${provider.id}`
    this.store.setState((state) => ({
      providers: [...state.providers.filter((v) => v.id !== provider.id), { ...provider, href }],
    }))
    this.url.navigate(href)
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
