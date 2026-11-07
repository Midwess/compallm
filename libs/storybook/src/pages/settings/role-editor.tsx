import { Button } from '../../components/buttons'
import { TextField } from '../../components/fields'
import { CodeEditor } from '../../components/code-block'
import { SectionSurface } from '../../components/surfaces'
import { SettingsSectionTitle } from '../../components/settings'
import { useSettings, useSettingsController, useCodeAppearance } from './controller'
import { PolicyTable } from './policy-table'
import { SettingsContent, Heading, Section, Rows, Row, Empty } from './shared'

function EditingMode() {
  const c = useSettingsController(),
    mode = useSettings((s) => s.roleDraft?.mode),
    invalid = useSettings((s) => Boolean(s.roleDraft?.parsing || s.roleDraft?.diagnostics.length))
  return (
    <div
      role="group"
      aria-label="Policy editing mode"
      className="flex h-7 w-28 items-center gap-0.5 rounded-full bg-inset p-0.5"
    >
      {(['visual', 'cedar'] as const).map((value) => (
        <Button
          key={value}
          aria-pressed={mode === value}
          variant={mode === value ? 'neutral' : 'ghost'}
          className="h-6 min-h-0 min-w-0 flex-1 rounded-full px-1 text-xs leading-[18px]"
          disabled={value === 'visual' && mode === 'cedar' && invalid}
          onClick={() => c.setPolicyMode(value)}
        >
          {value === 'visual' ? 'Visual' : 'Cedar'}
        </Button>
      ))}
    </div>
  )
}

function CedarEditor({ locked }: { locked: boolean }) {
  const c = useSettingsController(),
    draft = useSettings((s) => s.roleDraft)
  const appearance = useCodeAppearance()
  if (!draft) return null
  const invalid = draft.parsing || draft.diagnostics.length > 0
  return (
    <SectionSurface className="flex min-h-0 flex-1 flex-col p-2">
      <CodeEditor
        {...appearance}
        label="Cedar policy source"
        language="cedar"
        actions={
          <>
            <Button variant="ghost" className="h-6 px-2 text-xs" disabled={invalid || locked} onClick={() => c.formatPolicy()}>
              Format
            </Button>
            <Button variant="ghost" className="h-6 px-2 text-xs" onClick={() => c.copy(draft.source)}>
              Copy
            </Button>
          </>
        }
        invalid={draft.diagnostics.length > 0}
        readOnly={locked}
        value={draft.source}
        onValueChange={(value) => c.setPolicySource(value)}
      />
    </SectionSurface>
  )
}

export function RoleEditorSettings() {
  const c = useSettingsController(),
    draft = useSettings((s) => s.roleDraft),
    readonly = useSettings((s) => s.readonly),
    busy = useSettings((s) => s.busy)
  if (!draft)
    return (
      <SettingsContent>
        <Empty title="Role unavailable">Choose a role from the sidebar.</Empty>
      </SettingsContent>
    )
  const locked = readonly || draft.builtin,
    invalid = draft.parsing || draft.diagnostics.length > 0,
    advanced = draft.projection.some((p) => p.advanced || !p.editable || !p.action)
  return (
    <SettingsContent className="flex min-h-[min(48rem,90vh)] max-h-[90vh] max-w-208 flex-col pt-14 pb-6">
      <Heading className="mb-8 shrink-0">{draft.id ? draft.name : 'New role'}</Heading>
      <div className="flex min-h-0 flex-1 flex-col gap-6 overflow-y-auto">
        <Section title="Role details" className="shrink-0">
          <Rows className="[&_[role=separator]]:bg-border">
            <Row label={<label htmlFor="role-name">Name</label>}>
              <TextField
                id="role-name"
                className="w-144 max-w-full border-border max-lg:w-96"
                placeholder="Role name…"
                value={draft.name}
                disabled={locked || busy}
                onChange={(event) => c.updateRole({ name: event.target.value })}
              />
            </Row>
            <Row label={<label htmlFor="role-description">Description</label>} last>
              <TextField
                id="role-description"
                className="w-144 max-w-full border-border max-lg:w-96"
                placeholder="Describe this role…"
                value={draft.description}
                disabled={locked || busy}
                onChange={(event) => c.updateRole({ description: event.target.value })}
              />
            </Row>
          </Rows>
        </Section>
        <section aria-labelledby="role-permissions-title" className="flex min-h-0 flex-1 flex-col gap-2">
          <div className="grid shrink-0 justify-items-start gap-2 px-4 pt-1">
            <SettingsSectionTitle id="role-permissions-title" className="leading-6">
              Permissions
            </SettingsSectionTitle>
            <EditingMode />
          </div>
          {draft.mode === 'cedar' ? (
            <CedarEditor locked={locked || busy} />
          ) : draft.builtin ? (
            <Empty title="System-owned role">Built-in grants follow the accepted organization policy.</Empty>
          ) : (
            <PolicyTable />
          )}
          {draft.mode === 'visual' && advanced ? (
            <div className="flex shrink-0 flex-wrap items-center justify-between gap-2 px-4 text-xs text-muted">
              <span>Advanced Cedar retained. Review its source to edit conditions or resource restrictions.</span>
              <Button variant="ghost" className="h-7 px-0 text-xs" onClick={() => c.setPolicyMode('cedar')}>
                Review Cedar
              </Button>
            </div>
          ) : null}
          <div aria-live="polite" className="mt-2 shrink-0 px-4 text-xs leading-[18px] text-muted">
            {draft.parsing ? (
              'Checking policy…'
            ) : draft.diagnostics.length ? (
              <div role="alert" className="whitespace-pre-wrap text-error">
                {draft.diagnostics.join('\n')}
              </div>
            ) : draft.mode === 'cedar' ? (
              'Cedar syntax is valid.'
            ) : (
              'Only selected policies are included. Unselected policies grant no access.'
            )}
          </div>
        </section>
        {!locked ? (
          <div className="flex shrink-0 flex-wrap justify-end gap-2">
            {draft.id ? (
              <Button variant="ghost" className="mr-auto text-error" onClick={() => c.openDialog('delete-role', draft.id!)}>
                Delete role
              </Button>
            ) : null}
            <Button variant="ghost" className="w-24 rounded-md" disabled={busy} onClick={() => c.resetRole()}>
              Cancel
            </Button>
            <Button
              variant="primary"
              className="w-32 rounded-md"
              loading={busy}
              disabled={!draft.name.trim() || !draft.source.trim() || invalid}
              onClick={() => c.saveRole()}
            >
              {draft.id ? 'Save changes' : 'Create role'}
            </Button>
          </div>
        ) : null}
      </div>
    </SettingsContent>
  )
}
