import { memo, useCallback, useContext, useEffect, useMemo, useRef, useState, type ReactNode } from 'react'
import { CollectionContext, CollectionRowContext } from '../context'
import { CollectionForms } from './collection-forms'
import type { CollectionPaging } from './collection-types'

export function scrollOwner(element: HTMLElement): HTMLElement {
  let parent = element.parentElement
  while (parent) {
    const style = getComputedStyle(parent)
    if (/(auto|scroll)/.test(style.overflowY) && (style.overflowX !== 'auto' || parent.scrollHeight > parent.clientHeight + 1))
      return parent
    parent = parent.parentElement
  }
  return document.scrollingElement as HTMLElement
}

export function useCollectionOwner(suppliedScrollElement?: HTMLElement | null) {
  const [scrollElement, setScrollElement] = useState<HTMLElement | null>(null)
  const [forms] = useState(() => new CollectionForms())
  const attach = useCallback(
    (element: HTMLElement | null) => setScrollElement(element ? (suppliedScrollElement ?? scrollOwner(element)) : null),
    [suppliedScrollElement],
  )
  useEffect(() => forms.dispose, [forms])
  const scope = useMemo(() => ({ scrollElement, forms }), [scrollElement, forms])
  return { attach, scope }
}

export function useCollectionRows<T>(items: readonly T[], getKey: (item: T) => string) {
  const owner = useContext(CollectionContext)
  const rows = useMemo(() => items.map((item, index) => ({ item, index, key: getKey(item) })), [items, getKey])
  return { rows, forms: owner?.forms, scrollElement: owner?.scrollElement }
}

export const CollectionContent = memo(function CollectionContent<T>({ item, render }: { item: T; render(item: T): ReactNode }) {
  return render(item)
}) as <T>(props: { item: T; render(item: T): ReactNode }) => ReactNode

export interface CollectionRecordProps<T> {
  index: number
  rowKey: string
  item: T
  render(item: T): ReactNode
  forms: CollectionForms
}

export const CollectionRecord = memo(function CollectionRecord<T>({ item, render, ...scope }: CollectionRecordProps<T>) {
  return (
    <CollectionRow {...scope}>
      <CollectionContent item={item} render={render} />
    </CollectionRow>
  )
}) as <T>(props: CollectionRecordProps<T>) => ReactNode

export function CollectionRow({
  index,
  rowKey,
  forms,
  children,
}: {
  rowKey: string
  index: number
  forms: CollectionForms
  children: ReactNode
}) {
  const scope = useMemo(() => ({ index, key: rowKey, forms }), [index, rowKey, forms])
  return <CollectionRowContext value={scope}>{children}</CollectionRowContext>
}

export function useCollectionLoadMore(paging: CollectionPaging | undefined, scrollElement: HTMLElement | null | undefined) {
  const [sentinel, setSentinel] = useState<HTMLElement | null>(null)
  const callback = useRef(paging?.onLoadMore)
  useEffect(() => {
    callback.current = paging?.onLoadMore
  }, [paging?.onLoadMore])
  useEffect(() => {
    if (!sentinel || !scrollElement || !paging?.hasMore || paging.loading || paging.error) return
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          observer.disconnect()
          callback.current?.()
        }
      },
      { root: scrollElement === document.scrollingElement ? null : scrollElement, rootMargin: '160px' },
    )
    observer.observe(sentinel)
    return () => observer.disconnect()
  }, [sentinel, scrollElement, paging?.hasMore, paging?.loading, paging?.error])
  return setSentinel
}
