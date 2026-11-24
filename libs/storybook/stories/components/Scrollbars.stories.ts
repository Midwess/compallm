import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ComponentGallery from './ScrollbarsDemo.svelte'
const meta = {
  title: 'Components/Scrollbars',
  component: ComponentGallery,
  parameters: { layout: 'fullscreen' },
} satisfies Meta<typeof ComponentGallery>
export default meta
type Story = StoryObj<typeof meta>
export const Universal: Story = {}
