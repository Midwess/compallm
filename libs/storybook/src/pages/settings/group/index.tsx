import { PermissionRequest, usePermission, usePermissionChecks } from '../../../components/permission'
import { GroupCreateInput, GroupUpdateInput } from '@compallm/contract/validation'
import { FormError } from '../../../components/form'
import { DisabledTooltip } from '../../../components/feedback'
import { useEffect, useSyncExternalStore } from 'react'
import { Form, contractResult } from '../../../components/form'
import { Button } from '../../../components/buttons'
import { TrashIcon } from '@phosphor-icons/react/dist/csr/Trash'
import { ChoicePicker } from '../../../components/choice-picker'
import { TextField, SearchField } from '../../../components/fields'
import { Switch } from '../../../components/selection'
import { SectionSurface } from '../../../components/surfaces'
import { SettingsSectionTitle } from '../../../components/settings'
import { Table } from '../../../components/table'
import { useSettings, useSettingsController } from '../controller'
import { useGroup, useGroupController } from './controller'
import { SettingsContent, Heading, Section, Rows, Row, Indicator, Empty } from '../shared'

function GroupMembers() {
  const group = useGroupController(),
    draft = useGroup(
      (s) => (s.draft ? { id: s.draft.id, memberIds: s.draft.memberIds, memberSelection: s.draft.memberSelection } : null),
      ['draft.id', 'draft.memberIds', 'draft.memberSelection'],
    ),
    fallbackMembers = useSettings((s) => s.members),
    pageMembers = useGroup((s) => s.members),
    count = useGroup((s) => s.memberCount),
    hasNextPage = useGroup((s) => s.hasNextPage),
    membersLoading = useGroup((s) => s.membersLoading),
    query = useGroup((s) => s.query),
    readonly = useSettings((s) => s.readonly),
    busy = useGroup((s) => s.busy)
  if (!draft) return null
  const members = pageMembers ?? fallbackMembers
  const entries = members.map((member) => ({
    id: member.id,
    name: member.name,
    detail: member.email,
    available: !('available' in member) || member.available,
  }))
  const selected = draft.memberSelection
    ? Object.keys(draft.memberSelection).filter((id) => draft.memberSelection?.[id])
    : draft.memberIds
  const normalized = query.trim().toLocaleLowerCase()
  const visible = entries.filter((e) => `${e.name} ${e.detail}`.toLocaleLowerCase().includes(normalized))
  const title = 'Members'
  return (
    <section className="flex min-w-0 flex-col gap-4" aria-label={title}>
      <div className="flex shrink-0 items-center gap-2 px-4">
        <SettingsSectionTitle>{title}</SettingsSectionTitle>
        <span className="text-xs text-muted-foreground">{count ?? selected.length} selected</span>
      </div>
      <SectionSurface className="flex min-w-0 flex-col p-0">
        <div className="shrink-0 p-4">
          <SearchField
            aria-label="Search group members"
            placeholder="Search members…"
            className="[&>input]:border-border"
            value={query}
            onChange={(event) => group.setQuery(event.target.value)}
          />
        </div>
        <div
          role="region"
          aria-label={`${title} selection`}
          tabIndex={0}
          className="mx-4 min-w-0 overflow-x-auto outline-offset-2 focus-visible:outline-2 focus-visible:outline-ring"
        >
          <Table.Root className="overflow-visible">
            <Table.Content aria-label="Select group members" className="min-w-md">
              <colgroup>
                <col className="w-[42%]" />
                <col />
                <col className="w-12" />
              </colgroup>
              <Table.Header className="sticky top-0 z-10 bg-inset">
                <Table.Row>
                  <Table.Head className="px-0">Name</Table.Head>
                  <Table.Head className="px-0">Email</Table.Head>
                  <Table.Head className="px-0">Include</Table.Head>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {visible.map((entry) => (
                  <Table.Row key={entry.id}>
                    <Table.Cell className="px-0 pr-3">
                      <span className="flex items-center gap-2 truncate text-xsm leading-4.5">{entry.name}</span>
                    </Table.Cell>
                    <Table.Cell className="truncate px-0 pr-3 text-xs text-muted-foreground" title={entry.detail}>
                      {entry.detail}
                    </Table.Cell>
                    <Table.Cell className="px-0">
                      <DisabledTooltip
                        disabled={readonly || busy || (!entry.available && !selected.includes(entry.id))}
                        reason={
                          readonly
                            ? 'This group is read-only.'
                            : busy
                              ? 'Wait for the current update to finish.'
                              : 'This member is inactive and cannot be added.'
                        }
                      >
                        <PermissionRequest
                          actions={[selected.includes(entry.id) ? 'GROUP_MEMBER_REMOVE' : 'GROUP_MEMBER_ADD']}
                          resource={{ type: 'GROUP', id: draft.id }}
                          context={{ memberId: entry.id }}
                          restricted="disabled"
                          asChild
                        >
                          <Switch
                            aria-label={`Include ${entry.name}`}
                            name={`draft.memberSelection.${entry.id}`}
                            disabled={readonly || busy || (!entry.available && !selected.includes(entry.id))}
                          />
                        </PermissionRequest>
                      </DisabledTooltip>
                    </Table.Cell>
                  </Table.Row>
                ))}
                {!visible.length ? (
                  <Table.Row>
                    <Table.Cell colSpan={3} className="py-6 text-center text-muted-foreground">
                      No members match your search.
                    </Table.Cell>
                  </Table.Row>
                ) : null}
              </Table.Body>
            </Table.Content>
          </Table.Root>
        </div>
        <FormError name="draft.memberSelection" />
        {hasNextPage ? (
          <Button variant="ghost" loading={membersLoading} onClick={() => group.loadMore?.()}>
            Load more members
          </Button>
        ) : null}
      </SectionSurface>
    </section>
  )
}

