<script lang="ts">
  import { getLoadingContext } from '../context'
  import type { Snippet } from 'svelte'
  let { children }: { children?: Snippet } = $props()
  const loading = getLoadingContext()
</script>

<div class="contents" inert={loading.initial}>{@render children?.()}</div>
