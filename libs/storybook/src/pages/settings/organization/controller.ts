import type { SettingsOrganization } from '../types'

export interface OrganizationController {
  updateOrganization(value: Partial<SettingsOrganization>): void
  setAvatar(file: File | null): void
}
