import { useId, useState } from 'react'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { FunnelIcon } from '@phosphor-icons/react/dist/csr/Funnel'
import { Button } from '../../components/buttons'
import { SearchField } from '../../components/fields'
import { Popup } from '../../components/popup'
import { Switch } from '../../components/selection'
import { SectionSurface } from '../../components/surfaces'
import { Table } from '../../components/table'
import { useSettings, useSettingsController } from './controller'
import type { PolicyProjection, SettingsPolicyAction } from './types'

const defaultExpanded = ['Models', 'Providers', 'Usage & limits']
const accessLevels = ['Read', 'Use', 'Write', 'Admin', 'Sensitive']

function PolicyFilters({ categories }: { categories: string[] }) {
  const c = useSettingsController(),
    filters = useSettings((s) => s.filters)
  const options = (key: string, values: { value: string; label: string }[]) =>
    values.map(({ value, label }) => (
      <Popup.Item
        key={value}
        role="menuitemradio"
        aria-checked={(filters[key] || 'all') === value}
        className="justify-between gap-4"
        onClick={() => c.setFilter(key, value)}
      >
        {label}
        {(filters[key] || 'all') === value ? <CheckIcon aria-hidden="true" className="size-3" /> : null}
      </Popup.Item>
    ))
  return (
    <Popup>
      <Popup.IconTrigger aria-label="Filter policies" title="Filter policies" className="size-8 shrink-0 rounded-full">
        <FunnelIcon aria-hidden="true" className="size-3" />
      </Popup.IconTrigger>
      <Popup.Content aria-label="Policy filters">
        <Popup.Submenu label="Resource type">
          {options('policy-resource', [
            { value: 'all', label: 'All resources' },
            ...categories.map((value) => ({ value, label: value })),
          ])}
        </Popup.Submenu>
        <Popup.Submenu label="Selection">
          {options('policy-selection', [
            { value: 'all', label: 'All policies' },
            { value: 'enabled', label: 'Enabled' },
            { value: 'disabled', label: 'Disabled' },
          ])}
        </Popup.Submenu>
        <Popup.Submenu label="Access">
          {options('policy-access', [
            { value: 'all', label: 'All access levels' },
            ...accessLevels.map((value) => ({ value, label: value })),
          ])}
        </Popup.Submenu>
        <Popup.Separator />
        <Popup.Item
          onClick={() => {
            for (const key of ['policy-resource', 'policy-selection', 'policy-access']) c.setFilter(key, 'all')
          }}
        >
          Clear all filters
        </Popup.Item>
      </Popup.Content>
    </Popup>
  )
}

function PermissionRow({
  id,
  action,
  statements,
  disabled,
}: {
  id: string
  action: SettingsPolicyAction
  statements: PolicyProjection[]
  disabled: boolean
}) {
  const c = useSettingsController()
  const advanced = statements.some((p) => !p.editable || p.advanced)
  return (
    <Table.Row id={id} className="[&>td]:border-b [&>td]:border-divider">
      <Table.Cell className="py-1.5 pl-6 pr-4">
        <span className="block truncate text-xsm leading-4.5 font-medium" title={action.description}>
          {action.label}
        </span>
        <span className="block truncate text-[11px] leading-4 text-muted">{action.id}</span>
      </Table.Cell>
      <Table.Cell className="px-1 py-0 text-xs text-muted">
        {statements.some((p) => p.resourceRestricted)
          ? 'Advanced scope'
          : action.scope.trim().toLowerCase() === 'this organization'
            ? null
            : action.scope}
      </Table.Cell>
      <Table.Cell className="px-2 py-0">
        <Switch
          aria-label={action.label}
          checked={statements.length > 0}
          disabled={disabled || advanced}
          onChange={() => c.togglePolicyAction(action.id)}
        />
      </Table.Cell>
    </Table.Row>
  )
}

