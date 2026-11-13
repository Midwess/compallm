import { useId, type HTMLAttributes, type ReactNode } from 'react'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { MagnifyingGlassIcon } from '@phosphor-icons/react/dist/csr/MagnifyingGlass'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import { ToolbarAction } from './navigation'
import { cx } from './shared'
import { animated, useDisclosure } from './motion'
import { Popup } from './popup'

const focus = 'focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring'

export function Sidebar({
  children,
  className,
  navigationLabel = 'Workspace navigation',
  ...props
}: HTMLAttributes<HTMLElement> & { navigationLabel?: string }) {
  return (
    <aside
      {...props}
      className={cx('flex h-full min-h-0 w-61 shrink-0 flex-col overflow-x-hidden bg-background px-3 py-1', className)}
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
  return (
    <div className="mt-3 flex h-10 w-full min-w-0 shrink-0 items-center justify-between gap-2 pl-2 pr-1">
      <Popup>
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
          <Popup.Item onClick={onSettings}>Settings</Popup.Item>
          <Popup.Submenu label="Switch organization">
            {organizations.length ? (
              <Popup.RadioGroup value={selectedOrganizationId} onValueChange={(id: string) => onOrganizationChange?.(id)}>
                {organizations.map((organization) => (
                  <Popup.RadioItem key={organization.id} value={organization.id} className="aria-checked:bg-border">
                    <span className="min-w-0 flex-1 truncate">{organization.name}</span>
                  </Popup.RadioItem>
                ))}
              </Popup.RadioGroup>
            ) : (
              <p className="p-2 text-xs text-muted-foreground">No organizations available</p>
            )}
          </Popup.Submenu>
          {onSignOut ? (
            <Popup.Item className="text-destructive" onClick={onSignOut}>
              Sign out
            </Popup.Item>
          ) : null}
        </Popup.Content>
      </Popup>
      <ToolbarAction label="Search workspace" onClick={onSearch} className="size-8 shrink-0 rounded-full">
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
    <ToolbarAction label={label} onClick={onAdd} className="absolute top-1.5 right-0 size-4 rounded-full text-foreground">
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
flex h-7 cursor-pointer list-none items-center gap-1.5 rounded-xl border-y border-transparent bg-clip-padding
px-2 pr-7 text-xsm font-normal leading-4 text-muted-foreground
hover:bg-elevated marker:hidden [&::-webkit-details-marker]:hidden
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
text-xsm font-medium leading-4.25 text-muted-foreground hover:bg-elevated hover:text-foreground
aria-[current=page]:bg-elevated aria-[current=page]:text-foreground
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
