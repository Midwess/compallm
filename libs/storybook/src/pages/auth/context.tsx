import { createContext, useContext, type ReactNode } from 'react'
import { useStore } from 'zustand'
import type { AuthController, AuthState } from './types'

const ControllerContext = createContext<AuthController | null>(null)

export function AuthControllerProvider({ controller, children }: { controller: AuthController; children: ReactNode }) {
  return <ControllerContext value={controller}>{children}</ControllerContext>
}

export function useAuthController() {
  const controller = useContext(ControllerContext)
  if (!controller) throw new Error('Authentication UI requires an AuthControllerProvider.')
  return controller
}

export function useAuthState<T>(selector: (state: AuthState) => T): T {
  return useStore(useAuthController().store, selector)
}
