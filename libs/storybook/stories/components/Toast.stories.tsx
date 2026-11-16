import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { Toast, type ToastProps } from '../../src/components/feedback'
import { Button } from '../../src/components/buttons'
import { Preview } from './Preview'

function ToastDemo({ action, onDismiss, ...props }: ToastProps) {
  const [visible, setVisible] = useState(true)
  const [result, setResult] = useState('')
  return (
    <Preview>
      <section className="grid w-fit min-w-0 max-w-full grid-cols-1 justify-items-start gap-2">
        <Button
          onClick={() => {
            setVisible(true)
            setResult('')
          }}
        >
          Show notification
        </Button>
        {visible ? (
          <Toast
            {...props}
            onDismiss={() => {
              onDismiss()
              setVisible(false)
            }}
            action={
              action
                ? {
                    ...action,
                    onClick: () => {
                      action.onClick()
                      setResult(`${action.label} selected`)
                      setVisible(false)
                    },
                  }
                : undefined
            }
          />
        ) : null}
        {result ? (
          <p role="status" className="text-xsm text-muted-foreground">
            {result}
          </p>
        ) : null}
      </section>
    </Preview>
  )
}

const meta = {
  title: 'Components/Feedback & overlays/Toast',
  component: Toast,
  parameters: { layout: 'fullscreen' },
  args: {
    tone: 'info',
    title: 'Mobile notifications turned off',
    message: 'You’ll no longer receive alerts on your mobile.',
    onDismiss: () => {},
    action: { label: 'Undo', onClick: () => {} },
  },
  render: (args) => <ToastDemo {...args} />,
} satisfies Meta<typeof Toast>
export default meta
type Story = StoryObj<typeof meta>

export const Info: Story = {}
export const Error: Story = {
  args: {
    tone: 'error',
    title: 'Changes couldn’t be saved',
    message: 'Your changes are still here. Please try again.',
    action: { label: 'Try again', onClick: () => {} },
  },
}
export const Warning: Story = {
  args: {
    tone: 'warning',
    title: 'Your session expires soon',
    message: 'Stay signed in to keep working without interruption.',
    action: { label: 'Stay signed in', onClick: () => {} },
  },
}
export const Success: Story = {
  args: { tone: 'success', title: 'Changes saved', message: 'Your preferences have been updated.' },
}
export const Loading: Story = {
  args: {
    tone: 'loading',
    title: 'Saving your changes…',
    message: 'This may take a moment. You can keep working.',
    action: undefined,
  },
}
export const LongMessage: Story = {
  args: {
    tone: 'error',
    title: 'Changes couldn’t be saved',
    message:
      'Couldn’t save your changes because the connection was interrupted. ' +
      'Check your connection and try again. Your current input has been preserved.',
    action: { label: 'Try again', onClick: () => {} },
  },
}
export const MessageOnly: Story = {
  args: { title: undefined, message: 'Your changes have been saved.', action: undefined },
}
export const Narrow: Story = {
  decorators: [
    (Story) => (
      <div className="w-72 max-w-full">
        <Story />
      </div>
    ),
  ],
  args: {
    tone: 'warning',
    title: 'Your session expires soon',
    message: 'Stay signed in to keep working without interruption.',
    action: { label: 'Stay signed in', onClick: () => {} },
  },
}
