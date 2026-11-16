import { ArrowDownLeftIcon } from '@phosphor-icons/react/dist/csr/ArrowDownLeft'
import { Tooltip } from '../../components/feedback'
import { useCallback, useEffect, useState, useSyncExternalStore, type ReactNode } from 'react'
import useInfiniteScroll from 'react-infinite-scroll-hook'
import { EyeIcon } from '@phosphor-icons/react/dist/csr/Eye'
import { EyeSlashIcon } from '@phosphor-icons/react/dist/csr/EyeSlash'
import { UserPlusIcon } from '@phosphor-icons/react/dist/csr/UserPlus'
import { DotsThreeIcon } from '@phosphor-icons/react/dist/csr/DotsThree'
import { ChoicePicker } from '../../components/choice-picker'
import { Searchbox } from '../../components/fields'
import { PermissionRequest, usePermissionChecks } from '../../components/permission'
import { Form, useManagedForm, useFormValue } from '../../components/form'
import { SearchField, TokenField, TextField } from '../../components/fields'
import { Button } from '../../components/buttons'
import { Table } from '../../components/table'
import { Avatar } from '../../components/content'
import { FunnelIcon } from '@phosphor-icons/react/dist/csr/Funnel'
import { Select } from '../../components/dropdowns'
import { Dialog } from '../../components/dialog'
import { Popup } from '../../components/popup'
import type { PermissionCheck } from '../../permission'
import type { SettingsMember } from './types'
import { InvitationCreateInput } from '@compallm/contract/validation'
import { invitationTextSchema } from '@compallm/contract/invitation'
import { zodResolver } from '@hookform/resolvers/zod'
import { SettingsContent, Heading } from './shared'
import { useSettings, useSettingsController } from './controller'

function InviteMember() {
  const c = useSettingsController()
  const busy = useSettings((s) => s.invitationBusy)
  const link = useSettings((s) => s.invitationLink)
  const admitted = useSettings((s) => s.invitationEmailAdmitted)
  const links = useSettings((s) => s.invitationLinks ?? [])
  const form = useManagedForm(
    { email: '' },
    {
      resolver: zodResolver(invitationTextSchema),
      submit: async (values) => ((await c.pages.members.invite?.(values.email)) ? values : false),
    },
  )
  return (
    <div className="space-y-4">
      <Form.Root model={form} className="space-y-4">
        <label className="grid gap-1 text-xsm font-medium">
          Email
          <div className="relative">
            <TokenField
              name="email"
              parseToken={(email) => {
                const result = InvitationCreateInput.safeParse({ email })
                return result.success ? result.data.email : null
              }}
              aria-label="Invite by email"
              placeholder="email@example.com, another@example.com…"
            />
            <Form.Error name="email" />
          </div>
        </label>
        <label className="grid gap-1 text-xsm font-medium">
          Role
          <Select.Root value="member">
            <Select.Trigger aria-label="Invitation role" className="h-8 w-full rounded-lg bg-elevated text-left">
              <Select.Value>
                Member <span className="font-normal text-muted-foreground">— Standard organization access</span>
              </Select.Value>
            </Select.Trigger>
            <Select.Content>
              <Select.Item value="member">Member</Select.Item>
            </Select.Content>
          </Select.Root>
        </label>
        {link ? (
          <div className="space-y-2">
            <p className="text-xs text-muted-foreground">
              {admitted ? 'Invitation email queued.' : 'Email was not queued. Share this link or resend explicitly.'}
            </p>
            {links.length ? (
              links.map((item) => (
                <div key={item.email} className="flex min-w-0 items-center gap-2">
                  <span className="min-w-0 flex-1 truncate text-xs">{item.email}</span>
                  <Button onClick={() => c.pages.dialogs.copy(item.url)}>Copy link</Button>
                </div>
              ))
            ) : (
              <>
                <TextField aria-label="New invitation link" variant="constant" value={link} />
                <Button onClick={() => c.pages.dialogs.copy(link)}>Copy invitation link</Button>
              </>
            )}
          </div>
        ) : null}
        <div className="flex justify-end pt-6">
          <Button type="submit" variant="primary" loading={busy} className="h-8 px-3">
            Send invitations
          </Button>
        </div>
      </Form.Root>
    </div>
  )
}

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

