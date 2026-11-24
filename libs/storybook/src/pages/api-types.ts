import type {
  AuthSession2$result,
  OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$result,
  PreferenceUserUserPreferences$result,
  SettingsRoleRole$result,
  SettingsGroupGroupAndRolesAndMembers$result,
  SettingsMembersInvitations$result,
  PasskeyPasskeys$result,
  AuthInvitationControllerInvitationContext$result,
  ProviderCatalogEffectiveProviderCatalog$result,
} from '@compallm/contract/houdini'

export type Mutable<T> = T extends readonly (infer Item)[]
  ? Mutable<Item>[]
  : T extends object
    ? { -readonly [K in keyof T]: Mutable<T[K]> }
    : T
export type Session = Mutable<AuthSession2$result['session']['node']>
export type User = Mutable<
  NonNullable<OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$result['userProfile']>['node']
>
export type UserPreferences = Mutable<PreferenceUserUserPreferences$result['userPreferences']['node']>
export type Role = Mutable<NonNullable<SettingsRoleRole$result['role']>['node']>
export type Group = Mutable<NonNullable<SettingsGroupGroupAndRolesAndMembers$result['group']>['node']>
export type Invitation = Mutable<SettingsMembersInvitations$result['invitations']['edges'][number]['node']>
export type PasskeySummary = Mutable<PasskeyPasskeys$result['passkeys']['edges'][number]['node']>
export type InvitationContext = Mutable<AuthInvitationControllerInvitationContext$result['invitationContext']['node']>
export type ProviderCatalog = Mutable<
  NonNullable<ProviderCatalogEffectiveProviderCatalog$result['effectiveProviderCatalog']>['node']
>
