import type { Meta, StoryObj } from '@storybook/react-vite'
import { MotionConfig } from 'motion/react'
import { useState } from 'react'
import { AppLoadingOverlay } from '../../src/components/app-loading-overlay'

const meta = {
  title: 'Components/Feedback & overlays/App loading overlay',
  component: AppLoadingOverlay,
  parameters: { layout: 'fullscreen' },
  args: { active: true },
} satisfies Meta<typeof AppLoadingOverlay>
export default meta
type Story = StoryObj<typeof meta>

export const Loading: Story = {}
export const ReducedMotion: Story = {
  render: (args) => (
    <MotionConfig reducedMotion="always">
      <AppLoadingOverlay {...args} />
    </MotionConfig>
  ),
}
function RetryPreview() {
  const [error, setError] = useState<string | undefined>('Could not load your preferences.')
  return <AppLoadingOverlay active error={error} onRetry={() => setError(undefined)} />
}
export const Failure: Story = { render: () => <RetryPreview /> }
