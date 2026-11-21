import { useLayoutEffect } from 'react'
import { AccessPanel } from '@compallm/storybook/pages/signin'
import { useAuthController } from '../auth/use-auth-controller'

export function Component() {
  const controller = useAuthController()
  useLayoutEffect(() => controller.start('signup'), [controller])
  return <AccessPanel intent="signup" />
}
