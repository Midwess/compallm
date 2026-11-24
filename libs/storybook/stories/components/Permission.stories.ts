import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ComponentGallery from './PermissionDemo.svelte'
const meta = {
  title: 'Components/Permissions',
  component: ComponentGallery,
  parameters: { layout: 'fullscreen' },
} satisfies Meta<typeof ComponentGallery>
export default meta
type Story = StoryObj<typeof meta>
export const Allowed: Story = { args: { scenario: 'Allowed' } }
export const Denied: Story = { args: { scenario: 'Denied' } }
export const Failed: Story = { args: { scenario: 'Failed' } }
export const Reactive: Story = { args: { scenario: 'Reactive' } }
