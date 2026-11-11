import { useEffect, useId, useRef, useState, type CSSProperties, type HTMLAttributes, type ReactNode } from 'react'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { MagnifyingGlassIcon } from '@phosphor-icons/react/dist/csr/MagnifyingGlass'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import { ToolbarAction } from './navigation'
import { cx } from './shared'
import { animated, overlay, useDisclosure } from './motion'

const focus = 'focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus'

export function Sidebar({
  children,
  className,
  navigationLabel = 'Workspace navigation',
  ...props
}: HTMLAttributes<HTMLElement> & { navigationLabel?: string }) {
  return (
    <aside
      {...props}
      className={cx('flex h-full min-h-0 w-[244px] shrink-0 flex-col overflow-x-hidden bg-background px-3 py-1', className)}
    >
      <nav
        aria-label={navigationLabel}
        className="flex min-h-0 min-w-0 flex-1 flex-col gap-2.5 overflow-x-hidden overflow-y-auto"
      >
        {children}
      </nav>
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
  const id = useId()
  const anchor = `--workspace-${id.replace(/[^a-zA-Z0-9-]/g, '')}`
  const menu = useRef<HTMLDivElement>(null)
  const trigger = useRef<HTMLButtonElement>(null)
  const preference = useRef<HTMLButtonElement>(null)
  const [open, setOpen] = useState(false)
  const [switching, setSwitching] = useState(false)
  const submenu = useRef<HTMLDivElement>(null)
  const switcher = useRef<HTMLButtonElement>(null)
  const hideTimer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined)
  const submenuId = `${id}-organizations`
  const submenuAnchor = `${anchor}-organizations`
  const cancelHide = () => {
    if (hideTimer.current) clearTimeout(hideTimer.current)
  }
  const showOrganizations = (focus = false) => {
    cancelHide()
    submenu.current?.showPopover()
    if (focus) submenu.current?.querySelector<HTMLButtonElement>('button')?.focus()
  }
  const hideOrganizations = () => {
    cancelHide()
    hideTimer.current = setTimeout(() => submenu.current?.hidePopover(), 150)
  }
  useEffect(
    () => () => {
      if (hideTimer.current) clearTimeout(hideTimer.current)
    },
    [],
  )
  const close = () => {
    submenu.current?.hidePopover()
    menu.current?.hidePopover()
    trigger.current?.focus()
  }

  return (
    <div className="mt-3 flex h-10 w-full min-w-0 shrink-0 items-center justify-between gap-2 pl-2 pr-1">
      <animated.button
        ref={trigger}
        popoverTarget={id}
        style={{ anchorName: anchor } as CSSProperties}
        type="button"
        aria-label={`Switch workspace: ${workspace}`}
        aria-haspopup="menu"
        aria-expanded={open}
        aria-controls={id}
        onClick={onWorkspaceClick}
        onKeyDown={(event) => {
          if (event.key === 'ArrowDown') {
            event.preventDefault()
            menu.current?.showPopover()
            preference.current?.focus()
          }
        }}
        className={cx('flex min-w-0 flex-1 cursor-pointer items-center gap-1 rounded text-xs leading-4 text-text', focus)}
      >
        {mark ?? (
          <span
            aria-hidden="true"
            className={`
  flex aspect-square p-0.5 h-4.5 shrink-0 items-center justify-center rounded-md bg-[#ff6b00]
  text-xs leading-none font-bold text-white
`}
          >
            c
          </span>
        )}
        <span className="truncate font-medium leading-4">{workspace}</span>
        <CaretDownIcon size={10} className="ml-0.5 shrink-0 text-muted" aria-hidden="true" />
      </animated.button>
      <overlay.div
        ref={menu}
        id={id}
        popover="auto"
        role="menu"
        aria-label="Workspace options"
        style={{ positionAnchor: anchor } as CSSProperties}
        onToggle={(event) => {
          if (event.target !== event.currentTarget) return
          const expanded = event.newState === 'open'
          setOpen(expanded)
          if (expanded) preference.current?.focus()
        }}
        onKeyDown={(event) => {
          if (['ArrowDown', 'ArrowUp', 'Home', 'End'].includes(event.key)) {
            event.preventDefault()
            const items = Array.from(menu.current?.querySelectorAll<HTMLButtonElement>(':scope > button') ?? [])
            const index = items.indexOf(document.activeElement as HTMLButtonElement)
            const next =
              event.key === 'Home'
                ? 0
                : event.key === 'End'
                  ? items.length - 1
                  : (index + (event.key === 'ArrowDown' ? 1 : -1) + items.length) % items.length
            items[next]?.focus()
          }
          if (event.key === 'Escape') {
            event.preventDefault()
            menu.current?.hidePopover()
            trigger.current?.focus()
          }
          if (event.key === 'Tab') menu.current?.hidePopover()
        }}
        className={`
  fixed inset-auto top-[anchor(bottom)] left-[anchor(left)] z-30 mt-2 w-[220px] max-w-[calc(100vw-16px)]
  rounded-lg border-[0.5px] border-border bg-menu p-1 text-text shadow-menu
  [position-try-fallbacks:flip-block,flip-inline]
`}
      >
        <animated.button
          ref={preference}
          type="button"
          role="menuitem"
          className={cx('flex h-8 w-full cursor-pointer items-center gap-2 rounded px-2 text-xs hover:bg-border', focus)}
          onClick={() => {
            close()
            onSettings()
          }}
        >
          Settings
        </animated.button>
        <animated.button
          ref={switcher}
          type="button"
          role="menuitem"
          aria-haspopup="menu"
          aria-expanded={switching}
          aria-controls={submenuId}
          style={{ anchorName: submenuAnchor } as CSSProperties}
          className={cx('flex h-8 w-full cursor-pointer items-center gap-2 rounded px-2 text-xs hover:bg-border', focus)}
          onPointerEnter={() => showOrganizations()}
          onPointerLeave={hideOrganizations}
          onClick={() => showOrganizations(true)}
          onKeyDown={(event) => {
            if (event.key === 'ArrowRight') {
              event.preventDefault()
              event.stopPropagation()
              showOrganizations(true)
            }
          }}
        >
          <span className="flex-1 text-left">Switch organization</span>
        </animated.button>
        <overlay.div
          ref={submenu}
          id={submenuId}
          popover="auto"
          role="menu"
          aria-label="Organizations"
          style={{ positionAnchor: submenuAnchor } as CSSProperties}
          onPointerEnter={cancelHide}
          onPointerLeave={hideOrganizations}
          onToggle={(event) => setSwitching(event.newState === 'open')}
          onKeyDown={(event) => {
            event.stopPropagation()
            if (event.key === 'Escape' || event.key === 'ArrowLeft') {
              event.preventDefault()
              submenu.current?.hidePopover()
              switcher.current?.focus()
            } else if (['ArrowDown', 'ArrowUp', 'Home', 'End'].includes(event.key)) {
              event.preventDefault()
              const items = Array.from(submenu.current?.querySelectorAll<HTMLButtonElement>('button') ?? [])
              const index = items.indexOf(document.activeElement as HTMLButtonElement)
              const next =
                event.key === 'Home'
                  ? 0
                  : event.key === 'End'
                    ? items.length - 1
                    : (index + (event.key === 'ArrowDown' ? 1 : -1) + items.length) % items.length
              items[next]?.focus()
            } else if (event.key === 'Tab') close()
          }}
          className={`
            fixed inset-auto top-[anchor(top)] left-[anchor(right)] z-40 m-0 w-55 max-w-[calc(100vw-16px)]
            rounded-lg border-[0.5px] border-border bg-menu p-1 text-text shadow-menu
            [position-try-fallbacks:flip-inline,flip-block]
          `}
        >
          {organizations.length ? (
            organizations.map((organization) => (
              <animated.button
                key={organization.id}
                type="button"
                role="menuitemradio"
                aria-checked={organization.id === selectedOrganizationId}
                className={cx(
                  'flex min-h-8 w-full cursor-pointer items-center rounded px-2 text-left text-xs',
                  'hover:bg-border aria-checked:bg-border',
                  focus,
                )}
                onClick={() => {
                  close()
                  onOrganizationChange?.(organization.id)
                }}
              >
                <span className="min-w-0 flex-1 truncate">{organization.name}</span>
              </animated.button>
            ))
          ) : (
            <p className="p-2 text-xs text-muted">No organizations available</p>
          )}
        </overlay.div>
        {onSignOut ? (
          <animated.button
            type="button"
            role="menuitem"
            className={cx('flex h-8 w-full cursor-pointer items-center rounded px-2 text-xs text-error hover:bg-border', focus)}
            onClick={() => {
              close()
              onSignOut()
            }}
          >
            Sign out
          </animated.button>
        ) : null}
      </overlay.div>
      <ToolbarAction label="Search workspace" onClick={onSearch} className="size-3.5 shrink-0 rounded">
        <MagnifyingGlassIcon size={14} aria-hidden="true" />
      </ToolbarAction>
    </div>
  )
}

