<script lang="ts">
  import * as Tooltip from './tooltip-parts'
  import QuestionIcon from 'phosphor-svelte/lib/QuestionIcon'
  import type { Snippet } from 'svelte'
  import { interaction } from './motion'
  let { label, children }: { label: string; children?: Snippet } = $props()
</script>

<Tooltip.Root>
  <Tooltip.Trigger>
    {#snippet child({ props })}<button
        {...props}
        type="button"
        aria-label={label}
        class="inline-flex size-5 shrink-0 cursor-help items-center justify-center rounded text-muted-foreground hover:text-foreground focus-visible:text-foreground focus-visible:outline-1 focus-visible:outline-muted-foreground"
        use:interaction
      >
        <QuestionIcon class="size-4" aria-hidden="true" />
      </button>{/snippet}
  </Tooltip.Trigger><Tooltip.Content class="rounded-xl text-left leading-5">{@render children?.()}</Tooltip.Content>
</Tooltip.Root>
