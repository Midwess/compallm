<script lang="ts">
  import { interaction } from './motion'

  import type { HTMLButtonAttributes } from 'svelte/elements'
  import { cx } from './shared'
  let { selected = false, children, class: className, ...props }: HTMLButtonAttributes & { selected?: boolean } = $props()
</script>

<button
  use:interaction
  type="button"
  {...props}
  aria-pressed={selected}
  class={cx(
    'min-h-7 cursor-pointer rounded-full bg-transparent px-2 py-0 text-ui-12 text-muted-foreground aria-pressed:bg-tab-selection aria-pressed:text-selection-foreground focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring',
    className,
  )}
>
  {@render children?.()}
</button>
