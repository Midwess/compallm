import { Skeleton, useInitialLoading } from './loading'
import { memo, type ComponentPropsWithRef, type ReactNode } from 'react'
import { CollectionContext } from '../context'
import { CollectionRecord, useCollectionOwner, useCollectionRows, useCollectionLoadMore } from './collection'
import type { CollectionPaging } from './collection-types'
import { composeRefs } from './shared'

function Root({ ref, scrollElement, ...props }: ComponentPropsWithRef<'div'> & { scrollElement?: HTMLElement | null }) {
  const owner = useCollectionOwner(scrollElement)
  return (
    <CollectionContext value={owner.scope}>
      <div {...props} ref={composeRefs(ref, owner.attach)} />
    </CollectionContext>
  )
}
function Items<T>({
  items,
  getKey,
  children,
  paging,
  ...props
}: Omit<ComponentPropsWithRef<'ul'>, 'children'> & {
  items: readonly T[]
  getKey(item: T): string
  children(item: T): ReactNode
  paging?: CollectionPaging
}) {
  const initial = useInitialLoading()
  const collection = useCollectionRows(items, getKey)
  const loadMoreRef = useCollectionLoadMore(paging, collection.scrollElement)
  return (
    <>
      <ul {...props}>
        {initial
          ? Array.from({ length: 4 }, (_, index) => (
              <li key={index} aria-hidden="true" className="flex h-16 items-center px-4">
                <Skeleton loading>
                  <span className="h-4 w-3/4 rounded-md" />
                </Skeleton>
              </li>
            ))
          : collection.rows.map((row) => (
              <ListRecord
                key={row.key}
                item={row.item}
                render={children}
                rowKey={row.key}
                index={row.index}
                count={items.length}
                forms={collection.forms!}
              />
            ))}
      </ul>
      {paging ? (
        <div ref={loadMoreRef} className="min-h-0.5">
          <LoadMore {...paging} />
        </div>
      ) : null}
    </>
  )
}
const ListRecord = memo(function ListRecord<T>({
  item,
  render,
  rowKey,
  index,
  count,
  forms,
}: {
  item: T
  render(item: T): ReactNode
  rowKey: string
  index: number
  count: number
  forms: import('./collection-forms').CollectionForms
}) {
  return (
    <li data-collection-key={rowKey} aria-posinset={index + 1} aria-setsize={count}>
      <CollectionRecord index={index} rowKey={rowKey} forms={forms} item={item} render={render} />
    </li>
  )
}) as <T>(props: {
  item: T
  render(item: T): ReactNode
  rowKey: string
  index: number
  count: number
  forms: import('./collection-forms').CollectionForms
}) => ReactNode
function LoadMore({ hasMore, loading, error, onLoadMore }: CollectionPaging) {
  if (!hasMore) return null
  return (
    <button
      type="button"
      disabled={loading}
      onClick={onLoadMore}
      className={error ? 'text-xs text-muted-foreground' : 'sr-only focus:not-sr-only'}
    >
      {error ? 'Retry loading' : 'Load more items'}
    </button>
  )
}
export const List = { Root, Items, LoadMore }
