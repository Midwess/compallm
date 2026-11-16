import { CollectionViewport } from './collection-viewport'
import { useVirtualizer, defaultRangeExtractor } from '@tanstack/react-virtual'
import { memo, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState, type ReactNode } from 'react'
import { CollectionContext, CollectionRowContext } from '../context'
import { CollectionForms } from './collection-forms'
import { yieldToBrowser } from './scheduling'
import type { CollectionPaging } from './collection-types'

const controlsSelector =
  'button:not(:disabled), a[href], input:not(:disabled), select:not(:disabled), ' + 'textarea:not(:disabled), [tabindex="0"]'

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
  const [viewport] = useState(() => new CollectionViewport())
  const attach = useCallback(
    (element: HTMLElement | null) => setScrollElement(element ? (suppliedScrollElement ?? scrollOwner(element)) : null),
    [suppliedScrollElement],
  )
  useEffect(() => forms.dispose, [forms])
  useLayoutEffect(() => {
    if (scrollElement) viewport.connect(scrollElement)
    return viewport.dispose
  }, [scrollElement, viewport])
  const scope = useMemo(() => ({ scrollElement, forms, viewport }), [scrollElement, forms, viewport])
  return { attach, scope }
}

export function useRowRetention(open: boolean) {
  const row = useContext(CollectionRowContext)
  useEffect(() => (open ? row?.retain({}) : undefined), [row, open])
}

