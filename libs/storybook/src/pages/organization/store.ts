import { userPreferenceSidebarDestinations, userPreferenceDefaults } from '@compallm/contract/preference'
import { createStore } from 'zustand/vanilla'
import type { OrganizationState } from './types'

export function createOrganizationStore(initial: Partial<OrganizationState> = {}) {
  return createStore<OrganizationState>()(() => ({
    organizationId: '',
    organizations: [],
    providers: [],
    models: [],
    conversations: [],
    links: { stats: '#', keys: '#', members: '#', notebooks: '#', plugins: '#' },
    sidebarOrder: [...userPreferenceSidebarDestinations],
    sidebarLayout: userPreferenceDefaults.sidebar.layout.map((section) => ({ ...section, items: [...section.items] })),
    selectedDestination: null,
    ...initial,
  }))
}
