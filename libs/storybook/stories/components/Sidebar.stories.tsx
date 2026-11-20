import type { Meta, StoryObj } from '@storybook/react-vite'
import { SidebarDemo } from './sidebar-demo'
import { useState } from 'react'
import { Sidebar, SidebarSection, SidebarFolder, SidebarItem } from '../../src/components/sidebar'
import { CubeIcon } from '@phosphor-icons/react/dist/csr/Cube'
import { Button } from '../../src/components/buttons'

const meta = { title: 'Components/Navigation controls/Sidebar', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>

export const Expanded: Story = {
  render: () => (
    <div data-ui="component-preview" className="h-svh bg-background font-sans text-foreground">
      <SidebarDemo />
    </div>
  ),
}
export const Collapsed: Story = {
  render: () => (
    <div data-ui="component-preview" className="h-svh bg-background font-sans text-foreground">
      <SidebarDemo defaultOpen={false} />
    </div>
  ),
}
export const Selected: Story = {
  render: () => (
    <div data-ui="component-preview" className="h-svh bg-background font-sans text-foreground">
      <SidebarDemo initialSelection="#members" />
    </div>
  ),
}

export const LongWorkspaceName: Story = {
  render: () => (
    <div data-ui="component-preview" className="h-80 bg-background font-sans text-foreground">
      <SidebarDemo workspaceName="Research and development organization with a long workspace name" />
    </div>
  ),
}

function NestedDisclosure() {
  const [items, setItems] = useState(['First model'])
  return (
    <div className="h-svh bg-background font-sans text-foreground">
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

function SettingsNavigationDemo() {
  const [settings, setSettings] = useState(false)
  return (
    <div data-ui="component-preview" className="flex h-svh bg-background font-sans text-foreground">
      {!settings ? <SidebarDemo onSettings={() => setSettings(true)} /> : null}
      <main className="min-w-0 flex-1 p-6">
        <h1 className="mb-4 text-lg font-medium">{settings ? 'Settings' : 'Workspace'}</h1>
        {settings ? (
          <Button onClick={() => setSettings(false)}>Back to workspace</Button>
        ) : (
          <p className="text-xs text-muted-foreground">
            Open the workspace menu, hover Switch organization, then select Settings.
          </p>
        )}
      </main>
    </div>
  )
}

export const SettingsNavigation: Story = { render: () => <SettingsNavigationDemo /> }

function ResizableSidebarDemo() {
  const [width, setWidth] = useState(244)
  const [feedback, setFeedback] = useState('')
  return (
    <div className="flex h-svh bg-background font-sans text-foreground">
      <Sidebar width={width} onWidthChange={setWidth} onResizeFeedback={(kind, next) => setFeedback(`${kind}: ${next}px`)}>
        <SidebarSection label="Workspace">
          <SidebarItem label="Models" href="#" icon={<CubeIcon />} />
        </SidebarSection>
      </Sidebar>
      <div className="min-w-0 flex-1 p-6">
        <h1 className="text-lg">Resizable sidebar</h1>
        <p className="mt-3 text-xs text-muted-foreground">
          Drag the divider or focus it and use Left/Right. Double-click to reset.
        </p>
        <output className="mt-3 block text-xs">
          {width}px · {feedback}
        </output>
      </div>
    </div>
  )
}
export const Resizable: Story = { render: () => <ResizableSidebarDemo /> }