export function useCollectionRows<T>(items: readonly T[], getKey: (item: T) => string, estimateSize: number, leading = 0) {
  const owner = useContext(CollectionContext)
  const [element, setElement] = useState<HTMLElement | null>(null)
  const [offset, setOffset] = useState(0)
  const [retained, setRetained] = useState<Set<string>>(() => new Set())
  const [reasons] = useState(() => new Map<string, Set<object>>())
  const retain = useCallback(
    (key: string, reason: object) => {
      const set = reasons.get(key) ?? new Set()
      set.add(reason)
      reasons.set(key, set)
      setRetained(new Set(reasons.keys()))
      return () => {
        set.delete(reason)
        if (!set.size) reasons.delete(key)
        setRetained(new Set(reasons.keys()))
      }
    },
    [reasons],
  )
  const keys = useMemo(() => items.map(getKey), [items, getKey])
  const signature = useMemo(() => JSON.stringify(keys), [keys])
  const keyIndices = useMemo(() => new Map(keys.map((key, index) => [key, index])), [keys])
  const retainedIndices = [...retained].flatMap((key) => {
    const index = keyIndices.get(key)
    return index === undefined ? [] : [index]
  })
  const [dataset] = useState(() => ({ keys }))
  useLayoutEffect(() => {
    const append = dataset.keys.every((key, index) => key === keys[index])
    if (!append && keys.length && element && owner?.scrollElement)
      owner.scrollElement.scrollTo({ top: Math.max(0, offset - leading), behavior: 'instant' })
    dataset.keys = keys
  }, [keys, dataset, element, owner, offset, leading])
  useLayoutEffect(() => {
    if (!element || !owner?.scrollElement) return
    const root = owner.scrollElement
    const update = () => {
      const bounds = element.getBoundingClientRect()
      const top = root === document.scrollingElement ? 0 : root.getBoundingClientRect().top
      const scroll = root === document.scrollingElement ? window.scrollY : root.scrollTop
      const header = element.querySelector<HTMLElement>('[data-slot="table-category-header"]')
      const gap = header?.previousElementSibling?.getBoundingClientRect().height ?? 0
      const inset = header ? gap + header.offsetHeight : leading
      setOffset(bounds.top + inset - top + scroll)
    }
    update()
    const observer = new ResizeObserver(update)
    observer.observe(element)
    if (element.parentElement) observer.observe(element.parentElement)
    window.addEventListener('resize', update)
    return () => {
      observer.disconnect()
      window.removeEventListener('resize', update)
    }
  }, [element, owner?.scrollElement, signature, leading])
  // TanStack manages measurement state outside React; its adapter schedules render notifications.
  // eslint-disable-next-line react-hooks/incompatible-library
  const virtualizer = useVirtualizer<HTMLElement, HTMLElement>({
    count: items.length,
    getScrollElement: () => owner?.scrollElement ?? null,
    estimateSize: () => estimateSize,
    getItemKey: useCallback((index: number) => keys[index], [keys]),
    scrollMargin: offset,
    overscan: 2,
    scrollToFn: (offset, { adjustments = 0, behavior }) => {
      const root = owner?.scrollElement
      if (!root) return
      const top = offset + adjustments
      if (root === document.scrollingElement) window.scrollTo({ top, behavior: behavior ?? 'instant' })
      else root.scrollTo({ top, behavior: behavior ?? 'instant' })
    },
    observeElementRect: (_instance, callback) =>
      owner?.viewport.observe(() => callback({ width: owner.viewport.width, height: owner.viewport.height })),
    observeElementOffset: (_instance, callback) =>
      owner?.viewport.observe(() => callback(owner.viewport.offset, owner.viewport.scrolling)),
    rangeExtractor: (range) => [...new Set([...defaultRangeExtractor(range), ...retainedIndices])].sort((a, b) => a - b),
  })
  const wanted = virtualizer.getVirtualItems()
  const before = offset > (virtualizer.scrollOffset ?? 0) + (virtualizer.scrollRect?.height ?? 0) + estimateSize * 2
  const after = offset + virtualizer.getTotalSize() < (virtualizer.scrollOffset ?? 0) - estimateSize * 2
  const target =
    element && owner?.scrollElement && (before || after)
      ? []
      : wanted.length
        ? wanted.map((row) => row.index)
        : items.slice(0, 3).map((_, index) => index)
  const targetKey = JSON.stringify(target.map((index) => keys[index]))
  const targetKeys = target.map((index) => keys[index])
  const [revealed, setRevealed] = useState(() => ({ signature: targetKey, keys: targetKeys.slice(0, 3) }))
  let shown = revealed.keys
  if (revealed.signature !== targetKey) {
    const previous = new Set(revealed.keys)
    shown = [...targetKeys.filter((key) => previous.has(key)), ...targetKeys.filter((key) => !previous.has(key)).slice(0, 3)]
    setRevealed({ signature: targetKey, keys: shown })
  }
  const shownKey = JSON.stringify(shown)
  useEffect(() => {
    const current = new Set(JSON.parse(shownKey) as string[])
    const missing = (JSON.parse(targetKey) as string[]).filter((key) => !current.has(key))
    if (!missing.length) return
    const abort = new AbortController()
    void yieldToBrowser(abort.signal)
      .then(() => setRevealed({ signature: targetKey, keys: [...current, ...missing.slice(0, 3)] }))
      .catch(() => undefined)
    return () => abort.abort()
  }, [targetKey, shownKey])
  const allowed = new Set(shown)
  const indices = new Set([...target.filter((index) => allowed.has(keys[index])), ...retainedIndices])
  useLayoutEffect(() => {
    if (!element || !owner) return
    return owner.viewport.register(element, items.length)
  }, [element, owner, items.length, signature, virtualizer, leading])
  useLayoutEffect(() => owner?.viewport.metadata(), [owner, shownKey])
  const rows = [...indices]
    .sort((a, b) => a - b)
    .map((index) => ({
      index,
      key: keys[index],
      item: items[index],
      start: (virtualizer.measurementsCache[index]?.start ?? offset + index * estimateSize) - offset,
      size: virtualizer.measurementsCache[index]?.size ?? estimateSize,
    }))
  const [focusRequest, setFocusRequest] = useState<{ index: number; backwards: boolean } | null>(null)
  const focusIndex = useCallback(
    (index: number, backwards: boolean) => {
      if (!keys[index]) return
      const release = retain(keys[index], {})
      setFocusRequest({ index, backwards })
      virtualizer.scrollToIndex(index, { align: 'auto' })
      return release
    },
    [keys, retain, virtualizer],
  )
  const [focusKeeper] = useState<{ release?: () => void; key?: string }>(() => ({}))
  useLayoutEffect(() => {
    if (!focusRequest || !element) return
    const frame = requestAnimationFrame(() => {
      const key = CSS.escape(keys[focusRequest.index])
      const row = element.querySelector<HTMLElement>(`[data-collection-key="${key}"]`)
      if (!row) return
      const controls = [...row.querySelectorAll<HTMLElement>(controlsSelector)].filter(
        (node) => node.getClientRects().length && !node.closest('[inert]'),
      )
      const target = focusRequest.backwards ? controls.at(-1) : controls[0]
      if (target) target.focus()
      else {
        row.tabIndex = -1
        row.focus()
      }
      focusKeeper.release?.()
      focusKeeper.release = undefined
      setFocusRequest(null)
    })
    return () => cancelAnimationFrame(frame)
  }, [focusRequest, element, keys, targetKey, focusKeeper])
  useEffect(() => () => focusKeeper.release?.(), [focusKeeper])
  const requestFocus = (index: number, backwards: boolean) => {
    focusKeeper.release?.()
    focusKeeper.release = focusIndex(index, backwards)
  }
  const focusOutside = (backwards: boolean) => {
    if (!element) return
    const candidates = [
      ...document.querySelectorAll<HTMLElement>(
        'button:not(:disabled), a[href], input:not(:disabled), select:not(:disabled), ' +
          'textarea:not(:disabled), [tabindex="0"]',
      ),
    ].filter(
      (node) =>
        !element.contains(node) &&
        !node.hasAttribute('data-collection-entry') &&
        node.getClientRects().length &&
        !node.closest('[inert]') &&
        Boolean(
          element.compareDocumentPosition(node) &
          (backwards ? Node.DOCUMENT_POSITION_PRECEDING : Node.DOCUMENT_POSITION_FOLLOWING),
        ),
    )
    ;(backwards ? candidates.at(-1) : candidates[0])?.focus()
  }
  const onFocus = (event: import('react').FocusEvent<HTMLElement>) => {
    focusKeeper.key = (event.target as HTMLElement).closest<HTMLElement>('[data-collection-key]')?.dataset.collectionKey
  }
  useLayoutEffect(() => {
    if (!focusKeeper.key || keys.includes(focusKeeper.key) || !element) return
    const table = element.closest<HTMLElement>('table') ?? element
    table.tabIndex = -1
    table.focus()
    focusKeeper.key = undefined
  }, [keys, element, focusKeeper])
  const onKeyDown = (event: import('react').KeyboardEvent<HTMLElement>) => {
    if (event.key !== 'Tab' || event.defaultPrevented || !element) return
    const row = (event.target as HTMLElement).closest<HTMLElement>('[data-collection-key]')
    if (!row) return
    const controls = [
      ...row.querySelectorAll<HTMLElement>(
        'button:not(:disabled), a[href], input:not(:disabled), select:not(:disabled), ' +
          'textarea:not(:disabled), [tabindex="0"]',
      ),
    ].filter((node) => node.getClientRects().length && !node.closest('[inert]'))
    if (event.target !== (event.shiftKey ? controls[0] : controls.at(-1))) return
    event.preventDefault()
    const index = keys.indexOf(row.dataset.collectionKey!) + (event.shiftKey ? -1 : 1)
    if (index < 0 || index >= items.length) focusOutside(event.shiftKey)
    else requestFocus(index, event.shiftKey)
  }
  const enter = (backwards: boolean, previous: EventTarget | null) => {
    if (previous instanceof Node && element?.contains(previous)) focusOutside(backwards)
    else requestFocus(backwards ? items.length - 1 : 0, backwards)
  }
  return {
    attach: setElement,
    onFocus,
    onKeyDown,
    enter,
    requestFocus,
    rows,
    total: virtualizer.getTotalSize(),
    measure: virtualizer.measureElement,
    retain,
    forms: owner?.forms,
    revealing: shown.length < target.length,
    virtualizer,
  }
}

