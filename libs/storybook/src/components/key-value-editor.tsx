import { useFormField } from './form'
import { motion } from 'motion/react'
import { animated, useDisclosure, useTransition } from './motion'
import { createContext, useContext, useRef, type ReactNode } from 'react'
import { useReorder } from './use-reorder'
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
  name,
  value: suppliedValue,
  onValueChange: suppliedChange,
  disabled = false,
  children,
  className,
  acceptEntry,
  keyMaxLength,
}: {
  name?: string
  value?: KeyValueEntry[]
  onValueChange?(entries: KeyValueEntry[]): void
  disabled?: boolean
  children: ReactNode
  className?: string
  keyMaxLength?: number
  acceptEntry?(entry: KeyValueEntry): boolean
}) {
  const field = useFormField<KeyValueEntry[]>(name)
  const value = field ? (field.value ?? []) : (suppliedValue ?? [])
  const onValueChange = (value: KeyValueEntry[]) => {
    if (field) field.change(value)
    else suppliedChange?.(value)
  }
  return (
    <EditorContext value={{ entries: value, change: onValueChange, disabled, acceptEntry, keyMaxLength }}>
      <div
        onBlur={(event) => {
          if (!event.currentTarget.contains(event.relatedTarget)) field?.blur()
        }}
        className={cx('w-full min-w-0 max-w-full', className)}
      >
        {children}
      </div>
    </EditorContext>
  )
}
function Disclosure({ children = 'Headers', defaultOpen = false }: { children?: ReactNode; defaultOpen?: boolean }) {
  const { detailsRef, contentRef, caretRef, toggle } = useDisclosure(defaultOpen)
  return (
    <details ref={detailsRef} className="group/headers w-full min-w-0 max-w-full" open={defaultOpen || undefined}>
      <summary
        onClick={(event) => {
          event.preventDefault()
          toggle()
        }}
        className="flex h-7 w-fit cursor-pointer list-none items-center gap-1 text-sm font-medium"
      >
        {children}
        <CaretDownIcon weight="fill" aria-hidden="true" className="size-2" ref={caretRef} />
      </summary>
      <div ref={contentRef} className="min-w-0 pt-3">
        <Content />
      </div>
    </details>
  )
}
function Content() {
  const timing = useTransition('reorder')
  const { entries, change, disabled, acceptEntry, keyMaxLength } = useEditor()
  const pendingFocus = useRef<string | null>(null)
  const { rowNodes, drag, preview, announcement, id, keyboardMove, startDrag, pointerMove, endDrag } = useReorder(
    entries.map((entry) => entry.id),
    (order) => change(order.flatMap((key) => entries.filter((entry) => entry.id === key))),
    disabled,
    'Header',
  )
  function update(index: number, patch: Partial<KeyValueEntry>) {
    const next = { ...entries[index], ...patch }
    if (acceptEntry && !acceptEntry(next)) return
    change(entries.map((entry, position) => (position === index ? next : entry)))
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
        <motion.div
          key={entry.id}
          initial={false}
          animate={{ y: preview?.offsets[entry.id] ?? 0 }}
          transition={timing}
          role="row"
          data-dragging={preview?.key === entry.id || undefined}
          ref={(node) => {
            if (node) rowNodes.current.set(entry.id, node)
            else rowNodes.current.delete(entry.id)
          }}
          className={cx('relative flex min-w-0 items-center gap-2', preview?.key === entry.id && 'opacity-0')}
        >
          <span role="cell" className="w-6 shrink-0">
            <animated.button
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
            </animated.button>
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
              <animated.button
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
              </animated.button>
            </span>
          </div>
        </motion.div>
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
