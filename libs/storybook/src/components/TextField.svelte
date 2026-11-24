<script lang="ts">
  import { interaction } from './motion'
  import Skeleton from './Skeleton.svelte'

  import type { HTMLInputAttributes } from 'svelte/elements'
  import { fieldModel } from './form-context.svelte'
  import { cx } from './shared'
  let {
    name,
    variant = 'default',
    class: className,
    value = $bindable(''),
    element = $bindable(),
    oninput,
    onblur,
    onkeydown,
    ...props
  }: HTMLInputAttributes & { variant?: 'default' | 'constant'; element?: HTMLInputElement } = $props()
  const context = fieldModel(() => name)
  let form = $derived(variant === 'constant' ? null : context)
  $effect(() => {
    if (form && name) value = String(form.get(name) ?? '')
  })
  const base = `block h-8 w-full min-w-0 rounded-xl border-(length:--border-hairline) border-input bg-inset px-1.5
py-1 text-left text-sm leading-5.5 text-foreground placeholder:text-muted-foreground placeholder:opacity-100
focus:border-muted-foreground/60 focus:outline-none aria-invalid:border-destructive disabled:cursor-not-allowed
disabled:opacity-50`
</script>

<Skeleton>
  <input
    use:interaction
    {...props}
    {name}
    bind:this={element}
    bind:value
    class={cx(
      base,
      variant === 'constant' &&
        'pointer-events-none cursor-default bg-elevated caret-transparent select-none disabled:cursor-default disabled:opacity-100',
      className,
    )}
    data-variant={variant}
    readonly={variant === 'constant' || props.readonly}
    disabled={variant === 'constant' || props.disabled}
    tabindex={variant === 'constant' ? -1 : props.tabindex}
    aria-invalid={form && name ? form.invalid(name) || undefined : props['aria-invalid']}
    aria-describedby={props['aria-describedby'] ?? (form && name ? `${name}-error` : undefined)}
    oninput={(event) => {
      if (form && name) form.change(name, event.currentTarget.value)
      else oninput?.(event)
    }}
    onblur={(event) => {
      if (form && name) form.blur(name)
      onblur?.(event)
    }}
    onkeydown={(event) => {
      onkeydown?.(event)
      if (event.defaultPrevented || event.isComposing) return
      if (event.key === 'Enter' && form && name) {
        event.preventDefault()
        form.enter(name)
      }
      if (event.key === 'Escape' && name && form) {
        event.preventDefault()
        form.escape(name)
      }
    }}
  />
</Skeleton>
