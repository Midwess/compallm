<script lang="ts">
  import { Select as UI } from 'bits-ui'
  import { untrack } from 'svelte'
  import type { Snippet } from 'svelte'
  import { setSelect } from './select-context'
  import { fieldModel } from './form-context.svelte'
  let {
    items = [],
    name,
    value = $bindable(''),
    open = $bindable(false),
    defaultValue = '',
    defaultOpen = false,
    onOpenChange,
    disabled = false,
    onValueChange,
    children,
  }: {
    defaultValue?: string
    defaultOpen?: boolean
    onOpenChange?: (open: boolean) => void
    items?: { value: string; label: string }[]
    name?: string
    value?: string
    open?: boolean
    disabled?: boolean
    onValueChange?: (value: string) => void
    children?: Snippet
  } = $props()
  untrack(() => {
    if (!value) value = defaultValue
    if (defaultOpen) open = true
  })
  const form = fieldModel(() => name)
  $effect(() => {
    if (form && name) value = String(form.get(name) ?? '')
  })
  setSelect({
    get name() {
      return name
    },
    get invalid() {
      return !!(form && name && form.invalid(name))
    },
    get value() {
      return value
    },
    get items() {
      return items
    },
  })
</script>

<UI.Root
  type="single"
  bind:value
  bind:open
  {disabled}
  onOpenChange={(next) => {
    if (!next && form && name) form.blur(name)
    onOpenChange?.(next)
  }}
  onValueChange={(next) => {
    if (form && name) form.change(name, next)
    else onValueChange?.(next)
  }}
>
  {@render children?.()}
</UI.Root>
