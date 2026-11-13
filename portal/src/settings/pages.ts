import type { SettingsPageControllers, SettingsState, UserPreferenceController } from '@compallm/storybook/pages/settings'
import type { FormStore } from '@compallm/storybook'
import { ProfileController, type ProfileAuth } from './profile/controller'
import { OrganizationController } from './organization/controller'
import { MembersController } from './members/controller'
import { DialogsController } from './dialogs/controller'
import { SearchController } from './search/controller'
import { RoleController } from './role/controller'

export function createSettingsPages(
  store: FormStore<SettingsState>,
  roleDeleted: (id: string) => void,
  user: UserPreferenceController,
  auth: ProfileAuth = {},
) {
  return {
    role: new RoleController(store),
    search: new SearchController(store),
    preferences: { user },
    profile: new ProfileController(store, auth),
    organization: new OrganizationController(),
    members: new MembersController(),
    dialogs: new DialogsController(store, roleDeleted),
  } satisfies SettingsPageControllers
}