interface DisclosureProps {
  label: string
  children: ReactNode
  defaultOpen?: boolean
  onAdd?: () => void
  addLabel?: string
  className?: string
}

function AddAction({ onAdd, label }: { onAdd?: () => void; label: string }) {
  return onAdd ? (
    <ToolbarAction label={label} onClick={onAdd} className="absolute top-1.5 right-0 size-4 rounded text-text">
      <PlusIcon size={8.28} aria-hidden="true" />
    </ToolbarAction>
  ) : null
}

export function SidebarSection({ label, children, defaultOpen = true, onAdd, addLabel, className }: DisclosureProps) {
  const id = useId()
  const { detailsRef, contentRef, caretRef, toggle } = useDisclosure(defaultOpen)
  return (
    <div className={cx('relative shrink-0', className)}>
      <details ref={detailsRef} open={defaultOpen} className="group/sidebar-section">
        <summary
          onClick={(event) => {
            event.preventDefault()
            toggle()
          }}
          aria-controls={id}
          className={cx(
            `
flex h-7 cursor-pointer list-none items-center gap-1 rounded px-2 pr-7 text-xs font-medium leading-3
 text-muted marker:hidden
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
  onAdd,
  addLabel,
  className,
}: DisclosureProps & {
  icon: ReactNode
}) {
  const id = useId()
  const { detailsRef, contentRef, caretRef, toggle, expand } = useDisclosure(defaultOpen)
  return (
    <div className={cx('relative', className)}>
      <details ref={detailsRef} open={defaultOpen} className="group/sidebar-folder">
        <summary
          onClick={(event) => {
            event.preventDefault()
            toggle()
          }}
          aria-controls={id}
          className={cx(
            `
flex h-7 cursor-pointer list-none items-center gap-1.5 rounded px-2 pr-7 text-xsm font-normal
leading-4 text-muted hover:bg-elevated marker:hidden [&::-webkit-details-marker]:hidden
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
flex h-7 min-w-0 items-center gap-1.5 rounded-[6px] px-2 text-xsm font-medium leading-[17px] text-muted
hover:bg-elevated hover:text-text aria-[current=page]:bg-elevated aria-[current=page]:text-text
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
