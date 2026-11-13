import { motion } from 'motion/react'
import { animation, useTransition } from '../../../components/motion'
import { Button } from '../../../components/buttons'
import { TextField } from '../../../components/fields'
import { CodeEditor } from '../../../components/code-block'
import { SectionSurface } from '../../../components/surfaces'
import { SettingsSectionTitle } from '../../../components/settings'
import { useSettings, useSettingsController, useCodeAppearance } from '../controller'
import { useRole, useRoleController } from './controller'
import { PolicyTable } from './policy-table'
import { DisabledTooltip } from '../../../components/feedback'

const ownerExplanation = 'The Owner role cannot be edited or deleted.'
import { SettingsContent, Heading, Section, Rows, Row, Empty } from '../shared'

function EditingMode() {
  const timing = useTransition('panel')
  const c = useRoleController(),
    mode = useRole((s) => s.draft?.mode),
    invalid = useRole((s) => Boolean(s.draft?.parsing || s.draft?.diagnostics.length))
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

function CedarEditor({ locked, owner }: { locked: boolean; owner: boolean }) {
  const c = useRoleController(),
    shared = useSettingsController(),
    draft = useRole((s) => s.draft)
  const appearance = useCodeAppearance()
  if (!draft) return null
  const invalid = draft.parsing || draft.diagnostics.length > 0
  return (
    <SectionSurface className="flex min-w-0 w-full flex-col p-2">
      <div className="flex shrink-0 flex-wrap items-center justify-end gap-2 px-2 pb-2 text-xs">
        <DisabledTooltip disabled={owner} reason={ownerExplanation}>
          <Button variant="ghost" className="h-6 px-2 text-xs" disabled={invalid || locked} onClick={() => c.formatPolicy()}>
            Format
          </Button>
        </DisabledTooltip>
        <Button variant="ghost" className="h-6 px-2 text-xs" onClick={() => shared.pages.dialogs.copy(draft.source)}>
          Copy
        </Button>
        <span className="rounded-lg bg-elevated px-2.5 py-1 font-sans text-xs leading-4 text-text">Cedar</span>
      </div>
      <DisabledTooltip disabled={owner} reason={ownerExplanation} allowInteraction>
        <CodeEditor
          {...appearance}
          label="Cedar policy source"
          language="cedar"
          showToolbar={false}
          invalid={draft.diagnostics.length > 0}
          readOnly={locked}
          value={draft.source}
          onValueChange={(value) => c.setPolicySource(value)}
          onCommit={() => c.commitPolicySource?.()}
        />
      </DisabledTooltip>
    </SectionSurface>
  )
}

export function RoleEditorSettings() {
  const timing = useTransition('panel')
  const c = useRoleController(),
    shared = useSettingsController(),
    draft = useRole((s) => s.draft),
    readonly = useSettings((s) => s.readonly),
    loading = useRole((s) => s.loading),
    busy = useRole((s) => s.busy)
  if (loading) return <SettingsContent aria-busy="true" aria-label="Loading role" />
  if (!draft)
    return (
      <SettingsContent>
        <Empty title="Role unavailable">Choose a role from the sidebar.</Empty>
      </SettingsContent>
    )
  const locked = readonly || draft.builtin,
    invalid = draft.parsing || draft.diagnostics.length > 0
  return (
    <SettingsContent className="flex flex-col pt-14 pb-6">
      <Heading className="mb-8 shrink-0">{draft.id ? draft.name : 'New role'}</Heading>
      <div className="flex min-w-0 flex-1 flex-col gap-6">
        <Section title="Role details" className="shrink-0">
          <Rows className="**:[[role=separator]]:bg-border">
            <Row label={<label htmlFor="role-name">Name</label>}>
              <DisabledTooltip disabled={draft.builtin} reason={ownerExplanation}>
                <TextField
                  id="role-name"
                  className="w-xl max-w-full border-border max-lg:w-sm"
                  placeholder="Role name…"
                  value={draft.name}
                  disabled={busy || draft.builtin}
                  onChange={(event) => c.updateRole({ name: event.target.value })}
                  onCommit={() => c.commitRoleField?.('name')}
                />
              </DisabledTooltip>
            </Row>
            <Row label={<label htmlFor="role-description">Description</label>} last>
              <DisabledTooltip disabled={draft.builtin} reason={ownerExplanation}>
                <TextField
                  id="role-description"
                  className="w-xl max-w-full border-border max-lg:w-sm"
                  placeholder="Describe this role…"
                  value={draft.description}
                  disabled={busy || draft.builtin}
                  onChange={(event) => c.updateRole({ description: event.target.value })}
                  onCommit={() => c.commitRoleField?.('description')}
                />
              </DisabledTooltip>
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
          <div className="relative grid min-w-0 w-full">
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
                position: draft.mode === 'visual' ? 'relative' : 'absolute',
                pointerEvents: draft.mode === 'visual' ? 'auto' : 'none',
              }}
              className="col-start-1 row-start-1 min-w-0 w-full"
            >
              <PolicyTable />
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
                position: draft.mode === 'cedar' ? 'relative' : 'absolute',
                pointerEvents: draft.mode === 'cedar' ? 'auto' : 'none',
              }}
              className="col-start-1 row-start-1 min-w-0 w-full"
            >
              <CedarEditor locked={busy || draft.builtin} owner={draft.builtin} />
            </motion.div>
          </div>
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
              className="w-32 rounded-xl"
              loading={busy}
              disabled={!draft.name.trim() || !draft.source.trim() || invalid}
              onClick={() => c.saveRole()}
            >
              Create role
            </Button>
          </div>
        ) : null}
        {draft.id ? (
          <Section title="Danger" className="shrink-0">
            <Rows>
              <Row label="Delete role" last>
                <DisabledTooltip disabled={draft.builtin} reason={ownerExplanation}>
                  <Button
                    variant="ghost"
                    className="text-error"
                    disabled={busy || draft.builtin}
                    onClick={() => shared.pages.dialogs.openDialog('delete-role', draft.id!)}
                  >
                    Delete role
                  </Button>
                </DisabledTooltip>
              </Row>
            </Rows>
          </Section>
        ) : null}
      </div>
    </SettingsContent>
  )
}
