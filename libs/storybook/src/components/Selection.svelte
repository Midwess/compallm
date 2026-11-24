<script lang="ts">
  import { interaction, selectionMotion } from './motion'

  import { untrack, onMount } from 'svelte'
  import type { HTMLInputAttributes } from 'svelte/elements'
  import CheckIcon from 'phosphor-svelte/lib/CheckIcon'
  import MinusIcon from 'phosphor-svelte/lib/MinusIcon'
  import { fieldModel } from './form-context.svelte'
  import { cx } from './shared'
  let {
    kind = 'checkbox',
    name,
    value = 'on',
    checked = $bindable(false),
    defaultChecked = false,
    indeterminate = false,
    size = 'standard',
    class: className,
    onchange,
    onblur,
    element = $bindable(),
    ...props
  }: Omit<HTMLInputAttributes, 'size'> & {
    kind?: 'checkbox' | 'radio' | 'switch'
    indeterminate?: boolean
    size?: 'compact' | 'standard'
    defaultChecked?: boolean
    element?: HTMLInputElement
  } = $props()
  untrack(() => {
    if (defaultChecked) checked = true
  })
  const form = fieldModel(() => name)
  let current = $derived(form && name ? form.get(name) : undefined)
  let selected = $derived(
    form
      ? Array.isArray(current)
        ? current.includes(String(value))
        : kind === 'radio'
          ? current === String(value)
          : Boolean(current)
      : checked,
  )
  $effect(() => {
    if (element) element.indeterminate = indeterminate
  })
  onMount(() => {
    if (!element) return
    const node = element
    node.defaultChecked = node.checked
    const reset = (event: Event) =>
      queueMicrotask(() => {
        if (!event.defaultPrevented) checked = node.checked
      })
    node.form?.addEventListener('reset', reset)
    return () => node.form?.removeEventListener('reset', reset)
  })
  function change(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
    const next = event.currentTarget.checked
    checked = next
    if (form && name)
      form.change(
        name,
        Array.isArray(current)
          ? next
            ? [...current, String(value)]
            : current.filter((item) => item !== String(value))
          : kind === 'radio'
            ? String(value)
            : next,
      )
    else onchange?.(event)
  }
</script>

<span
  class={cx(
    'relative inline-flex shrink-0 items-center justify-center has-disabled:opacity-50',
    kind === 'switch'
      ? 'h-8 w-10.5 flex-none align-middle'
      : kind === 'radio'
        ? 'size-4'
        : size === 'compact'
          ? 'size-5'
          : 'size-6',
  )}
>
  <input
    use:interaction
    use:selectionMotion={{ kind, indeterminate, selected: !!selected }}
    {...props}
    {name}
    {value}
    bind:this={element}
    checked={selected}
    type={kind === 'radio' ? 'radio' : 'checkbox'}
    role={kind === 'switch' ? 'switch' : undefined}
    aria-invalid={form && name ? form.invalid(name) || undefined : props['aria-invalid']}
    aria-describedby={props['aria-describedby'] ?? (form && name ? `${name}-error` : undefined)}
    onchange={change}
    onblur={(e) => {
      if (form && name) form.blur(name)
      onblur?.(e)
    }}
    class={cx(
      'peer absolute inset-0 size-full cursor-pointer appearance-none bg-transparent focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring disabled:cursor-not-allowed',
      kind === 'radio' ? 'rounded-full' : kind === 'switch' ? 'rounded-lg' : 'rounded',
      className,
    )}
  />
  <span
    data-selection-track
    aria-hidden="true"
    class={cx(
      'pointer-events-none absolute border border-input',
      kind === 'switch'
        ? 'top-1.5 left-1.5 h-5 w-7.5 rounded-full'
        : kind === 'radio'
          ? 'inset-0 rounded-full'
          : size === 'compact'
            ? 'inset-1 rounded-sm'
            : 'inset-0.75 rounded-sm',
      selected || indeterminate ? 'border-control bg-control' : kind === 'switch' ? 'bg-disabled' : 'bg-transparent',
    )}
  ></span>
  <span
    data-selection-indicator
    aria-hidden="true"
    class={cx(
      'pointer-events-none relative inline-flex text-control-foreground',
      kind === 'switch'
        ? 'absolute top-2.25 left-2.25 size-3.5 rounded-full bg-control-foreground'
        : kind === 'radio'
          ? 'size-1.5 rounded-full bg-control-foreground'
          : '',
    )}
  >
    {#if kind === 'checkbox'}{#if indeterminate}<MinusIcon
          class={size === 'compact' ? 'size-2.5' : 'size-3.5'}
        />{:else}<CheckIcon class={size === 'compact' ? 'size-2.5' : 'size-3.5'} />{/if}{/if}
  </span>
</span>
