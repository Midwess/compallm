import { AuthControllerContext } from '@compallm/storybook/context'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router'
import { SignupPage } from '@compallm/storybook/pages/signup'
import { AuthController } from '../auth/controller'

export function Component() {
  const navigate = useNavigate()
  const [controller] = useState(
    () =>
      new AuthController('signup', (href) => {
        void navigate(href)
      }),
  )
  useEffect(() => controller.start(), [controller])
  return (
    <AuthControllerContext value={controller}>
      <SignupPage />
    </AuthControllerContext>
  )
}
