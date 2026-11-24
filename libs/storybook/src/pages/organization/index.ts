import type { OrganizationState } from './types'
export { createOrganizationStore } from './store'
export { OrganizationCatalogPage } from './catalog'
export type { OrganizationState, OrganizationController, OrganizationDestination } from './types'
export const useController = getOrganizationController
export function useOrganizationState<T>(selector: (state: OrganizationState) => T) {
  return selector(getOrganizationController().store.getState())
}
export { default as OrganizationPage } from './index-OrganizationPage.svelte'
export { default as OrganizationHeader } from './index-OrganizationHeader.svelte'
export { default as Destination } from './index-Destination.svelte'
export { default as LoadedDestinations } from './index-LoadedDestinations.svelte'
export { default as CustomizableDestinations } from './index-CustomizableDestinations.svelte'
export { default as Conversations } from './index-Conversations.svelte'
export { default as OrganizationSidebar } from './index-OrganizationSidebar.svelte'
export { default as SidebarDestinations } from './index-SidebarDestinations.svelte'
export { default as PreferenceDestinations } from './index-PreferenceDestinations.svelte'

import { getOrganizationController } from '../../context'
