<script lang="ts">
  import FunnelIcon from 'phosphor-svelte/lib/FunnelIcon'
  import { Popup } from '../../../components/popup'
  import { CapabilityIcon } from '../../../components/capability-icon'
  import { capabilityOptions, textModelOptions } from './model-options'
  import type { ModelStatusFilter, TextModelProtocol } from './types'
  import { untrack } from 'svelte'
  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'

  const __provider = getCreateProviderController()
  const controller = untrack(() => getCreateProviderController())
  let templatePreview = $derived(readStore(__provider.store, (state) => state.templatePreview))
  let status = $derived(readStore(__provider.store, (state) => state.modelStatusFilter))
  let protocol = $derived(readStore(__provider.store, (state) => state.modelProtocolFilter))
  let capabilities = $derived(readStore(__provider.store, (state) => state.modelCapabilityFilter))
</script>

<Popup.Root>
  <Popup.IconTrigger aria-label="Filter models" title="Filter models">
    <FunnelIcon class="size-3.5" aria-hidden="true"></FunnelIcon>
  </Popup.IconTrigger><Popup.Content aria-label="Model filters">
    {#if !templatePreview}<Popup.Sub>
        <Popup.SubTrigger>Enabled</Popup.SubTrigger><Popup.SubContent aria-label="Enabled">
          <Popup.RadioGroup
            value={status}
            onValueChange={(value) => {
              if (value === 'all' || value === 'enabled' || value === 'disabled')
                controller.onModelFiltersChanged?.({ status: value })
            }}
          >
            {#each ['all', 'enabled', 'disabled'] as ModelStatusFilter[] as value (value)}<Popup.RadioItem
                {value}
                disabled={!controller.onModelFiltersChanged}
              >
                {#if value === 'all'}All{:else}{#if value === 'enabled'}Enabled{:else}Disabled{/if}{/if}
              </Popup.RadioItem>{/each}
          </Popup.RadioGroup>
        </Popup.SubContent>
      </Popup.Sub>{:else}{/if}<Popup.Sub>
      <Popup.SubTrigger>Model type</Popup.SubTrigger><Popup.SubContent aria-label="Model type">
        <Popup.RadioGroup
          value={protocol}
          onValueChange={(value) => {
            if (value === 'all' || textModelOptions.some((option) => option.value === value))
              controller.onModelFiltersChanged?.({ protocol: value as TextModelProtocol | 'all' })
          }}
        >
          {#each [{ value: 'all' as const, label: 'All model types' }, ...textModelOptions] as option (option.value)}<Popup.RadioItem
              value={option.value}
              disabled={!controller.onModelFiltersChanged}
            >
              {option.label}
            </Popup.RadioItem>{/each}
        </Popup.RadioGroup>
      </Popup.SubContent>
    </Popup.Sub><Popup.Sub>
      <Popup.SubTrigger>Capabilities</Popup.SubTrigger><Popup.SubContent aria-label="Capabilities">
        {#each capabilityOptions as option (option.value)}<Popup.CheckboxItem
            checked={capabilities.includes(option.value)}
            disabled={!controller.onModelFiltersChanged}
            onCheckedChange={(checked) =>
              controller.onModelFiltersChanged?.({
                capabilities: checked
                  ? [...capabilities, option.value]
                  : capabilities.filter((value) => value !== option.value),
              })}
          >
            <CapabilityIcon capability={option.value} class="size-3.5"></CapabilityIcon>{option.label}
          </Popup.CheckboxItem>{/each}
      </Popup.SubContent>
    </Popup.Sub><Popup.Separator></Popup.Separator><Popup.Item
      class="text-muted-foreground"
      onSelect={() => controller.onClearModelFilters?.()}
      disabled={!controller.onClearModelFilters}
    >
      Clear all filters
    </Popup.Item>
  </Popup.Content>
</Popup.Root>
