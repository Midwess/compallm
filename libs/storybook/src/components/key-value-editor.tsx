import { createContext, useContext, useEffect, useLayoutEffect, useId, useRef, useState, type ReactNode } from 'react'
import { DotsSixVerticalIcon } from '@phosphor-icons/react/dist/csr/DotsSixVertical'
import { TrashIcon } from '@phosphor-icons/react/dist/csr/Trash'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { TextField } from './fields'
import { cx } from './shared'

export interface KeyValueEntry {
  id: string
  key: string
  value: string
}
interface Scope {
  entries: KeyValueEntry[]
  change(entries: KeyValueEntry[]): void
  disabled: boolean
  keyMaxLength?: number
  acceptEntry?(entry: KeyValueEntry): boolean
}
const EditorContext = createContext<Scope | null>(null)
function useEditor() {
  const scope = useContext(EditorContext)
  if (!scope) throw new Error('KeyValueEditor parts require KeyValueEditor.')
  return scope
}
function Root({
  value,
  onValueChange,
  disabled = false,
  children,
  className,
  acceptEntry,
  keyMaxLength,
}: {
  value: KeyValueEntry[]
  onValueChange(entries: KeyValueEntry[]): void
  disabled?: boolean
  children: ReactNode
  className?: string
  keyMaxLength?: number
  acceptEntry?(entry: KeyValueEntry): boolean
}) {
  return (
    <EditorContext value={{ entries: value, change: onValueChange, disabled, acceptEntry, keyMaxLength }}>
      <div className={cx('w-full min-w-0 max-w-full', className)}>{children}</div>
    </EditorContext>
  )
}
function Disclosure({ children = 'Headers', defaultOpen = false }: { children?: ReactNode; defaultOpen?: boolean }) {
  return (
    <details className="group/headers w-full min-w-0 max-w-full" open={defaultOpen || undefined}>
      <summary className="flex h-7 w-fit cursor-pointer list-none items-center gap-1 text-sm font-medium">
        {children}
        <CaretDownIcon
          weight="fill"
          aria-hidden="true"
          className="size-2 -rotate-90 transition-transform group-open/headers:rotate-0"
        />
      </summary>
      <div className="min-w-0 pt-3">
        <Content />
      </div>
    </details>
  )
}
interface DragState {
  pointerId: number
  key: string
  startY: number
  offsetY: number
  left: number
  width: number
  moved: boolean
  initial: string[]
  order: string[]
  slots: Array<{ key: string; top: number; height: number; offset: number }>
}
function Content() {
  const { entries, change, disabled, acceptEntry, keyMaxLength } = useEditor()
  const pendingFocus = useRef<string | null>(null)
  const rowNodes = useRef(new Map<string, HTMLDivElement>())
  const beforeMove = useRef(new Map<string, number>())
  const drag = useRef<DragState | null>(null)
  const ghost = useRef<HTMLElement | null>(null)
  const [announcement, announce] = useState('')
  const [preview, setPreview] = useState<{ key: string; offsets: Record<string, number> } | null>(null)
  const id = useId()
  useEffect(() => () => ghost.current?.remove(), [])
  useLayoutEffect(() => {
    const previous = beforeMove.current
    if (!previous.size) return
    const reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches
    for (const [key, node] of rowNodes.current) {
      const top = previous.get(key)
      if (top === undefined || reduced) continue
      const offset = top - node.offsetTop
      if (Math.abs(offset) > 1) {
        for (const animation of node.getAnimations()) animation.cancel()
        node.animate([{ transform: `translateY(${offset}px)` }, { transform: 'translateY(0)' }], {
          duration: 150,
          easing: 'ease-out',
        })
      }
    }
    beforeMove.current.clear()
  }, [entries])
  function keyboardMove(from: number, to: number) {
    if (disabled || from === to || to < 0 || to >= entries.length) return
    beforeMove.current = new Map(Array.from(rowNodes.current, ([key, node]) => [key, node.offsetTop]))
    const next = [...entries]
    const [entry] = next.splice(from, 1)
    next.splice(to, 0, entry)
    change(next)
    announce(`Header moved to position ${to + 1} of ${next.length}`)
  }
  function update(index: number, patch: Partial<KeyValueEntry>) {
    const next = { ...entries[index], ...patch }
    if (acceptEntry && !acceptEntry(next)) return
    change(entries.map((entry, position) => (position === index ? next : entry)))
  }
  function endDrag(commit: boolean) {
    const current = drag.current
    if (current?.moved && commit) {
      change(current.order.flatMap((key) => entries.filter((entry) => entry.id === key)))
      announce(`Header placed at position ${current.order.indexOf(current.key) + 1} of ${entries.length}`)
    }
    drag.current = null
    ghost.current?.remove()
    ghost.current = null
    setPreview(null)
  }
  function startDrag(key: string, event: import('react').PointerEvent<HTMLButtonElement>) {
    if (disabled || event.button !== 0) return
    const row = rowNodes.current.get(key)
    if (!row) return
    const rect = row.getBoundingClientRect()
    drag.current = {
      pointerId: event.pointerId,
      key,
      startY: event.clientY,
      offsetY: event.clientY - rect.top,
      left: rect.left,
      width: rect.width,
      moved: false,
      initial: entries.map((entry) => entry.id),
      order: entries.map((entry) => entry.id),
      slots: entries.flatMap((entry) => {
        const node = rowNodes.current.get(entry.id)
        if (!node) return []
        const bounds = node.getBoundingClientRect()
        return [{ key: entry.id, top: bounds.top, height: bounds.height, offset: node.offsetTop }]
      }),
    }
    event.currentTarget.setPointerCapture(event.pointerId)
  }
  function pointerMove(event: import('react').PointerEvent<HTMLButtonElement>) {
    const current = drag.current
    if (!current || current.pointerId !== event.pointerId) return
    if (!current.moved && Math.abs(event.clientY - current.startY) < 4) return
    event.preventDefault()
    if (!current.moved) {
      const row = rowNodes.current.get(current.key)
      if (!row) return
      const image = row.cloneNode(true) as HTMLElement
      image.removeAttribute('style')
      image.className =
        'pointer-events-none fixed top-0 left-0 z-50 flex items-center gap-2 rounded-lg bg-inset shadow-menu ring-1 ring-border'
      image.style.width = `${current.width}px`
      image.setAttribute('aria-hidden', 'true')
      image.inert = true
      image.querySelectorAll('[id], [aria-label], [aria-labelledby]').forEach((node) => {
        node.removeAttribute('id')
        node.removeAttribute('aria-label')
        node.removeAttribute('aria-labelledby')
      })
      const entry = entries.find((entry) => entry.id === current.key)!
      image.querySelectorAll('input').forEach((input, index) => {
        input.value = index === 0 ? entry.key : entry.value
      })
      ;(row.closest('dialog') ?? document.body).appendChild(image)
      ghost.current = image
      current.moved = true
    }
    if (ghost.current) ghost.current.style.transform = `translate3d(${current.left}px,${event.clientY - current.offsetY}px,0)`
    let destination = 0
    for (let index = 1; index < current.slots.length; index++) {
      if (
        Math.abs(event.clientY - (current.slots[index].top + current.slots[index].height / 2)) <
        Math.abs(event.clientY - (current.slots[destination].top + current.slots[destination].height / 2))
      )
        destination = index
    }
    const order = current.initial.filter((key) => key !== current.key)
    order.splice(destination, 0, current.key)
    current.order = order
    const positions = Object.fromEntries(current.slots.map((slot) => [slot.key, slot.offset]))
    const offsets = Object.fromEntries(order.map((key, index) => [key, current.slots[index].offset - positions[key]]))
    setPreview({ key: current.key, offsets })
  }
  return (
    <div role="table" aria-label="Custom headers" className="grid w-full min-w-0 max-w-full grid-cols-[minmax(0,1fr)] gap-2">
      <div
        role="row"
        className="grid grid-cols-[2rem_minmax(0,1fr)_minmax(0,1fr)_1.5rem] items-center gap-1 text-xs text-muted"
      >
        <span role="columnheader">Order</span>
        <span role="columnheader" className="pl-1.5">
          Key
        </span>
        <span role="columnheader" className="pl-1">
          Value
        </span>
        <span role="columnheader" className="sr-only">
          Remove
        </span>
      </div>
      {entries.map((entry, index) => (
        <div
          key={entry.id}
          role="row"
          data-dragging={preview?.key === entry.id || undefined}
          ref={(node) => {
            if (node) rowNodes.current.set(entry.id, node)
            else rowNodes.current.delete(entry.id)
          }}
          style={preview ? { transform: `translateY(${preview.offsets[entry.id] ?? 0}px)` } : undefined}
          className={cx(
            'relative flex min-w-0 items-center gap-2',
            !!preview && 'motion-safe:transition-transform motion-safe:duration-150',
            preview?.key === entry.id && 'opacity-0',
          )}
        >
          <span role="cell" className="w-6 shrink-0">
            <button
              type="button"
              disabled={disabled}
              aria-label={`Reorder header ${index + 1}`}
              aria-describedby={`${id}-reorder`}
              className={`
flex size-6 touch-none cursor-grab items-center justify-center text-muted hover:text-text
active:cursor-grabbing focus-visible:outline-muted disabled:opacity-40
`}
              onPointerDown={(event) => startDrag(entry.id, event)}
              onPointerMove={pointerMove}
              onPointerUp={(event) => {
                if (drag.current?.pointerId === event.pointerId) {
                  endDrag(true)
                  event.currentTarget.releasePointerCapture(event.pointerId)
                }
              }}
              onPointerCancel={() => endDrag(false)}
              onKeyDown={(event) => {
                if (event.key === 'Escape' && drag.current) {
                  event.preventDefault()
                  endDrag(false)
                } else if (event.key === 'ArrowUp' || event.key === 'ArrowDown') {
                  event.preventDefault()
                  keyboardMove(index, index + (event.key === 'ArrowUp' ? -1 : 1))
                }
              }}
            >
              <DotsSixVerticalIcon aria-hidden="true" className="size-3" />
            </button>
          </span>
          <div
            className={`
grid min-w-0 flex-1 grid-cols-[minmax(0,1fr)_minmax(0,1fr)_1.5rem] items-center gap-1 rounded-lg bg-elevated
p-1
`}
          >
            <span role="cell" className="min-w-0">
              <TextField
                aria-label={`Header ${index + 1} key`}
                maxLength={keyMaxLength}
                value={entry.key}
                placeholder="Enter key"
                disabled={disabled}
                ref={(element) => {
                  if (element && pendingFocus.current === entry.id) {
                    pendingFocus.current = null
                    element.focus()
                    element.setSelectionRange(entry.key.length, entry.key.length)
                  }
                }}
                className="h-8 border-transparent text-xs"
                onChange={(event) => update(index, { key: event.target.value })}
              />
            </span>
            <span role="cell" className="min-w-0">
              <TextField
                aria-label={`Header ${index + 1} value`}
                value={entry.value}
                placeholder="Enter value"
                disabled={disabled}
                className="h-8 border-transparent text-xs"
                onChange={(event) => update(index, { value: event.target.value })}
              />
            </span>
            <span role="cell">
              <button
                type="button"
                disabled={disabled}
                aria-label={`Remove header ${index + 1}`}
                className={`
flex size-6 cursor-pointer items-center justify-center text-muted hover:text-text focus-visible:outline-muted
disabled:opacity-40
`}
                onClick={() => change(entries.filter((row) => row.id !== entry.id))}
              >
                <TrashIcon aria-hidden="true" className="size-3" />
              </button>
            </span>
          </div>
        </div>
      ))}
      <div role="row" className="flex min-w-0 gap-2">
        <span role="cell" className="w-6 shrink-0" />
        <div role="cell" className="min-w-0 flex-1 rounded-lg bg-elevated p-1">
          <TextField
            aria-label="New header key"
            maxLength={keyMaxLength}
            placeholder="Enter key"
            value=""
            disabled={disabled}
            className="h-8 border-transparent text-xs"
            onChange={(event) => {
              if (!event.target.value) return
              const key = crypto.randomUUID()
              pendingFocus.current = key
              const entry = { id: key, key: event.target.value, value: '' }
              if (acceptEntry && !acceptEntry(entry)) return
              change([...entries, entry])
            }}
          />
        </div>
      </div>
      <p id={`${id}-reorder`} className="sr-only">
        Drag to reorder, or focus the grip and use Up and Down arrow keys.
      </p>
      <span role="status" className="sr-only">
        {announcement}
      </span>
    </div>
  )
}
export const KeyValueEditor = Object.assign(Root, { Disclosure, Content })