function GroupRoles() {
  const c = useSettingsController(),
    group = useGroupController(),
    draft = useGroup((s) => (s.draft ? { id: s.draft.id, roleIds: s.draft.roleIds } : null), ['draft.id', 'draft.roleIds']),
    fallbackRoles = useSettings((s) => s.roles),
    roleOptions = useGroup((s) => s.roles),
    readonly = useSettings((s) => s.readonly),
    busy = useGroup((s) => s.busy)
  const roles = roleOptions ?? fallbackRoles
  const optionPermissions = usePermissionChecks(
    draft?.id
      ? roles.map((role) => ({
          action: draft.roleIds.includes(role.id) ? ('GROUP_ROLE_UNASSIGN' as const) : ('GROUP_ROLE_ASSIGN' as const),
          resourceType: 'GROUP',
          resourceId: draft.id,
          roleId: role.id,
        }))
      : [],
  )
  if (!draft) return null
  const selected = roles.filter((role) => draft.roleIds.includes(role.id))
  const missing = draft.roleIds.filter((id) => !roles.some((role) => role.id === id))
  return (
    <section className="flex min-w-0 flex-col gap-4" aria-label="Assigned roles">
      <div className="flex items-center justify-between gap-2 px-4">
        <SettingsSectionTitle>Assigned roles</SettingsSectionTitle>
        <DisabledTooltip
          disabled={readonly || busy}
          reason={readonly ? 'This group is read-only.' : 'Wait for the current update to finish.'}
        >
          <ChoicePicker.Control
            label="Add role"
            searchLabel="Search roles"
            placeholder="Search roles…"
            emptyMessage="No roles found."
            emptyAction={
              roles.length === 0 ? (
                <Button variant="ghost" className="mt-2" onClick={() => c.navigate('new-role')}>
                  Create role
                </Button>
              ) : undefined
            }
            disabled={readonly || busy}
            options={roles.map((role, index) => ({
              id: role.id,
              label: role.name,
              description: role.description,
              color: role.color,
              disabled: optionPermissions[index] !== 'allowed',
            }))}
            name="draft.roleIds"
          />
        </DisabledTooltip>
        <FormError name="draft.roleIds" />
      </div>
      <SectionSurface className="p-0">
        {draft.roleIds.length ? (
          <ul aria-label="Roles assigned to this group">
            {missing.map((id) => (
              <li key={id} className="px-4 py-3 text-xs text-muted-foreground">
                Role unavailable: {id}
              </li>
            ))}
            {selected.map((role) => (
              <li
                key={role.id}
                className="group/assigned-role flex min-h-14 min-w-0 items-center gap-3
                border-b border-divider px-4 py-3 last:border-0"
              >
                <Indicator color={role.color} />
                <div className="flex min-w-0 flex-1 items-baseline gap-3">
                  <span className="min-w-0 max-w-60 truncate text-xsm">{role.name}</span>
                  {role.description ? (
                    <span
                      className="min-w-0 max-w-80 flex-1 whitespace-normal wrap-break-word
                      text-xs leading-4.5 text-muted-foreground"
                    >
                      {role.description}
                    </span>
                  ) : null}
                </div>
                <DisabledTooltip
                  disabled={readonly || busy}
                  reason={readonly ? 'This group is read-only.' : 'Wait for the current update to finish.'}
                >
                  <PermissionRequest
                    actions={['GROUP_ROLE_UNASSIGN']}
                    resource={{ type: 'GROUP', id: draft.id }}
                    context={{ roleId: role.id }}
                    restricted="disabled"
                    asChild
                  >
                    <Button
                      variant="ghost"
                      aria-label={`Remove ${role.name} from group`}
                      className="pointer-events-none size-6 shrink-0 rounded-full p-0 text-sm opacity-0
                    group-hover/assigned-role:pointer-events-auto group-hover/assigned-role:opacity-100
                    group-focus-within/assigned-role:pointer-events-auto group-focus-within/assigned-role:opacity-100"
                      disabled={readonly || busy}
                      onClick={() =>
                        group.store.form.change(
                          'draft.roleIds',
                          draft.roleIds.filter((id) => id !== role.id),
                        )
                      }
                    >
                      <TrashIcon aria-hidden="true" className="size-3.5" />
                    </Button>
                  </PermissionRequest>
                </DisabledTooltip>
              </li>
            ))}
          </ul>
        ) : (
          <p className="px-4 py-5 text-xs text-muted-foreground">No roles assigned.</p>
        )}
      </SectionSurface>
    </section>
  )
}