function LoadMoreRows({
  root,
  loading,
  more,
  failed,
  onLoadMore,
  label,
  disabled = false,
}: {
  root: HTMLElement | null
  loading: boolean
  more: boolean
  failed: boolean
  onLoadMore(): void
  label: string
  disabled?: boolean
}) {
  const [sentry, { rootRef }] = useInfiniteScroll({
    loading,
    hasNextPage: more,
    onLoadMore,
    disabled: disabled || failed || !root,
    rootMargin: '0px 0px 160px 0px',
  })
  useEffect(() => rootRef(root), [root, rootRef])
  if (!more) return null
  return (
    <Table.Row className="h-8 border-0">
      <Table.Cell colSpan={6}>
        <div ref={sentry} className="flex h-8 items-center justify-center" aria-busy={loading}>
          {failed ? <Button onClick={onLoadMore}>Retry {label}</Button> : <span className="sr-only">{label}</span>}
        </div>
      </Table.Cell>
    </Table.Row>
  )
}

function Invitations({ root }: { root: HTMLElement | null }) {
  const [open, setOpen] = useState(true)
  const c = useSettingsController()
  const invitations = useSettings((s) => s.invitations.filter((invitation) => invitation.status === 'pending'))
  const loading = useSettings((s) => s.invitationsLoading)
  const more = useSettings((s) => s.invitationsHasNextPage)
  const failed = useSettings((s) => s.invitationsLoadError ?? false)
  const busy = useSettings((s) => s.invitationBusy)
  if (!invitations.length) return null
  return (
    <Table.Category
      variant="directory"
      label="Invited"
      count={`${invitations.length}${more ? ' shown' : ''}`}
      columns={6}
      open={open}
      onOpenChange={setOpen}
    >
      {invitations.map((row) => (
        <Table.Row key={row.id} className="h-14 border-0">
          <Table.Cell column="name" className="pl-0">
            <div className="flex items-center gap-2.5 py-1">
              <span
                className="grid size-6 shrink-0 place-items-center rounded-full border border-dashed
border-border text-ui-11 text-muted-foreground"
              >
                @
              </span>
              <span className="truncate text-xsm font-medium">Invited member</span>
            </div>
          </Table.Cell>
          <Table.Cell column="email" className="text-muted-foreground">
            <MemberEmail key={row.email} email={row.email} name={`invitation ${row.id}`} />
          </Table.Cell>
          <Table.Cell column="role" className="px-2 text-muted-foreground">
            Member (Invited)
          </Table.Cell>
          <Table.Cell column="groups" className="px-2 text-muted-foreground">
            —
          </Table.Cell>
          <Table.Cell column="joined">
            <DirectoryDate value={row.issuedAt ?? row.expiresAt - 604800} />
          </Table.Cell>
          <Table.Cell column="actions" className="px-0">
            <PermissionRequest actions={['member.invite']}>
              {(allowed) =>
                allowed && (row.status === 'pending' || row.status === 'expired') ? (
                  <Popup.Root>
                    <Popup.Trigger aria-label={`Actions for invitation ${row.id}`} className="size-8 p-0">
                      <DotsThreeIcon className="size-4 text-muted-foreground" aria-hidden="true" />
                    </Popup.Trigger>
                    <Popup.Content aria-label="Invitation actions">
                      <Popup.Item onSelect={() => void c.pages.members.resend?.(row.id)}>Resend invitation</Popup.Item>
                      <Popup.Item className="text-destructive" onSelect={() => void c.pages.members.revoke?.(row.id)}>
                        Revoke invitation
                      </Popup.Item>
                    </Popup.Content>
                  </Popup.Root>
                ) : null
              }
            </PermissionRequest>
          </Table.Cell>
        </Table.Row>
      ))}
      <LoadMoreRows
        root={root}
        loading={loading || busy}
        more={more}
        failed={failed}
        disabled={!open}
        onLoadMore={() => c.pages.members.loadMoreInvitations?.()}
        label="invitations"
      />
    </Table.Category>
  )
}

