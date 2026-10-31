import { useEffect, useId, useRef, useState, type CSSProperties, type HTMLAttributes, type ReactNode } from 'react'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { MagnifyingGlassIcon } from '@phosphor-icons/react/dist/csr/MagnifyingGlass'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import { ToolbarAction } from './navigation'
import { cx, popupMotion } from './shared'

const focus = 'focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus'

export function Sidebar({ children, className, ...props }: HTMLAttributes<HTMLElement>) {
  return (
    <aside {...props} className={cx('flex h-full min-h-0 w-[244px] shrink-0 flex-col bg-background px-3 py-1', className)}>
      <nav aria-label="Workspace navigation" className="flex min-h-0 flex-1 flex-col gap-2 overflow-y-auto">
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
  organizations = [],
  selectedOrganizationId,
  onOrganizationChange,
}: {
  workspace: string
  mark?: ReactNode
  onWorkspaceClick: () => void
  onSearch: () => void
  onSettings: () => void
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
    <div className="mt-7 flex h-[34px] w-[221px] shrink-0 items-center justify-between pl-2 pr-1">
      <button
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
        className={cx('flex min-w-0 cursor-pointer items-center gap-0.5 rounded text-xs leading-[15px] text-text', focus)}
      >
        {mark ?? (
          <span
            aria-hidden="true"
            className={`
  flex size-[13px] shrink-0 items-center justify-center rounded-[4px] bg-[#ff6b00]
  text-[8px] leading-none font-bold text-white
`}
          >
            c
          </span>
        )}
        <span className="truncate font-bold leading-[15px]">{workspace}</span>
        <CaretDownIcon size={10} className="ml-0.5 shrink-0 text-muted" aria-hidden="true" />
      </button>
      <div
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
            const next =
              event.key === 'Home'
                ? preference.current
                : event.key === 'End'
                  ? switcher.current
                  : document.activeElement === preference.current
                    ? switcher.current
                    : preference.current
            next?.focus()
          }
          if (event.key === 'Escape') {
            event.preventDefault()
            menu.current?.hidePopover()
            trigger.current?.focus()
          }
          if (event.key === 'Tab') menu.current?.hidePopover()
        }}
        className={`${popupMotion}
  fixed inset-auto top-[anchor(bottom)] left-[anchor(left)] z-30 mt-2 w-[220px] max-w-[calc(100vw-16px)]
  rounded-lg border-[0.5px] border-border bg-menu p-1 text-text shadow-menu
  [position-try-fallbacks:flip-block,flip-inline]
`}
      >
        <button
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
        </button>
        <button
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
        </button>
        <div
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
          className={`${popupMotion}
            fixed inset-auto top-[anchor(top)] left-[anchor(right)] z-40 m-0 w-[220px] max-w-[calc(100vw-16px)]
            rounded-lg border-[0.5px] border-border bg-menu p-1 text-text shadow-menu
            [position-try-fallbacks:flip-inline,flip-block]
          `}
        >
          {organizations.length ? (
            organizations.map((organization) => (
              <button
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
              </button>
            ))
          ) : (
            <p className="p-2 text-xs text-muted">No organizations available</p>
          )}
        </div>
      </div>
      <ToolbarAction label="Search workspace" onClick={onSearch} className="size-3.5 rounded">
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
}

function AddAction({ onAdd, label }: { onAdd?: () => void; label: string }) {
  return onAdd ? (
    <ToolbarAction label={label} onClick={onAdd} className="absolute top-1.5 right-0 size-4 rounded text-disabled">
      <PlusIcon size={8.28} aria-hidden="true" />
    </ToolbarAction>
  ) : null
}

export function SidebarSection({ label, children, defaultOpen = true, onAdd, addLabel }: DisclosureProps) {
  const id = useId()
  return (
    <div className="relative shrink-0">
      <details open={defaultOpen} className="group/sidebar-section">
        <summary
          aria-controls={id}
          className={cx(
            `
flex h-7 cursor-pointer list-none items-center gap-1 rounded px-2 pr-7 text-xs font-medium leading-[13px]
text-disabled marker:hidden
[&::-webkit-details-marker]:hidden
`,
            focus,
          )}
        >
          {label}
          <CaretDownIcon size={8} weight="fill" aria-hidden="true" className="-rotate-90 group-open/sidebar-section:rotate-0" />
        </summary>
        <div id={id} className="mt-0.5 grid gap-0.5">
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
}: DisclosureProps & {
  icon: ReactNode
}) {
  const id = useId()
  return (
    <div className="relative">
      <details open={defaultOpen} className="group/sidebar-folder">
        <summary
          aria-controls={id}
          className={cx(
            `
flex h-7 cursor-pointer list-none items-center gap-1.5 rounded px-2 pr-7 text-xs font-medium
leading-[17px] text-muted hover:bg-elevated marker:hidden [&::-webkit-details-marker]:hidden
`,
            focus,
          )}
        >
          <span aria-hidden="true" className="flex size-3.5 shrink-0 items-center justify-center [&>svg]:size-3.5">
            {icon}
          </span>
          <span className="min-w-0 truncate">{label}</span>
          <CaretDownIcon size={8} weight="fill" aria-hidden="true" className="-rotate-90 group-open/sidebar-folder:rotate-0" />
        </summary>
        <div id={id} className="mt-0.5 grid gap-0.5 pl-4 pr-4 [&_a]:font-normal">
          {children}
        </div>
      </details>
      <AddAction onAdd={onAdd} label={addLabel ?? `Add ${label.toLowerCase()}`} />
    </div>
  )
}

export function SidebarItem({
  label,
  href,
  icon,
  selected = false,
  onNavigate,
}: {
  label: string
  href: string
  icon: ReactNode
  selected?: boolean
  onNavigate?: (href: string) => void
}) {
  return (
    <a
      href={href}
      aria-current={selected ? 'page' : undefined}
      title={label}
      className={cx(
        `
flex h-7 min-w-0 items-center gap-1.5 rounded-[6px] px-2 text-xs font-medium leading-[17px] text-muted
hover:bg-elevated hover:text-text aria-[current=page]:bg-elevated aria-[current=page]:text-text
`,
        focus,
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
    </a>
  )
}
