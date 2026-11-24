<script lang="ts">
  import { interaction } from './motion'

  import type { HTMLButtonAttributes } from 'svelte/elements'
  import { cx } from './shared'
  let { label, children, class: className, ...props }: HTMLButtonAttributes & { label: string } = $props()
</script>

<button
  use:interaction
  type="button"
  {...props}
  aria-label={label}
  title={label}
  class={cx(
    'inline-flex size-7 flex-none cursor-pointer items-center justify-center rounded-full bg-transparent text-muted-foreground hover:bg-hover hover:text-foreground focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring disabled:cursor-not-allowed disabled:opacity-50',
    className,
  )}
>
  {@render children?.()}
</button>
