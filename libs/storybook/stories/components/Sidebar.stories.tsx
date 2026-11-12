import type { Meta, StoryObj } from '@storybook/react-vite'
import { SidebarDemo } from './sidebar-demo'
import { useState } from 'react'
import { Sidebar, SidebarSection, SidebarFolder, SidebarItem } from '../../src/components/sidebar'
import { CubeIcon } from '@phosphor-icons/react/dist/csr/Cube'

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

function NestedDisclosure() {
  const [items, setItems] = useState(['First model'])
  return (
    <div className="h-svh bg-background font-sans text-text">
      <Sidebar>
        <SidebarSection label="Workspace">
          <SidebarFolder
            label="Models"
            icon={<CubeIcon />}
            defaultOpen={false}
            onAdd={() => setItems((current) => [...current, `Model ${current.length + 1}`])}
            addLabel="Add sample model"
          >
            {items.map((label) => (
              <SidebarItem key={label} label={label} href="#" icon={<CubeIcon />} />
            ))}
          </SidebarFolder>
        </SidebarSection>
        <SidebarSection label="Following category">
          <SidebarItem label="Sample destination" href="#" icon={<CubeIcon />} />
        </SidebarSection>
      </Sidebar>
    </div>
  )
}

export const NestedDisclosureAndAdding: Story = { render: () => <NestedDisclosure /> }
