import type { Meta, StoryObj } from '@storybook/react-vite'
import { SidebarDemo } from './sidebar-demo'

const meta = { title: 'Components/Navigation controls/Sidebar', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>

export const Expanded: Story = {
  render: () => (
    <div data-ui="component-preview" className="h-svh bg-background font-sans text-text">
      <SidebarDemo />
    </div>
  ),
}
export const Collapsed: Story = {
  render: () => (
    <div data-ui="component-preview" className="h-svh bg-background font-sans text-text">
      <SidebarDemo defaultOpen={false} />
    </div>
  ),
}
export const Selected: Story = {
  render: () => (
    <div data-ui="component-preview" className="h-svh bg-background font-sans text-text">
      <SidebarDemo initialSelection="#members" />
    </div>
  ),
}

export const LongWorkspaceName: Story = {
  render: () => (
    <div data-ui="component-preview" className="h-80 bg-background font-sans text-text">
      <SidebarDemo workspaceName="Research and development organization with a long workspace name" />
    </div>
  ),
}
