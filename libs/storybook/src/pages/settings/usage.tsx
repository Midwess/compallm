import { useState } from 'react'
import { SlidersHorizontalIcon } from '@phosphor-icons/react/dist/csr/SlidersHorizontal'
import { Button } from '../../components/buttons'
import { Select } from '../../components/dropdowns'
import { SearchField } from '../../components/fields'
import { Table } from '../../components/table'
import { Popup } from '../../components/popup'
import { SettingsContent, Heading, Section, Rows, Row } from './shared'
import { useSettings, useSettingsController } from './controller'

export function UsageLimitSettings() {
  const c = useSettingsController(),
    org = useSettings((s) => s.organizationLimitUsd),
    member = useSettings((s) => s.memberDefaultLimitUsd)
  const frequency = useSettings((s) => s.resetFrequency),
    timezone = useSettings((s) => s.timezone)
  const amount = (v: string | null) => (v === null ? 'No limit set' : `$${v}`)
  return (
    <SettingsContent>
      <Heading>Usage & limits</Heading>
      <div className="grid gap-12">
        <Section title="Reset schedule" description="Review the active accounting period.">
          <Rows>
            <Row label="Reset schedule" description={`${frequency} · ${timezone}`} last>
              <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('reset-schedule')}>
                Edit
              </Button>
            </Row>
          </Rows>
        </Section>
        <Section title="Organization limit" description="Review the spending limit for this organization.">
          <Rows>
            <Row label="Default limit" description={amount(org)}>
              <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('organization-limit')}>
                Set limit
              </Button>
            </Row>
            <Row label="Reset frequency" description="Monthly spending period" last>
              <span className="text-xs">{frequency}</span>
            </Row>
          </Rows>
        </Section>
        <Section title="Member limits" description="Set defaults and review individual overrides.">
          <Rows>
            <Row label="Per-member limit" description={amount(member)}>
              <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('member-limit')}>
                Set limit
              </Button>
            </Row>
            <Row label="Reset frequency" description="Monthly spending period">
              <span className="text-xs">{frequency}</span>
            </Row>
            <Row label="Overrides" description="View and manage per-member limits and usage" last>
              <Button variant="ghost" onClick={() => c.navigate('member-limits')}>
                Manage overrides
              </Button>
            </Row>
          </Rows>
        </Section>
        <div className="px-4">
          <Button variant="ghost" onClick={() => c.navigate('usage-explorer')}>
            Usage explorer →
          </Button>
        </div>
      </div>
    </SettingsContent>
  )
}
export function MemberLimitSettings() {
  const c = useSettingsController(),
    members = useSettings((s) => s.members),
    query = useSettings((s) => s.query)
  const defaultLimit = useSettings((s) => s.memberDefaultLimitUsd),
    [columns, setColumns] = useState(['previous', 'current', 'used'])
  const visible = members.filter((m) => `${m.name} ${m.email}`.toLowerCase().includes(query.toLowerCase()))
  const money = (value: string | null) => (value === null ? '—' : `$${value}`)
  return (
    <SettingsContent wide>
      <Heading description="Review individual monthly limits and usage.">Member limit overrides</Heading>
      <div className="mb-6 flex items-center justify-between gap-4 px-4">
        <SearchField
          aria-label="Search member limits"
          placeholder="Search by name…"
          value={query}
          onChange={(e) => c.pages.search.setQuery(e.target.value)}
          className="w-75 max-w-full"
        />
        <Popup.Root>
          <Popup.Trigger aria-label="Display properties">
            <SlidersHorizontalIcon className="size-4" />
          </Popup.Trigger>
          <Popup.Content aria-label="Display properties" className="w-60">
            {['previous', 'current', 'used'].map((key) => (
              <Popup.CheckboxItem
                key={key}
                checked={columns.includes(key)}
                onCheckedChange={() =>
                  setColumns((items) => (items.includes(key) ? items.filter((item) => item !== key) : [...items, key]))
                }
              >
                {{ previous: 'Last month', current: 'This month', used: 'Limit used' }[key]}
              </Popup.CheckboxItem>
            ))}
          </Popup.Content>
        </Popup.Root>
      </div>
      <Table.Root>
        <Table.Content aria-label="Member spending limits" className="min-w-180">
          <Table.Header>
            <Table.Row>
              <Table.Head>Name</Table.Head>
              {columns.includes('previous') ? <Table.Head>Last month</Table.Head> : null}
              {columns.includes('current') ? <Table.Head>This month</Table.Head> : null}
              <Table.Head>Limit (monthly)</Table.Head>
              {columns.includes('used') ? <Table.Head>Limit used</Table.Head> : null}
            </Table.Row>
          </Table.Header>
          <Table.Body>
            {visible.map((m) => (
              <Table.Row key={m.id}>
                <Table.Cell>
                  <span className="block text-xsm font-medium">{m.name}</span>
                  <span className="text-muted-foreground">{m.email}</span>
                </Table.Cell>
                {columns.includes('previous') ? <Table.Cell>{money(m.previousUsageUsd)}</Table.Cell> : null}
                {columns.includes('current') ? <Table.Cell>{money(m.currentUsageUsd)}</Table.Cell> : null}
                <Table.Cell>
                  <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('member-limit', m.id)}>
                    {m.limitUsd === null
                      ? defaultLimit === null
                        ? 'Organization default'
                        : `$${defaultLimit} · default`
                      : money(m.limitUsd)}
                  </Button>
                </Table.Cell>
                {columns.includes('used') ? (
                  <Table.Cell>
                    {m.currentUsageUsd !== null && Number(m.limitUsd ?? defaultLimit) > 0
                      ? `${Math.round((Number(m.currentUsageUsd) / Number(m.limitUsd ?? defaultLimit)) * 100)}%`
                      : '—'}
                  </Table.Cell>
                ) : null}
              </Table.Row>
            ))}
          </Table.Body>
        </Table.Content>
      </Table.Root>
    </SettingsContent>
  )
}
function ReportFilter({ name, options }: { name: string; options: string[] }) {
  const c = useSettingsController(),
    value = useSettings((s) => s.filters[name] ?? 'all')
  return (
    <Select.Control
      aria-label={name}
      className="min-w-36 flex-1"
      value={value}
      options={[{ value: 'all', label: `All ${name.toLowerCase()}` }, ...options.map((label) => ({ value: label, label }))]}
      onValueChange={(v) => c.pages.search.setFilter(name, v)}
    />
  )
}
export function UsageExplorerSettings() {
  const data = useSettings((s) => s.usage),
    models = useSettings((s) => s.models),
    members = useSettings((s) => s.members)
  const keys = useSettings((s) => s.apiKeys)
  return (
    <SettingsContent wide>
      <Heading description="Explore model usage and provider measurements for Compallm.">Usage explorer</Heading>
      <div className="mb-8 flex flex-wrap gap-4 px-4">
        <ReportFilter name="Models" options={models} />
        <ReportFilter name="Members" options={members.map((m) => m.name)} />
        <ReportFilter name="API keys" options={keys.map((k) => k.name)} />
        <ReportFilter name="Providers" options={[]} />
      </div>
      <div className="mb-12 grid grid-cols-2 gap-4 px-4 lg:grid-cols-4">
        {['Client requests', 'Input tokens', 'Output tokens', 'Provider cost estimate'].map((label) => (
          <div key={label} className="rounded-control bg-inset p-4">
            <p className="text-xs text-muted-foreground">{label}</p>
            <p className="mt-5 text-2xl">—</p>
          </div>
        ))}
      </div>
      <Section title="Usage by model" description="Token measurements and estimated upstream costs.">
        <Table.Root>
          <Table.Content aria-label="Usage by model" className="min-w-180">
            <Table.Header>
              <Table.Row>
                <Table.Head>Model</Table.Head>
                <Table.Head>Requests</Table.Head>
                <Table.Head>Input tokens</Table.Head>
                <Table.Head>Output tokens</Table.Head>
                <Table.Head>Provider cost</Table.Head>
              </Table.Row>
            </Table.Header>
            <Table.Body>
              {data.map((row) => (
                <Table.Row key={row.id}>
                  <Table.Cell>{row.model}</Table.Cell>
                  <Table.Cell>{row.requests}</Table.Cell>
                  <Table.Cell>{row.inputTokens}</Table.Cell>
                  <Table.Cell>{row.outputTokens}</Table.Cell>
                  <Table.Cell>{row.providerCost}</Table.Cell>
                </Table.Row>
              ))}
            </Table.Body>
          </Table.Content>
        </Table.Root>
        {!data.length ? (
          <Table.Empty>No usage to display. Usage measurements will appear when records are available.</Table.Empty>
        ) : null}
      </Section>
      <p className="mt-6 px-4 text-xs leading-5 text-muted-foreground">
        Unknown measurements are shown as —. Provider cost is an estimate from saved rates.
      </p>
    </SettingsContent>
  )
}
export function AuditSettings() {
  const c = useSettingsController(),
    data = useSettings((s) => s.audit),
    query = useSettings((s) => s.query)
  const actors = useSettings((s) => s.members),
    filters = useSettings((s) => s.filters)
  const visible = data.filter(
    (e) =>
      `${e.actor} ${e.action} ${e.resource}`.toLowerCase().includes(query.toLowerCase()) &&
      (!filters.Actors || filters.Actors === 'all' || e.actor === filters.Actors) &&
      (!filters.Actions || filters.Actions === 'all' || e.action === filters.Actions),
  )
  return (
    <SettingsContent wide>
      <Heading description="Review security and permission changes in Compallm.">Audit log</Heading>
      <div className="mb-8 flex flex-wrap gap-4 px-4">
        <SearchField
          aria-label="Search events"
          placeholder="Search events…"
          value={query}
          onChange={(e) => c.pages.search.setQuery(e.target.value)}
          className="w-xs max-w-full"
        />
        <ReportFilter name="Actors" options={actors.map((m) => m.name)} />
        <ReportFilter name="Actions" options={[...new Set(data.map((e) => e.action))]} />
      </div>
      <Table.Root>
        <Table.Content aria-label="Audit events" className="min-w-160">
          <Table.Header>
            <Table.Row>
              {['Actor', 'Action', 'Resource', 'Result'].map((label) => (
                <Table.Head key={label}>{label}</Table.Head>
              ))}
            </Table.Row>
          </Table.Header>
          <Table.Body>
            {visible.map((e) => (
              <Table.Row key={e.id}>
                <Table.Cell>{e.actor}</Table.Cell>
                <Table.Cell>{e.action}</Table.Cell>
                <Table.Cell>{e.resource}</Table.Cell>
                <Table.Cell>{e.result}</Table.Cell>
              </Table.Row>
            ))}
          </Table.Body>
        </Table.Content>
      </Table.Root>
      {!visible.length ? (
        <Table.Empty>No events to display. Security and permission events will appear here.</Table.Empty>
      ) : null}
    </SettingsContent>
  )
}
