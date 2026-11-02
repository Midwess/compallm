import type { StoreApi } from 'zustand/vanilla'

export interface OrganizationDestination {
  id: string
  name: string
  href: string
}

export interface OrganizationState {
  organizationId: string
  organizations: { id: string; name: string }[]
  providers: OrganizationDestination[]
  models: OrganizationDestination[]
  conversations: { id: string; name: string; href: string }[]
  links: { stats: string; keys: string; members: string; notebooks: string; plugins: string }
  selectedDestination: string | null
}

export interface OrganizationController {
  readonly store: Pick<StoreApi<OrganizationState>, 'getState' | 'getInitialState' | 'subscribe'>
  getCurrentOrganization(): OrganizationState['organizations'][number] | null
  onOpenChat(conversationId?: string): void
  onSelectOrganization(id: string): void
  onSignOut(): void
  onOpenSettings(): void
  onSearch(): void
  onOpenDestination(id: string): void
  onCreateProvider(): void
  onCreateRouter(): void
  onCreateModel(): void
}
