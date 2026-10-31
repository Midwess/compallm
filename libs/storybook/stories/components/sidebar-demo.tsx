import { useState } from 'react'
import { HardDrivesIcon } from '@phosphor-icons/react/dist/csr/HardDrives'
import { ChartBarIcon } from '@phosphor-icons/react/dist/csr/ChartBar'
import { KeyIcon } from '@phosphor-icons/react/dist/csr/Key'
import { CubeIcon } from '@phosphor-icons/react/dist/csr/Cube'
import { LightningIcon } from '@phosphor-icons/react/dist/csr/Lightning'
import { BrainIcon } from '@phosphor-icons/react/dist/csr/Brain'
import { UsersIcon } from '@phosphor-icons/react/dist/csr/Users'
import { NotebookIcon } from '@phosphor-icons/react/dist/csr/Notebook'
import { PlugsIcon } from '@phosphor-icons/react/dist/csr/Plugs'
import { ChatCircleIcon } from '@phosphor-icons/react/dist/csr/ChatCircle'
import { Sidebar, SidebarHeader, SidebarSection, SidebarFolder, SidebarItem } from '../../src/components/sidebar'

export function SidebarDemo({
  defaultOpen = true,
  initialSelection = '',
}: {
  defaultOpen?: boolean
  initialSelection?: string
}) {
  const [selected, setSelected] = useState(initialSelection)
  const [organization, setOrganization] = useState('compallm')
  const organizations = [
    { id: 'compallm', name: 'Compallm' },
    { id: 'research', name: 'Research team' },
  ]
  const [action, setAction] = useState('')
  const item = (label: string, href: string, icon: React.ReactNode) => (
    <SidebarItem label={label} href={href} icon={icon} selected={selected === href} onNavigate={setSelected} />
  )
  return (
    <>
      <Sidebar>
        <SidebarHeader
          workspace={organizations.find((item) => item.id === organization)!.name}
          organizations={organizations}
          selectedOrganizationId={organization}
          onOrganizationChange={setOrganization}
          onWorkspaceClick={() => setAction('Workspace switcher')}
          onSearch={() => setAction('Workspace search')}
          onSettings={() => setAction('Settings')}
        />
        <SidebarSection
          label="Providers"
          defaultOpen={defaultOpen}
          onAdd={() => setAction('Add provider')}
          addLabel="Add provider"
        >
          {item('OpenAI', '#provider-openai', <HardDrivesIcon />)}
          {item('Anthropic', '#provider-anthropic', <HardDrivesIcon />)}
          {item('Gemini', '#provider-gemini', <HardDrivesIcon />)}
        </SidebarSection>
        <SidebarSection label="Routers" defaultOpen={defaultOpen} onAdd={() => setAction('Add router')} addLabel="Add router">
          {item('Stats', '#stats', <ChartBarIcon />)}
          {item('API key', '#api-key', <KeyIcon />)}
          <SidebarFolder
            label="Models"
            icon={<CubeIcon />}
            defaultOpen={defaultOpen}
            onAdd={() => setAction('Add model')}
            addLabel="Add model"
          >
            {item('Fast Chat', '#fast-chat', <LightningIcon />)}
            {item('claude-3.5-sonnet', '#sonnet', <BrainIcon />)}
          </SidebarFolder>
        </SidebarSection>
        <SidebarSection label="Workspace" defaultOpen={defaultOpen}>
          {item('Members', '#members', <UsersIcon />)}
          {item('Notebooks', '#notebooks', <NotebookIcon />)}
          {item('Plugins', '#plugins', <PlugsIcon />)}
        </SidebarSection>
        <SidebarSection
          label="Conversations"
          defaultOpen={defaultOpen}
          onAdd={() => setAction('New conversation')}
          addLabel="New conversation"
        >
          {item('Exploring a new idea', '#new-idea', <ChatCircleIcon />)}
          {item('Research notes', '#research', <ChatCircleIcon />)}
          {item('Model comparison', '#comparison', <ChatCircleIcon />)}
        </SidebarSection>
      </Sidebar>
      <output aria-label="Sidebar action" className="sr-only">
        {action}
      </output>
    </>
  )
}
