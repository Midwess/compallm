import { CollectionContext, CollectionRowContext } from '../context'
import { CollectionContent, CollectionRow, useCollectionOwner, useCollectionRows, useCollectionLoadMore } from './collection'
import type { CollectionPaging } from './collection-types'
import type { CollectionForms } from './collection-forms'
import { Fragment } from 'react'
import { animate } from 'motion'
import { motion } from 'motion/react'
import { animated, animation, transition, useTransition } from './motion'
import {
  useEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
  useContext,
  createContext,
  type ReactNode,
  type ComponentPropsWithRef,
} from 'react'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { cx, composeRefs } from './shared'
import { useControllableState } from './composition'
export interface TableColumn {
  id: string
  className?: string
  visibilityClassName?: string
}
const emptyColumns: TableColumn[] = []
const columnDisplays: Record<string, string> = {
  'table-cell': 'table-column',
  'sm:table-cell': 'sm:table-column',
  'md:table-cell': 'md:table-column',
  'lg:table-cell': 'lg:table-column',
  'xl:table-cell': 'xl:table-column',
  '2xl:table-cell': '2xl:table-column',
}
const ColumnsContext = createContext<{ columns: TableColumn[]; visibleCount?: number }>({ columns: [] })
function useColumn(id?: string) {
  const { columns } = useContext(ColumnsContext)
  return id ? columns.find((column) => column.id === id)?.visibilityClassName : undefined
}

function Root({
  className,
  ref: forwardedRef,
  forms,
  scrollElement,
  ...props
}: ComponentPropsWithRef<'div'> & { forms?: CollectionForms; scrollElement?: HTMLElement | null }) {
  const collection = useCollectionOwner(scrollElement)
  const scope = useMemo(() => ({ ...collection.scope, forms: forms ?? collection.scope.forms }), [collection.scope, forms])
  const ref = useRef<HTMLDivElement>(null)
  useEffect(() => {
    const root = ref.current
    if (!root) return
    let scroll: HTMLElement | null = root
    while (scroll && !/(auto|scroll)/.test(getComputedStyle(scroll).overflowY)) scroll = scroll.parentElement
    if (!scroll) return
    let frame = 0
    const update = () => {
      frame = 0
      const top = scroll.getBoundingClientRect().top + 32
      const categories = [...root.querySelectorAll<HTMLElement>('[data-slot="table-category"]')]
      const offsets = categories.map((body) => {
        const header = body.querySelector<HTMLElement>('[data-slot="table-category-header"]')
        return { header, offset: Math.min(0, body.getBoundingClientRect().bottom - top - (header?.offsetHeight ?? 0)) }
      })
      for (const { header, offset } of offsets) header?.style.setProperty('--category-offset', `${offset}px`)
    }
    const schedule = () => {
      if (!frame) frame = requestAnimationFrame(update)
    }
    scroll.addEventListener('scroll', schedule, { passive: true })
    const observer = new ResizeObserver(schedule)
    observer.observe(root)
    update()
    return () => {
      scroll.removeEventListener('scroll', schedule)
      observer.disconnect()
      cancelAnimationFrame(frame)
    }
  }, [])
  return (
    <CollectionContext value={scope}>
      <div
        ref={composeRefs(ref, forwardedRef, collection.attach)}
        data-slot="table-container"
        {...props}
        className={cx('min-w-0 w-full max-w-full overflow-x-auto', className)}
      />
    </CollectionContext>
  )
}
function Content({
  className,
  columns = emptyColumns,
  children,
  ref: forwardedRef,
  ...props
}: ComponentPropsWithRef<'table'> & { columns?: TableColumn[] }) {
  const ref = useRef<HTMLTableElement>(null)
  const [visibleCount, setVisibleCount] = useState<number>()
  useLayoutEffect(() => {
    const table = ref.current
    if (!table) return
    const update = () => {
      const row = table.tHead?.rows[0]
      if (row)
        setVisibleCount(
          [...row.cells]
            .filter((cell) => getComputedStyle(cell).display !== 'none')
            .reduce((count, cell) => count + cell.colSpan, 0),
        )
    }
    update()
    const observer = new ResizeObserver(update)
    observer.observe(table)
    window.addEventListener('resize', update)
    return () => {
      observer.disconnect()
      window.removeEventListener('resize', update)
    }
  }, [columns, children])
  const context = useMemo(() => ({ columns, visibleCount }), [columns, visibleCount])
  return (
    <ColumnsContext value={context}>
      <table
        ref={composeRefs(ref, forwardedRef)}
        data-slot="table"
        {...props}
        className={cx('w-full max-w-full table-fixed text-left text-xs', className)}
      >
        {columns.length ? (
          <colgroup>
            {columns.map((column) => (
              <col
                key={column.id}
                className={cx(
                  column.className,
                  column.visibilityClassName
                    ?.split(/\s+/)
                    .map((value) => columnDisplays[value] ?? value)
                    .join(' '),
                )}
              />
            ))}
          </colgroup>
        ) : null}
        {children}
      </table>
    </ColumnsContext>
  )
}

