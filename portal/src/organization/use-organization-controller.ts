import { useContext } from 'react'
import { OrganizationControllerContext } from '@compallm/storybook/context'

export function useOrganizationController() {
  const controller = useContext(OrganizationControllerContext)
  if (!controller) throw new Error('OrganizationControllerContext is required within an organization route.')
  return controller
}
