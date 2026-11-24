import type { Meta, StoryObj } from '@storybook/svelte-vite'
import AuthDemo from './AuthDemo.svelte'
import ControllerSubscriptions from './ControllerSubscriptions.svelte'
const meta = {
  title: 'Pages/signup',
  component: AuthDemo,
  parameters: { layout: 'fullscreen' },
  args: { mode: 'signup' },
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
export const VerifyEmail: Story = { args: { initial: { step: 'code', email: 'alex@example.com' } } }
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
export const EmailAlreadyRegistered: Story = {
  args: {
    initial: {
      step: 'email',
      email: 'registered@example.com',
      error: 'email-registered',
      serverError: 'Email already registered. Sign in instead.',
    },
  },
}
export const ResendCooldown: Story = { args: { initial: { step: 'code', email: 'alex@example.com', remaining: 28 } } }

export const Subscriptions = { render: () => ({ Component: ControllerSubscriptions, props: {} }) }
