<script lang="ts">
  import { DropdownMenu as UI } from 'bits-ui'
  import CheckIcon from 'phosphor-svelte/lib/CheckIcon'
  import type { ComponentProps, Snippet } from 'svelte'
  import { fieldModel } from './form-context.svelte'
  import { cx } from './shared'
  let {
    children,
    checked = $bindable(false),
    onCheckedChange,
    class: className,
    name,
    selectionValue = '',
    ...props
  }: Omit<ComponentProps<typeof UI.CheckboxItem>, 'children' | 'checked' | 'onCheckedChange'> & {
    children?: Snippet
    checked?: boolean
    onCheckedChange?: (checked: boolean) => void
    name?: string
    selectionValue?: string
  } = $props()
  const form = fieldModel(() => name)
  let value = $derived(form && name ? form.get(name) : undefined)
  let selected = $derived(form ? (Array.isArray(value) ? value.includes(selectionValue) : Boolean(value)) : checked)
  function change(next: boolean) {
    checked = next
    if (form && name)
      form.change(
        name,
        Array.isArray(value) ? (next ? [...value, selectionValue] : value.filter((item) => item !== selectionValue)) : next,
      )
    else onCheckedChange?.(next)
  }
</script>

<UI.CheckboxItem
  {...props}
  checked={selected}
  onCheckedChange={change}
  closeOnSelect={false}
  class={cx(
    'flex min-h-9 w-full cursor-pointer items-center gap-3 rounded-md px-2 text-xsm outline-none data-[highlighted]:bg-menu-hover focus-visible:outline-ring data-[disabled]:cursor-not-allowed data-[disabled]:opacity-50',
    className,
  )}
>
  <span aria-hidden="true" class="flex size-4 shrink-0 items-center justify-center rounded-sm border border-input">
    {#if selected}<CheckIcon class="size-3" />{/if}
  </span>{@render children?.()}
</UI.CheckboxItem>
