import { Skeleton, useInitialLoading } from './loading'
import { CollectionContext, CollectionRowContext } from '../context'
import { CollectionContent, CollectionRecord, useCollectionOwner, useCollectionRows, useCollectionLoadMore } from './collection'
import type { CollectionPaging } from './collection-types'
import type { CollectionForms } from './collection-forms'
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
  loadingWidthClassName?: string
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
  paging?: CollectionPaging
  children(item: T): ReactNode
  loadingRowClassName?: string
}
function Body<T>(
  props: ComponentPropsWithRef<'tbody'> | (Omit<ComponentPropsWithRef<'tbody'>, 'children'> & TableItemsProps<T>),
) {
  const initial = useInitialLoading()
  if (initial) return <SkeletonBody rowClassName={'loadingRowClassName' in props ? props.loadingRowClassName : undefined} />
  if ('items' in props) return <DataBody {...props} />
  return <tbody data-slot="table-body" {...props} />
}
function SkeletonBody({ rowClassName = 'h-12' }: { rowClassName?: string }) {
  const { columns } = useContext(ColumnsContext)
  return (
    <tbody data-slot="table-body" aria-hidden="true">
      {Array.from({ length: 8 }, (_, index) => (
        <Row key={index} className={rowClassName}>
          {(columns.length ? columns : [{ id: 'content' }]).map((column) => (
            <Cell key={column.id} column={column.id}>
              <Skeleton loading>
                <span className={cx('block h-4 w-3/4 rounded-md', column.loadingWidthClassName)} />
              </Skeleton>
            </Cell>
          ))}
        </Row>
      ))}
    </tbody>
  )
}
function DataBody<T>({
  items,
  getKey,
  paging,
  children,
  loadingRowClassName: _loadingRowClassName,
  ...props
}: Omit<ComponentPropsWithRef<'tbody'>, 'children'> & TableItemsProps<T>) {
  void _loadingRowClassName
  const rows = useCollectionRows(items, getKey)
  return (
    <tbody {...props} data-slot="table-body" aria-busy={paging?.loading}>
      <DataRows collection={rows} count={items.length} render={children} paging={paging} />
    </tbody>
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
  const loadMoreRef = useCollectionLoadMore(disabled ? undefined : paging, collection.scrollElement)
  return (
    <>
      {collection.rows.map((row) =>
        collection.forms ? (
          <CollectionRecord
            key={row.key}
            index={row.index}
            rowKey={row.key}
            forms={collection.forms}
            item={row.item}
            render={render}
          />
        ) : (
          <CollectionContent key={row.key} item={row.item} render={render} />
        ),
      )}
      {paging ? <LoadMore ref={loadMoreRef} {...paging} /> : null}
      <tr className="sr-only">
        <td colSpan={span} aria-live="polite">
          {count} rows loaded
        </td>
      </tr>
    </>
  )
}

function LoadMore({
  ref,
  hasMore,
  loading,
  error,
  onLoadMore,
}: CollectionPaging & { ref?: (element: HTMLTableRowElement | null) => void }) {
  const { visibleCount, columns } = useContext(ColumnsContext)
  if (!hasMore) return null
  return (
    <tr ref={ref}>
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

function Row({ className, ref, ...props }: ComponentPropsWithRef<'tr'>) {
  const scope = useContext(CollectionRowContext)
  return (
    <tr
      ref={ref}
      data-index={scope?.index}
      data-collection-key={scope?.key}
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
  paging,
}: Omit<TableCategoryProps, 'children'> & {
  children: ReactNode | ((item: T) => ReactNode)
  items?: readonly T[]
  getKey?: (item: T) => string
  paging?: CollectionPaging
}) {
  const { visibleCount } = useContext(ColumnsContext)
  columns = visibleCount || columns
  const directory = variant === 'directory'
  const timing = useTransition('disclosure')
  const [expanded, setExpanded] = useControllableState(open, defaultOpen, onOpenChange)
  const data = useCollectionRows(expanded ? (items ?? []) : [], getKey ?? (() => 'row'))
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
      ref={body}
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
            <DataRows collection={data} count={items.length} render={children} paging={paging} />
          </>
        ) : null
      ) : (
        (children as ReactNode)
      )}
    </tbody>
  )
}

export const Table = { Root, Content, Header, Body, Row, Head, Cell, Empty, Category, LoadMore }
