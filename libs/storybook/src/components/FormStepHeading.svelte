<script lang="ts">
  import FormStepMarker from './FormStepMarker.svelte'
  import type { Snippet } from 'svelte'
  let { step, children }: { step: number; children?: Snippet } = $props()
</script>

<h2 class="flex items-center gap-1.5 text-ui-16 leading-6 font-medium">
  <FormStepMarker>{step}</FormStepMarker>{@render children?.()}
</h2>