function MemberRoles({ member }: { member: SettingsMember }) {
  const roles = useSettings((s) => s.roles)
  const actorIsOwner = useSettings((s) => s.actorIsOwner)
  const currentMemberId = useSettings((s) => s.currentMemberId)
  const ownerIds = useSettings((s) => s.ownerMemberIds ?? [])
  const [confirmation, setConfirmation] = useState<string | null>(null)
  const first = member.role
  const known = Boolean(first)
  const label = first?.name ?? 'Unavailable'
  const c = useSettingsController()
  const checksFor = (id: string): PermissionCheck[] => [
    ...(id
      ? [
          { action: 'member.role.assign' as const, resourceType: 'Member' as const, resourceId: member.id, roleId: id },
          { action: 'role.assign' as const, resourceType: 'Role' as const, resourceId: id },
        ]
      : []),
    ...(first && first.id !== id
      ? [
          { action: 'member.role.unassign' as const, resourceType: 'Member' as const, resourceId: member.id, roleId: first.id },
          { action: 'role.unassign' as const, resourceType: 'Role' as const, resourceId: first.id },
        ]
      : []),
  ]
  const protectedOwner = first?.owner
  const options = roles.filter((r) => !r.owner || actorIsOwner)
  const optionChecks = options.map((role) => checksFor(role.id))
  const statuses = usePermissionChecks(optionChecks.flat())
  const allowed = optionChecks.map((checks, index) => {
    const offset = optionChecks.slice(0, index).reduce((total, previous) => total + previous.length, 0)
    return statuses.slice(offset, offset + checks.length).every((status) => status === 'allowed')
  })
  const form = useManagedForm(
    { roleId: first?.id ?? '' },
    {
      mode: 'change',
      submit: async (values) => {
        const result = await c.pages.members.setMemberRole(member.id, values.roleId)
        if (result === false) return false
        setConfirmation(null)
        return values
      },
    },
  )
  const selected = useFormValue<string>(form, 'roleId')
  const pending = useSyncExternalStore(
    form.subscribe,
    () => form.pending,
    () => false,
  )
  const cancelConfirmation = () => {
    if (pending) return
    form.escape('roleId')
    setConfirmation(null)
  }
  if (member.id === currentMemberId || (protectedOwner && (!actorIsOwner || ownerIds.length <= 1)))
    return <span className="line-clamp-2 whitespace-normal wrap-break-word text-left text-xs">{label}</span>
  return (
    <Form.Root model={form}>
      <Select.Root
        value={selected}
        disabled={pending || confirmation !== null}
        onValueChange={(value) => {
          if (value === null || value === selected || pending) return
          if (Boolean(protectedOwner) !== Boolean(roles.find((r) => r.id === value)?.owner)) setConfirmation(value)
          else form.change('roleId', value)
        }}
      >
        <Select.Trigger
          aria-label={`Roles for ${member.name}`}
          className="h-auto min-h-6 w-full max-w-30 rounded-lg px-2 py-0.5 text-left text-xs"
        >
          <Select.Value>
            {selected === (first?.id ?? '') ? label : (roles.find((r) => r.id === selected)?.name ?? 'Unavailable')}
          </Select.Value>
        </Select.Trigger>
        <Select.Content aria-label={`Roles for ${member.name}`} className="w-64">
          {options.map((role, index) => {
            const unavailable =
              !known || !allowed[index] || (protectedOwner && (!actorIsOwner || ownerIds.length <= 1) && role.id !== first?.id)
            if (unavailable) return null
            return (
              <Select.Item key={role.id} value={role.id}>
                {role.color ? (
                  <span
                    aria-hidden="true"
                    className="mr-1 size-1.5 shrink-0 rounded-full"
                    style={{ backgroundColor: `color-mix(in oklab, ${role.color} 70%, var(--foreground))` }}
                  />
                ) : null}
                <span className="min-w-0">
                  <span className="block truncate">{role.name}</span>
                  {role.description ? (
                    <span className="block truncate text-xs text-muted-foreground">{role.description}</span>
                  ) : null}
                </span>
              </Select.Item>
            )
          })}
        </Select.Content>
      </Select.Root>
      <Form.Error name="roleId" />
      {confirmation !== null ? (
        <Dialog.Root
          open
          onOpenChange={(open) => {
            if (!open) cancelConfirmation()
          }}
        >
          <Dialog.Content className="w-100">
            <Dialog.Header>
              <Dialog.Title>
                {roles.find((r) => r.id === confirmation)?.owner
                  ? `Make ${member.name} an owner?`
                  : `Change ${member.name}’s role?`}
              </Dialog.Title>
              {!pending ? <Dialog.Close /> : null}
            </Dialog.Header>
            <Dialog.Footer>
              <Button disabled={pending} onClick={cancelConfirmation}>
                Cancel
              </Button>
              <Button
                variant="primary"
                loading={pending}
                onClick={() => {
                  form.change('roleId', confirmation)
                  void form.submit()
                }}
              >
                Confirm
              </Button>
            </Dialog.Footer>
          </Dialog.Content>
        </Dialog.Root>
      ) : null}
    </Form.Root>
  )
}