function GroupHeading() {
  const form = useGroupController().store.form
  return useSyncExternalStore(
    form.subscribe,
    () => form.baseline.draft?.name ?? 'Group',
    () => form.baseline.draft?.name ?? 'Group',
  )
}
function GroupEditorContent() {
  const c = useSettingsController(),
    loading = useGroup((s) => s.loading),
    draft = useGroup((s) => (s.draft ? { id: s.draft.id } : null), ['draft.id']),
    readonly = useSettings((s) => s.readonly),
    busy = useGroup((s) => s.busy)
  const metadataPermission = usePermission({
    actions: ['GROUP_METADATA_UPDATE'],
    resource: { type: 'GROUP', id: draft?.id },
    enabled: !!draft?.id,
  })
  if (loading) return <SettingsContent aria-busy="true" aria-label="Loading group" />
  if (!draft)
    return (
      <SettingsContent>
        <Empty title="Group unavailable">Choose a group from the sidebar.</Empty>
      </SettingsContent>
    )
  return (
    <SettingsContent className="flex flex-col pt-14 pb-6">
      <Heading className="mb-8 shrink-0">
        <GroupHeading />
      </Heading>
      <div className="flex min-w-0 flex-col gap-6">
        <Section title="Group details" className="shrink-0">
          <Rows className="**:[[role=separator]]:bg-border">
            <Row label={<label htmlFor="group-name">Name</label>}>
              <DisabledTooltip
                disabled={readonly || busy || !metadataPermission.allowed}
                reason={
                  !metadataPermission.allowed
                    ? 'You do not have permission to edit this group.'
                    : readonly
                      ? 'This group is read-only.'
                      : 'Wait for the current update to finish.'
                }
              >
                <TextField
                  id="group-name"
                  className="w-xl max-w-full border-border max-lg:w-sm"
                  name="draft.name"
                  placeholder="Group name…"
                  disabled={readonly || busy || !metadataPermission.allowed}
                />
              </DisabledTooltip>
              <FormError name="draft.name" />
            </Row>
            <Row label={<label htmlFor="group-description">Description</label>} last>
              <DisabledTooltip
                disabled={readonly || busy || !metadataPermission.allowed}
                reason={
                  !metadataPermission.allowed
                    ? 'You do not have permission to edit this group.'
                    : readonly
                      ? 'This group is read-only.'
                      : 'Wait for the current update to finish.'
                }
              >
                <TextField
                  id="group-description"
                  className="w-xl max-w-full border-border max-lg:w-sm"
                  name="draft.description"
                  placeholder="Describe this group…"
                  disabled={readonly || busy || !metadataPermission.allowed}
                />
              </DisabledTooltip>
              <FormError name="draft.description" />
            </Row>
          </Rows>
        </Section>
        <PermissionRequest actions={['ROLE_READ']}>
          <GroupRoles />
        </PermissionRequest>
        <PermissionRequest actions={['MEMBER_READ']}>
          <GroupMembers />
        </PermissionRequest>
        {!readonly && draft.id ? (
          <PermissionRequest actions={['GROUP_DELETE']} resource={{ type: 'GROUP', id: draft.id }}>
            <Section title="Danger" className="shrink-0">
              <Rows>
                <Row label="Delete group" last>
                  <Button
                    variant="ghost"
                    className="text-destructive"
                    disabled={busy}
                    onClick={() => c.pages.dialogs.openDialog('delete-group', draft.id!)}
                  >
                    Delete group
                  </Button>
                </Row>
              </Rows>
            </Section>
          </PermissionRequest>
        ) : null}
      </div>
    </SettingsContent>
  )
}

