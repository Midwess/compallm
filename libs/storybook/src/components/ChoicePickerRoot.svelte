<script lang="ts">
  import { untrack } from 'svelte'
  import { Combobox } from 'bits-ui'
  import type { Snippet } from 'svelte'
  import { setChoice, type ChoiceOption } from './choice-picker-context'
  import { fieldModel } from './form-context.svelte'
  let {
    options = [],
    value = $bindable<string[]>([]),
    open = $bindable(false),
    defaultOpen = false,
    defaultValue = [],
    onOpenChange,
    disabled = false,
    name,
    onValueChange,
    children,
  }: {
    options?: ChoiceOption[]
    value?: string[]
    defaultOpen?: boolean
    defaultValue?: string[]
    onOpenChange?: (open: boolean) => void
    open?: boolean
    disabled?: boolean
    name?: string
    onValueChange?: (value: string[]) => void
    children?: Snippet
  } = $props()
  untrack(() => {
    if (!value.length && defaultValue.length) value = defaultValue
    if (defaultOpen) open = true
  })
  const form = fieldModel(() => name)
  let query = $state('')
  let trigger = $state<HTMLElement | null>(null)
  $effect(() => {
    if (form && name) value = (form.get(name) as string[]) ?? []
  })
  function change(next: string[]) {
    value = next
    if (form && name) form.change(name, next)
    else onValueChange?.(next)
  }
  setChoice({
    get name() {
      return name
    },
    get invalid() {
      return !!(name && form?.invalid(name))
    },
    get trigger() {
      return trigger
    },
    setTrigger(node) {
      trigger = node
    },
    get options() {
      return options.filter((option) => option.label.toLowerCase().includes(query.toLowerCase()))
    },
    get selected() {
      return value
    },
    get open() {
      return open
    },
    get query() {
      return query
    },
    setQuery(next) {
      query = next
    },
    toggle(id) {
      change(value.includes(id) ? value.filter((v) => v !== id) : [...value, id])
    },
  })
</script>

<Combobox.Root
  type="multiple"
  bind:value
  bind:open
  {disabled}
  onValueChange={change}
  onOpenChange={(next) => {
    if (!next && name) form?.blur(name)
    onOpenChange?.(next)
  }}
>
  {@render children?.()}
</Combobox.Root>
