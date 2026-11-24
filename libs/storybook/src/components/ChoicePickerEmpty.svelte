<script lang="ts">
  import type { Snippet } from 'svelte'
  import { getChoice } from './choice-picker-context'
  let { children }: { children?: Snippet } = $props()
  const scope = getChoice()
</script>

{#if !scope.options.length}<div class="px-3 py-4 text-center text-xs text-muted-foreground">
    {#if children}{@render children()}{:else}No results{/if}
  </div>{/if}
