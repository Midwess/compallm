import { createContext } from 'svelte'
import type { AuthController } from './auth/controller'
import type { UrlController } from './url/controller'
import type { PasskeyController } from './passkey/controller'
import type { UserPreferenceController } from './preference/user/controller'
import type { OrganizationController } from './organization/controller'
import type { PermissionController } from './permission/controller'
import type { OnboardingController } from './onboarding/controller'
export const [getPortalControllers, setPortalControllers] = createContext<{
  auth: AuthController
  url: UrlController
  passkeys: PasskeyController
  preferences: UserPreferenceController
  permissions: PermissionController
}>()
export const [getWorkspaceControllers, setWorkspaceControllers] = createContext<{
  organization: OrganizationController
  permissions: PermissionController
  onboarding: OnboardingController
}>()
