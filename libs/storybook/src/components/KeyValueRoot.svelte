<script lang="ts">
  import { setKeyValue, type KeyValueEntry } from './key-value-context'
  import { fieldModel } from './form-context.svelte'
  import { untrack, type Snippet } from 'svelte'
  import { cx } from './shared'
  let {
    name,
    value = $bindable<KeyValueEntry[]>([]),
    defaultValue = [],
    disabled = false,
    keyMaxLength,
    acceptEntry,
    onValueChange,
    children,
    class: className,
  }: {
    name?: string
    value?: KeyValueEntry[]
    defaultValue?: KeyValueEntry[]
    disabled?: boolean
    keyMaxLength?: number
    acceptEntry?: (entry: KeyValueEntry) => boolean
    onValueChange?: (entries: KeyValueEntry[]) => void
    children?: Snippet
    class?: string
  } = $props()
  const form = fieldModel(() => name)
  untrack(() => {
    if (value.length === 0 && defaultValue.length) value = defaultValue
  })
  setKeyValue({
    get entries() {
      return form && name ? ((form.get(name) as KeyValueEntry[]) ?? []) : value
    },
    get disabled() {
      return disabled
    },
    get keyMaxLength() {
      return keyMaxLength
    },
    acceptEntry: (entry) => acceptEntry?.(entry) ?? true,
    change(entries) {
      if (form && name) form.change(name, entries)
      else {
        value = entries
        onValueChange?.(entries)
      }
    },
  })
</script>

<div
  data-form-field={name}
  class={cx('w-full min-w-0 max-w-full', className)}
  onfocusout={(e) => {
    if (name && !e.currentTarget.contains(e.relatedTarget as Node)) form?.blur(name)
  }}
>
  {@render children?.()}
</div>
