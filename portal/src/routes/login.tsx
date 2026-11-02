import { useEffect } from 'react'
import { SigninPage } from '@compallm/storybook/pages/signin'
import { useAuthController } from '../auth/use-auth-controller'

export function Component() {
  const controller = useAuthController()
  useEffect(() => controller.start('signin'), [controller])
  return <SigninPage />
}
