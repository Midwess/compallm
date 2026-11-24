<script lang="ts">
  import { Button } from '../../components/buttons'

  import { SettingsContent, Heading, Section, Rows, Row } from './shared'
  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController())
  let org = $derived(readStore(__settings.store, (s) => s.organizationLimitUsd))
  let member = $derived(readStore(__settings.store, (s) => s.memberDefaultLimitUsd))
  let frequency = $derived(readStore(__settings.store, (s) => s.resetFrequency))
  let timezone = $derived(readStore(__settings.store, (s) => s.timezone))
  const amount = (v: string | null) => (v === null ? 'No limit set' : `$${v}`)
</script>

<SettingsContent>
  <Heading>Usage & limits</Heading>
  <div class="grid gap-12">
    <Section title="Reset schedule" description="Review the active accounting period.">
      <Rows>
        <Row label="Reset schedule" description={`${frequency} · ${timezone}`} last>
          <Button variant="ghost" onclick={() => c.pages.dialogs.openDialog('reset-schedule')}>Edit</Button>
        </Row>
      </Rows>
    </Section><Section title="Organization limit" description="Review the spending limit for this organization.">
      <Rows>
        <Row label="Default limit" description={amount(org)}>
          <Button variant="ghost" onclick={() => c.pages.dialogs.openDialog('organization-limit')}>Set limit</Button>
        </Row><Row label="Reset frequency" description="Monthly spending period" last>
          <span class="text-xs">{frequency}</span>
        </Row>
      </Rows>
    </Section><Section title="Member limits" description="Set defaults and review individual overrides.">
      <Rows>
        <Row label="Per-member limit" description={amount(member)}>
          <Button variant="ghost" onclick={() => c.pages.dialogs.openDialog('member-limit')}>Set limit</Button>
        </Row><Row label="Reset frequency" description="Monthly spending period">
          <span class="text-xs">{frequency}</span>
        </Row><Row label="Overrides" description="View and manage per-member limits and usage" last>
          <Button variant="ghost" onclick={() => c.navigate('member-limits')}>Manage overrides</Button>
        </Row>
      </Rows>
    </Section>
    <div class="px-4"><Button variant="ghost" onclick={() => c.navigate('usage-explorer')}>Usage explorer →</Button></div>
  </div>
</SettingsContent>
