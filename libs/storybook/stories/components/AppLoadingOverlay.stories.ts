import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ComponentGallery from './OverlayDemo.svelte'
const meta = {
  title: 'Components/Feedback & overlays/App loading overlay',
  component: ComponentGallery,
  parameters: { layout: 'fullscreen' },
} satisfies Meta<typeof ComponentGallery>
export default meta
type Story = StoryObj<typeof meta>
export const Loading: Story = { args: { scenario: 'Loading' } }
export const ReducedMotion: Story = { args: { scenario: 'ReducedMotion' } }
export const Failure: Story = { args: { scenario: 'Failure' } }
