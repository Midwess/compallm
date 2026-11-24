<script lang="ts">
  import type { Snippet } from 'svelte'
  let {
    change = 'context',
    lineNumber,
    children,
  }: { change?: 'context' | 'added' | 'removed'; lineNumber?: number; children?: Snippet } = $props()
</script>

<div
  data-change={change}
  class="flex min-h-5 gap-1 overflow-x-auto text-ui-12 whitespace-pre data-[change=added]:bg-success/12 data-[change=removed]:bg-destructive/12"
>
  <span class="min-w-8 text-right text-muted-foreground">{lineNumber ?? ''}</span><span
    aria-label={change === 'context' ? undefined : change}
  >
    {change === 'added' ? '+' : change === 'removed' ? '−' : ' '}
  </span><code>{@render children?.()}</code>
</div>
