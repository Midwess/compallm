import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { createStore } from 'zustand/vanilla'
import { InvitationPage, type InvitationController } from '../../src/pages/invitation'
import { InvitationControllerContext } from '../../src/context'

function Demo({
  step = 'continue',
  busy = false,
  error = null,
  unavailable = false,
  loading = false,
}: {
  step?: 'continue' | 'code' | 'confirmed'
  busy?: boolean
  error?: string | null
  unavailable?: boolean
  loading?: boolean
}) {
  const [controller] = useState<InvitationController>(() => {
    const store = createStore<ReturnType<InvitationController['store']['getState']>>(() => ({
      context: unavailable
        ? null
        : { organizationName: 'Research', email: 'alex@example.com', expiresAt: 2000000000, signup: true },
      loading,
      step,
      error: unavailable ? 'Invitation is expired, revoked, consumed, or unavailable.' : error,
      busy,
    }))
    return {
      store,
      continue: async () => {
        store.setState({ step: 'code' })
        return true
      },
      confirm: async () => {
        store.setState({ error: 'Incorrect verification code' })
        return false
      },
    }
  })
  return (
    <InvitationControllerContext value={controller}>
      <InvitationPage />
    </InvitationControllerContext>
  )
}
const meta = { title: 'Pages/Invitation', component: InvitationPage, parameters: { layout: 'fullscreen' } } satisfies Meta<
  typeof InvitationPage
>
export default meta
type Story = StoryObj<typeof meta>
export const Continue: Story = { render: () => <Demo /> }
export const VerifyEmail: Story = { render: () => <Demo step="code" /> }
export const Unavailable: Story = { render: () => <Demo unavailable /> }
export const Loading: Story = { render: () => <Demo loading /> }

export const IncorrectCode: Story = { render: () => <Demo step="code" error="Incorrect verification code" /> }
export const CheckingCode: Story = { render: () => <Demo step="code" busy /> }
export const Confirmed: Story = { render: () => <Demo step="confirmed" /> }