function Header({ className, ...props }: ComponentPropsWithRef<'tbody'>) {
  return <thead data-slot="table-header" {...props} className={cx('h-8 text-muted-foreground [&>tr]:h-8', className)} />
}
export interface TableItemsProps<T> {
  items: readonly T[]
  getKey(item: T): string
  estimateSize?: number
  paging?: CollectionPaging
  children(item: T): ReactNode
}
function Body<T>(
  props: ComponentPropsWithRef<'tbody'> | (Omit<ComponentPropsWithRef<'tbody'>, 'children'> & TableItemsProps<T>),
) {
  if ('items' in props) return <DataBody {...props} />
  return <tbody data-slot="table-body" {...props} />
}
// Collection snapshots contain callback refs; they are passed to React rather than read during render.
/* eslint-disable react-hooks/refs */
function DataBody<T>({
  items,
  getKey,
  estimateSize = 48,
  paging,
  children,
  ...props
}: Omit<ComponentPropsWithRef<'tbody'>, 'children'> & TableItemsProps<T>) {
  const rows = useCollectionRows(items, getKey, estimateSize)
  return (
    <tbody
      {...props}
      ref={rows.attach}
      onKeyDown={rows.onKeyDown}
      onFocusCapture={rows.onFocus}
      data-slot="table-body"
      aria-busy={rows.revealing || paging?.loading}
    >
      <Entry collection={rows} backwards={false} />
      <DataRows collection={rows} count={items.length} render={children} paging={paging} />
      <Entry collection={rows} backwards />
    </tbody>
  )
}
/* eslint-enable react-hooks/refs */
function Entry<T>({ collection, backwards }: { collection: ReturnType<typeof useCollectionRows<T>>; backwards: boolean }) {
  const { visibleCount, columns } = useContext(ColumnsContext)
  if (!collection.rows.length) return null
  return (
    <tr role="presentation">
      <td className="h-0 border-0 p-0" colSpan={visibleCount ?? (columns.length || 1)}>
        <button
          type="button"
          className="sr-only"
          onFocus={(event) => collection.enter(backwards, event.relatedTarget)}
          data-collection-entry
          aria-label={backwards ? 'Last row' : 'First row'}
        />
      </td>
    </tr>
  )
}
function DataRows<T>({
  collection,
  count,
  render,
  paging,
  disabled = false,
}: {
  collection: ReturnType<typeof useCollectionRows<T>>
  count: number
  render(item: T): ReactNode
  paging?: CollectionPaging
  disabled?: boolean
}) {
  const { visibleCount, columns } = useContext(ColumnsContext)
  const span = visibleCount ?? (columns.length || 1)
  const virtual = collection.virtualizer
  const bottom = (virtual.scrollOffset ?? 0) + (virtual.scrollRect?.height ?? 0) - virtual.options.scrollMargin
  useCollectionLoadMore(paging, collection.total - bottom, disabled || collection.revealing)
  const last = collection.rows.at(-1)
  const end = last ? last.start + last.size : 0
  return (
    <>
      {collection.rows.map((row, index) => {
        const previous = collection.rows[index - 1]
        const gap = Math.max(0, row.start - (previous ? previous.start + previous.size : 0))
        const content = <CollectionContent item={row.item} render={render} />
        return (
          <Fragment key={row.key}>
            {gap > 0 ? (
              <tr aria-hidden="true">
                <td colSpan={span} className="p-0 border-0" style={{ height: gap }} />
              </tr>
            ) : null}
            {collection.forms ? (
              <CollectionRow
                index={row.index}
                measure={collection.measure}
                rowKey={row.key}
                retain={collection.retain}
                forms={collection.forms}
              >
                {content}
              </CollectionRow>
            ) : (
              content
            )}
          </Fragment>
        )
      })}
      {collection.total > end ? (
        <tr aria-hidden="true">
          <td colSpan={span} className="p-0 border-0" style={{ height: collection.total - end }} />
        </tr>
      ) : null}
      {paging ? <LoadMore {...paging} /> : null}
      <tr className="sr-only">
        <td colSpan={span} aria-live="polite">
          {count} rows loaded
        </td>
      </tr>
    </>
  )
}

