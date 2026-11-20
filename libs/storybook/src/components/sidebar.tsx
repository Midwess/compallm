import { useId, useRef, type HTMLAttributes, type ReactNode, type PointerEvent } from 'react'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { MagnifyingGlassIcon } from '@phosphor-icons/react/dist/csr/MagnifyingGlass'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import { ToolbarAction } from './navigation'
import { cx } from './shared'
import { animated, useDisclosure } from './motion'
import { Popup } from './popup'
import { useControllableState } from './composition'
import { interactionFeedback, type InteractionFeedback } from './interaction-feedback'

const focus = 'focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring'

export interface SidebarProps extends HTMLAttributes<HTMLElement> {
  navigationLabel?: string
  resizable?: boolean
  width?: number
  defaultWidth?: number
  minWidth?: number
  maxWidth?: number
  onWidthChange?(width: number): void
  onResizeFeedback?(kind: InteractionFeedback, width: number): void
}

export function Sidebar({
  children,
  className,
  id: suppliedId,
  navigationLabel = 'Workspace navigation',
  resizable = true,
  width: suppliedWidth,
  defaultWidth = 244,
  minWidth = 200,
  maxWidth = 400,
  onWidthChange,
  onResizeFeedback,
  style,
  ...props
}: SidebarProps) {
  const generatedId = useId()
  const id = suppliedId ?? generatedId
  const root = useRef<HTMLElement>(null)
  const grip = useRef<HTMLDivElement>(null)
  const drag = useRef<{ pointer: number; startX: number; startWidth: number; width: number } | null>(null)
  const [width, changeWidth] = useControllableState(suppliedWidth, defaultWidth, onWidthChange)
  const minimum = Math.max(160, minWidth)
  const maximum = Math.max(minimum, maxWidth)
  const constrain = (next: number) => Math.round(Math.max(minimum, Math.min(maximum, next)))
  const feedback = (kind: InteractionFeedback, next: number) => {
    if (onResizeFeedback) onResizeFeedback(kind, next)
    else interactionFeedback(kind)
  }
  const paint = (next: number) => {
    if (root.current) root.current.style.width = `${next}px`
    grip.current?.setAttribute('aria-valuenow', String(next))
    grip.current?.setAttribute('aria-valuetext', `${next} pixels`)
  }
  const finish = (cancel = false) => {
    const current = drag.current
    if (!current) return
    drag.current = null
    root.current?.removeAttribute('data-resizing')
    const next = cancel ? current.startWidth : current.width
    paint(next)
    if (!cancel) {
      changeWidth(next)
      if (next !== current.startWidth) feedback('commit', next)
    }
  }
  const move = (event: PointerEvent<HTMLDivElement>) => {
    const current = drag.current
    if (!current || current.pointer !== event.pointerId) return
    const raw = constrain(current.startWidth + event.clientX - current.startX)
    const snap = [minimum, constrain(defaultWidth), maximum].find((point) => Math.abs(raw - point) <= 6) ?? null
    const next = snap ?? raw
    if (snap !== null && next !== current.width) feedback('snap', next)
    current.width = next
    paint(next)
  }
  return (
    <aside
      {...props}
      id={id}
      ref={root}
      style={{ ...style, width: constrain(width) }}
      className={cx(
        'group/sidebar relative flex h-full min-h-0 shrink-0 flex-col border-r border-divider bg-sidebar px-3 py-1 select-none',
        '[&_input]:select-text [&_textarea]:select-text **:[[contenteditable=true]]:select-text',
        className,
      )}
    >
      <nav
        aria-label={navigationLabel}
        className="flex min-h-0 min-w-0 flex-1 flex-col gap-2.5 overflow-x-hidden overflow-y-auto overscroll-none"
      >
        {children}
      </nav>
      {resizable ? (
        <div
          ref={grip}
          role="separator"
          aria-label="Resize sidebar"
          aria-orientation="vertical"
          aria-controls={id}
          aria-describedby={`${id}-resize-hint`}
          aria-valuemin={minimum}
          aria-valuemax={maximum}
          aria-valuenow={constrain(width)}
          aria-valuetext={`${constrain(width)} pixels`}
          tabIndex={0}
          title="Drag to resize. Double-click to reset."
          className="group/grip absolute inset-y-0 -right-1 z-20 flex w-2 touch-none cursor-col-resize
          items-center justify-center outline-none"
          onPointerDown={(event) => {
            if (event.button !== 0 || !root.current) return
            event.preventDefault()
            event.currentTarget.focus({ preventScroll: true })
            const current = root.current.getBoundingClientRect().width
            drag.current = { pointer: event.pointerId, startX: event.clientX, startWidth: current, width: current }
            root.current.dataset.resizing = ''
            event.currentTarget.setPointerCapture(event.pointerId)
          }}
          onPointerMove={move}
          onPointerUp={(event) => {
            if (drag.current?.pointer !== event.pointerId) return
            finish()
            event.currentTarget.releasePointerCapture(event.pointerId)
          }}
          onPointerCancel={() => finish(true)}
          onLostPointerCapture={() => finish(true)}
          onDoubleClick={() => {
            const next = constrain(defaultWidth)
            paint(next)
            changeWidth(next)
            feedback('snap', next)
          }}
          onKeyDown={(event) => {
            if (event.key === 'Escape' && drag.current) {
              event.preventDefault()
              const pointer = drag.current.pointer
              finish(true)
              if (event.currentTarget.hasPointerCapture(pointer)) event.currentTarget.releasePointerCapture(pointer)
              return
            }
            const current = root.current?.getBoundingClientRect().width ?? width
            const step = event.shiftKey ? 24 : 8
            const next =
              event.key === 'ArrowLeft'
                ? current - step
                : event.key === 'ArrowRight'
                  ? current + step
                  : event.key === 'Home'
                    ? minimum
                    : event.key === 'End'
                      ? maximum
                      : event.key === 'Enter'
                        ? defaultWidth
                        : null
            if (next === null) return
            event.preventDefault()
            const bounded = constrain(next)
            paint(bounded)
            changeWidth(bounded)
            if (bounded !== current) feedback('snap', bounded)
          }}
        >
          <span
            aria-hidden="true"
            className="pointer-events-none h-10 w-0.5 rounded-full bg-divider
            group-hover/grip:bg-muted-foreground/60 group-focus-visible/grip:bg-ring
            group-data-resizing/sidebar:bg-ring"
          />
        </div>
      ) : null}
      {resizable ? (
        <span id={`${id}-resize-hint`} className="sr-only">
          Drag to resize, or use Left and Right arrow keys. Shift moves faster. Home and End select the limits. Enter or
          double-click resets the width. Escape cancels a drag.
        </span>
      ) : null}
    </aside>
  )
}

