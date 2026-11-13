import type { SettingsOrganization } from '../types'

export interface OrganizationController {
  updateOrganization(
    value: Partial<SettingsOrganization>,
  ): void | false | Partial<SettingsOrganization> | Promise<void | false | Partial<SettingsOrganization>>
  setAvatar(
    file: File | null,
  ): void | false | Partial<SettingsOrganization> | Promise<void | false | Partial<SettingsOrganization>>
}