function LoadMore({ hasMore, loading, error, onLoadMore }: CollectionPaging) {
  const { visibleCount, columns } = useContext(ColumnsContext)
  if (!hasMore) return null
  return (
    <tr>
      <td colSpan={visibleCount ?? (columns.length || 1)} className="h-8 p-0 text-center">
        <button
          type="button"
          disabled={loading}
          onClick={onLoadMore}
          className={error ? 'text-xs text-muted-foreground' : 'sr-only focus:not-sr-only'}
        >
          {error ? 'Retry loading' : 'Load more rows'}
        </button>
      </td>
    </tr>
  )
}

function Row({ className, ref, onFocusCapture, onBlurCapture, ...props }: ComponentPropsWithRef<'tr'>) {
  const scope = useContext(CollectionRowContext)
  const release = useRef<(() => void) | undefined>(undefined)
  useEffect(() => () => release.current?.(), [])
  return (
    <tr
      ref={composeRefs(ref, scope?.measure)}
      data-index={scope?.index}
      data-collection-key={scope?.key}
      onFocusCapture={(event) => {
        onFocusCapture?.(event)
        release.current ??= scope?.retain({})
      }}
      onBlurCapture={(event) => {
        onBlurCapture?.(event)
        if (!event.currentTarget.contains(event.relatedTarget as Node | null)) {
          release.current?.()
          release.current = undefined
        }
      }}
      data-slot="table-row"
      {...props}
      className={cx('h-12 border-b border-divider last:border-b-0', className)}
    />
  )
}
function Head({ className, column, children, ...props }: ComponentPropsWithRef<'th'> & { column?: string }) {
  const visibility = useColumn(column)
  return (
    <th
      data-slot="table-head"
      data-column={column}
      scope="col"
      {...props}
      className={cx('min-w-0 overflow-hidden text-left font-normal', visibility, className)}
    >
      <div className="min-w-0 truncate px-2 **:min-w-0 *:max-w-full">{children}</div>
    </th>
  )
}
function Cell({ className, column, children, ...props }: ComponentPropsWithRef<'td'> & { column?: string }) {
  const visibility = useColumn(column)
  const { visibleCount } = useContext(ColumnsContext)
  const colSpan = props.colSpan && visibleCount ? Math.min(props.colSpan, visibleCount) : props.colSpan
  return (
    <td
      data-slot="table-cell"
      data-column={column}
      {...props}
      colSpan={colSpan}
      className={cx('min-w-0 overflow-hidden text-left', visibility, className)}
    >
      <div className="min-w-0 truncate px-2 **:min-w-0 *:max-w-full">{children}</div>
    </td>
  )
}

function Empty({ className, ...props }: ComponentPropsWithRef<'div'>) {
  return (
    <div
      {...props}
      className={cx('flex min-h-20 items-center justify-center py-4 text-center text-xs text-muted-foreground', className)}
    />
  )
}
export interface TableCategoryProps {
  variant?: 'default' | 'directory'
  label: string
  count?: ReactNode
  action?: ReactNode
  columns?: number
  open?: boolean
  defaultOpen?: boolean
  onOpenChange?: (open: boolean) => void
  children: ReactNode
}

