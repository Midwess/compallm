import { useId, useState } from 'react'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { FunnelIcon } from '@phosphor-icons/react/dist/csr/Funnel'
import { Button } from '../../../components/buttons'
import { SearchField } from '../../../components/fields'
import { Popup } from '../../../components/popup'
import { Switch } from '../../../components/selection'
import { SectionSurface } from '../../../components/surfaces'
import { Table } from '../../../components/table'
import { useSettings } from '../controller'
import { useRole, useRoleController } from './controller'
import type { PolicyProjection, SettingsPolicyAction } from '../types'

const defaultExpanded = ['Models', 'Providers', 'Usage & limits']
const accessLevels = ['Read', 'Use', 'Write', 'Admin', 'Sensitive']

function PolicyFilters({ categories }: { categories: string[] }) {
  const c = useRoleController(),
    filters = useRole((s) => s.filters)
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
  const c = useRoleController()
  const disabledPolicies = useRole((state) => state.disabledPolicies?.[action.id])
  const permits = statements.filter((p) => p.effect !== 'forbid')
  const advanced = permits.some((p) => !p.editable)
  const denied = statements.some((p) => p.effect === 'forbid')
  const reason = permits.find((p) => !p.editable)?.reason ?? (advanced ? 'Edit this action in Cedar.' : undefined)
  return (
    <Table.Row id={id} className="[&>td]:border-b [&>td]:border-divider">
      <Table.Cell className="py-1.5 pl-6 pr-4">
        <span className="block truncate text-xsm leading-4.5 font-medium" title={action.description}>
          {action.label}
        </span>
        <span className="block truncate text-[11px] leading-4 text-muted">
          {action.id}
          {denied ? ' · Deny rule' : null}
          {statements.some((p) => p.resourceRestricted) ? ' · Advanced scope' : null}
        </span>
        {reason ? <span className="block text-xs text-muted">{reason}</span> : null}
        {reason ? (
          <Button variant="ghost" className="h-6 px-0 text-xs" onClick={() => c.setPolicyMode('cedar')}>
            Edit in Cedar
          </Button>
        ) : null}
        {!permits.length && disabledPolicies?.length ? (
          <details className="mt-1 text-xs text-muted">
            <summary className="cursor-pointer">Re-enabling restores these rules</summary>
            <p>Retained until you leave this role or edit the Cedar source.</p>
            {disabledPolicies.map((source) => (
              <pre key={source} className="mt-1 whitespace-pre-wrap wrap-break-word">
                {source}
              </pre>
            ))}
          </details>
        ) : null}
        {statements.length ? (
          <details className="mt-1 text-xs text-muted">
            <summary className="cursor-pointer">Scope and rules</summary>
            {[...new Set(statements.map((entry) => entry.source))].map((source) => (
              <pre key={source} className="mt-1 whitespace-pre-wrap wrap-break-word">
                {source}
              </pre>
            ))}
          </details>
        ) : null}
      </Table.Cell>
      <Table.Cell className="px-1 py-0 text-xs text-muted">{action.description}</Table.Cell>
      <Table.Cell className="px-2 py-0">
        <Switch
          aria-label={action.label}
          checked={permits.length > 0}
          disabled={disabled || advanced}
          onChange={() => c.togglePolicyAction(action.id)}
        />
      </Table.Cell>
    </Table.Row>
  )
}

export function PolicyTable() {
  const c = useRoleController(),
    actions = useSettings((s) => s.policyActions),
    draft = useRole((s) => s.draft),
    query = useRole((s) => s.query),
    filters = useRole((s) => s.filters),
    busy = useRole((s) => s.busy)
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
    const selected = (projection.get(a.id) ?? []).some((entry) => entry.effect !== 'forbid')
    return (
      (!normalized || `${a.label} ${a.id} ${a.description} ${a.category}`.toLocaleLowerCase().includes(normalized)) &&
      (!filters['policy-resource'] || filters['policy-resource'] === 'all' || filters['policy-resource'] === a.category) &&
      (!filters['policy-access'] || filters['policy-access'] === 'all' || filters['policy-access'] === a.access) &&
      (filters['policy-selection'] === 'enabled' ? selected : filters['policy-selection'] === 'disabled' ? !selected : true)
    )
  })
  const filtering = Boolean(normalized) || Object.values(filters).some((v) => v && v !== 'all')
  const disabled = busy || draft.parsing || draft.diagnostics.length > 0
  const selectedCount = actions.filter((a) => (projection.get(a.id) ?? []).some((entry) => entry.effect !== 'forbid')).length
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
      {draft.projection.some((entry) => !entry.action) ? (
        <div className="px-4 pb-4 text-xs text-muted">
          {[...new Map(draft.projection.filter((entry) => !entry.action).map((entry) => [entry.source, entry])).values()].map(
            (entry) => (
              <details key={entry.source} className="mt-2">
                <summary className="cursor-pointer">{entry.reason ?? 'Advanced Cedar statement'}</summary>
                <pre className="mt-1 whitespace-pre-wrap wrap-break-word">{entry.source}</pre>
                <Button variant="ghost" className="h-6 px-0 text-xs" onClick={() => c.setPolicyMode('cedar')}>
                  Edit in Cedar
                </Button>
              </details>
            ),
          )}
        </div>
      ) : null}
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
                <Table.Head className="px-1 py-0">Description</Table.Head>
                <Table.Head className="px-2 py-0">Allow</Table.Head>
              </Table.Row>
            </Table.Header>
            {categories.map((category, index) => {
              const matched = filtered.filter((a) => a.category === category)
              if (!matched.length) return null
              const all = actions.filter((a) => a.category === category)
              const selected = all.filter((a) => (projection.get(a.id) ?? []).some((p) => p.effect !== 'forbid')).length
              const editable = matched.filter(
                (a) => !(projection.get(a.id) ?? []).some((p) => p.effect !== 'forbid' && !p.editable),
              )
              const allEditableSelected =
                editable.length > 0 && editable.every((a) => (projection.get(a.id) ?? []).some((p) => p.effect !== 'forbid'))
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
