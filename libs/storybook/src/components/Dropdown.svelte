<script lang="ts">
  import * as Select from './select-parts'
  import Content from './Content.svelte'
  import { fieldModel } from './form-context.svelte'
  import { untrack } from 'svelte'
  import { cx } from './shared'
  export interface SelectItem {
    value: string
    label: string
    disabled?: boolean
    description?: string
    icon?: import('./shared').Content
  }
  let {
    name,
    value = $bindable(''),
    defaultValue,
    options = [],
    label = 'Select…',
    'aria-label': ariaLabel,
    disabled = false,
    open = $bindable(false),
    defaultOpen = false,
    onOpenChange,
    onValueChange,
    class: className,
  }: {
    name?: string
    value?: string
    defaultValue?: string
    options: SelectItem[]
    label?: string
    'aria-label'?: string
    disabled?: boolean
    open?: boolean
    defaultOpen?: boolean
    onOpenChange?: (open: boolean) => void
    onValueChange?: (value: string) => void
    class?: string
  } = $props()
  untrack(() => {
    if (!value) value = defaultValue ?? options.find((item) => !item.disabled)?.value ?? ''
  })
  const form = fieldModel(() => name)
  let selected = $derived(form && name ? String(form.get(name) ?? '') : value)
  let item = $derived(options.find((item) => item.value === selected))
</script>

<div data-ui="dropdown" class={cx('relative inline-block w-60 max-w-full', className)}>
  <Select.Root {name} items={options} bind:value bind:open {defaultOpen} {disabled} {onValueChange} {onOpenChange}>
    <Select.Trigger
      aria-label={ariaLabel ?? label}
      aria-invalid={form && name ? form.invalid(name) || undefined : undefined}
      aria-describedby={form && name ? `${name}-error` : undefined}
    >
      <span class="line-clamp-2 min-w-0 flex-1 text-left"><Content value={item?.icon} />{item?.label ?? label}</span>
    </Select.Trigger>
    <Select.Content aria-label={ariaLabel ?? label}>
      {#each options as option (option.value)}<Select.Item value={option.value} label={option.label} disabled={option.disabled}>
          <Content value={option.icon} />{option.label}
        </Select.Item>{/each}
    </Select.Content>
  </Select.Root>
</div>
