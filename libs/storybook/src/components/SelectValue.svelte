<script lang="ts">
  import { getSelect } from './select-context'
  import type { Snippet } from 'svelte'
  let { children, placeholder = 'Select…' }: { children?: Snippet; placeholder?: string } = $props()
  const scope = getSelect()
</script>

<span class="line-clamp-2 min-w-0 flex-1 whitespace-normal text-left">
  {#if children}{@render children()}{:else}{scope.items.find((item) => item.value === scope.value)?.label ??
      scope.value ??
      placeholder}{/if}
</span>
