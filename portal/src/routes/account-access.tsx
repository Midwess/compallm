import { Outlet } from 'react-router'
import { AccessLayout } from '@compallm/storybook/pages/signin'

export function Component() {
  return (
    <AccessLayout>
      <Outlet />
    </AccessLayout>
  )
}
