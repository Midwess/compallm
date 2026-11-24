import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ToastDemo from './ToastDemo.svelte'
const meta = {
  title: 'Components/Feedback & overlays/Toast',
  component: ToastDemo,
  parameters: { layout: 'fullscreen' },
  args: {
    tone: 'info',
    title: 'Mobile notifications turned off',
    message: 'You’ll no longer receive alerts on your mobile.',
    onDismiss: () => {},
    action: { label: 'Undo', onClick: () => {} },
  },
} satisfies Meta<typeof ToastDemo>
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
      'Couldn’t save your changes because the connection was interrupted. Check your connection and try again. Your current input has been preserved.',
    action: { label: 'Try again', onClick: () => {} },
  },
}
export const MessageOnly: Story = { args: { title: undefined, message: 'Your changes have been saved.', action: undefined } }
export const Narrow: Story = {
  args: {
    tone: 'warning',
    title: 'Your session expires soon',
    message: 'Stay signed in to keep working without interruption.',
  },
}
