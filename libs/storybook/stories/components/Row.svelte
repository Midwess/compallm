<script lang="ts">
  import type { Snippet } from 'svelte'
  let { children }: { children?: Snippet } = $props()
</script>

<div class="flex max-w-full flex-wrap items-center gap-2">{@render children?.()}</div>