function MemberGroups({ member }: { member: SettingsMember }) {
  const c = useSettingsController()
  const catalog = useSettings((s) => s.groups)
  const confirmed = member.groups ?? catalog.filter((group) => group.memberIds.includes(member.id))
  const ids = confirmed.map((group) => group.id)
  const options = [...new Map([...catalog, ...confirmed].map((group) => [group.id, group])).values()]
  const checks = usePermissionChecks(
    options.map((group) => ({
      action: ids.includes(group.id) ? ('group.member.remove' as const) : ('group.member.add' as const),
      resourceType: 'Group',
      resourceId: group.id,
      memberId: member.id,
    })),
  )
  const form = useManagedForm(
    { groupIds: ids },
    {
      mode: 'change',
      submit: async (values) => {
        const result = await c.pages.members.setMemberGroups?.(member.id, values.groupIds)
        return result === false || !result ? false : { groupIds: result }
      },
    },
  )
  const selected = useFormValue<string[]>(form, 'groupIds') ?? []
  const selectedGroups = options.filter((group) => selected.includes(group.id))
  const editable = Boolean(c.pages.members.setMemberGroups)
  const pickerOptions = options.map((group, index) => ({
    id: group.id,
    label: group.name,
    description: group.description,
    disabled: !editable || checks[index] !== 'allowed',
  }))
  return (
    <PermissionRequest actions={['group.read']}>
      {(allowed) =>
        allowed ? (
          <Form.Root model={form}>
            <ChoicePicker.Root options={pickerOptions} value={selected} onValueChange={(next) => form.change('groupIds', next)}>
              <ChoicePicker.Trigger aria-label={`Groups for ${member.name}`}>
                {selectedGroups[0]?.name ?? 'None'}
                {selectedGroups.length > 1 ? ` +${selectedGroups.length - 1}` : ''}
              </ChoicePicker.Trigger>
              <ChoicePicker.Content>
                <div className="p-2 pb-0">
                  <Searchbox.Root>
                    <ChoicePicker.Input aria-label={`Search groups for ${member.name}`} placeholder="Search groups…" />
                  </Searchbox.Root>
                </div>
                <ChoicePicker.Empty>Empty.</ChoicePicker.Empty>
                <ChoicePicker.List>{(option) => <ChoicePicker.Item key={option.id} value={option} />}</ChoicePicker.List>
              </ChoicePicker.Content>
            </ChoicePicker.Root>
          </Form.Root>
        ) : null
      }
    </PermissionRequest>
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

function MemberRows({
  label,
  members,
  complete,
  open,
  onOpenChange,
  children,
}: {
  label: string
  members: SettingsMember[]
  complete: boolean
  open: boolean
  onOpenChange(open: boolean): void
  children?: ReactNode
}) {
  const currentMemberId = useSettings((s) => s.currentMemberId)
  return (
    <Table.Category
      variant="directory"
      label={label}
      open={open}
      onOpenChange={onOpenChange}
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
              {(allowed) => (allowed ? <MemberRoles member={member} /> : null)}
            </PermissionRequest>
          </Table.Cell>
          <Table.Cell column="groups">
            <MemberGroups member={member} />
          </Table.Cell>
          <Table.Cell column="joined">
            <DirectoryDate value={member.joinedAt} />
          </Table.Cell>
          <Table.Cell column="actions" className="px-0">
            <MemberActions member={member} />
          </Table.Cell>
        </Table.Row>
      ))}
      {children}
    </Table.Category>
  )
}

