import { useContext, type ReactNode } from 'react'
import { useStore } from 'zustand'
import { HardDrivesIcon } from '@phosphor-icons/react/dist/csr/HardDrives'
import { ChartBarIcon } from '@phosphor-icons/react/dist/csr/ChartBar'
import { KeyIcon } from '@phosphor-icons/react/dist/csr/Key'
import { CubeIcon } from '@phosphor-icons/react/dist/csr/Cube'
import { UsersIcon } from '@phosphor-icons/react/dist/csr/Users'
import { NotebookIcon } from '@phosphor-icons/react/dist/csr/Notebook'
import { PlugsIcon } from '@phosphor-icons/react/dist/csr/Plugs'
import { ChatCircleIcon } from '@phosphor-icons/react/dist/csr/ChatCircle'
import { OrganizationControllerContext } from '../../context'
import { MainPage } from '../main'
import { Sidebar, SidebarHeader, SidebarSection, SidebarFolder, SidebarItem } from '../../components/sidebar'
import type { OrganizationState, OrganizationDestination } from './types'

export { createOrganizationStore } from './store'
export type { OrganizationState, OrganizationController, OrganizationDestination } from './types'

function useController() {
  const controller = useContext(OrganizationControllerContext)
  if (!controller) throw new Error('OrganizationPage requires an OrganizationControllerContext value.')
  return controller
}

function useOrganizationState<T>(selector: (state: OrganizationState) => T) {
  return useStore(useController().store, selector)
}

export function OrganizationPage({ children }: { children?: ReactNode }) {
  return <MainPage sidebar={<OrganizationSidebar />}>{children}</MainPage>
}

function OrganizationHeader() {
  const controller = useController()
  const organizations = useOrganizationState((state) => state.organizations)
  const organizationId = useOrganizationState((state) => state.organizationId)
  return (
    <SidebarHeader
      workspace={organizations.find((item) => item.id === organizationId)?.name ?? 'Organization'}
      organizations={organizations}
      selectedOrganizationId={organizationId}
      onOrganizationChange={controller.onSelectOrganization}
      onWorkspaceClick={() => {}}
      onSearch={controller.onSearch}
      onSettings={controller.onOpenSettings}
      onSignOut={controller.onSignOut}
    />
  )
}

function Destination({ item, icon }: { item: OrganizationDestination; icon: ReactNode }) {
  const controller = useController()
  const selected = useOrganizationState((state) => state.selectedDestination === item.id)
  return (
    <SidebarItem
      label={item.name}
      href={item.href}
      icon={icon}
      selected={selected}
      onNavigate={() => controller.onOpenDestination(item.id)}
    />
  )
}

function Providers() {
  const controller = useController()
  const providers = useOrganizationState((state) => state.providers)
  return (
    <SidebarSection label="Providers" onAdd={controller.onCreateProvider} addLabel="Add provider">
      {providers.map((item) => (
        <Destination key={item.id} item={item} icon={<HardDrivesIcon />} />
      ))}
    </SidebarSection>
  )
}

function Models() {
  const controller = useController()
  const models = useOrganizationState((state) => state.models)
  return (
    <SidebarFolder label="Models" icon={<CubeIcon />} onAdd={controller.onCreateModel} addLabel="Add model">
      {models.map((item) => (
        <Destination key={item.id} item={item} icon={<CubeIcon />} />
      ))}
    </SidebarFolder>
  )
}

function Conversations() {
  const controller = useController()
  const conversations = useOrganizationState((state) => state.conversations)
  const selected = useOrganizationState((state) => state.selectedDestination)
  return (
    <SidebarSection label="Conversations" onAdd={() => controller.onOpenChat()} addLabel="New conversation">
      {conversations.map((item) => (
        <SidebarItem
          key={item.id}
          label={item.name}
          href={item.href}
          selected={selected === item.id}
          icon={<ChatCircleIcon />}
          onNavigate={() => controller.onOpenChat(item.id)}
        />
      ))}
    </SidebarSection>
  )
}

function OrganizationSidebar() {
  const controller = useController()
  const links = useOrganizationState((state) => state.links)
  return (
    <Sidebar>
      <OrganizationHeader />
      <Providers />
      <SidebarSection label="Routers" onAdd={controller.onCreateRouter} addLabel="Add router">
        <Destination item={{ id: 'stats', name: 'Stats', href: links.stats }} icon={<ChartBarIcon />} />
        <Destination item={{ id: 'api-key', name: 'API key', href: links.keys }} icon={<KeyIcon />} />
        <Models />
      </SidebarSection>
      <SidebarSection label="Workspace">
        <Destination item={{ id: 'members', name: 'Members', href: links.members }} icon={<UsersIcon />} />
        <Destination item={{ id: 'notebooks', name: 'Notebooks', href: links.notebooks }} icon={<NotebookIcon />} />
        <Destination item={{ id: 'plugins', name: 'Plugins', href: links.plugins }} icon={<PlugsIcon />} />
      </SidebarSection>
      <Conversations />
    </Sidebar>
  )
}
