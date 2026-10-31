import { AuthControllerContext } from '@compallm/storybook/context'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router'
import { SigninPage } from '@compallm/storybook/pages/signin'
import { AuthController } from '../auth/controller'

export function Component() {
  const navigate = useNavigate()
  const [controller] = useState(
    () =>
      new AuthController('signin', (href) => {
        void navigate(href)
      }),
  )
  useEffect(() => controller.start(), [controller])
  return (
    <AuthControllerContext value={controller}>
      <SigninPage />
    </AuthControllerContext>
  )
}
