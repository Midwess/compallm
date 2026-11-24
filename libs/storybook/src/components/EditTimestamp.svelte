<script lang="ts">
  import type { Snippet } from 'svelte'
  let { dateTime, children }: { dateTime: string; children?: Snippet } = $props()
</script>

<span class="text-ui-12 leading-4.5 text-muted-foreground">
  Last edited <time datetime={dateTime}>{@render children?.()}</time>
</span>
