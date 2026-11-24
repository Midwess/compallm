<script lang="ts" generics="T">
  import type { HTMLAttributes } from 'svelte/elements'
  import { getVisibleColumns } from './table-context'

  import TableLoadingRows from './TableLoadingRows.svelte'
  import type { Snippet } from 'svelte'
  import CollectionRow from './CollectionRow.svelte'
  import TableLoadMore from './TableLoadMore.svelte'
  import type { CollectionPaging } from './collection-types'
  import { getLoadingContext } from '../context'
  import { optionalContext } from './form-context.svelte'
  let {
    items,
    getKey,
    children,
    paging,
    columns = 1,
    loadingRowClass,
    ...props
  }: Omit<HTMLAttributes<HTMLTableSectionElement>, 'children'> & {
    loadingRowClass?: string
    items?: readonly T[]
    getKey?: (item: T) => string
    children?: Snippet<[T, number]> | Snippet
    paging?: CollectionPaging
    columns?: number
  } = $props()
  const loading = optionalContext(getLoadingContext)
  const visibleColumns = getVisibleColumns()
</script>

<tbody
  {...props}
  data-slot="table-body"
  aria-busy={loading?.initial ? undefined : paging?.loading}
  aria-hidden={loading?.initial || undefined}
>
  {#if loading?.initial}<TableLoadingRows
      rowClass={loadingRowClass}
    />{:else if items}{#each items as item, index (getKey?.(item) ?? index)}<CollectionRow
        rowKey={getKey?.(item) ?? String(index)}
        {index}
      >
        {@render (children as Snippet<[T, number]>)?.(item, index)}
      </CollectionRow>{/each}{:else}{@render (children as Snippet)?.()}{/if}{#if paging}<TableLoadMore {...paging} />{/if}
  {#if items}<tr class="sr-only">
      <td colspan={visibleColumns() || columns} aria-live="polite">{items.length} rows loaded</td>
    </tr>{/if}
</tbody>
