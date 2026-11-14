import type { SettingsPageControllers, SettingsState, UserPreferenceController } from '@compallm/storybook/pages/settings'
import type { FormStore } from '@compallm/storybook'
import { ProfileController, type ProfileAuth } from './profile/controller'
import { OrganizationController } from './organization/controller'
import { MembersController } from './members/controller'
import { DialogsController } from './dialogs/controller'
import { SearchController } from './search/controller'
import { RoleController } from './role/controller'
import type { PermissionController } from '@compallm/storybook'
import { GroupController } from './group/controller'

export function createSettingsPages(
  store: FormStore<SettingsState>,
  roleDeleted: (id: string) => void,
  user: UserPreferenceController,
  auth: ProfileAuth = {},
  renamed: (name: string) => void = () => {},
  groupDeleted: (id: string) => void = () => {},
  permissions?: PermissionController,
) {
  return {
    group: new GroupController(store, permissions),
    role: new RoleController(store, permissions),
    search: new SearchController(store),
    preferences: { user },
    profile: new ProfileController(store, auth),
    organization: new OrganizationController(store, auth, renamed, permissions),
    members: new MembersController(),
    dialogs: new DialogsController(store, roleDeleted, groupDeleted),
  } satisfies SettingsPageControllers
}
