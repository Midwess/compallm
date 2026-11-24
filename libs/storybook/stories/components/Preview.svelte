<script lang="ts">
  import type { Snippet } from 'svelte'
  let { children }: { children?: Snippet } = $props()
</script>

<main data-ui="component-preview" class="flex flex-wrap items-start gap-4 p-4 font-sans text-xsm text-foreground">
  {@render children?.()}
</main>
