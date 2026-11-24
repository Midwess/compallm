<script lang="ts">
  import FunnelIcon from 'phosphor-svelte/lib/FunnelIcon'

  import { Popup } from '../../../components/popup'

  import { untrack } from 'svelte'
  import { readStore } from '../../page-state.svelte'
  import { getSettingsController } from '../../../context'

  import { accessLevels } from './policy-table'
  import ContentValue from '../../../components/Content.svelte'

  let {
    categories,
  }: {
    categories: string[]
  } = $props()
  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController().pages.role)
  let filters = $derived(readStore(__settings.pages.role.store, (s) => s.filters))
</script>

{#snippet options(key: string, values: { value: string; label: string }[])}<Popup.RadioGroup
    value={filters[key] || 'all'}
    onValueChange={(value: string) => c.setFilter(key, value)}
  >
    {#each values as { value, label } (value)}<Popup.RadioItem {value}><ContentValue value={label} /></Popup.RadioItem>{/each}
  </Popup.RadioGroup>{/snippet}
<Popup.Root>
  <Popup.IconTrigger aria-label="Filter policies" title="Filter policies" class="size-8 shrink-0 rounded-full">
    <FunnelIcon aria-hidden="true" class="size-3"></FunnelIcon>
  </Popup.IconTrigger><Popup.Content aria-label="Policy filters">
    <Popup.Sub>
      <Popup.SubTrigger>Resource type</Popup.SubTrigger><Popup.SubContent aria-label="Resource type">
        {@render options('policy-resource', [
          { value: 'all', label: 'All resources' },
          ...categories.map((value) => ({ value, label: value })),
        ])}
      </Popup.SubContent>
    </Popup.Sub><Popup.Sub>
      <Popup.SubTrigger>Selection</Popup.SubTrigger><Popup.SubContent aria-label="Selection">
        {@render options('policy-selection', [
          { value: 'all', label: 'All policies' },
          { value: 'enabled', label: 'Enabled' },
          { value: 'disabled', label: 'Disabled' },
        ])}
      </Popup.SubContent>
    </Popup.Sub><Popup.Sub>
      <Popup.SubTrigger>Access</Popup.SubTrigger><Popup.SubContent aria-label="Access">
        {@render options('policy-access', [
          { value: 'all', label: 'All access levels' },
          ...accessLevels.map((value) => ({ value, label: value })),
        ])}
      </Popup.SubContent>
    </Popup.Sub><Popup.Separator></Popup.Separator><Popup.Item
      onSelect={() => {
        for (const key of ['policy-resource', 'policy-selection', 'policy-access']) c.setFilter(key, 'all')
      }}
    >
      Clear all filters
    </Popup.Item>
  </Popup.Content>
</Popup.Root>
