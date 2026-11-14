import { DisabledTooltip } from '../../src/components/feedback'
import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { PermissionControllerContext } from '../../src/context'
import { publishPermissions, type PermissionStatus } from '../../src/permission'
import { PermissionRequest } from '../../src/components/permission'
import { Button } from '../../src/components/buttons'
import { PreviewPermissionController } from '../permission-controller'

function Example({ status = 'allowed' }: { status?: PermissionStatus }) {
  const [controller] = useState(() => new PreviewPermissionController(() => status))
  return (
    <PermissionControllerContext value={controller}>
      <div className="grid justify-items-start gap-4 bg-background p-6 text-foreground">
        <p>Permission decision: {status}</p>
        <PermissionRequest actions={['role.create']}>
          {(allowed) => (allowed ? <Button>Create role (hidden when restricted)</Button> : null)}
        </PermissionRequest>
        <PermissionRequest actions={['role.metadata.update']} resource={{ type: 'Role', id: '12' }}>
          {(allowed) => (
            <DisabledTooltip disabled={!allowed} reason="You do not have permission to perform this action.">
              <Button disabled={!allowed}>Edit role</Button>
            </DisabledTooltip>
          )}
        </PermissionRequest>
        <PermissionRequest actions={['role.read']}>
          {(allowed) => (allowed ? <p>Role list content</p> : <p>You do not have access to roles.</p>)}
        </PermissionRequest>
      </div>
    </PermissionControllerContext>
  )
}
const meta = { title: 'Components/Permissions', component: Example } satisfies Meta<typeof Example>
export default meta
type Story = StoryObj<typeof meta>
export const Allowed: Story = {}
export const Denied: Story = { args: { status: 'denied' } }
export const Failed: Story = { args: { status: 'error' } }

function ReactiveExample() {
  const [controller] = useState(() => new PreviewPermissionController())
  const [restricted, setRestricted] = useState(false)
  const toggle = () => {
    const next = !restricted
    setRestricted(next)
    const entries = Object.fromEntries(
      Object.entries(controller.store.getState().entries).map(([key, entry]) => [
        key,
        { ...entry, status: next ? ('denied' as const) : ('allowed' as const) },
      ]),
    )
    publishPermissions(controller, entries)
  }
  return (
    <PermissionControllerContext value={controller}>
      <div className="grid justify-items-start gap-4 p-6">
        <Button onClick={toggle}>{restricted ? 'Restore fixture access' : 'Revoke fixture access'}</Button>
        <PermissionRequest actions={['role.create']}>
          {(allowed) => (
            <DisabledTooltip disabled={!allowed} reason="You do not have permission to perform this action.">
              <Button disabled={!allowed}>Create role</Button>
            </DisabledTooltip>
          )}
        </PermissionRequest>
        <PermissionRequest actions={['role.read']}>
          {(allowed) => (allowed ? <p>Visible role content</p> : null)}
        </PermissionRequest>
      </div>
    </PermissionControllerContext>
  )
}
export const Reactive: Story = { render: () => <ReactiveExample /> }