export function SidebarHeader({
  workspace,
  mark,
  onWorkspaceClick,
  onSearch,
  onSettings,
  onSignOut,
  organizations = [],
  selectedOrganizationId,
  onOrganizationChange,
}: {
  workspace: string
  mark?: ReactNode
  onWorkspaceClick: () => void
  onSearch: () => void
  onSettings: () => void
  onSignOut?: () => void
  organizations?: { id: string; name: string }[]
  selectedOrganizationId?: string
  onOrganizationChange?: (id: string) => void
}) {
  return (
    <div className="mt-3 flex h-10 w-full min-w-0 shrink-0 items-center justify-between gap-2 pl-2 pr-1">
      <Popup.Root>
        <Popup.Trigger
          aria-label={`Switch workspace: ${workspace}`}
          onClick={onWorkspaceClick}
          className={cx(
            'h-8 w-auto min-w-0 flex-1 justify-start gap-1 -ml-2 rounded-xl px-2 text-xs leading-4 text-foreground',
            focus,
          )}
        >
          {mark ?? (
            <span
              aria-hidden="true"
              className="flex aspect-square h-4.5 shrink-0 items-center justify-center rounded-md bg-brand-mark
              p-0.5 text-xs leading-none font-bold text-brand-mark-foreground"
            >
              c
            </span>
          )}
          <span className="truncate font-medium leading-4">{workspace}</span>
          <CaretDownIcon size={10} className="ml-0.5 shrink-0 text-muted-foreground" aria-hidden="true" />
        </Popup.Trigger>
        <Popup.Content aria-label="Workspace options" align="start" sideOffset={8} className="w-55">
          <Popup.Item onSelect={onSettings}>Settings</Popup.Item>
          <Popup.Sub>
            <Popup.SubTrigger>Switch organization</Popup.SubTrigger>
            <Popup.SubContent aria-label="Switch organization">
              {organizations.length ? (
                <Popup.RadioGroup value={selectedOrganizationId} onValueChange={(id: string) => onOrganizationChange?.(id)}>
                  {organizations.map((organization) => (
                    <Popup.RadioItem
                      key={organization.id}
                      value={organization.id}
                      className="aria-checked:bg-menu-selection aria-checked:text-selection-foreground"
                    >
                      <span className="min-w-0 flex-1 truncate">{organization.name}</span>
                    </Popup.RadioItem>
                  ))}
                </Popup.RadioGroup>
              ) : (
                <p className="p-2 text-xs text-muted-foreground">No organizations available</p>
              )}
            </Popup.SubContent>
          </Popup.Sub>
          {onSignOut ? (
            <Popup.Item className="text-destructive" onSelect={onSignOut}>
              Sign out
            </Popup.Item>
          ) : null}
        </Popup.Content>
      </Popup.Root>
      <ToolbarAction label="Search workspace" onClick={onSearch} className="size-8 shrink-0 rounded-full">
        <MagnifyingGlassIcon size={14} aria-hidden="true" />
      </ToolbarAction>
    </div>
  )
}

interface DisclosureProps {
  label: string
  children: ReactNode
  open?: boolean
  onOpenChange?(open: boolean): void
  defaultOpen?: boolean
  onAdd?: () => void
  addLabel?: string
  className?: string
}

