import { useEffect, useState } from 'react'
import { InvitationPage } from '@compallm/storybook'
import { InvitationControllerContext } from '@compallm/storybook/context'
import { InvitationController } from '../auth/invitation-controller'

export function Component() {
  const [controller] = useState(() => new InvitationController())
  useEffect(() => controller.start(), [controller])
  return (
    <InvitationControllerContext value={controller}>
      <InvitationPage />
    </InvitationControllerContext>
  )
}
