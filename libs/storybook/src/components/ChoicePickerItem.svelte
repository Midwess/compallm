<script lang="ts">
  import { Combobox } from 'bits-ui'
  import CheckIcon from 'phosphor-svelte/lib/CheckIcon'
  import type { Snippet } from 'svelte'
  import { cx } from './shared'
  import { getChoice, type ChoiceOption } from './choice-picker-context'
  let {
    value,
    disabled,
    children,
    class: className,
  }: { value: ChoiceOption; disabled?: boolean; children?: Snippet; class?: string } = $props()
  const scope = getChoice()
</script>

<Combobox.Item
  value={value.id}
  label={value.label}
  disabled={disabled ?? value.disabled}
  class={cx(
    'flex min-h-9 cursor-pointer items-center gap-2 rounded-md px-2 py-1.5 text-left text-xsm outline-none data-highlighted:bg-hover data-disabled:cursor-not-allowed data-disabled:opacity-50',
    className,
  )}
>
  {#if children}{@render children()}{:else}
    {#if value.color}<span class="size-1.5 shrink-0 rounded-full" style:background-color={value.color}></span>{/if}
    <span class="min-w-0 flex-1">
      <span class="block truncate">{value.label}</span>{#if value.description}<span
          class="block truncate text-xs text-muted-foreground"
        >
          {value.description}
        </span>{/if}
    </span>
    {#if scope.selected.includes(value.id)}<CheckIcon size={14} class="shrink-0" aria-hidden="true" />{/if}
  {/if}
</Combobox.Item>
