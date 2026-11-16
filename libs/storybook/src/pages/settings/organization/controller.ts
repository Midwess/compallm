import type { SettingsOrganization } from '../types'

export interface OrganizationController {
  reload?(): void
  saveProfile(
    value: Partial<Pick<SettingsOrganization, 'name'>>,
    file: File | null,
    signal: AbortSignal,
  ): false | Partial<SettingsOrganization> | Promise<false | Partial<SettingsOrganization>>

  updateOrganization(
    value: Partial<SettingsOrganization>,
  ): void | false | Partial<SettingsOrganization> | Promise<void | false | Partial<SettingsOrganization>>
  setAvatar(
    file: File | null,
  ): void | false | Partial<SettingsOrganization> | Promise<void | false | Partial<SettingsOrganization>>
}
