<script lang="ts">
  import * as Tooltip from './tooltip-parts'
  import type { Snippet } from 'svelte'
  import { cx } from './shared'
  let {
    disabled,
    reason,
    allowInteraction = false,
    children,
  }: { disabled: boolean; reason: string; allowInteraction?: boolean; children?: Snippet } = $props()
</script>

{#if disabled}<Tooltip.Root>
    <Tooltip.Trigger>
      {#snippet child({ props })}<span
          {...props}
          tabindex={0}
          aria-label={reason}
          role="button"
          class={cx(allowInteraction ? 'inline-flex w-full max-w-full' : 'inline-flex max-w-full **:pointer-events-none')}
        >
          {@render children?.()}
        </span>{/snippet}
    </Tooltip.Trigger><Tooltip.Content>{reason}</Tooltip.Content>
  </Tooltip.Root>{:else}{@render children?.()}{/if}
