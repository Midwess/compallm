import { useEffect, useLayoutEffect, useState } from 'react'
import { useStore } from 'zustand'
import { Outlet, useMatch } from 'react-router'
import { OnboardingControllerContext, OrganizationControllerContext } from '@compallm/storybook/context'
import { OrganizationPage } from '@compallm/storybook/pages/organization'
import { useUrlController } from '../url/use-url-controller'
import { useAuthController } from '../auth/use-auth-controller'
import { userPreferenceController } from '../preference/user/controller'
import { OrganizationController } from '../organization/controller'
import { PermissionControllerContext } from '@compallm/storybook'
import { PermissionController } from '../permission/controller'
import { OnboardingDialog } from '@compallm/storybook/pages/onboarding'
import { OnboardingController } from '../onboarding/controller'
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
  const [onboarding] = useState(() => new OnboardingController(auth, url, permissions))
  useEffect(() => onboarding.start(), [onboarding])
  const firstPresentation = useStore(onboarding.store, (s) => s.firstPresentation)
  const visible = useStore(onboarding.store, (s) => s.visible)
  const [controller] = useState(() => new OrganizationController(user, url, auth))
  useEffect(() => controller.setCurrentUser(user), [controller, user])
  useEffect(() => userPreferenceController.connectOrganization(controller, url), [controller, url])
  return (
    <PermissionControllerContext value={permissions}>
      <OnboardingControllerContext value={onboarding}>
        <OrganizationControllerContext value={controller}>
          {settings ? (
            <Outlet />
          ) : (
            <OrganizationPage>
              <Outlet />
            </OrganizationPage>
          )}
          <OnboardingDialog
            open={visible}
            firstPresentation={firstPresentation}
            onOpenChange={(open) => {
              if (!open) onboarding.dismiss()
            }}
          />
        </OrganizationControllerContext>
      </OnboardingControllerContext>
    </PermissionControllerContext>
  )
}
