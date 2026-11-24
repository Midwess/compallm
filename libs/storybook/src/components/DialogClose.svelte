<script lang="ts">
  import XIcon from 'phosphor-svelte/lib/XIcon'
  import type { HTMLButtonAttributes } from 'svelte/elements'
  import { getDialog } from './dialog-context'
  import { cx } from './shared'
  import { interaction } from './motion'
  let { children, onclick, class: className, ...props }: HTMLButtonAttributes = $props()
  const scope = getDialog()
</script>

<button
  type="button"
  aria-label="Close dialog"
  {...props}
  class={cx('cursor-pointer rounded text-muted-foreground hover:text-foreground focus-visible:outline-ring', className)}
  use:interaction
  onclick={(event) => {
    onclick?.(event)
    if (!event.defaultPrevented && !props.disabled) scope.close()
  }}
>
  {#if children}{@render children()}{:else}<XIcon aria-hidden="true" class="size-4" />{/if}
</button>
