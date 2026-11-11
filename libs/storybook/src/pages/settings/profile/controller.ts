import type { SettingsProfile } from '../types'

export interface ProfileController {
  updateProfile(value: Partial<SettingsProfile>): void
  setAvatar(file: File | null): void
}
