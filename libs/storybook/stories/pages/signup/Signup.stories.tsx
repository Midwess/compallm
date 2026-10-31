import type { Meta, StoryObj } from '@storybook/react-vite'
import { userEvent, within } from 'storybook/test'
import { AuthDemo } from '../auth-demo'
import { ControllerSubscriptions } from '../controller-subscriptions'

const meta = {
  title: 'Pages/signup',
  parameters: {
    layout: 'fullscreen',
    docs: {
      description: {
        component: 'Route: /signup. Demo only: use code 123456; registered@example.com shows the existing-account error.',
      },
    },
  },
} satisfies Meta
export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = { render: () => <AuthDemo mode="signup" /> }
export const Subscriptions: Story = { render: () => <ControllerSubscriptions /> }
export const SendingCode: Story = {
  render: () => <AuthDemo mode="signup" scenario="pending" initialState={{ step: 'email', email: 'alex@example.com' }} />,
  play: async ({ canvasElement }) => {
    await userEvent.click(within(canvasElement).getByRole('button', { name: 'Continue' }))
  },
}
export const CheckingCode: Story = {
  render: () => <AuthDemo mode="signup" scenario="pending" initialState={{ step: 'code', email: 'alex@example.com' }} />,
  play: async ({ canvasElement }) => {
    const firstDigit = within(canvasElement).getAllByRole('textbox')[0]
    await userEvent.type(firstDigit, '123456')
  },
}
export const VerifyEmail: Story = {
  render: () => <AuthDemo mode="signup" initialState={{ step: 'code', email: 'alex@example.com' }} />,
}
export const IncorrectCode: Story = {
  render: () => (
    <AuthDemo
      mode="signup"
      initialState={{ step: 'code', email: 'alex@example.com', code: '654321', error: 'incorrect-code' }}
    />
  ),
}
export const EmailAlreadyRegistered: Story = {
  render: () => (
    <AuthDemo mode="signup" initialState={{ step: 'email', email: 'registered@example.com', error: 'email-registered' }} />
  ),
}
export const ExpiredCode: Story = {
  render: () => (
    <AuthDemo
      mode="signup"
      scenario="expired"
      initialState={{ step: 'code', email: 'alex@example.com', code: '123456', error: 'expired-code' }}
    />
  ),
}
export const TemporaryFailure: Story = {
  render: () => <AuthDemo mode="signup" scenario="network-error" initialState={{ step: 'code', email: 'alex@example.com' }} />,
}
export const ResendCooldown: Story = {
  render: () => <AuthDemo mode="signup" initialState={{ step: 'code', email: 'alex@example.com', retryAfterSeconds: 28 }} />,
}
