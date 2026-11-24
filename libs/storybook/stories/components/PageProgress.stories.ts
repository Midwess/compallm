import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ComponentGallery from './ProgressDemo.svelte'
const meta = {
  title: 'Components/Feedback & overlays/Page progress',
  component: ComponentGallery,
  parameters: { layout: 'fullscreen' },
} satisfies Meta<typeof ComponentGallery>
export default meta
type Story = StoryObj<typeof meta>
export const Requests: Story = { args: { scenario: 'Requests' } }
export const Loading: Story = { args: { scenario: 'Loading' } }
export const Idle: Story = { args: { scenario: 'Idle' } }
export const AboveDialog: Story = { args: { scenario: 'AboveDialog' } }
export const Light: Story = { args: { scenario: 'Light' }, globals: { theme: 'basic-white' } }
