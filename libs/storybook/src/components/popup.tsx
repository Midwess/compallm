import {
  createContext,
  useContext,
  useId,
  useRef,
  type RefObject,
  type ReactNode,
  type ButtonHTMLAttributes,
  type HTMLAttributes,
  type CSSProperties,
} from 'react'
import { cx, popupMotion } from './shared'
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
          content.current?.querySelector<HTMLButtonElement>('[role=menuitem]:not(:disabled)')?.focus()
      }}
      onKeyDown={(event) => {
        props.onKeyDown?.(event)
        if (event.defaultPrevented) return
        const items = Array.from(content.current?.querySelectorAll<HTMLButtonElement>('[role=menuitem]:not(:disabled)') ?? [])
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
        `${popupMotion} fixed inset-auto top-[anchor(bottom)] right-[anchor(right)] z-30 mt-1 w-48
    rounded-lg border border-border bg-menu p-1 text-text shadow-menu`,
        className,
      )}
    />
  )
}
function Item({ className, onClick, ...props }: ButtonHTMLAttributes<HTMLButtonElement>) {
  const { trigger, content } = usePopup()
  return (
    <button
      type="button"
      {...props}
      role="menuitem"
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
export const Popup = Object.assign(Root, { Trigger, Content, Item })
