import { createContext } from 'react'
import type { AuthController } from './pages/auth/types'

export const AuthControllerContext = createContext<AuthController | null>(null)
