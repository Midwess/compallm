<script lang="ts">
  import { interaction } from './motion'

  import type { HTMLTextareaAttributes } from 'svelte/elements'
  import { fieldModel } from './form-context.svelte'
  import { cx } from './shared'
  let {
    name,
    element = $bindable(),
    value = $bindable(''),
    class: className,
    oninput,
    onblur,
    onkeydown,
    ...props
  }: HTMLTextareaAttributes & { autocorrect?: string; element?: HTMLTextAreaElement } = $props()
  const form = fieldModel(() => name)
  $effect(() => {
    if (form && name) value = String(form.get(name) ?? '')
  })
</script>

<textarea
  use:interaction
  {...props}
  {name}
  bind:this={element}
  bind:value
  class={cx(
    `block w-full min-w-0 rounded-xl border-(length:--border-hairline) border-input bg-inset px-1.5
py-1 text-left text-sm leading-5.5 text-foreground placeholder:text-muted-foreground placeholder:opacity-100
focus:border-muted-foreground/60 focus:outline-none aria-invalid:border-destructive disabled:cursor-not-allowed
disabled:opacity-50
h-auto min-h-16 resize-y`,
    className,
  )}
  aria-describedby={props['aria-describedby'] ?? (form && name ? `${name}-error` : undefined)}
  aria-invalid={form && name ? form.invalid(name) || undefined : props['aria-invalid']}
  oninput={(e) => {
    if (form && name) form.change(name, e.currentTarget.value)
    else oninput?.(e)
  }}
  onblur={(e) => {
    if (form && name) form.blur(name)
    onblur?.(e)
  }}
  onkeydown={(e) => {
    onkeydown?.(e)
    if (e.isComposing || e.defaultPrevented) return
    if (e.key === 'Enter' && (e.ctrlKey || e.metaKey) && name) {
      e.preventDefault()
      form?.enter(name)
    }
    if (e.key === 'Escape' && name && form) {
      e.preventDefault()
      form.escape(name)
    }
  }}></textarea>
