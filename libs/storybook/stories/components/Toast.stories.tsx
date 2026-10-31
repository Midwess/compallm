import { useState } from 'react'
import { fn } from 'storybook/test'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { Toast } from '../../src/components/feedback'
import { Button } from '../../src/components/buttons'
import { Preview } from './Preview'

function ToastDemo({ tone, message }: { tone: 'info' | 'error' | 'warning'; message: string }) {
  const [visible, setVisible] = useState(true)
  return (
    <Preview>
      <section className="grid w-[360px] max-w-full gap-2">
        <h3 className="text-[12px] text-muted">{tone}</h3>
        <Button onClick={() => setVisible(true)}>Show notification</Button>
        {visible ? <Toast tone={tone} message={message} onDismiss={() => setVisible(false)} /> : null}
      </section>
    </Preview>
  )
}

const meta = {
  title: 'Components/Feedback & overlays/Toast',
  component: Toast,
  parameters: { layout: 'fullscreen' },
  args: { tone: 'info', message: 'Your changes have been saved.', onDismiss: fn() },
  render: ({ tone = 'info', message }) => <ToastDemo tone={tone} message={message} />,
} satisfies Meta<typeof Toast>
export default meta
type Story = StoryObj<typeof meta>

export const Info: Story = {}
export const Error: Story = { args: { tone: 'error', message: 'Couldn’t save changes. Please try again.' } }
export const Warning: Story = { args: { tone: 'warning', message: 'Your session will expire soon.' } }
export const LongMessage: Story = {
  args: {
    tone: 'error',
    message:
      'Couldn’t save your changes because the connection was interrupted. ' +
      'Check your connection and try again. Your current input has been preserved.',
  },
}
