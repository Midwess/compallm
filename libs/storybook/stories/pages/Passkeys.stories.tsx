import { DemoAuthController } from './demo-auth-controller'
import { AuthVerificationHost } from '../../src/pages/auth/access-panel'
import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { createStore } from 'zustand/vanilla'
import { AuthControllerContext, PasskeyControllerContext } from '../../src/context'
import {
  SecurityEnhancementPage,
  PasskeySettings,
  PasskeyDialogs,
  type PasskeyController,
  type PasskeyState,
} from '../../src/pages/passkey'
const meta = { title: 'Pages/Passkeys', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
function Preview({ screen = false, initial = {} }: { screen?: boolean; initial?: Partial<PasskeyState> }) {
  const [controller] = useState<PasskeyController>(() => {
    const store = createStore<PasskeyState>(() => ({
      records: [],
      enrollment: 'name',
      editingId: null,
      currentPasskeyId: null,
      status: 'ready',
      dialog: null,
      busy: false,
      error: null,
      reauthentication: false,
      emailSent: false,
      ...initial,
    }))
    return {
      store,
      load: async () => {
        store.setState({ status: 'ready', error: null })
      },
      openAdd: () => store.setState({ dialog: { kind: 'add' } }),
      openRemove: (id) => store.setState({ dialog: { kind: 'remove', id } }),
      close: () => store.setState({ dialog: null }),
      prepare: async () => {
        store.setState({ enrollment: 'name' })
        return true
      },
      edit: (id) => store.setState({ editingId: id }),
      rename: async (id, name) => {
        store.setState((state) => ({ records: state.records.map((p) => (p.id === id ? { ...p, name } : p)), editingId: null }))
        return true
      },
      add: async () => false,
      remove: async () => false,
    }
  })
  const [auth] = useState(() => {
    const result = new DemoAuthController('signin')
    Object.assign(result, { requestVerification: async () => false })
    result.store.setState({ verificationActive: initial.reauthentication, step: initial.emailSent ? 'code' : 'choose' })
    return result
  })
  return (
    <AuthControllerContext value={auth}>
      <PasskeyControllerContext value={controller}>
        {screen ? (
          <SecurityEnhancementPage onSkip={() => {}} />
        ) : (
          <main className="min-h-dvh bg-surface p-8 text-foreground">
            <div className="mx-auto max-w-160">
              <PasskeySettings />
            </div>
          </main>
        )}
        <PasskeyDialogs />
        <AuthVerificationHost />
      </PasskeyControllerContext>
    </AuthControllerContext>
  )
}
export const SecurityStep: Story = { render: () => <Preview screen /> }
export const Empty: Story = { render: () => <Preview /> }
export const Loading: Story = { render: () => <Preview initial={{ status: 'loading' }} /> }
export const LoadError: Story = {
  render: () => <Preview initial={{ status: 'error', error: 'Passkeys could not be loaded. Try again.' }} />,
}
export const Registered: Story = {
  render: () => (
    <Preview initial={{ records: [{ id: '42', name: 'Personal laptop', registeredAt: '1794838000', lastUsedAt: null }] }} />
  ),
}
export const AddDialog: Story = { render: () => <Preview screen initial={{ dialog: { kind: 'add' } }} /> }
export const Reauthentication: Story = {
  render: () => <Preview initial={{ dialog: { kind: 'add' }, reauthentication: true }} />,
}
export const EmailVerification: Story = {
  render: () => <Preview initial={{ dialog: { kind: 'add' }, reauthentication: true, emailSent: true }} />,
}
export const Cancelled: Story = {
  render: () => (
    <Preview
      screen
      initial={{
        dialog: { kind: 'add' },
        enrollment: 'device',
        error: 'Passkey verification was cancelled. Try again or skip for now.',
      }}
    />
  ),
}

export const Rename: Story = {
  render: () => (
    <Preview
      initial={{ records: [{ id: '42', name: 'Macbook', registeredAt: '1794838000', lastUsedAt: null }], editingId: '42' }}
    />
  ),
}
export const Revoke: Story = {
  render: () => (
    <Preview
      initial={{
        records: [{ id: '42', name: 'Macbook', registeredAt: '1794838000', lastUsedAt: null }],
        dialog: { kind: 'remove', id: '42' },
      }}
    />
  ),
}
export const CurrentSession: Story = {
  render: () => (
    <Preview
      initial={{
        records: [{ id: '42', name: 'Macbook', registeredAt: '1794838000', lastUsedAt: null }],
        currentPasskeyId: '42',
      }}
    />
  ),
}
