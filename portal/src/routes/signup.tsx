import { useEffect } from 'react'
import { SignupPage } from '@compallm/storybook/pages/signup'
import { useAuthController } from '../auth/use-auth-controller'

export function Component() {
  const controller = useAuthController()
  useEffect(() => controller.start('signup'), [controller])
  return <SignupPage />
}
