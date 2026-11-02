import { createContext } from 'react'
import type { AuthController } from './pages/auth/types'
import type { OrganizationController } from './pages/organization/types'
import type { ChatController } from './pages/organization/chat/types'
import type { CreateProviderController } from './pages/organization/create-provider/types'

export const AuthControllerContext = createContext<AuthController | null>(null)
export const OrganizationControllerContext = createContext<OrganizationController | null>(null)
export const ChatControllerContext = createContext<ChatController | null>(null)
export const CreateProviderControllerContext = createContext<CreateProviderController | null>(null)
