import type { ButtonHTMLAttributes, HTMLAttributes, ReactNode } from 'react'
import { ArrowLeftIcon } from '@phosphor-icons/react/dist/csr/ArrowLeft'
import { CaretRightIcon } from '@phosphor-icons/react/dist/csr/CaretRight'
import { DotsSixVerticalIcon } from '@phosphor-icons/react/dist/csr/DotsSixVertical'
import { SparkleIcon } from '@phosphor-icons/react/dist/csr/Sparkle'
import { Button, type ButtonProps } from './buttons'
import { cx } from './shared'

export interface BreadcrumbItem { label: string; href?: string }
export function BreadcrumbTrail({ items, label = 'Breadcrumb' }: { items: BreadcrumbItem[]; label?: string }) {
  return <nav aria-label={label}><ol className="base-breadcrumbs">{items.map((item, index) => <li key={`${item.label}-${index}`}>
    {index > 0 ? <CaretRightIcon size={12} aria-hidden="true" /> : null}
    {item.href ? <a href={item.href} aria-current={index === items.length - 1 ? 'page' : undefined}>{item.label}</a> : <span aria-current={index === items.length - 1 ? 'page' : undefined}>{item.label}</span>}
  </li>)}</ol></nav>
}
export function BackBreadcrumb({ href, children }: { href: string; children: ReactNode }) { return <a className="base-back-breadcrumb" href={href}><ArrowLeftIcon size={14} aria-hidden="true" />{children}</a> }
export function ViewPill({ className, ...props }: ButtonProps) { return <Button {...props} className={cx('base-view-pill', className)} /> }
export function AgentDock({ children = 'Agent', ...props }: ButtonProps) { return <Button variant="ghost" {...props}><SparkleIcon size={14} aria-hidden="true" />{children}</Button> }
export function NavigationTab({ selected = false, className, ...props }: ButtonHTMLAttributes<HTMLButtonElement> & { selected?: boolean }) {
  return <button type="button" {...props} aria-pressed={selected} className={cx('base-navigation-tab', className)} />
}
export function ToolbarAction({ label, children, className, ...props }: Omit<ButtonHTMLAttributes<HTMLButtonElement>, 'aria-label'> & { label: string }) {
  return <button type="button" {...props} aria-label={label} title={label} className={cx('base-toolbar-action', className)}>{children}</button>
}
export function DragHandle({ label, className, ...props }: Omit<ButtonHTMLAttributes<HTMLButtonElement>, 'aria-label'> & { label: string }) {
  return <ToolbarAction {...props} label={label} className={cx('base-drag-handle', className)}><DotsSixVerticalIcon size={14} aria-hidden="true" /></ToolbarAction>
}
export function NavigationGroup({ className, ...props }: HTMLAttributes<HTMLElement>) { return <nav {...props} className={cx('base-navigation-group', className)} /> }