export function PolicyTable() {
  const c = useSettingsController(),
    actions = useSettings((s) => s.policyActions),
    draft = useSettings((s) => s.roleDraft),
    query = useSettings((s) => s.query),
    filters = useSettings((s) => s.filters),
    readonly = useSettings((s) => s.readonly),
    busy = useSettings((s) => s.busy)
  const [disclosure, setDisclosure] = useState<Record<string, boolean>>({})
  const id = useId()
  if (!draft) return null
  const projection = new Map<string, PolicyProjection[]>()
  for (const statement of draft.projection) {
    const entries = projection.get(statement.action) ?? []
    entries.push(statement)
    projection.set(statement.action, entries)
  }
  const categories = [...new Set(actions.map((a) => a.category))]
  const normalized = query.trim().toLocaleLowerCase()
  const filtered = actions.filter((a) => {
    const selected = projection.has(a.id)
    return (
      (!normalized || `${a.label} ${a.id} ${a.description} ${a.category}`.toLocaleLowerCase().includes(normalized)) &&
      (!filters['policy-resource'] || filters['policy-resource'] === 'all' || filters['policy-resource'] === a.category) &&
      (!filters['policy-access'] || filters['policy-access'] === 'all' || filters['policy-access'] === a.access) &&
      (filters['policy-selection'] === 'enabled' ? selected : filters['policy-selection'] === 'disabled' ? !selected : true)
    )
  })
  const filtering = Boolean(normalized) || Object.values(filters).some((v) => v && v !== 'all')
  const disabled = readonly || draft.builtin || busy || draft.parsing || draft.diagnostics.length > 0
  const selectedCount = actions.filter((a) => projection.has(a.id)).length
  return (
    <SectionSurface className="flex min-w-0 flex-col overflow-hidden p-0">
      <div className="flex shrink-0 flex-wrap items-center gap-2 p-4">
        <SearchField
          aria-label="Search policies"
          placeholder="Search policies…"
          className="min-w-0 flex-1 [&>input]:border-border"
          value={query}
          onChange={(event) => c.setQuery(event.target.value)}
        />
        <PolicyFilters categories={categories} />
        {(['policy-resource', 'policy-selection', 'policy-access'] as const).map((key) =>
          filters[key] && filters[key] !== 'all' ? (
            <Button key={key} variant="ghost" className="h-7 px-2 text-xs" onClick={() => c.setFilter(key, 'all')}>
              {filters[key]} <span aria-hidden="true">×</span>
              <span className="sr-only">Clear {key.replace('policy-', '')} filter</span>
            </Button>
          ) : null,
        )}
      </div>
      <div
        tabIndex={0}
        role="region"
        aria-label="Permission catalog"
        className="mx-2 min-w-0 overflow-x-auto outline-offset-2 focus-visible:outline-2 focus-visible:outline-focus"
      >
        <Table className="overflow-visible">
          <Table.Content aria-label="Permission policies" className="min-w-md border-separate border-spacing-0">
            <colgroup>
              <col className="w-[60%]" />
              <col />
              <col className="w-16" />
            </colgroup>
            <Table.Header className="sticky top-0 z-20 bg-inset">
              <Table.Row>
                <Table.Head className="px-2 py-0">Policy</Table.Head>
                <Table.Head className="px-1 py-0">Resource scope</Table.Head>
                <Table.Head className="px-2 py-0">Allow</Table.Head>
              </Table.Row>
            </Table.Header>
            {categories.map((category, index) => {
              const matched = filtered.filter((a) => a.category === category)
              if (!matched.length) return null
              const all = actions.filter((a) => a.category === category)
              const selected = all.filter((a) => projection.has(a.id)).length
              const editable = matched.filter((a) => !(projection.get(a.id) ?? []).some((p) => !p.editable))
              const allEditableSelected = editable.length > 0 && editable.every((a) => projection.has(a.id))
              const open = disclosure[category] ?? (filtering || defaultExpanded.includes(category))
              return (
                <Table.Category
                  key={category}
                  label={category}
                  count={selected + ' / ' + all.length}
                  open={open}
                  onOpenChange={(next) => setDisclosure((current) => ({ ...current, [category]: next }))}
                  action={
                    <Switch
                      aria-label={'Allow ' + category + ' policies'}
                      checked={allEditableSelected}
                      disabled={disabled || editable.length === 0}
                      onChange={() =>
                        c.setPolicyActions(
                          editable.map((a) => a.id),
                          !allEditableSelected,
                        )
                      }
                    />
                  }
                >
                  {matched.map((action) => (
                    <PermissionRow
                      key={action.id}
                      id={id + '-category-' + index + '-' + action.id}
                      action={action}
                      statements={projection.get(action.id) ?? []}
                      disabled={disabled}
                    />
                  ))}
                </Table.Category>
              )
            })}
            {!filtered.length ? (
              <Table.Body>
                <Table.Row>
                  <Table.Cell colSpan={3} className="py-8 text-center text-muted">
                    No policies match your search or filters.
                  </Table.Cell>
                </Table.Row>
              </Table.Body>
            ) : null}
          </Table.Content>
        </Table>
      </div>
      <p className="shrink-0 px-4 py-4 text-xs leading-4.5 text-muted" aria-live="polite">
        {selectedCount} selected · {filtered.length} policies
      </p>
    </SectionSurface>
  )
}
