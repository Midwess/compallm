import { useContext } from 'react'
import { UserPreferenceControllerContext } from '@compallm/storybook/context'

export function useUserPreferenceController() {
  const controller = useContext(UserPreferenceControllerContext)
  if (!controller) throw new Error('Root UserPreferenceControllerContext is required.')
  return controller
}
