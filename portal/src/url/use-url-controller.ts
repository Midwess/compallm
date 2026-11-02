import { useContext } from 'react'
import { UrlControllerContext } from '@compallm/storybook/context'
import { UrlController } from './controller'
export function useUrlController() {
  const controller = useContext(UrlControllerContext)
  if (!(controller instanceof UrlController)) throw new Error('Portal UrlControllerContext is required.')
  return controller
}
