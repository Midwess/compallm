import type { SettingsPageControllers, SettingsState } from '@compallm/storybook/pages/settings'
import type { StoreApi } from 'zustand/vanilla'
import { ProfileController } from './profile/controller'
import { OrganizationController } from './organization/controller'
import { PreferencesController } from './preferences/controller'
import { MembersController } from './members/controller'
import { DialogsController } from './dialogs/controller'
import { SearchController } from './search/controller'
import { RoleController } from './role/controller'

export function createSettingsPages(store: StoreApi<SettingsState>, organizationId?: string) {
  return {
    role: new RoleController(store, organizationId),
    search: new SearchController(store),
    preferences: new PreferencesController(store),
    profile: new ProfileController(),
    organization: new OrganizationController(),
    members: new MembersController(),
    dialogs: new DialogsController(store),
  } satisfies SettingsPageControllers
}
