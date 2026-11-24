<script lang="ts">
  import type { Content } from './shared'
  let { value }: { value?: Content | null } = $props()
</script>

{#if typeof value === 'function'}{@render value()}{:else if value != null}{value}{/if}
