import type { Meta, StoryObj } from '@storybook/svelte-vite'
import Preview from './Passkeys.stories-Preview.svelte'
const meta = { title: 'Pages/Passkeys', parameters: { layout: 'fullscreen' }, component: Preview } satisfies Meta<
  typeof Preview
>
export default meta
type Story = StoryObj<typeof meta>
export const SecurityStep: Story = { args: { screen: true } }
export const Empty: Story = { args: {} }
export const Loading: Story = { args: { initial: { status: 'loading' } } }
export const LoadError: Story = { args: { initial: { status: 'error', error: 'Passkeys could not be loaded. Try again.' } } }
export const Registered: Story = {
  args: { initial: { records: [{ id: '42', name: 'Personal laptop', registeredAt: '1794838000', lastUsedAt: null }] } },
}
export const AddDialog: Story = { args: { screen: true, initial: { dialog: { kind: 'add' } } } }
export const Reauthentication: Story = { args: { initial: { dialog: { kind: 'add' }, reauthentication: true } } }
export const EmailVerification: Story = {
  args: { initial: { dialog: { kind: 'add' }, reauthentication: true, emailSent: true } },
}
export const Cancelled: Story = {
  args: {
    screen: true,
    initial: {
      dialog: { kind: 'add' },
      enrollment: 'device',
      error: 'Passkey verification was cancelled. Try again or skip for now.',
    },
  },
}
export const Rename: Story = {
  args: {
    initial: { records: [{ id: '42', name: 'Macbook', registeredAt: '1794838000', lastUsedAt: null }], editingId: '42' },
  },
}
export const Revoke: Story = {
  args: {
    initial: {
      records: [{ id: '42', name: 'Macbook', registeredAt: '1794838000', lastUsedAt: null }],
      dialog: { kind: 'remove', id: '42' },
    },
  },
}
export const CurrentSession: Story = {
  args: {
    initial: {
      records: [{ id: '42', name: 'Macbook', registeredAt: '1794838000', lastUsedAt: null }],
      currentPasskeyId: '42',
    },
  },
}
