<script lang="ts" generics="T">
  import type { HTMLAttributes } from 'svelte/elements'
  import type { Snippet } from 'svelte'
  import CollectionRow from './CollectionRow.svelte'
  import LoadMore from './LoadMore.svelte'
  import Skeleton from './Skeleton.svelte'
  import { getCollectionContext, getLoadingContext } from '../context'
  import { optionalContext } from './form-context.svelte'
  import { loadMore } from './collection'
  import type { CollectionPaging } from './collection-types'
  let {
    items,
    getKey,
    children,
    paging,
    ...props
  }: Omit<HTMLAttributes<HTMLUListElement>, 'children'> & {
    items: readonly T[]
    getKey: (item: T) => string
    children: Snippet<[T, number]>
    paging?: CollectionPaging
  } = $props()
  const collection = optionalContext(getCollectionContext)
  let continuation = $derived(paging ? { ...paging, scrollElement: collection?.scrollElement } : undefined)
  const loading = optionalContext(getLoadingContext)
</script>

<ul {...props}>
  {#if loading?.initial}{#each Array(4) as _, index (index)}<li aria-hidden="true" class="flex h-16 items-center px-4">
        <Skeleton loading><span class="inline-block h-4 w-3/4 rounded-md"></span></Skeleton>
      </li>{/each}{:else}
    {#each items as item, index (getKey(item))}<li
        data-collection-key={getKey(item)}
        aria-posinset={index + 1}
        aria-setsize={items.length}
      >
        <CollectionRow rowKey={getKey(item)} {index}>{@render children(item, index)}</CollectionRow>
      </li>{/each}
  {/if}
</ul>
{#if paging}<div use:loadMore={loading?.initial ? undefined : continuation}><LoadMore {...paging} /></div>{/if}
