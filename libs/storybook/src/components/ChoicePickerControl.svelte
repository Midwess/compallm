<script lang="ts">
  import SearchboxRoot from './SearchboxRoot.svelte'
  import type { Snippet } from 'svelte'
  import * as ChoicePicker from './choice-picker-parts'
  import type { ChoiceOption } from './choice-picker-context'
  let {
    options,
    value = $bindable<string[]>([]),
    label = 'Select…',
    searchLabel = 'Search options',
    placeholder = 'Search…',
    emptyMessage = 'No results',
    name,
    emptyAction,
    open = $bindable(false),
    defaultOpen = false,
    defaultValue = [],
    onOpenChange,
    disabled = false,
    onValueChange,
  }: {
    options: ChoiceOption[]
    value?: string[]
    label?: string
    searchLabel?: string
    placeholder?: string
    emptyMessage?: string
    emptyAction?: Snippet
    open?: boolean
    defaultOpen?: boolean
    defaultValue?: string[]
    onOpenChange?: (open: boolean) => void
    name?: string
    disabled?: boolean
    onValueChange?: (value: string[]) => void
  } = $props()
</script>

<ChoicePicker.Root {options} bind:value bind:open {defaultOpen} {defaultValue} {onOpenChange} {name} {disabled} {onValueChange}>
  <ChoicePicker.Trigger aria-label={label}>
    {label}
  </ChoicePicker.Trigger><ChoicePicker.Content>
    <div class="p-2 pb-0"><SearchboxRoot><ChoicePicker.Input aria-label={searchLabel} {placeholder} /></SearchboxRoot></div>
    <ChoicePicker.Empty>
      <div>{emptyMessage}</div>
      {@render emptyAction?.()}
    </ChoicePicker.Empty><ChoicePicker.List>
      {#snippet children(item)}<ChoicePicker.Item value={item} />{/snippet}
    </ChoicePicker.List>
  </ChoicePicker.Content>
</ChoicePicker.Root>
