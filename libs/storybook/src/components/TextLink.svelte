<script lang="ts">
  import type { HTMLAnchorAttributes } from 'svelte/elements'
  import { cx } from './shared'
  import { interaction } from './motion'
  let {
    children,
    tone = 'secondary',
    disabled = false,
    class: className,
    href,
    onclick,
    ...props
  }: HTMLAnchorAttributes & { tone?: 'secondary' | 'muted'; disabled?: boolean } = $props()
</script>

<a
  {...props}
  href={disabled ? undefined : href}
  aria-disabled={disabled || undefined}
  tabindex={disabled ? -1 : props.tabindex}
  onclick={(e) => {
    if (disabled) e.preventDefault()
    else onclick?.(e)
  }}
  class={cx(
    'text-ui-14 [text-decoration-line:var(--content-link-decoration)] underline-offset-[3px] hover:underline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring aria-disabled:cursor-not-allowed aria-disabled:text-disabled',
    tone === 'muted' ? 'text-muted-foreground' : 'text-ring',
    className,
  )}
  use:interaction
>
  {@render children?.()}
</a>
