import { useState } from 'react'
import { DotsThreeIcon } from '@phosphor-icons/react/dist/csr/DotsThree'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { SearchField } from '../../components/fields'
import { Table } from '../../components/table'
import { Popup } from '../../components/popup'
import { Avatar } from '../../components/content'
import { Select } from '../../components/dropdowns'
import { SettingsContent, Heading } from './shared'
import { useSettings, useSettingsController } from './controller'
import type { SettingsMember } from './types'

function MemberRoles({ member }: { member: SettingsMember }) {
  const c = useSettingsController(),
    roles = useSettings((s) => s.roles),
    groups = useSettings((s) => s.groups)
  const readonly = useSettings((s) => s.readonly),
    [search, setSearch] = useState('')
  const inherited = new Set(groups.filter((g) => g.memberIds.includes(member.id)).flatMap((g) => g.roleIds))
  const ids = new Set([...member.roleIds, ...inherited]),
    first = roles.find((r) => member.roleIds.includes(r.id))
  return (
    <Popup>
      <Popup.Trigger
        aria-label={`Roles for ${member.name}`}
        disabled={readonly}
        className="h-6 w-32 justify-between gap-2 px-1.5 text-xs text-text"
      >
        <span>
          {first?.name ?? 'No roles'}
          {ids.size > 1 ? ` +${ids.size - 1}` : ''}
        </span>
        <CaretDownIcon className="size-2.5 shrink-0" aria-hidden="true" />
      </Popup.Trigger>
      <Popup.Content aria-label={`Roles for ${member.name}`} className="w-2xs bg-elevated p-2">
        <SearchField
          aria-label="Search roles"
          placeholder="Search roles…"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="mb-2"
        />
        {roles
          .filter((r) => r.name.toLowerCase().includes(search.toLowerCase()))
          .map((r) => (
            <Popup.CheckboxItem
              key={r.id}
              aria-label={r.name}
              checked={ids.has(r.id)}
              disabled={inherited.has(r.id)}
              onChange={() => c.pages.members.toggleMemberRole(member.id, r.id)}
            >
              <span className="flex-1 text-left">{r.name}</span>
              {inherited.has(r.id) ? <span className="text-[11px] text-muted">Group</span> : null}
            </Popup.CheckboxItem>
          ))}
        <Popup.Separator />
        <p className="px-1 py-2 text-[11px] text-muted">Group roles are managed in Groups.</p>
      </Popup.Content>
    </Popup>
  )
}
function MemberActions({ member }: { member: SettingsMember }) {
  const c = useSettingsController(),
    readonly = useSettings((s) => s.readonly)
  return (
    <Popup>
      <Popup.Trigger aria-label={`Actions for ${member.name}`} disabled={readonly} className="size-8">
        <DotsThreeIcon className="size-4" aria-hidden="true" />
      </Popup.Trigger>
      <Popup.Content aria-label={`Actions for ${member.name}`} className="bg-elevated">
        <Popup.Item className="h-10 text-error" onClick={() => c.pages.dialogs.openDialog('remove-member', member.id)}>
          Remove member
        </Popup.Item>
      </Popup.Content>
    </Popup>
  )
}
export function MembersSettings() {
  const c = useSettingsController(),
    members = useSettings((s) => s.members),
    query = useSettings((s) => s.query),
    roles = useSettings((s) => s.roles),
    groups = useSettings((s) => s.groups)
  const roleFilter = useSettings((s) => s.filters.memberRole ?? 'all')
  const visible = members.filter(
    (m) =>
      `${m.name} ${m.email}`.toLowerCase().includes(query.toLowerCase()) &&
      (roleFilter === 'all' ||
        m.roleIds.includes(roleFilter) ||
        groups.some((g) => g.memberIds.includes(m.id) && g.roleIds.includes(roleFilter))),
  )
  return (
    <SettingsContent wide>
      <Heading>Members</Heading>
      <div className="-mt-7 mb-5 flex min-w-0 flex-wrap items-center gap-2">
        <SearchField
          aria-label="Search members"
          placeholder="Search by name or email"
          value={query}
          onChange={(e) => c.pages.search.setQuery(e.target.value)}
          className="w-75 max-w-full"
        />
        <Select
          aria-label="Filter members by role"
          value={roleFilter}
          className={roleFilter === 'all' ? 'w-16' : 'w-48'}
          onValueChange={(v) => c.pages.search.setFilter('memberRole', v)}
          options={[{ value: 'all', label: 'All' }, ...roles.map((r) => ({ value: r.id, label: r.name }))]}
        />
        <span className="ml-auto text-xs text-muted">{visible.length} members</span>
      </div>
      <Table className="lg:-mx-12 lg:w-[calc(100%+6rem)]">
        <Table.Content className="min-w-200 border-separate border-spacing-0" aria-label="Organization members">
          <colgroup>
            <col className="w-[40%]" />
            <col className="w-[34%]" />
            <col className="w-[20%]" />
            <col className="w-[6%]" />
          </colgroup>
          <Table.Header>
            <Table.Row className="border-0">
              <Table.Head className="px-4 lg:px-13">Name ↓</Table.Head>
              <Table.Head className="px-4">Email</Table.Head>
              <Table.Head>Roles</Table.Head>
              <Table.Head>
                <span className="sr-only">Actions</span>
              </Table.Head>
            </Table.Row>
          </Table.Header>
          <Table.Body>
            <Table.Row className="h-8 border-0">
              <Table.Cell colSpan={4} className="rounded-lg bg-inset/50 px-4 text-muted lg:px-13">
                Active <span className="ml-2">{visible.length}</span>
              </Table.Cell>
            </Table.Row>
            {visible.map((m) => (
              <Table.Row key={m.id} className="h-14 border-0">
                <Table.Cell className="px-4 lg:px-13">
                  <div className="flex items-center gap-2.5">
                    <Avatar name={m.name} size={24} />
                    <span className="text-xsm font-medium">{m.name}</span>
                  </div>
                </Table.Cell>
                <Table.Cell className="px-4 text-muted">{m.email}</Table.Cell>
                <Table.Cell>
                  <MemberRoles member={m} />
                </Table.Cell>
                <Table.Cell>
                  <MemberActions member={m} />
                </Table.Cell>
              </Table.Row>
            ))}
          </Table.Body>
        </Table.Content>
      </Table>
      {!visible.length ? <Table.Empty>No members match your search.</Table.Empty> : null}
    </SettingsContent>
  )
}
