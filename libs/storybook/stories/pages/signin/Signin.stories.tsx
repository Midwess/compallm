import type { Meta, StoryObj } from '@storybook/react-vite'
import { userEvent, within } from 'storybook/test'
import { AuthDemo } from '../auth-demo'

const meta = {
  title: 'Pages/signin',
  parameters: {
    layout: 'fullscreen',
    docs: {
      description: { component: 'Route: /signin. Preview only; no email or API calls. Use 123456 to complete the demo.' },
    },
  },
} satisfies Meta
export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = { render: () => <AuthDemo mode="signin" /> }
export const SendingCode: Story = {
  render: () => <AuthDemo mode="signin" scenario="pending" initialState={{ step: 'email', email: 'alex@example.com' }} />,
  play: async ({ canvasElement }) => {
    await userEvent.click(within(canvasElement).getByRole('button', { name: 'Continue' }))
  },
}
export const CheckingCode: Story = {
  render: () => <AuthDemo mode="signin" scenario="pending" initialState={{ step: 'code', email: 'alex@example.com' }} />,
  play: async ({ canvasElement }) => {
    await userEvent.type(within(canvasElement).getAllByRole('textbox')[0], '123456')
  },
}
export const CheckEmail: Story = {
  render: () => <AuthDemo mode="signin" initialState={{ step: 'code', email: 'alex@example.com' }} />,
}
export const IncorrectCode: Story = {
  render: () => (
    <AuthDemo
      mode="signin"
      initialState={{ step: 'code', email: 'alex@example.com', code: '654321', error: 'incorrect-code' }}
    />
  ),
}
export const ExpiredCode: Story = {
  render: () => (
    <AuthDemo
      mode="signin"
      scenario="expired"
      initialState={{ step: 'code', email: 'alex@example.com', code: '123456', error: 'expired-code' }}
    />
  ),
}
export const TemporaryFailure: Story = {
  render: () => <AuthDemo mode="signin" scenario="network-error" initialState={{ step: 'code', email: 'alex@example.com' }} />,
}
export const RequestThrottled: Story = { render: () => <AuthDemo mode="signin" scenario="throttled" /> }
