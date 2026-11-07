import {
  createContext,
  useContext,
  useId,
  useLayoutEffect,
  useRef,
  useState,
  type RefObject,
  type ReactNode,
  type ButtonHTMLAttributes,
  type HTMLAttributes,
  type CSSProperties,
} from 'react'
import { CaretRightIcon } from '@phosphor-icons/react/dist/csr/CaretRight'
import { cx, popupMotion } from './shared'
import { Checkbox, type CheckboxProps } from './selection'
interface Scope {
  id: string
  anchor: string
  trigger: RefObject<HTMLButtonElement | null>
  content: RefObject<HTMLDivElement | null>
}
const PopupContext = createContext<Scope | null>(null)
function usePopup() {
  const value = useContext(PopupContext)
  if (!value) throw new Error('Popup parts require Popup.')
  return value
}
function Root({ children }: { children: ReactNode }) {
  const id = useId(),
    trigger = useRef<HTMLButtonElement>(null),
    content = useRef<HTMLDivElement>(null)
  return (
    <PopupContext value={{ id, anchor: `--popup-${id.replace(/[^a-z0-9]/gi, '')}`, trigger, content }}>{children}</PopupContext>
  )
}
function Trigger({ className, ...props }: ButtonHTMLAttributes<HTMLButtonElement>) {
  const { id, anchor, trigger, content } = usePopup()
  return (
    <button
      type="button"
      {...props}
      ref={trigger}
      popoverTarget={id}
      aria-haspopup="menu"
      style={{ ...props.style, anchorName: anchor } as CSSProperties}
      className={cx(
        `flex size-7 cursor-pointer items-center justify-center rounded text-muted hover:bg-elevated
focus-visible:outline-focus`,
        className,
      )}
      onKeyDown={(event) => {
        props.onKeyDown?.(event)
        if (!event.defaultPrevented && ['ArrowDown', 'ArrowUp'].includes(event.key)) {
          event.preventDefault()
          content.current?.showPopover()
        }
      }}
    />
  )
}
function Content({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  const { id, anchor, trigger, content } = usePopup()
  return (
    <div
      {...props}
      id={id}
      ref={content}
      popover="auto"
      role="menu"
      style={{ ...props.style, positionAnchor: anchor } as CSSProperties}
      onToggle={(event) => {
        props.onToggle?.(event)
        if (event.newState === 'open')
          content.current?.querySelector<HTMLButtonElement>('[role^=menuitem]:not(:disabled)')?.focus()
      }}
      onKeyDown={(event) => {
        props.onKeyDown?.(event)
        if (event.defaultPrevented) return
        const items = Array.from(content.current?.querySelectorAll<HTMLButtonElement>('[role^=menuitem]:not(:disabled)') ?? [])
        if (['ArrowDown', 'ArrowUp', 'Home', 'End'].includes(event.key)) {
          event.preventDefault()
          const current = items.indexOf(document.activeElement as HTMLButtonElement)
          const next =
            event.key === 'Home'
              ? 0
              : event.key === 'End'
                ? items.length - 1
                : (current + (event.key === 'ArrowDown' ? 1 : -1) + items.length) % items.length
          items[next]?.focus()
        } else if (event.key === 'Escape' || event.key === 'Tab') {
          if (event.key === 'Escape') event.preventDefault()
          content.current?.hidePopover()
          trigger.current?.focus()
        }
      }}
      className={cx(
        `${popupMotion} fixed inset-auto top-[anchor(bottom)] right-[anchor(right)] z-30 mt-1 w-48 overflow-visible
    rounded-lg border border-border bg-menu p-1 text-text shadow-menu`,
        className,
      )}
    />
  )
}
function Item({ className, onClick, role = 'menuitem', ...props }: ButtonHTMLAttributes<HTMLButtonElement>) {
  const { trigger, content } = usePopup()
  return (
    <button
      type="button"
      {...props}
      role={role}
      className={cx(
        `flex h-8 w-full cursor-pointer items-center rounded px-2 text-xs hover:bg-border
focus-visible:outline-focus disabled:opacity-50`,
        className,
      )}
      onClick={(event) => {
        onClick?.(event)
        if (!event.defaultPrevented) {
          content.current?.hidePopover()
          trigger.current?.focus()
        }
      }}
    />
  )
}
function IconTrigger(props: ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <Trigger
      {...props}
      className={cx(
        'size-7 rounded-full border border-border/60 bg-inset text-muted hover:border-input-border hover:text-text',
        props.className,
      )}
    />
  )
}
function Separator() {
  return <div role="separator" className="my-1 h-px bg-border/60" />
}
function CheckboxItem({ children, className, ...props }: CheckboxProps & { children: ReactNode }) {
  return (
    <label className={cx('flex min-h-9 cursor-pointer items-center gap-3 rounded px-2 text-[13px] hover:bg-border', className)}>
      <Checkbox {...props} role="menuitemcheckbox" />
      {children}
    </label>
  )
}
function Submenu({ label, children }: { label: ReactNode; children: ReactNode }) {
  const [open, setOpen] = useState(false)
  const [left, setLeft] = useState(false)
  const [top, setTop] = useState(-4)
  const trigger = useRef<HTMLButtonElement>(null)
  const content = useRef<HTMLDivElement>(null)
  const id = useId()
  useLayoutEffect(() => {
    if (!open || !content.current || !trigger.current) return
    const bounds = trigger.current.getBoundingClientRect()
    const height = content.current.offsetHeight
    setTop(Math.max(8 - bounds.top, Math.min(-4, window.innerHeight - 24 - bounds.top - height)))
  }, [open])
  function show() {
    const rect = trigger.current?.getBoundingClientRect()
    setLeft(!!rect && rect.right + 200 > window.innerWidth)
    setOpen(true)
  }
  return (
    <div className="relative" onPointerEnter={show} onPointerLeave={() => setOpen(false)}>
      <button
        ref={trigger}
        type="button"
        role="menuitem"
        aria-haspopup="menu"
        aria-expanded={open}
        aria-controls={open ? id : undefined}
        className={`
flex h-8 w-full cursor-pointer items-center justify-between rounded px-2 text-xs hover:bg-border
focus-visible:outline-muted
`}
        onClick={() => {
          if (open) setOpen(false)
          else show()
        }}
        onKeyDown={(event) => {
          if (event.key === 'ArrowRight') {
            event.preventDefault()
            event.stopPropagation()
            show()
            requestAnimationFrame(() => content.current?.querySelector<HTMLButtonElement>('button')?.focus())
          }
        }}
      >
        {label}
        <CaretRightIcon className="size-3" aria-hidden="true" />
      </button>
      {open ? (
        <div
          id={id}
          ref={content}
          role="menu"
          aria-label={typeof label === 'string' ? label : undefined}
          style={{ top }}
          className={cx(
            `
motion-safe:animate-popup-in absolute z-40 max-h-[calc(100dvh-1rem)] w-48 overflow-y-auto
rounded-lg border border-border bg-menu p-1 text-text
shadow-menu
`,
            left ? 'right-full' : 'left-full',
          )}
          onKeyDown={(event) => {
            const items = Array.from(
              content.current?.querySelectorAll<HTMLButtonElement>('[role^=menuitem]:not(:disabled)') ?? [],
            )
            if (['ArrowDown', 'ArrowUp', 'Home', 'End'].includes(event.key)) {
              event.preventDefault()
              event.stopPropagation()
              const current = items.indexOf(document.activeElement as HTMLButtonElement)
              const next =
                event.key === 'Home'
                  ? 0
                  : event.key === 'End'
                    ? items.length - 1
                    : (current + (event.key === 'ArrowDown' ? 1 : -1) + items.length) % items.length
              items[next]?.focus()
            } else if (['ArrowLeft', 'Escape'].includes(event.key)) {
              event.preventDefault()
              event.stopPropagation()
              setOpen(false)
              trigger.current?.focus()
            }
          }}
        >
          {children}
        </div>
      ) : null}
    </div>
  )
}
export const Popup = Object.assign(Root, { Trigger, IconTrigger, Content, Item, CheckboxItem, Separator, Submenu })
