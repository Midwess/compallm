import type { Meta, StoryObj } from '@storybook/svelte-vite'
import AuthDemo from './AuthDemo.svelte'
const meta = {
  title: 'Pages/signin',
  component: AuthDemo,
  parameters: { layout: 'fullscreen' },
  args: { mode: 'signin' },
} satisfies Meta<typeof AuthDemo>
export default meta
type Story = StoryObj<typeof meta>
export const Default: Story = { args: {} }
export const SendingCode: Story = {
  args: { scenario: 'pending', initial: { step: 'email', email: 'alex@example.com', pending: 'request' } },
}
export const CheckingCode: Story = {
  args: { scenario: 'pending', initial: { step: 'code', email: 'alex@example.com', pending: 'verify' } },
}
export const CheckEmail: Story = { args: { initial: { step: 'code', email: 'alex@example.com' } } }
export const IncorrectCode: Story = {
  args: { initial: { step: 'code', email: 'alex@example.com', error: 'incorrect-code' }, draft: { code: '654321' } },
}
export const ExpiredCode: Story = {
  args: {
    scenario: 'expired',
    initial: { step: 'code', email: 'alex@example.com', error: 'expired-code' },
    draft: { code: '123456' },
  },
}
export const TemporaryFailure: Story = {
  args: { scenario: 'network-error', initial: { step: 'code', email: 'alex@example.com' } },
}
export const RequestThrottled: Story = { args: { scenario: 'throttled' } }
export const Passkey: Story = { args: { initial: { step: 'passkey' } } }
export const PasskeyPrompt: Story = { args: { waiting: true, initial: { step: 'passkey', pending: 'passkey' } } }
