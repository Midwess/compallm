import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ComponentGallery from './PageHeaderDemo.svelte'
const meta = {
  title: 'Components/Navigation controls/Page header',
  component: ComponentGallery,
  parameters: { layout: 'fullscreen' },
} satisfies Meta<typeof ComponentGallery>
export default meta
type Story = StoryObj<typeof meta>
export const Default: Story = { args: { scenario: 'Default' } }
export const LongTitle: Story = { args: { scenario: 'LongTitle' } }
