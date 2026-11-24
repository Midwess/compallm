<script lang="ts">
  import { getCollectionContext, getLoadingContext } from '../context'
  import { optionalContext } from './form-context.svelte'
  import { getColumns, getVisibleColumns } from './table-context'
  import type { CollectionPaging } from './collection-types'
  import { loadMore } from './collection'
  let {
    hasMore,
    loading,
    error,
    onLoadMore,
    initialLoading = false,
  }: CollectionPaging & { initialLoading?: boolean } = $props()
  const collection = optionalContext(getCollectionContext)
  const region = optionalContext(getLoadingContext)
  const visibleColumns = getVisibleColumns()
  const columns = getColumns()
  let continuation = $derived({ hasMore, loading, error, onLoadMore, scrollElement: collection?.scrollElement })
</script>

{#if hasMore || error}<tr use:loadMore={initialLoading || region?.initial ? undefined : continuation}>
    <td colspan={visibleColumns() ?? (columns().length || 1)} class="h-8 p-0 text-center">
      <button
        type="button"
        disabled={loading}
        onclick={onLoadMore}
        class={error ? 'text-xs text-muted-foreground' : 'sr-only focus:not-sr-only'}
      >
        {error ? 'Retry loading' : 'Load more rows'}
      </button>
    </td>
  </tr>{/if}
