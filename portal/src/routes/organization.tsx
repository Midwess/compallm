import { useEffect, useState } from 'react'
import { useStore } from 'zustand'
import { Outlet, useNavigate } from 'react-router'
import { OrganizationControllerContext } from '@compallm/storybook/context'
import { OrganizationPage } from '@compallm/storybook/pages/organization'
import { useAuthController } from '../auth/use-auth-controller'
import { OrganizationController } from '../organization/controller'
import type { CurrentUserResponse } from '../contract/api'

export function Component() {
  const auth = useAuthController()
  const user = useStore(auth.store, (state) => state.currentUser)
  if (!user) return <p role="status">Checking session…</p>
  return <Workspace key={user.organizationId} user={user} />
}

function Workspace({ user }: { user: CurrentUserResponse }) {
  const auth = useAuthController()
  const navigate = useNavigate()
  const [controller] = useState(
    () =>
      new OrganizationController(
        user,
        (path) => {
          void navigate(path)
        },
        auth,
      ),
  )
  useEffect(() => controller.setCurrentUser(user), [controller, user])
  return (
    <OrganizationControllerContext value={controller}>
      <OrganizationPage>
        <Outlet />
      </OrganizationPage>
    </OrganizationControllerContext>
  )
}
