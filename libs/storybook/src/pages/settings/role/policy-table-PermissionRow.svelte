<script lang="ts">
  import { PermissionRequest } from '../../../components/permission'

  import { Button } from '../../../components/buttons'

  import { Switch } from '../../../components/selection'

  import { Table } from '../../../components/table'
  import { DisabledTooltip } from '../../../components/feedback'
  import type { PolicyProjection, SettingsPolicyAction } from '../types'
  import { untrack } from 'svelte'

  import { readStore } from '../../page-state.svelte'
  import { getSettingsController } from '../../../context'

  import { ownerExplanation } from './policy-table'

  let {
    id,
    action,
    statements,
    disabled,
    owner,
    busy,
  }: {
    id: string
    action: SettingsPolicyAction
    statements: PolicyProjection[]
    disabled: boolean
    owner: boolean
    busy: boolean
  } = $props()
  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController().pages.role)
  let disabledPolicies = $derived(readStore(__settings.pages.role.store, (state) => state.disabledPolicies?.[action.id]))
  let permits = $derived(statements.filter((p) => p.effect !== 'forbid'))
  let advanced = $derived(permits.some((p) => !p.editable))
  let denied = $derived(statements.some((p) => p.effect === 'forbid'))
  let reason = $derived(permits.find((p) => !p.editable)?.reason ?? (advanced ? 'Edit this action in Cedar.' : undefined))
</script>

<Table.Row {id} class="[&>td]:border-b [&>td]:border-divider">
  <Table.Cell class="py-1.5 pl-6 pr-4">
    <span class="block truncate text-xsm leading-4.5 font-medium" title={action.description}>{action.label}</span><span
      class="block truncate text-ui-11 leading-4 text-muted-foreground"
    >
      {action.id}{#if denied}
        · Deny rule{:else}{/if}{#if statements.some((p) => p.resourceRestricted)}
        · Advanced scope{:else}{/if}
    </span>{#if reason}<span class="block text-xs text-muted-foreground">{reason}</span>{:else}{/if}{#if reason}<Button
        variant="ghost"
        class="h-6 px-0 text-xs"
        onclick={() => c.setPolicyMode('cedar')}
      >
        Edit in Cedar
      </Button>{:else}{/if}{#if !permits.length && disabledPolicies?.length}<details class="mt-1 text-xs text-muted-foreground">
        <summary class="cursor-pointer">Re-enabling restores these rules</summary>
        <p>Retained until you leave this role or edit the Cedar source.</p>
        {#each disabledPolicies as source (source)}<pre class="mt-1 whitespace-pre-wrap wrap-break-word">{source}</pre>{/each}
      </details>{:else}{/if}
  </Table.Cell><Table.Cell class="px-1 py-0 text-xs text-muted-foreground">{action.description}</Table.Cell><Table.Cell
    class="px-0 py-0"
  >
    <div class="flex justify-center">
      <DisabledTooltip disabled={owner} reason={ownerExplanation}>
        <PermissionRequest actions={['role.policy.update']} resource={{ type: 'Role', id: c.store.getState().draft?.id }}>
          {#snippet children(allowed)}<DisabledTooltip
              disabled={!allowed}
              reason="You do not have permission to perform this action."
            >
              <Switch
                aria-label={action.label}
                checked={permits.length > 0}
                disabled={!allowed || disabled || advanced}
                aria-disabled={!allowed || disabled || advanced || busy}
                onchange={() => {
                  if (!busy) void c.store.form.perform(() => c.togglePolicyAction(action.id), ['draft.source'])
                }}
              ></Switch>
            </DisabledTooltip>{/snippet}
        </PermissionRequest>
      </DisabledTooltip>
    </div>
  </Table.Cell>
</Table.Row>
