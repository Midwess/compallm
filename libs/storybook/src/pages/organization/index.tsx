import { PermissionRequest, usePermission } from '../../components/permission'
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
export { OrganizationCatalogPage } from './catalog'
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

function CustomizableDestinations() {
  const controller = useController()
  const providerCreate = usePermission({ actions: ['provider.create'] })
  const modelCreate = usePermission({ actions: ['model.create'] })
  const providers = useOrganizationState((state) => state.providers)
  const models = useOrganizationState((state) => state.models)
  const links = useOrganizationState((state) => state.links)
  const layout = useOrganizationState((state) => state.sidebarLayout)
  return (
    <>
      {layout
        .filter((section) => section.items.length > 0)
        .map((section) => (
          <SidebarSection
            key={section.category}
            label={section.category[0] + section.category.slice(1).toLowerCase()}
            onAdd={section.category === 'ROUTERS' ? controller.onCreateRouter : undefined}
            addLabel={section.category === 'ROUTERS' ? 'Add router' : undefined}
          >
            {section.items.map((item) => {
              switch (item) {
                case 'PROVIDERS':
                  return (
                    <PermissionRequest key={item} actions={['provider.read']}>
                      <SidebarFolder
                        label="Providers"
                        icon={<HardDrivesIcon />}
                        onAdd={providerCreate.allowed ? controller.onCreateProvider : undefined}
                        addLabel="Add provider"
                      >
                        {providers.map((entry) => (
                          <Destination key={entry.id} item={entry} icon={<HardDrivesIcon />} />
                        ))}
                      </SidebarFolder>
                    </PermissionRequest>
                  )
                case 'MODELS':
                  return (
                    <PermissionRequest key={item} actions={['model.read']}>
                      <SidebarFolder
                        label="Models"
                        icon={<CubeIcon />}
                        onAdd={modelCreate.allowed ? controller.onCreateModel : undefined}
                        addLabel="Add model"
                      >
                        {models.map((entry) => (
                          <Destination key={entry.id} item={entry} icon={<CubeIcon />} />
                        ))}
                      </SidebarFolder>
                    </PermissionRequest>
                  )
                case 'NOTEBOOKS':
                  return (
                    <Destination
                      key={item}
                      item={{ id: 'notebooks', name: 'Notebooks', href: links.notebooks }}
                      icon={<NotebookIcon />}
                    />
                  )
                case 'PLUGINS':
                  return (
                    <Destination
                      key={item}
                      item={{ id: 'plugins', name: 'Plugins', href: links.plugins }}
                      icon={<PlugsIcon />}
                    />
                  )
                case 'STATS':
                  return (
                    <Destination key={item} item={{ id: 'stats', name: 'Stats', href: links.stats }} icon={<ChartBarIcon />} />
                  )
                case 'API_KEYS':
                  return (
                    <Destination key={item} item={{ id: 'api-key', name: 'API key', href: links.keys }} icon={<KeyIcon />} />
                  )
                case 'MEMBERS':
                  return (
                    <PermissionRequest key={item} actions={['member.read']}>
                      <Destination
                        key={item}
                        item={{ id: 'members', name: 'Members', href: links.members }}
                        icon={<UsersIcon />}
                      />
                    </PermissionRequest>
                  )
                case 'CONVERSATIONS':
                  return <Conversations key={item} />
                default:
                  return null
              }
            })}
          </SidebarSection>
        ))}
    </>
  )
}

function Conversations() {
  const controller = useController()
  const conversations = useOrganizationState((state) => state.conversations)
  const selected = useOrganizationState((state) => state.selectedDestination)
  return (
    <SidebarFolder
      label="Conversations"
      icon={<ChatCircleIcon />}
      onAdd={() => controller.onOpenChat()}
      addLabel="New conversation"
    >
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
    </SidebarFolder>
  )
}

function OrganizationSidebar() {
  return (
    <Sidebar>
      <OrganizationHeader />
      <CustomizableDestinations />
    </Sidebar>
  )
}