function InvitationDialog() {
  const open = useSettings((s) => s.invitationsOpen ?? false)
  return (
    <Dialog.Root defaultOpen={open}>
      <Dialog.Trigger asChild>
        <Button variant="neutral" className="ml-auto h-8 shrink-0 gap-1.5 px-3">
          <UserPlusIcon className="size-3.5" aria-hidden="true" />
          Invite
        </Button>
      </Dialog.Trigger>
      <Dialog.Content className="top-[22vh] bottom-auto max-h-[76dvh] w-140 rounded-xl bg-inset md:left-61">
        <Dialog.Header className="h-14 border-0">
          <Dialog.Title className="flex items-center gap-2 text-ui-15 font-medium">Send invitations</Dialog.Title>
          <Dialog.Close />
        </Dialog.Header>
        <Dialog.Body className="pt-3 pb-4">
          <PermissionRequest actions={['member.invite']}>{(allowed) => (allowed ? <InviteMember /> : null)}</PermissionRequest>
        </Dialog.Body>
      </Dialog.Content>
    </Dialog.Root>
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
  const failed = useSettings((s) => s.membersLoadError ?? false)
  const [root, setRoot] = useState<HTMLElement | null>(null)
  const attachTable = useCallback((element: HTMLDivElement | null) => {
    setRoot(element?.closest<HTMLElement>('[aria-label="Settings content"]') ?? null)
  }, [])
  const [activeOpen, setActiveOpen] = useState(true)
  const [inactiveOpen, setInactiveOpen] = useState(true)
  const invitations = useSettings((s) => s.invitations.length)
  const confirmedComplete = useSettings((s) => s.membersComplete !== false)
  const complete = confirmedComplete && !loading && !next
  const active = members.filter((member) => member.available !== false)
  const inactive = members.filter((member) => member.available === false)
  const descending = order === 'DESC_NULLS_LAST'
  return (
    <SettingsContent wide className="flex min-h-full flex-col pt-16 pb-6">
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
        {c.pages.members.invite || invitations ? (
          <PermissionRequest actions={['member.invite']}>
            {(allowed) => (allowed ? <InvitationDialog /> : null)}
          </PermissionRequest>
        ) : null}
      </div>
      <Table.Root ref={attachTable} className="overflow-visible">
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
          <Invitations root={root} />
          {active.length || !inactive.length ? (
            <MemberRows label="Active" members={active} complete={complete} open={activeOpen} onOpenChange={setActiveOpen}>
              {!inactive.length ? (
                <LoadMoreRows
                  root={root}
                  loading={loading}
                  more={next}
                  failed={failed}
                  disabled={!activeOpen}
                  onLoadMore={() => c.pages.members.loadMore?.()}
                  label="members"
                />
              ) : null}
            </MemberRows>
          ) : null}
          {inactive.length ? (
            <MemberRows
              label="Inactive"
              members={inactive}
              complete={complete}
              open={inactiveOpen}
              onOpenChange={setInactiveOpen}
            >
              <LoadMoreRows
                root={root}
                loading={loading}
                more={next}
                failed={failed}
                disabled={!inactiveOpen}
                onLoadMore={() => c.pages.members.loadMore?.()}
                label="members"
              />
            </MemberRows>
          ) : null}
        </Table.Content>
      </Table.Root>
      {!members.length && !loading ? <Table.Empty>No members match your search.</Table.Empty> : null}
    </SettingsContent>
  )
}
