import type { UrlController } from './url'
import { createContext, type RefObject } from 'react'
import type { AuthController } from './pages/auth/types'
import type { OrganizationController } from './pages/organization/types'
import type { ChatController } from './pages/organization/chat/types'
import type { CreateProviderController } from './pages/organization/create-provider/types'
import type { UserPreferenceController } from './pages/settings/preferences/user/controller'
import type { SettingsController } from './pages/settings/types'
import type { FormScope } from './components/form-model'
import type { AppController } from './app'

export const AppControllerContext = createContext<AppController | null>(null)

export const FormContext = createContext<FormScope | null>(null)

export const AuthControllerContext = createContext<AuthController | null>(null)
export const OrganizationControllerContext = createContext<OrganizationController | null>(null)
export const ChatControllerContext = createContext<ChatController | null>(null)
export const CreateProviderControllerContext = createContext<CreateProviderController | null>(null)
export const UserPreferenceControllerContext = createContext<UserPreferenceController | null>(null)
export const SettingsControllerContext = createContext<SettingsController | null>(null)

export const UrlControllerContext = createContext<UrlController | null>(null)

export const FloatingContainerContext = createContext<RefObject<HTMLElement | null> | undefined>(undefined)
