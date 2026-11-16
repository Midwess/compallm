/* eslint-disable react-hooks/refs -- Collection snapshots contain TanStack callback refs, attached only through React refs. */
import { useContext, useEffect, useRef, type ComponentPropsWithRef, type ReactNode } from 'react'
import { CollectionContext, CollectionRowContext } from '../context'
import { CollectionContent, CollectionRow, useCollectionOwner, useCollectionRows, useCollectionLoadMore } from './collection'
import type { CollectionPaging } from './collection-types'
import { cx, composeRefs } from './shared'

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
  estimateSize = 48,
  paging,
  ...props
}: Omit<ComponentPropsWithRef<'ul'>, 'children'> & {
  items: readonly T[]
  getKey(item: T): string
  children(item: T): ReactNode
  estimateSize?: number
  paging?: CollectionPaging
}) {
  const collection = useCollectionRows(items, getKey, estimateSize)
  const virtual = collection.virtualizer
  const bottom = (virtual.scrollOffset ?? 0) + (virtual.scrollRect?.height ?? 0) - virtual.options.scrollMargin
  useCollectionLoadMore(paging, collection.total - bottom, collection.revealing)
  return (
    <>
      {items.length ? (
        <button
          type="button"
          data-collection-entry
          className="sr-only"
          aria-label="First item"
          onFocus={(event) => collection.enter(false, event.relatedTarget)}
        />
      ) : null}
      <ul
        {...props}
        ref={collection.attach}
        onKeyDown={collection.onKeyDown}
        onFocusCapture={collection.onFocus}
        style={{ ...props.style, height: collection.total, position: 'relative' }}
      >
        {collection.rows.map((row) => (
          <CollectionRow
            key={row.key}
            index={row.index}
            measure={collection.measure}
            rowKey={row.key}
            retain={collection.retain}
            forms={collection.forms!}
          >
            <Item position={row.start} count={items.length}>
              <CollectionContent item={row.item} render={children} />
            </Item>
          </CollectionRow>
        ))}
      </ul>
      {items.length ? (
        <button
          type="button"
          data-collection-entry
          className="sr-only"
          aria-label="Last item"
          onFocus={(event) => collection.enter(true, event.relatedTarget)}
        />
      ) : null}
      {paging ? <LoadMore {...paging} /> : null}
    </>
  )
}
function Item({ position, count, children }: { position: number; count: number; children: ReactNode }) {
  const scope = useContext(CollectionRowContext)!
  const release = useRef<(() => void) | undefined>(undefined)
  useEffect(() => () => release.current?.(), [])
  return (
    <li
      ref={scope.measure}
      data-index={scope.index}
      data-collection-key={scope.key}
      aria-posinset={scope.index + 1}
      aria-setsize={count}
      onFocusCapture={() => {
        release.current ??= scope.retain({})
      }}
      onBlurCapture={(event) => {
        if (!event.currentTarget.contains(event.relatedTarget as Node | null)) {
          release.current?.()
          release.current = undefined
        }
      }}
      className={cx('absolute inset-x-0 top-0')}
      style={{ transform: `translateY(${position}px)` }}
    >
      {children}
    </li>
  )
}
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
