<script lang="ts">
  import FunnelIcon from 'phosphor-svelte/lib/FunnelIcon'
  import { Popup } from '../../../components/popup'

  import { textModelOptions } from './model-options'
  import type { TextModelProtocol } from './types'
  import { untrack } from 'svelte'
  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'

  const __provider = getCreateProviderController()
  const controller = untrack(() => getCreateProviderController())
  let filter = $derived(readStore(__provider.store, (state) => state.providerProtocolFilter))
</script>

<Popup.Root>
  <Popup.IconTrigger aria-label="Filter providers" title="Filter providers">
    <FunnelIcon class="size-3.5" aria-hidden="true"></FunnelIcon>
  </Popup.IconTrigger><Popup.Content aria-label="Provider filters">
    <Popup.Sub>
      <Popup.SubTrigger>Model type</Popup.SubTrigger><Popup.SubContent aria-label="Model type">
        <Popup.RadioGroup
          value={filter}
          onValueChange={(value) => {
            if (value === 'all' || textModelOptions.some((option) => option.value === value))
              controller.onProviderFilterChanged?.(value as TextModelProtocol | 'all')
          }}
        >
          {#each [{ value: 'all' as const, label: 'All model types' }, ...textModelOptions] as option (option.value)}<Popup.RadioItem
              value={option.value}
              disabled={!controller.onProviderFilterChanged}
            >
              {option.label}
            </Popup.RadioItem>{/each}
        </Popup.RadioGroup>
      </Popup.SubContent>
    </Popup.Sub><Popup.Separator></Popup.Separator><Popup.Item
      class="text-muted-foreground"
      disabled={!controller.onProviderFilterChanged}
      onSelect={() => controller.onProviderFilterChanged?.('all')}
    >
      Clear all filters
    </Popup.Item>
  </Popup.Content>
</Popup.Root>
