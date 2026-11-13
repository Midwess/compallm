import type { SettingsProfile } from '../types'

export interface ProfileController {
  reload?(): void
  saveProfile(
    value: Partial<Pick<SettingsProfile, 'name' | 'username'>>,
    file: File | null,
    signal: AbortSignal,
  ): false | Partial<SettingsProfile> | Promise<false | Partial<SettingsProfile>>
}