export function GroupEditorSettings() {
  const group = useGroupController()
  const id = useGroup((state) => state.draft?.id, ['draft.id'])
  const members = useGroup((state) => state.members?.map((member) => member.id).join('|'))
  const fallbackMembers = useSettings((state) => state.members.map((member) => member.id).join('|'))
  const memberIds = members ?? fallbackMembers
  const form = group.store.form
  const settings = useSettingsController()
  useEffect(() => {
    form.configure({
      mode: 'blur',
      names: ['draft.name', 'draft.description', 'draft.roleIds', 'draft.memberSelection'],
      changed: (name, value) => {
        if (name === 'draft.roleIds') group.rolesChanged?.(value as string[])
      },
      fields: {
        'draft.name': { group: 'metadata' },
        'draft.description': { group: 'metadata' },
        'draft.roleIds': { mode: 'change', group: 'roles' },
        ...Object.fromEntries(
          memberIds
            .split('|')
            .filter(Boolean)
            .map((id) => [`draft.memberSelection.${id}`, { mode: 'change' as const, group: 'members' }]),
        ),
      },
      validationNames: (names) => (id ? names : ['draft']),
      error: (error, retry) =>
        settings.apiError(error, () => {
          if (group.store.getState().draft?.id === id) retry()
        }),
      parse: (values, names) => {
        const draft = values.draft
        if (id && draft) {
          if (names.includes('draft.roleIds')) {
            const roles = GroupCreateInput.shape.roleIds.safeParse(draft.roleIds)
            if (!roles.success)
              return { issues: roles.error.issues.map((issue) => ({ ...issue, path: ['draft', 'roleIds', ...issue.path] })) }
          }
          const patch: Record<string, unknown> = {}
          if (names.includes('draft.name')) patch.name = draft.name
          if (names.includes('draft.description')) patch.description = draft.description
          const result = GroupUpdateInput.safeParse(patch)
          return contractResult(
            result,
            result.success
              ? {
                  ...values,
                  draft: {
                    ...draft,
                    name: result.data.name ?? draft.name,
                    description: result.data.description ?? draft.description,
                  },
                }
              : values,
            'draft',
          )
        }
        return { issues: [] }
      },
      submit: async (values, _patch, signal, names) => {
        if (!values.draft?.id) return false
        const result = await group.updateGroup(values.draft, names, signal)
        return result && typeof result === 'object' ? { draft: result } : false
      },
    })
  }, [group, form, id, settings, memberIds])
  return (
    <Form.Root model={form}>
      <GroupEditorContent />
    </Form.Root>
  )
}
