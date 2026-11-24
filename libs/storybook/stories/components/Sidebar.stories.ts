import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ComponentGallery from './SidebarDemo.svelte'
const meta = {
  title: 'Components/Navigation controls/Sidebar',
  component: ComponentGallery,
  parameters: { layout: 'fullscreen' },
} satisfies Meta<typeof ComponentGallery>
export default meta
type Story = StoryObj<typeof meta>
export const Expanded: Story = { args: { scenario: 'Expanded' } }
export const Collapsed: Story = { args: { scenario: 'Collapsed' } }
export const Selected: Story = { args: { scenario: 'Selected' } }
export const LongWorkspaceName: Story = { args: { scenario: 'LongWorkspaceName' } }
export const NestedDisclosureAndAdding: Story = { args: { scenario: 'NestedDisclosureAndAdding' } }
export const SettingsNavigation: Story = { args: { scenario: 'SettingsNavigation' } }
export const Resizable: Story = { args: { scenario: 'Resizable' } }
