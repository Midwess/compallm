import { ArrowDownLeftIcon } from '@phosphor-icons/react/dist/csr/ArrowDownLeft'
import { Tooltip } from '../../components/feedback'
import { useState } from 'react'
import { EyeIcon } from '@phosphor-icons/react/dist/csr/Eye'
import { EyeSlashIcon } from '@phosphor-icons/react/dist/csr/EyeSlash'
import { DotsThreeIcon } from '@phosphor-icons/react/dist/csr/DotsThree'
import { PermissionRequest } from '../../components/permission'
import { SearchField } from '../../components/fields'
import { Button } from '../../components/buttons'
import { Table } from '../../components/table'
import { Avatar } from '../../components/content'
import { FunnelIcon } from '@phosphor-icons/react/dist/csr/Funnel'
import { Popup } from '../../components/popup'
import type { SettingsMember } from './types'
import { SettingsContent, Heading } from './shared'
import { useSettings, useSettingsController } from './controller'

function DirectoryDate({ value }: { value?: number | null }) {
  const [year] = useState(() => new Date().getUTCFullYear())
  if (!value) return <span className="text-muted-foreground">—</span>
  const date = new Date(value * 1000)
  const sameYear = date.getUTCFullYear() === year
  return (
    <time
      dateTime={date.toISOString()}
      title={date.toLocaleDateString()}
      className="whitespace-nowrap text-xs text-muted-foreground"
    >
      {date.toLocaleDateString(undefined, sameYear ? { month: 'short', day: 'numeric' } : { month: 'short', year: 'numeric' })}
    </time>
  )
}

function MemberActions({ member }: { member: SettingsMember }) {
  return (
    <Popup.Root>
      <Popup.Trigger aria-label={`Actions for ${member.name}`} className="size-8 p-0">
        <DotsThreeIcon className="size-4 text-muted-foreground" aria-hidden="true" />
      </Popup.Trigger>
      <Popup.Content aria-label={`Actions for ${member.name}`} className="bg-elevated">
        <Popup.Item asChild>
          <a
            href={`mailto:${encodeURIComponent(member.email)}`}
            className="flex h-8 w-full items-center rounded-md px-2 text-xs outline-none
data-highlighted:bg-menu-hover focus-visible:outline-ring"
          >
            Send email
          </a>
        </Popup.Item>
      </Popup.Content>
    </Popup.Root>
  )
}

function MemberEmail({ email, name }: { email: string; name: string }) {
  const [visible, setVisible] = useState(false)
  const Icon = visible ? EyeSlashIcon : EyeIcon
  return (
    <div className="flex min-w-0 items-center justify-start gap-1.5 text-left">
      <span className="line-clamp-2 min-w-0 break-all whitespace-normal text-left">{visible ? email : '••••••••'}</span>
      <Button
        variant="ghost"
        className="size-6 shrink-0 p-0 text-muted-foreground"
        aria-label={`${visible ? 'Hide' : 'Show'} email for ${name}`}
        aria-pressed={visible}
        onClick={() => setVisible((value) => !value)}
      >
        <Icon className="size-3" aria-hidden="true" />
      </Button>
    </div>
  )
}

function MemberAvatar({ member }: { member: SettingsMember }) {
  const tone = Array.from(member.name).reduce((sum, letter, index) => sum + letter.charCodeAt(0) * (index + 1), 0) % 3
  const colors = [
    '[&>span]:bg-[color-mix(in_oklab,var(--elevated)_75%,var(--success))]',
    '[&>span]:bg-[color-mix(in_oklab,var(--elevated)_75%,var(--primary))]',
    '[&>span]:bg-[color-mix(in_oklab,var(--elevated)_75%,var(--muted-foreground))]',
  ]
  return (
    <span className={`${colors[tone]} inline-flex shrink-0 [&>span]:text-[9px] [&>span]:font-normal`}>
      <Avatar name={member.name} size={24} />
    </span>
  )
}

