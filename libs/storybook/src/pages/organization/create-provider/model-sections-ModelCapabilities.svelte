<script lang="ts">
  import TrashIcon from 'phosphor-svelte/lib/TrashIcon'
  import PlusIcon from 'phosphor-svelte/lib/PlusIcon'
  import { Popup } from '../../../components/popup'

  import { CapabilityIcon } from '../../../components/capability-icon'

  import { capabilityOptions } from './model-options'
  import type { ModelCapability } from './types'

  import { fieldState } from '../../page-state.svelte'
  import { ModelSection } from './model-sections'
  import { capabilityDescriptions } from './model-sections'

  const field = fieldState<ModelCapability[]>(() => 'modelDraft.capabilities')!
  let selected = $derived(field.value ?? [])
</script>

<ModelSection
  title="Capabilities"
  help={`
Declare the features this model actually supports, such as images or tool calling. These labels help discovery
and filtering; adding a capability does not enable it at the provider. Use the provider’s model documentation to
confirm support.
`}
>
  {#snippet action()}<Popup.Root>
      <Popup.Trigger class="flex h-8 w-auto items-center gap-1 rounded-lg bg-elevated px-2 text-xs">
        <PlusIcon class="size-3"></PlusIcon>Add capability
      </Popup.Trigger><Popup.Content align="start" class="w-80 text-left" aria-label="Add capability">
        {#each capabilityOptions as option (option.value)}<Popup.Item
            disabled={selected.includes(option.value)}
            class="h-auto min-h-10 justify-start gap-2 py-1.5 text-left"
            onSelect={() => field.change([...selected, option.value])}
          >
            <CapabilityIcon capability={option.value} class="size-3.5"></CapabilityIcon><span class="min-w-0">
              <span class="block">{option.label}</span><span class="block text-ui-11 text-muted-foreground">
                {capabilityDescriptions[option.value]}
              </span>
            </span>
          </Popup.Item>{/each}
      </Popup.Content>
    </Popup.Root>{/snippet}
  <div class="divide-y divide-divider rounded-control bg-inset">
    {#if selected.length}{#each selected as value (value)}<div
          class="flex min-h-14 items-center justify-between gap-3 px-4 py-2"
        >
          <div class="flex min-w-0 items-center gap-3">
            <CapabilityIcon capability={value} class="size-4"></CapabilityIcon>
            <div class="min-w-0">
              <p class="text-xsm">{capabilityOptions.find((o) => o.value === value)?.label}</p>
              <p class="text-xs text-muted-foreground">{capabilityDescriptions[value]}</p>
            </div>
          </div>
          <button
            type="button"
            aria-label={`Remove ${capabilityOptions.find((o) => o.value === value)?.label}`}
            class={`
flex size-6 shrink-0 items-center justify-center rounded text-muted-foreground hover:bg-hover
focus-visible:outline-ring
`}
            onclick={() => field.change(selected.filter((item) => item !== value))}
          >
            <TrashIcon class="size-3"></TrashIcon>
          </button>
        </div>{/each}{:else}<p class="px-4 py-5 text-xs text-muted-foreground">No capabilities added.</p>{/if}
  </div>
</ModelSection>
