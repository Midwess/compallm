import { Button } from '../../components/buttons'
import { TextField, SearchField } from '../../components/fields'
import { Switch } from '../../components/selection'
import { SectionSurface } from '../../components/surfaces'
import { SettingsSectionTitle } from '../../components/settings'
import { Table } from '../../components/table'
import { useSettings, useSettingsController } from './controller'
import { SettingsContent, Heading, Section, Rows, Row, Indicator, Empty } from './shared'

function GroupSelection({ kind }: { kind: 'members' | 'roles' }) {
  const c = useSettingsController(),
    draft = useSettings((s) => s.groupDraft),
    members = useSettings((s) => s.members),
    roles = useSettings((s) => s.roles),
    query = useSettings((s) => s.filters[`group-${kind}-query`] ?? ''),
    readonly = useSettings((s) => s.readonly),
    busy = useSettings((s) => s.busy)
  if (!draft) return null
  const entries =
    kind === 'members'
      ? members.map((m) => ({ id: m.id, name: m.name, detail: m.email, color: '' }))
      : roles.map((r) => ({ id: r.id, name: r.name, detail: r.description, color: r.color }))
  const selected = kind === 'members' ? draft.memberIds : draft.roleIds
  const normalized = query.trim().toLocaleLowerCase()
  const visible = entries.filter((e) => `${e.name} ${e.detail}`.toLocaleLowerCase().includes(normalized))
  const title = kind === 'members' ? 'Members' : 'Assigned roles'
  return (
    <section className="flex min-w-0 flex-1 flex-col gap-4" aria-label={title}>
      <div className="flex shrink-0 items-center gap-2 px-4">
        <SettingsSectionTitle>{title}</SettingsSectionTitle>
        <span className="text-xs text-muted">
          {selected.length} / {entries.length}
        </span>
      </div>
      <SectionSurface className="flex flex-1 flex-col">
        <div className="shrink-0 p-4">
          <SearchField
            aria-label={`Search group ${kind}`}
            placeholder={kind === 'members' ? 'Search members…' : 'Search roles…'}
            className="[&>input]:border-border"
            value={query}
            onChange={(event) => c.setFilter(`group-${kind}-query`, event.target.value)}
          />
        </div>
        <div
          role="region"
          aria-label={`${title} selection`}
          tabIndex={0}
          className="mx-4 min-h-32 max-h-[90vh] flex-1 overflow-auto overscroll-contain outline-offset-2
[scrollbar-gutter:stable] focus-visible:outline-2 focus-visible:outline-focus"
        >
          <Table className="overflow-visible min-h-80">
            <Table.Content aria-label={`Select group ${kind}`} className="min-w-112">
              <colgroup>
                <col className="w-[42%]" />
                <col className="w-[46%]" />
                <col className="w-[12%]" />
              </colgroup>
              <Table.Header className="sticky top-0 z-10 bg-inset">
                <Table.Row>
                  <Table.Head className="px-0">{kind === 'members' ? 'Name' : 'Role'}</Table.Head>
                  <Table.Head className="px-0">{kind === 'members' ? 'Email' : 'Description'}</Table.Head>
                  <Table.Head className="px-0">Include</Table.Head>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {visible.map((entry) => (
                  <Table.Row key={entry.id}>
                    <Table.Cell className="px-0 pr-3">
                      <span className="flex items-center gap-2 truncate text-[13px] leading-[18px]">
                        {kind === 'roles' ? <Indicator color={entry.color} /> : null}
                        {entry.name}
                      </span>
                    </Table.Cell>
                    <Table.Cell className="truncate px-0 pr-3 text-xs text-muted" title={entry.detail}>
                      {entry.detail}
                    </Table.Cell>
                    <Table.Cell className="px-0">
                      <Switch
                        aria-label={`Include ${entry.name}`}
                        checked={selected.includes(entry.id)}
                        disabled={readonly || busy}
                        onChange={() => (kind === 'members' ? c.toggleGroupMember(entry.id) : c.toggleGroupRole(entry.id))}
                      />
                    </Table.Cell>
                  </Table.Row>
                ))}
                {!visible.length ? (
                  <Table.Row>
                    <Table.Cell colSpan={3} className="py-6 text-center text-muted">
                      No {kind} match your search.
                    </Table.Cell>
                  </Table.Row>
                ) : null}
              </Table.Body>
            </Table.Content>
          </Table>
        </div>
      </SectionSurface>
    </section>
  )
}

export function GroupEditorSettings() {
  const c = useSettingsController(),
    draft = useSettings((s) => s.groupDraft),
    readonly = useSettings((s) => s.readonly),
    busy = useSettings((s) => s.busy)
  if (!draft)
    return (
      <SettingsContent>
        <Empty title="Group unavailable">Choose a group from the sidebar.</Empty>
      </SettingsContent>
    )
  return (
    <SettingsContent className="flex min-h-[min(48rem,90vh)] max-w-208 flex-col pt-14 pb-6">
      <Heading className="mb-8 shrink-0">{draft.id ? draft.name : 'New group'}</Heading>
      <div className="flex min-h-0 flex-1 flex-col gap-6 overflow-y-auto">
        <Section title="Group details" className="shrink-0">
          <Rows className="[&_[role=separator]]:bg-border">
            <Row label={<label htmlFor="group-name">Name</label>}>
              <TextField
                id="group-name"
                className="w-144 max-w-full border-border max-lg:w-96"
                value={draft.name}
                placeholder="Group name…"
                disabled={readonly || busy}
                onChange={(event) => c.updateGroup({ name: event.target.value })}
              />
            </Row>
            <Row label={<label htmlFor="group-description">Description</label>} last>
              <TextField
                id="group-description"
                className="w-144 max-w-full border-border max-lg:w-96"
                value={draft.description}
                placeholder="Describe this group…"
                disabled={readonly || busy}
                onChange={(event) => c.updateGroup({ description: event.target.value })}
              />
            </Row>
          </Rows>
        </Section>
        <GroupSelection kind="members" />
        <GroupSelection kind="roles" />
        {!readonly ? (
          <div className="flex shrink-0 flex-wrap justify-end gap-2">
            {draft.id ? (
              <Button
                variant="ghost"
                className="mr-auto text-error"
                disabled={busy}
                onClick={() => c.openDialog('delete-group', draft.id!)}
              >
                Delete group
              </Button>
            ) : null}
            <Button variant="ghost" className="w-24 rounded-md" disabled={busy} onClick={() => c.resetGroup()}>
              Cancel
            </Button>
            <Button
              variant="primary"
              className="w-32 rounded-md"
              loading={busy}
              disabled={!draft.name.trim()}
              onClick={() => c.saveGroup()}
            >
              {draft.id ? 'Save changes' : 'Create group'}
            </Button>
          </div>
        ) : null}
      </div>
    </SettingsContent>
  )
}
