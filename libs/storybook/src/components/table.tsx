import { animate } from 'motion'
import { motion } from 'motion/react'
import { animated, animation, transition, useTransition } from './motion'
import { useEffect, useLayoutEffect, useRef, type ReactNode, type ComponentPropsWithRef } from 'react'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { cx, composeRefs } from './shared'
import { useControllableState } from './composition'
function Root({ className, ref: forwardedRef, ...props }: ComponentPropsWithRef<'div'>) {
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
    <div
      ref={composeRefs(ref, forwardedRef)}
      data-slot="table-container"
      {...props}
      className={cx('w-full overflow-x-auto', className)}
    />
  )
}
function Content({ className, ...props }: ComponentPropsWithRef<'table'>) {
  return <table data-slot="table" {...props} className={cx('w-full table-fixed text-left text-xs', className)} />
}
function Header({ className, ...props }: ComponentPropsWithRef<'tbody'>) {
  return <thead data-slot="table-header" {...props} className={cx('h-8 text-muted-foreground [&>tr]:h-8', className)} />
}
function Body(props: ComponentPropsWithRef<'tbody'>) {
  return <tbody data-slot="table-body" {...props} />
}
function Row({ className, ...props }: ComponentPropsWithRef<'tr'>) {
  return <tr data-slot="table-row" {...props} className={cx('h-12 border-b border-divider last:border-b-0', className)} />
}
function Head({ className, ...props }: ComponentPropsWithRef<'th'>) {
  return <th data-slot="table-head" scope="col" {...props} className={cx('px-2 font-normal', className)} />
}
function Cell({ className, ...props }: ComponentPropsWithRef<'td'>) {
  return <td data-slot="table-cell" {...props} className={cx('px-2', className)} />
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
  label: string
  count?: ReactNode
  action?: ReactNode
  columns?: number
  open?: boolean
  defaultOpen?: boolean
  onOpenChange?: (open: boolean) => void
  children: ReactNode
}

function Category({ label, count, action, columns = 3, open, defaultOpen = true, onOpenChange, children }: TableCategoryProps) {
  const timing = useTransition('disclosure')
  const [expanded, setExpanded] = useControllableState(open, defaultOpen, onOpenChange)
  const body = useRef<HTMLTableSectionElement>(null)
  const previous = useRef(expanded)
  useLayoutEffect(() => {
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
  }, [expanded, children])
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
      <tr data-slot="table-category-header" className="h-10 border-0">
        <td
          colSpan={action ? columns - 1 : columns}
          className={cx(
            `sticky top-8 z-10 rounded-l-md bg-elevated p-0
translate-y-(--category-offset,0px)`,
            !action && 'rounded-r-md',
          )}
        >
          <animated.button
            type="button"
            aria-expanded={expanded}
            aria-label={`${expanded ? 'Collapse' : 'Expand'} ${label}`}
            className="flex min-h-10 w-full cursor-pointer items-center gap-2 px-2 text-left
focus-visible:outline-2 focus-visible:outline-ring"
            onClick={() => {
              const next = !expanded
              setExpanded(next)
            }}
          >
            <motion.span
              initial={false}
              animate={{ rotate: expanded ? 0 : animation.collapsedRotation }}
              transition={timing}
              className="inline-flex size-2 shrink-0"
            >
              <CaretDownIcon weight="fill" aria-hidden="true" className="size-2" />
            </motion.span>
            <span className="min-w-0 text-xsm font-medium">{label}</span>
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
      {children}
    </tbody>
  )
}

export const Table = { Root, Content, Header, Body, Row, Head, Cell, Empty, Category }
