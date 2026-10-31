import type { ButtonHTMLAttributes, HTMLAttributes, ReactNode } from 'react'
import { ArrowLeftIcon } from '@phosphor-icons/react/dist/csr/ArrowLeft'
import { CaretRightIcon } from '@phosphor-icons/react/dist/csr/CaretRight'
import { DotsSixVerticalIcon } from '@phosphor-icons/react/dist/csr/DotsSixVertical'
import { SparkleIcon } from '@phosphor-icons/react/dist/csr/Sparkle'
import { Button, type ButtonProps } from './buttons'
import { cx } from './shared'

export interface BreadcrumbItem {
  label: string
  href?: string
}
export function BreadcrumbTrail({ items, label = 'Breadcrumb' }: { items: BreadcrumbItem[]; label?: string }) {
  return (
    <nav aria-label={label}>
      <ol
        className={`
  flex list-none items-center gap-space-12 p-0 text-[13px] [&>li]:flex [&>li]:min-w-0 [&>li]:items-center
  [&>li]:gap-space-12 [&_a]:text-muted [&_a]:focus-visible:outline-2 [&_a]:focus-visible:outline-offset-2
  [&_a]:focus-visible:outline-focus
`}
      >
        {items.map((item, index) => (
          <li key={`${item.label}-${index}`}>
            {index > 0 ? <CaretRightIcon size={12} aria-hidden="true" /> : null}
            {item.href ? (
              <a href={item.href} aria-current={index === items.length - 1 ? 'page' : undefined}>
                {item.label}
              </a>
            ) : (
              <span aria-current={index === items.length - 1 ? 'page' : undefined}>{item.label}</span>
            )}
          </li>
        ))}
      </ol>
    </nav>
  )
}
export function BackBreadcrumb({ href, children }: { href: string; children: ReactNode }) {
  return (
    <a
      className={`
  inline-flex min-h-[28px] items-center gap-space-8 text-[13px] focus-visible:outline-2
  focus-visible:outline-offset-2 focus-visible:outline-focus
`}
      href={href}
    >
      <ArrowLeftIcon size={14} aria-hidden="true" />
      {children}
    </a>
  )
}
export function ViewPill({ className, ...props }: ButtonProps) {
  return <Button {...props} className={cx('h-[28px] bg-menu text-[12px] shadow-none', className)} />
}
export function AgentDock({ children = 'Agent', ...props }: ButtonProps) {
  return (
    <Button variant="ghost" {...props}>
      <SparkleIcon size={14} aria-hidden="true" />
      {children}
    </Button>
  )
}
export function NavigationTab({
  selected = false,
  className,
  ...props
}: ButtonHTMLAttributes<HTMLButtonElement> & { selected?: boolean }) {
  return (
    <button
      type="button"
      {...props}
      aria-pressed={selected}
      className={cx(
        `
  min-h-[28px] cursor-pointer rounded-full bg-transparent px-space-16 py-0 text-[12px] text-muted
  aria-pressed:bg-inset aria-pressed:text-text focus-visible:outline-2 focus-visible:outline-offset-2
  focus-visible:outline-focus
`,
        className,
      )}
    />
  )
}
export function ToolbarAction({
  label,
  children,
  className,
  ...props
}: Omit<ButtonHTMLAttributes<HTMLButtonElement>, 'aria-label'> & { label: string }) {
  return (
    <button
      type="button"
      {...props}
      aria-label={label}
      title={label}
      className={cx(
        `
  inline-flex size-[28px] flex-none cursor-pointer items-center justify-center rounded-full bg-transparent
  text-muted hover:bg-elevated hover:text-text focus-visible:outline-2 focus-visible:outline-offset-2
  focus-visible:outline-focus disabled:cursor-not-allowed disabled:opacity-50
`,
        className,
      )}
    >
      {children}
    </button>
  )
}
export function DragHandle({
  label,
  className,
  ...props
}: Omit<ButtonHTMLAttributes<HTMLButtonElement>, 'aria-label'> & { label: string }) {
  return (
    <ToolbarAction
      {...props}
      label={label}
      className={cx('w-[24px] cursor-grab rounded-[6px] active:cursor-grabbing', className)}
    >
      <DotsSixVerticalIcon size={14} aria-hidden="true" />
    </ToolbarAction>
  )
}
export function NavigationGroup({ className, ...props }: HTMLAttributes<HTMLElement>) {
  return <nav {...props} className={cx('flex items-center gap-space-8', className)} />
}
