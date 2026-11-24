import type { UserPreferences } from '../api-types'
import type { StateStore } from '../../state.svelte'

export interface OrganizationDestination {
  id: string
  name: string
  href: string
}

export interface OrganizationState {
  organizationId: string
  organizations: { id: string; name: string }[]
  providers: OrganizationDestination[]
  providersLoading?: boolean
  providersHasMore?: boolean
  providersLoadError?: boolean
  models: OrganizationDestination[]
  conversations: { id: string; name: string; href: string }[]
  links: { stats: string; keys: string; members: string; notebooks: string; plugins: string }
  sidebarOrder: UserPreferences['sidebar']['entries'][number]['item'][]
  sidebarLayout: {
    category: UserPreferences['sidebar']['layout'][number]['category']
    items: UserPreferences['sidebar']['entries'][number]['item'][]
  }[]
  selectedDestination: string | null
}

export interface OrganizationController {
  readonly store: Pick<StateStore<OrganizationState>, 'getState' | 'getInitialState' | 'subscribe'>
  getCurrentOrganization(): OrganizationState['organizations'][number] | null
  onOpenChat(conversationId?: string): void
  onSelectOrganization(id: string): void
  onSignOut(): void
  onOpenSettings(): void
  onSearch(): void
  onOpenDestination(id: string): void
  onProviderCreated?(provider: { id: string; name: string }): void
  onLoadMoreProviders?(): void
  onRetryProviders?(): void
  onCreateProvider(): void
  onCreateRouter(): void
  onCreateModel(): void
}
