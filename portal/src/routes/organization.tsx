import { useEffect, useLayoutEffect, useState } from 'react'
import { useStore } from 'zustand'
import { Outlet, useMatch } from 'react-router'
import { OrganizationControllerContext } from '@compallm/storybook/context'
import { OrganizationPage } from '@compallm/storybook/pages/organization'
import { useUrlController } from '../url/use-url-controller'
import { useAuthController } from '../auth/use-auth-controller'
import { userPreferenceController } from '../preference/user/controller'
import { OrganizationController } from '../organization/controller'
import { PermissionControllerContext, PermissionRecovery } from '@compallm/storybook'
import { PermissionController } from '../permission/controller'
import type { AuthUser } from '@compallm/storybook'

export function Component() {
  const auth = useAuthController()
  const user = useStore(auth.store, (state) => state.currentUser)
  if (!user) return <p role="status">Checking session…</p>
  return <Workspace key={`${user.id}:${user.userId}:${user.organizationId}`} user={user} />
}

function Workspace({ user }: { user: AuthUser }) {
  const settings = useMatch('/:organizationName/settings/*')
  const auth = useAuthController()
  const url = useUrlController()
  const [permissions] = useState(() => new PermissionController())
  useLayoutEffect(() => {
    permissions.start()
    return permissions.stop
  }, [permissions])
  const [controller] = useState(() => new OrganizationController(user, url, auth))
  useEffect(() => controller.setCurrentUser(user), [controller, user])
  useEffect(() => userPreferenceController.connectOrganization(controller, url), [controller, url])
  return (
    <PermissionControllerContext value={permissions}>
      <OrganizationControllerContext value={controller}>
        {settings ? (
          <Outlet />
        ) : (
          <OrganizationPage>
            <Outlet />
          </OrganizationPage>
        )}
      </OrganizationControllerContext>
      <PermissionRecovery />
    </PermissionControllerContext>
  )
}
