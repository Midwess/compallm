import { useContext } from 'react'
import { AuthControllerContext } from '@compallm/storybook/context'

export function useAuthController() {
  const controller = useContext(AuthControllerContext)
  if (!controller) throw new Error('Root AuthControllerContext is required.')
  return controller
}