function MemberRows({ label, members, complete }: { label: string; members: SettingsMember[]; complete: boolean }) {
  const currentMemberId = useSettings((s) => s.currentMemberId)
  return (
    <Table.Category
      variant="directory"
      label={label}
      columns={6}
      count={complete || members.length ? `${members.length}${complete ? '' : ' shown'}` : undefined}
    >
      {members.map((member) => (
        <Table.Row key={member.id} className="h-14 border-0">
          <Table.Cell column="name" className="pl-0">
            <div className="flex items-center gap-2.5 py-1">
              <MemberAvatar member={member} />
              <span className="relative min-w-0 pr-3 text-xsm font-medium">
                <span className="block truncate">{member.name}</span>
                {member.id === currentMemberId ? (
                  <Tooltip.Control content="This is you">
                    <button
                      type="button"
                      aria-label="This is you"
                      className="absolute -top-1 right-0 rounded text-foreground focus-visible:outline-ring"
                    >
                      <ArrowDownLeftIcon weight="bold" className="size-2.5" aria-hidden="true" />
                    </button>
                  </Tooltip.Control>
                ) : null}
              </span>
            </div>
          </Table.Cell>
          <Table.Cell column="email" className="text-muted-foreground">
            <MemberEmail key={member.email} email={member.email} name={member.name} />
          </Table.Cell>
          <Table.Cell column="role">
            <PermissionRequest actions={['role.read']}>
              {(allowed) =>
                allowed ? (
                  <span className="line-clamp-2 whitespace-normal break-words text-left text-xs">
                    {member.role?.name ?? 'Unavailable'}
                  </span>
                ) : null
              }
            </PermissionRequest>
          </Table.Cell>
          <Table.Cell column="groups">
            <PermissionRequest actions={['group.read']}>
              {(allowed) =>
                allowed ? (
                  <span className="line-clamp-2 whitespace-normal break-words text-left text-xs">
                    {member.groups?.map((group) => group.name).join(', ') || 'None'}
                  </span>
                ) : null
              }
            </PermissionRequest>
          </Table.Cell>
          <Table.Cell column="joined">
            <DirectoryDate value={member.joinedAt} />
          </Table.Cell>
          <Table.Cell column="actions" className="px-0">
            <MemberActions member={member} />
          </Table.Cell>
        </Table.Row>
      ))}
    </Table.Category>
  )
}

function MemberFilters() {
  const c = useSettingsController()
  const roles = useSettings((s) => s.roles)
  const selected = useSettings((s) => s.filters.memberRole ?? 'all')
  return (
    <PermissionRequest actions={['role.read', 'group.read']}>
      {(allowed) => (
        <Popup.Root>
          <Popup.IconTrigger
            aria-label="Filter members"
            title="Filter members"
            className={selected !== 'all' ? 'border-primary text-primary' : undefined}
          >
            <FunnelIcon className="size-3.5" aria-hidden="true" />
          </Popup.IconTrigger>
          <Popup.Content aria-label="Member filters">
            {allowed ? (
              <Popup.Sub>
                <Popup.SubTrigger>Role</Popup.SubTrigger>
                <Popup.SubContent aria-label="Filter by role">
                  <Popup.RadioGroup
                    value={selected}
                    onValueChange={(value: string) => c.pages.search.setFilter('memberRole', value)}
                  >
                    <Popup.RadioItem value="all">All roles</Popup.RadioItem>
                    {roles.map((role) => (
                      <Popup.RadioItem key={role.id} value={role.id}>
                        {role.name}
                      </Popup.RadioItem>
                    ))}
                  </Popup.RadioGroup>
                </Popup.SubContent>
              </Popup.Sub>
            ) : (
              <p className="px-2 py-2 text-xs text-muted-foreground">You do not have permission to filter by role.</p>
            )}
            <Popup.Separator />
            <Popup.Item onSelect={() => c.pages.search.setFilter('memberRole', 'all')}>Clear all filters</Popup.Item>
          </Popup.Content>
        </Popup.Root>
      )}
    </PermissionRequest>
  )
}

