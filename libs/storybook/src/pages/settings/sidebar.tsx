import { ArrowLeftIcon } from '@phosphor-icons/react/dist/csr/ArrowLeft'
import { GearIcon } from '@phosphor-icons/react/dist/csr/Gear'
import { ChatTextIcon } from '@phosphor-icons/react/dist/csr/ChatText'
import { CodeIcon } from '@phosphor-icons/react/dist/csr/Code'
import { UserIcon } from '@phosphor-icons/react/dist/csr/User'
import { WarningCircleIcon } from '@phosphor-icons/react/dist/csr/WarningCircle'
import { InfoIcon } from '@phosphor-icons/react/dist/csr/Info'
import { UsersIcon } from '@phosphor-icons/react/dist/csr/Users'
import { UsersThreeIcon } from '@phosphor-icons/react/dist/csr/UsersThree'
import { ChartLineIcon } from '@phosphor-icons/react/dist/csr/ChartLine'
import { ShieldCheckIcon } from '@phosphor-icons/react/dist/csr/ShieldCheck'
import { Sidebar, SidebarSection, SidebarFolder, SidebarItem } from '../../components/sidebar'
import { SearchField } from '../../components/fields'
import { Indicator } from './shared'
import { useSettings, useSettingsController } from './controller'
import type { SettingsDestination } from './types'

const personal = [
  { page: 'general', label: 'General', Icon: GearIcon },
  { page: 'messaging', label: 'Messaging', Icon: ChatTextIcon },
  { page: 'code', label: 'Code appearance', Icon: CodeIcon },
] as const
const account = [
  { page: 'profile', label: 'Profile', Icon: UserIcon },
  { page: 'danger', label: 'Danger', Icon: WarningCircleIcon },
] as const
const organization = [
  { page: 'organization', label: 'Info', Icon: InfoIcon },
  { page: 'members', label: 'Members', Icon: UsersIcon },
  { page: 'usage-limits', label: 'Usage & limits', Icon: ChartLineIcon },
] as const
const category = '[&>details>summary]:text-xs [&>details>summary]:font-normal'
const folder = '[&>details>summary>span:first-child]:size-3'
const item = 'font-normal [&>span:first-child]:size-3 [&>span:first-child>svg]:size-3'

export function SettingsNavigation({ onNavigate }: { onNavigate?: () => void }) {
  const c = useSettingsController(),
    location = useSettings((s) => s.location),
    query = useSettings((s) => s.navigationQuery)
  const roles = useSettings((s) => s.roles),
    groups = useSettings((s) => s.groups)
  const match = (label: string) => label.toLowerCase().includes(query.toLowerCase())
  const navigate = (page: SettingsDestination, id?: string) => {
    c.navigate(page, id)
    onNavigate?.()
  }
  const links = (entries: readonly { page: SettingsDestination; label: string; Icon: typeof GearIcon }[]) =>
    entries
      .filter((e) => match(e.label))
      .map(({ page, label, Icon }) => (
        <SidebarItem
          key={page}
          className={item}
          label={label}
          href={`#/settings/${page}`}
          icon={<Icon />}
          selected={location.page === page}
          onNavigate={() => navigate(page)}
        />
      ))
  return (
    <Sidebar navigationLabel="Settings navigation" className="h-full pt-4 pb-3">
      <button
        type="button"
        onClick={() => c.leaveSettings()}
        className="flex h-7 items-center gap-2 px-2 text-left text-xsm text-muted-foreground hover:text-foreground"
      >
        <ArrowLeftIcon className="size-3" aria-hidden="true" />
        Back to app
      </button>
      <SearchField
        aria-label="Search settings"
        placeholder="Search settings…"
        value={query}
        onChange={(e) => c.setNavigationQuery(e.target.value)}
      />
      <SidebarSection label="Personal settings" className={`${category} mt-2`}>
        {links(personal)}
      </SidebarSection>
      <SidebarSection label="Account" className={category}>
        {links(account)}
      </SidebarSection>
      <SidebarSection label="Organization" className={category}>
        {links(organization)}
      </SidebarSection>
      <SidebarSection label="Permissions" className={category}>
        <SidebarFolder
          label="Groups"
          icon={<UsersThreeIcon />}
          className={folder}
          onAdd={() => navigate('new-group')}
          addLabel="Create group"
        >
          {groups
            .filter((g) => match(g.name))
            .map((g) => (
              <SidebarItem
                key={g.id}
                label={g.name}
                href={`#/settings/groups/${g.id}`}
                icon={<Indicator color={g.color} />}
                className="gap-2 text-xsm [&>span:first-child]:size-1.5"
                selected={location.page === 'group' && location.id === g.id}
                onNavigate={() => navigate('group', g.id)}
              />
            ))}
        </SidebarFolder>
        <SidebarFolder
          label="Roles"
          icon={<ShieldCheckIcon />}
          className={folder}
          onAdd={() => navigate('new-role')}
          addLabel="Create role"
        >
          {roles
            .filter((r) => !r.builtin && match(r.name))
            .map((r) => (
              <SidebarItem
                key={r.id}
                label={r.name}
                href={`#/settings/roles/${r.id}`}
                icon={<Indicator color={r.color} />}
                className="gap-2 text-xsm [&>span:first-child]:size-1.5"
                selected={location.page === 'role' && location.id === r.id}
                onNavigate={() => navigate('role', r.id)}
              />
            ))}
        </SidebarFolder>
      </SidebarSection>
    </Sidebar>
  )
}
