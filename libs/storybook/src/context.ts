import { createContext } from 'svelte'
import type { AuthController } from './pages/auth/types'
import type { FormModel } from './components/form-model.svelte'
import type { CollectionForms } from './components/collection-forms'
import type { PermissionController } from './permission'
export const [getFormContext, setFormContext] = createContext<FormModel<object> | null>()
export const [getFieldContext, setFieldContext] = createContext<string>()
export const [getSectionContext, setSectionContext] = createContext<string>()
export const [getFloatingContainer, setFloatingContainer] = createContext<() => HTMLElement | null>()
export const [getLoadingContext, setLoadingContext] = createContext<{
  readonly initial: boolean
  readonly visible: boolean
  readonly refreshing: boolean
}>()
export const [getCollectionContext, setCollectionContext] = createContext<{
  forms: CollectionForms
  readonly scrollElement: HTMLElement | null
}>()
export const [getCollectionRow, setCollectionRow] = createContext<{ key: string; index: number; forms: CollectionForms }>()
export const [getPermissionController, setPermissionController] = createContext<PermissionController>()
export const [getCodeAppearance, setCodeAppearance] =
  createContext<() => { theme: 'dark' | 'light'; fontSize: number; fontFamily: string }>()

export const [getAuthController, setAuthController] = createContext<AuthController>()

import type { SettingsController } from './pages/settings/types'
export const [getSettingsController, setSettingsController] = createContext<SettingsController>()

import type { UserPreferenceController } from './pages/settings/preferences/user/controller'
export const [getUserPreferenceController, setUserPreferenceController] = createContext<UserPreferenceController>()

import type { OnboardingController } from './pages/onboarding/types'
export const [getOnboardingController, setOnboardingController] = createContext<OnboardingController>()

import type { CreateProviderController } from './pages/organization/create-provider/types'
export const [getCreateProviderController, setCreateProviderController] = createContext<CreateProviderController>()

import type { OrganizationController } from './pages/organization/types'
export const [getOrganizationController, setOrganizationController] = createContext<OrganizationController>()

import type { PasskeyController } from './pages/passkey/types'
export const [getPasskeyController, setPasskeyController] = createContext<PasskeyController>()

import type { InvitationController } from './pages/invitation'
export const [getInvitationController, setInvitationController] = createContext<InvitationController>()

import type { UrlController } from './url'
export const [getUrlController, setUrlController] = createContext<UrlController>()

import type { ChatController } from './pages/organization/chat/types'
export const [getChatController, setChatController] = createContext<ChatController>()
