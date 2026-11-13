import type { SettingsProfile } from '../types'

export interface ProfileController {
  updateProfile(
    value: Partial<SettingsProfile>,
  ): void | false | Partial<SettingsProfile> | Promise<void | false | Partial<SettingsProfile>>
  setAvatar(file: File | null): void | false | Partial<SettingsProfile> | Promise<void | false | Partial<SettingsProfile>>
}
