<script lang="ts">
  import type { Snippet } from 'svelte'
  import { loadingPresentation } from './loading-presentation.svelte'
  let { pending, fallback, children }: { pending: boolean; fallback: Snippet; children: Snippet } = $props()
  const presentation = loadingPresentation(() => pending)
</script>

{#if pending || presentation.visible}{@render fallback()}{:else}{@render children()}{/if}