function Category<T>({
  variant = 'default',
  label,
  count,
  action,
  columns = 3,
  open,
  defaultOpen = true,
  onOpenChange,
  children,
  items,
  getKey,
  estimateSize = 56,
  paging,
}: Omit<TableCategoryProps, 'children'> & {
  children: ReactNode | ((item: T) => ReactNode)
  items?: readonly T[]
  getKey?: (item: T) => string
  estimateSize?: number
  paging?: CollectionPaging
}) {
  const { visibleCount } = useContext(ColumnsContext)
  columns = visibleCount || columns
  const directory = variant === 'directory'
  const timing = useTransition('disclosure')
  const [expanded, setExpanded] = useControllableState(open, defaultOpen, onOpenChange)
  const data = useCollectionRows(expanded ? (items ?? []) : [], getKey ?? (() => 'row'), estimateSize, 40)
  const body = useRef<HTMLTableSectionElement>(null)
  const previous = useRef(expanded)
  useLayoutEffect(() => {
    if (items) return
    const rows = [...(body.current?.querySelectorAll<HTMLTableRowElement>('tr[data-slot="table-row"]') ?? [])]
    const changed = previous.current !== expanded
    previous.current = expanded
    for (const row of rows) {
      row.hidden = !expanded && !changed
      row.inert = !expanded
      if (!changed) {
        row.style.opacity = expanded ? '1' : '0'
        row.style.transform = ''
      }
    }
    if (!changed) return
    const reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches
    if (reduced || rows.length === 0) {
      for (const row of rows) row.hidden = !expanded
      return
    }
    let active = true
    const animations = rows.map((row) =>
      animate(row, { opacity: expanded ? 1 : 0, y: expanded ? 0 : -animation.distance }, transition('disclosure')),
    )
    void Promise.all(animations).then(() => {
      if (!active) return
      if (!expanded) for (const row of rows) row.hidden = true
      animations.forEach((control) => control.stop())
    })
    return () => {
      active = false
      animations.forEach((control) => control.stop())
    }
  }, [expanded, children, items])
  return (
    <tbody
      ref={composeRefs(body, data.attach)}
      onKeyDown={data.onKeyDown}
      onFocusCapture={data.onFocus}
      data-state={expanded ? 'open' : 'closed'}
      data-slot="table-category"
      className="[&:not(:first-of-type)>tr:first-child]:table-row"
    >
      <tr aria-hidden="true" className="hidden h-2">
        <td colSpan={columns} className="h-2 p-0" />
      </tr>
      <tr data-slot="table-category-header" className={directory ? 'h-8 border-0' : 'h-10 border-0'}>
        <td
          colSpan={action ? columns - 1 : columns}
          className={cx(
            `sticky top-8 z-10 rounded-l-md bg-elevated p-0
translate-y-(--category-offset,0px)`,
            !action && 'rounded-r-md',
            directory && 'rounded-lg bg-inset',
          )}
        >
          <animated.button
            type="button"
            aria-expanded={expanded}
            aria-label={`${expanded ? 'Collapse' : 'Expand'} ${label}`}
            className={cx(
              'flex w-full cursor-pointer items-center gap-2 px-2 text-left focus-visible:outline-2 focus-visible:outline-ring',
              directory ? 'h-8 text-xs text-muted-foreground' : 'min-h-10',
            )}
            onClick={() => setExpanded(!expanded)}
          >
            <motion.span
              initial={false}
              animate={{ rotate: expanded ? 0 : animation.collapsedRotation }}
              transition={timing}
              className="inline-flex size-2 shrink-0"
            >
              <CaretDownIcon weight="fill" aria-hidden="true" className="size-2" />
            </motion.span>
            <span className={directory ? 'min-w-0' : 'min-w-0 text-xsm font-medium'}>{label}</span>
            {count !== undefined ? <span className="text-xs font-normal text-muted-foreground">{count}</span> : null}
          </animated.button>
        </td>
        {action ? (
          <td
            className="sticky top-8 z-10 rounded-r-md bg-elevated px-2 py-0
translate-y-(--category-offset,0px)"
          >
            {action}
          </td>
        ) : null}
      </tr>
      {items && getKey && typeof children === 'function' ? (
        expanded ? (
          <>
            <Entry collection={data} backwards={false} />
            <DataRows collection={data} count={items.length} render={children} paging={paging} />
            <Entry collection={data} backwards />
          </>
        ) : null
      ) : (
        (children as ReactNode)
      )}
    </tbody>
  )
}

export const Table = { Root, Content, Header, Body, Row, Head, Cell, Empty, Category, LoadMore }