export function MembersSettings() {
  const c = useSettingsController()
  const members = useSettings((s) => s.members)
  const query = useSettings((s) => s.query)
  const order = useSettings((s) => s.memberOrder)
  const loading = useSettings((s) => s.membersLoading)
  const next = useSettings((s) => s.membersHasNextPage)
  const previous = useSettings((s) => s.membersHasPreviousPage)
  const page = useSettings((s) => s.membersPage)
  const confirmedComplete = useSettings((s) => s.membersComplete !== false)
  const complete = confirmedComplete && !loading && !next && !previous && page === 1
  const active = members.filter((member) => member.available !== false)
  const inactive = members.filter((member) => member.available === false)
  const descending = order === 'DESC_NULLS_LAST'
  return (
    <SettingsContent wide className="flex h-full min-h-0 flex-col pt-16 pb-0">
      <Heading className="shrink-0">Members</Heading>
      <div className="-mt-7 mb-5 flex min-w-0 shrink-0 flex-wrap items-center gap-2">
        <SearchField
          aria-label="Search members"
          placeholder="Search by name or email"
          value={query}
          onChange={(e) => c.pages.search.setQuery(e.target.value)}
          onKeyDown={(event) => {
            if (event.key === 'Enter' && !event.nativeEvent.isComposing) {
              event.preventDefault()
              c.pages.members.search?.(event.currentTarget.value)
            }
          }}
          className="w-75 max-w-full"
        />
        <MemberFilters />
      </div>
      <Table.Root className="min-h-0 flex-1 overflow-auto">
        <Table.Content
          className="border-separate border-spacing-0"
          aria-label="Organization members"
          aria-busy={loading}
          columns={[
            { id: 'name' },
            { id: 'email' },
            { id: 'role', className: 'w-1/6', visibilityClassName: 'hidden md:table-cell' },
            { id: 'groups', className: 'w-1/6', visibilityClassName: 'hidden md:table-cell' },
            { id: 'joined', className: 'w-20', visibilityClassName: 'hidden lg:table-cell' },
            { id: 'actions', className: 'w-12' },
          ]}
        >
          <Table.Header className="sticky top-0 z-20 bg-surface">
            <Table.Row className="border-0">
              <Table.Head column="name" className="pl-0" aria-sort={descending ? 'descending' : 'ascending'}>
                <Button
                  variant="ghost"
                  className="h-8 gap-0.5 rounded-none p-0 text-xs font-normal text-muted-foreground"
                  onClick={() => c.pages.members.setOrder?.(descending ? 'ASC_NULLS_LAST' : 'DESC_NULLS_LAST')}
                >
                  Name
                  <span aria-hidden="true">{descending ? '↑' : '↓'}</span>
                </Button>
              </Table.Head>
              <Table.Head column="email">Email</Table.Head>
              <Table.Head column="role" className="px-2">
                Role
              </Table.Head>
              <Table.Head column="groups">Groups</Table.Head>
              <Table.Head column="joined">Joined</Table.Head>
              <Table.Head column="actions">
                <span className="sr-only">Actions</span>
              </Table.Head>
            </Table.Row>
          </Table.Header>
          {active.length || !inactive.length ? <MemberRows label="Active" members={active} complete={complete} /> : null}
          {inactive.length ? <MemberRows label="Inactive" members={inactive} complete={complete} /> : null}
        </Table.Content>
      </Table.Root>
      {!members.length && !loading ? <Table.Empty>No members match your search.</Table.Empty> : null}
      <div className="flex shrink-0 items-center gap-2 py-4">
        {next || previous ? <span className="text-xs text-muted-foreground">Page {page}</span> : null}
        {previous ? <Button onClick={() => c.pages.members.previousPage?.()}>Previous page</Button> : null}
        {next ? <Button onClick={() => c.pages.members.nextPage?.()}>Next page</Button> : null}
      </div>
    </SettingsContent>
  )
}
