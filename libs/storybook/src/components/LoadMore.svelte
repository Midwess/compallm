<script lang="ts">
  import Button from './Button.svelte'
  import type { CollectionPaging } from './collection-types'
  let { hasMore, loading, error, onLoadMore }: CollectionPaging = $props()
</script>

{#if hasMore || error}<Button disabled={loading} {loading} onclick={onLoadMore}>
    {error ? 'Retry loading' : 'Load more'}
  </Button>{/if}
