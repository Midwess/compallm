import { motion } from 'motion/react'
import { animation, useTransition } from '../../components/motion'
import { Button } from '../../components/buttons'
import { TextField } from '../../components/fields'
import { CodeEditor } from '../../components/code-block'
import { SectionSurface } from '../../components/surfaces'
import { SettingsSectionTitle } from '../../components/settings'
import { useSettings, useSettingsController, useCodeAppearance } from './controller'
import { PolicyTable } from './policy-table'
import { SettingsContent, Heading, Section, Rows, Row, Empty } from './shared'

function EditingMode() {
  const timing = useTransition('panel')
  const c = useSettingsController(),
    mode = useSettings((s) => s.roleDraft?.mode),
    invalid = useSettings((s) => Boolean(s.roleDraft?.parsing || s.roleDraft?.diagnostics.length))
  return (
    <div
      role="group"
      aria-label="Policy editing mode"
      className="relative flex h-7 w-28 items-center gap-0.5 rounded-full bg-inset p-0.5"
    >
      <motion.span
        initial={false}
        animate={{ x: mode === 'cedar' ? 'calc(100% + 2px)' : 0 }}
        transition={timing}
        aria-hidden="true"
        className="pointer-events-none absolute inset-y-0.5 left-0.5 w-[calc((100%-0.375rem)/2)]
rounded-full bg-elevated shadow-control"
      />
      {(['visual', 'cedar'] as const).map((value) => (
        <Button
          key={value}
          aria-pressed={mode === value}
          variant="ghost"
          className="relative z-10 h-6 min-h-0 min-w-0 flex-1 rounded-full px-1 text-xs leading-4.5"
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
    <SectionSurface className="flex min-h-64 min-w-0 w-full flex-1 flex-col p-2">
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
  const timing = useTransition('panel')
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
    <SettingsContent className="flex flex-col pt-14 pb-6">
      <Heading className="mb-8 shrink-0">{draft.id ? draft.name : 'New role'}</Heading>
      <div className="flex min-w-0 flex-1 flex-col gap-6">
        <Section title="Role details" className="shrink-0">
          <Rows className="**:[[role=separator]]:bg-border">
            <Row label={<label htmlFor="role-name">Name</label>}>
              <TextField
                id="role-name"
                className="w-xl max-w-full border-border max-lg:w-sm"
                placeholder="Role name…"
                value={draft.name}
                disabled={locked || busy}
                onChange={(event) => c.updateRole({ name: event.target.value })}
              />
            </Row>
            <Row label={<label htmlFor="role-description">Description</label>} last>
              <TextField
                id="role-description"
                className="w-xl max-w-full border-border max-lg:w-sm"
                placeholder="Describe this role…"
                value={draft.description}
                disabled={locked || busy}
                onChange={(event) => c.updateRole({ description: event.target.value })}
              />
            </Row>
          </Rows>
        </Section>
        <section aria-labelledby="role-permissions-title" className="flex min-h-0 min-w-0 flex-1 flex-col gap-2">
          <div className="grid shrink-0 justify-items-start gap-2 px-4 pt-1">
            <SettingsSectionTitle id="role-permissions-title" className="leading-6">
              Permissions
            </SettingsSectionTitle>
            <EditingMode />
          </div>
          <div className="grid min-w-0 w-full">
            <motion.div
              initial={false}
              transition={timing}
              aria-hidden={draft.mode !== 'visual'}
              inert={draft.mode !== 'visual'}
              animate={{
                opacity: draft.mode === 'visual' ? 1 : 0,
                x: draft.mode === 'visual' ? 0 : -animation.panelDistance,
                display: 'block',
                transitionEnd: { display: draft.mode === 'visual' ? 'block' : 'none' },
              }}
              style={{
                pointerEvents: draft.mode === 'visual' ? 'auto' : 'none',
              }}
              className="col-start-1 row-start-1 min-w-0 w-full"
            >
              {draft.builtin ? (
                <Empty title="System-owned role">Built-in grants follow the accepted organization policy.</Empty>
              ) : (
                <PolicyTable />
              )}
            </motion.div>
            <motion.div
              initial={false}
              transition={timing}
              aria-hidden={draft.mode !== 'cedar'}
              inert={draft.mode !== 'cedar'}
              animate={{
                opacity: draft.mode === 'cedar' ? 1 : 0,
                x: draft.mode === 'cedar' ? 0 : animation.panelDistance,
                display: 'block',
                transitionEnd: { display: draft.mode === 'cedar' ? 'block' : 'none' },
              }}
              style={{
                pointerEvents: draft.mode === 'cedar' ? 'auto' : 'none',
              }}
              className="col-start-1 row-start-1 min-w-0 w-full"
            >
              <CedarEditor locked={locked || busy} />
            </motion.div>
          </div>
          {draft.mode === 'visual' && advanced ? (
            <div className="flex shrink-0 flex-wrap items-center justify-between gap-2 px-4 text-xs text-muted">
              <span>Advanced Cedar retained. Review its source to edit conditions or resource restrictions.</span>
              <Button variant="ghost" className="h-7 px-0 text-xs" onClick={() => c.setPolicyMode('cedar')}>
                Review Cedar
              </Button>
            </div>
          ) : null}
          <div aria-live="polite" className="mt-2 shrink-0 px-4 text-xs leading-4.5 text-muted">
            {draft.parsing ? (
              'Checking policy…'
            ) : draft.diagnostics.length ? (
              <div role="alert" className="whitespace-pre-wrap text-error">
                {draft.diagnostics.join('\n')}
              </div>
            ) : draft.mode === 'cedar' ? (
              'Cedar syntax is valid.'
            ) : (
              ''
            )}
          </div>
        </section>
        {!locked && !draft.id ? (
          <div className="flex shrink-0 flex-wrap justify-end gap-2">
            <Button
              variant="primary"
              className="w-32 rounded-md"
              loading={busy}
              disabled={!draft.name.trim() || !draft.source.trim() || invalid}
              onClick={() => c.saveRole()}
            >
              Create role
            </Button>
          </div>
        ) : null}
        {!locked && draft.id ? (
          <Section title="Danger" className="shrink-0">
            <Rows>
              <Row label="Delete role" last>
                <Button
                  variant="ghost"
                  className="text-error"
                  disabled={busy}
                  onClick={() => c.openDialog('delete-role', draft.id!)}
                >
                  Delete role
                </Button>
              </Row>
            </Rows>
          </Section>
        ) : null}
      </div>
    </SettingsContent>
  )
}
