import type { Meta, StoryObj } from '@storybook/svelte-vite'
import AuthDemo from './AuthDemo.svelte'
const meta = {
  title: 'Pages/Authentication',
  component: AuthDemo,
  parameters: {
    layout: 'fullscreen',
    docs: {
      description: {
        component:
          'Offline authentication and verification previews. Use 123456 to verify email. No email, credentials, or API requests are sent.',
      },
    },
  },
  args: { passkeySucceeds: true },
} satisfies Meta<typeof AuthDemo>
export default meta
type Story = StoryObj<typeof meta>
export const Login: Story = {}
export const LoginLight: Story = { globals: { theme: 'basic-white' } }
export const LoginMobile: Story = { globals: { viewport: { value: 'mobile2', isRotated: false } } }
export const ReturningWithEmail: Story = { args: { initial: { lastLoginMethod: 'email' } } }
export const ReturningWithPasskey: Story = { args: { initial: { lastLoginMethod: 'passkey' } } }
export const EmailEntry: Story = { args: { initial: { step: 'email' } } }
export const EmailCode: Story = { args: { initial: { step: 'code', email: 'alex@example.com', remaining: 24 } } }
export const WaitingForPasskey: Story = {
  args: { waiting: true, initial: { step: 'passkey', pending: 'passkey', lastLoginMethod: 'passkey' } },
}
export const PasskeyCancelled: Story = {
  args: {
    initial: {
      lastLoginMethod: 'passkey',
      serverError: 'Passkey verification was cancelled. Try again or continue with email.',
    },
  },
}
export const VerifyIdentity: Story = {
  args: { popup: true, initial: { lastLoginMethod: 'passkey', email: 'owner@compallm.com' } },
}
export const VerifyWithEmail: Story = {
  args: { popup: true, initial: { step: 'code', email: 'owner@compallm.com', remaining: 24 } },
}
export const Verified: Story = { args: { popup: true, initial: { step: 'complete', lastLoginMethod: 'email' } } }
export const Signup: Story = { args: { mode: 'signup' } }
export const SignupEmailCode: Story = {
  args: { mode: 'signup', initial: { step: 'code', email: 'alex@example.com', remaining: 24 } },
}