export const CollectionContent = memo(function CollectionContent<T>({ item, render }: { item: T; render(item: T): ReactNode }) {
  return render(item)
}) as <T>(props: { item: T; render(item: T): ReactNode }) => ReactNode

export function CollectionRow({
  index,
  measure,
  rowKey,
  retain,
  forms,
  children,
}: {
  rowKey: string
  index: number
  measure(element: HTMLElement | null): void
  retain(key: string, reason: object): () => void
  forms: CollectionForms
  children: ReactNode
}) {
  const scope = useMemo(
    () => ({ index, measure, key: rowKey, forms, retain: (reason: object) => retain(rowKey, reason) }),
    [index, measure, rowKey, forms, retain],
  )
  return <CollectionRowContext value={scope}>{children}</CollectionRowContext>
}

export function useCollectionLoadMore(paging: CollectionPaging | undefined, remaining: number, disabled: boolean) {
  const callback = useRef(paging?.onLoadMore)
  useEffect(() => {
    callback.current = paging?.onLoadMore
  }, [paging?.onLoadMore])
  const nearEnd = remaining <= 160
  useEffect(() => {
    if (!paging?.hasMore || paging.loading || paging.error || disabled || !nearEnd) return
    const timer = setTimeout(() => callback.current?.(), 100)
    return () => clearTimeout(timer)
  }, [paging?.hasMore, paging?.loading, paging?.error, nearEnd, disabled])
}