function AddAction({ onAdd, label }: { onAdd?: () => void; label: string }) {
  return onAdd ? (
    <ToolbarAction label={label} onClick={onAdd} className="absolute top-1.5 right-0 size-4 rounded-full text-foreground">
      <PlusIcon size={8.28} aria-hidden="true" />
    </ToolbarAction>
  ) : null
}

export function SidebarSection({
  label,
  children,
  defaultOpen = true,
  open,
  onOpenChange,
  onAdd,
  addLabel,
  className,
}: DisclosureProps) {
  const id = useId()
  const { initialOpen, detailsRef, contentRef, caretRef, toggle } = useDisclosure(defaultOpen, open, onOpenChange)
  return (
    <div className={cx('relative shrink-0', className)}>
      <details ref={detailsRef} open={initialOpen} className="group/sidebar-section">
        <summary
          onClick={(event) => {
            event.preventDefault()
            toggle()
          }}
          aria-controls={id}
          className={cx(
            `
flex h-7 cursor-pointer list-none items-center gap-1 rounded-xl px-2 pr-7 text-xs font-medium leading-3
 text-muted-foreground marker:hidden
[&::-webkit-details-marker]:hidden
`,
            focus,
          )}
        >
          {label}
          <svg width="7" height="7" viewBox="0 0 8 8" aria-hidden="true" ref={caretRef}>
            <path
              d="M2 1.72h4a.4.4 0 0 1 .35.6l-2 3.96a.4.4 0 0 1-.7 0l-2-3.96a.4.4 0 0 1 .35-.6Z"
              fill="currentColor"
              stroke="currentColor"
              strokeWidth="0.8"
              strokeLinejoin="round"
            />
          </svg>
        </summary>
        <div ref={contentRef} id={id} className="grid">
          {children}
        </div>
      </details>
      <AddAction onAdd={onAdd} label={addLabel ?? `Add ${label.toLowerCase()}`} />
    </div>
  )
}

export function SidebarFolder({
  label,
  icon,
  children,
  defaultOpen = true,
  open,
  onOpenChange,
  onAdd,
  addLabel,
  className,
}: DisclosureProps & {
  icon: ReactNode
}) {
  const id = useId()
  const { initialOpen, detailsRef, contentRef, caretRef, toggle, expand } = useDisclosure(defaultOpen, open, onOpenChange)
  return (
    <div className={cx('relative', className)}>
      <details ref={detailsRef} open={initialOpen} className="group/sidebar-folder">
        <summary
          onClick={(event) => {
            event.preventDefault()
            toggle()
          }}
          aria-controls={id}
          className={cx(
            `
flex h-7 cursor-pointer list-none items-center gap-1.5 rounded-xl border-y border-transparent bg-clip-padding
px-2 pr-7 text-xsm font-normal leading-4 text-muted-foreground
hover:bg-hover marker:hidden [&::-webkit-details-marker]:hidden
`,
            focus,
          )}
        >
          <span aria-hidden="true" className="flex size-3.5 shrink-0 items-center justify-center [&>svg]:size-3.5">
            {icon}
          </span>
          <span className="min-w-0 truncate">{label}</span>
          <svg width="7" height="7" viewBox="0 0 8 8" aria-hidden="true" ref={caretRef}>
            <path
              d="M2 1.72h4a.4.4 0 0 1 .35.6l-2 3.96a.4.4 0 0 1-.7 0l-2-3.96a.4.4 0 0 1 .35-.6Z"
              fill="currentColor"
              stroke="currentColor"
              strokeWidth="0.8"
              strokeLinejoin="round"
            />
          </svg>
        </summary>
        <div ref={contentRef} id={id} className="mt-0.5 grid pl-4 pr-4 [&_a]:font-normal">
          {children}
        </div>
      </details>
      <AddAction
        onAdd={
          onAdd
            ? () => {
                expand()
                onAdd()
              }
            : undefined
        }
        label={addLabel ?? `Add ${label.toLowerCase()}`}
      />
    </div>
  )
}

export function SidebarItem({
  label,
  href,
  icon,
  selected = false,
  onNavigate,
  className,
}: {
  label: string
  href: string
  icon: ReactNode
  selected?: boolean
  onNavigate?: (href: string) => void
  className?: string
}) {
  return (
    <animated.a
      href={href}
      aria-current={selected ? 'page' : undefined}
      title={label}
      className={cx(
        `
flex h-7 min-w-0 items-center gap-1.5 rounded-xl border-y border-transparent bg-clip-padding px-2
text-xsm font-medium leading-4.25 text-muted-foreground hover:bg-hover hover:text-foreground
aria-[current=page]:bg-selection aria-[current=page]:text-selection-foreground
`,
        focus,
        className,
      )}
      onClick={(event) => {
        if (
          !onNavigate ||
          event.defaultPrevented ||
          event.button !== 0 ||
          event.metaKey ||
          event.ctrlKey ||
          event.shiftKey ||
          event.altKey
        )
          return
        event.preventDefault()
        onNavigate(href)
      }}
    >
      <span aria-hidden="true" className="flex size-3.5 shrink-0 items-center justify-center [&>svg]:size-3.5">
        {icon}
      </span>
      <span className="min-w-0 truncate">{label}</span>
    </animated.a>
  )
}
