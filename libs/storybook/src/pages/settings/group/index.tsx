import { GroupCreateInput } from '@compallm/contract/validation'
import { FormError } from '../../../components/form'
import { useEffect } from 'react'
import { Form, contractResult } from '../../../components/form'
import { Button } from '../../../components/buttons'
import { XIcon } from '@phosphor-icons/react/dist/csr/X'
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
    draft = useGroup((s) => (s.draft ? { memberIds: s.draft.memberIds } : null), ['draft.memberIds']),
    members = useSettings((s) => s.members),
    query = useGroup((s) => s.query),
    readonly = useSettings((s) => s.readonly),
    busy = useGroup((s) => s.busy)
  if (!draft) return null
  const entries = members.map((member) => ({ id: member.id, name: member.name, detail: member.email }))
  const selected = draft.memberIds
  const normalized = query.trim().toLocaleLowerCase()
  const visible = entries.filter((e) => `${e.name} ${e.detail}`.toLocaleLowerCase().includes(normalized))
  const title = 'Members'
  return (
    <section className="flex min-w-0 flex-col gap-4" aria-label={title}>
      <div className="flex shrink-0 items-center gap-2 px-4">
        <SettingsSectionTitle>{title}</SettingsSectionTitle>
        <span className="text-xs text-muted">
          {selected.length} / {entries.length}
        </span>
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
          className="mx-4 min-w-0 overflow-x-auto outline-offset-2 focus-visible:outline-2 focus-visible:outline-focus"
        >
          <Table className="overflow-visible">
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
                    <Table.Cell className="truncate px-0 pr-3 text-xs text-muted" title={entry.detail}>
                      {entry.detail}
                    </Table.Cell>
                    <Table.Cell className="px-0">
                      <Switch
                        aria-label={`Include ${entry.name}`}
                        name="draft.memberIds"
                        value={entry.id}
                        disabled={readonly || busy}
                      />
                    </Table.Cell>
                  </Table.Row>
                ))}
                {!visible.length ? (
                  <Table.Row>
                    <Table.Cell colSpan={3} className="py-6 text-center text-muted">
                      No members match your search.
                    </Table.Cell>
                  </Table.Row>
                ) : null}
              </Table.Body>
            </Table.Content>
          </Table>
        </div>
        <FormError name="draft.memberIds" />
      </SectionSurface>
    </section>
  )
}

function GroupRoles() {
  const c = useSettingsController(),
    group = useGroupController(),
    draft = useGroup((s) => (s.draft ? { roleIds: s.draft.roleIds } : null), ['draft.roleIds']),
    roles = useSettings((s) => s.roles),
    readonly = useSettings((s) => s.readonly),
    busy = useGroup((s) => s.busy)
  if (!draft) return null
  const selected = roles.filter((role) => draft.roleIds.includes(role.id))
  const missing = draft.roleIds.filter((id) => !roles.some((role) => role.id === id))
  return (
    <section className="flex min-w-0 flex-col gap-4" aria-label="Assigned roles">
      <div className="flex items-center justify-between gap-2 px-4">
        <SettingsSectionTitle>Assigned roles</SettingsSectionTitle>
        <ChoicePicker
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
          options={roles.map((role) => ({ id: role.id, label: role.name, description: role.description, color: role.color }))}
          name="draft.roleIds"
        />
        <FormError name="draft.roleIds" />
      </div>
      <SectionSurface className="p-0">
        {draft.roleIds.length ? (
          <ul aria-label="Roles assigned to this group">
            {missing.map((id) => (
              <li key={id} className="px-4 py-3 text-xs text-muted">
                Role unavailable: {id}
              </li>
            ))}
            {selected.map((role) => (
              <li key={role.id} className="flex min-w-0 items-center gap-3 border-b border-divider px-4 py-2 last:border-0">
                <Indicator color={role.color} />
                <div className="min-w-0 flex-1">
                  <div className="truncate text-xsm">{role.name}</div>
                  {role.description ? <div className="truncate text-xs text-muted">{role.description}</div> : null}
                </div>
                <Button
                  variant="ghost"
                  aria-label={`Remove ${role.name} from group`}
                  className="size-8 shrink-0 rounded-full p-0"
                  disabled={readonly || busy}
                  onClick={() =>
                    group.store.form.change(
                      'draft.roleIds',
                      draft.roleIds.filter((id) => id !== role.id),
                    )
                  }
                >
                  <XIcon size={14} aria-hidden="true" />
                </Button>
              </li>
            ))}
          </ul>
        ) : (
          <p className="px-4 py-5 text-xs text-muted">No roles assigned.</p>
        )}
      </SectionSurface>
    </section>
  )
}

function GroupHeading() {
  const id = useGroup((s) => s.draft?.id, ['draft.id'])
  const name = useGroup((s) => s.draft?.name, ['draft.name'])
  return id ? name : 'New group'
}
function GroupEditorContent() {
  const c = useSettingsController(),
    draft = useGroup((s) => (s.draft ? { id: s.draft.id } : null), ['draft.id']),
    readonly = useSettings((s) => s.readonly),
    busy = useGroup((s) => s.busy)
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
              <TextField
                id="group-name"
                className="w-xl max-w-full border-border max-lg:w-sm"
                name="draft.name"
                placeholder="Group name…"
                disabled={readonly || busy}
              />
              <FormError name="draft.name" />
            </Row>
            <Row label={<label htmlFor="group-description">Description</label>} last>
              <TextField
                id="group-description"
                className="w-xl max-w-full border-border max-lg:w-sm"
                name="draft.description"
                placeholder="Describe this group…"
                disabled={readonly || busy}
              />
              <FormError name="draft.description" />
            </Row>
          </Rows>
        </Section>
        <GroupRoles />
        <GroupMembers />
        {!readonly && !draft.id ? (
          <div className="flex shrink-0 flex-wrap justify-end gap-2">
            <Button variant="primary" className="w-32 rounded-xl" loading={busy} type="submit">
              Create group
            </Button>
          </div>
        ) : null}
        {!readonly && draft.id ? (
          <Section title="Danger" className="shrink-0">
            <Rows>
              <Row label="Delete group" last>
                <Button
                  variant="ghost"
                  className="text-error"
                  disabled={busy}
                  onClick={() => c.pages.dialogs.openDialog('delete-group', draft.id!)}
                >
                  Delete group
                </Button>
              </Row>
            </Rows>
          </Section>
        ) : null}
      </div>
    </SettingsContent>
  )
}

export function GroupEditorSettings() {
  const group = useGroupController()
  const id = useGroup((state) => state.draft?.id, ['draft.id'])
  const form = group.store.form
  const settings = useSettingsController()
  useEffect(() => {
    form.reset(form.values)
    form.configure({
      mode: 'button',
      command: true,
      validationNames: () => ['draft'],
      error: (error, retry) => settings.apiError(error, retry),
      parse: (values) => {
        const draft = values.draft
        const result = GroupCreateInput.safeParse(
          draft ? { name: draft.name, description: draft.description, roleIds: draft.roleIds } : {},
        )
        return contractResult(
          result,
          result.success && draft ? { ...values, draft: { ...draft, ...result.data } } : values,
          'draft',
        )
      },
      submit: async (values) => {
        if (!values.draft) return false
        const result = await group.saveGroup(values.draft)
        return result && typeof result === 'object' ? { draft: result } : false
      },
    })
  }, [group, form, id, settings])
  return (
    <Form model={form}>
      <GroupEditorContent />
    </Form>
  )
}
