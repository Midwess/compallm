import type { HTMLAttributes, ReactNode } from 'react'
import { FolderIcon } from '@phosphor-icons/react/dist/csr/Folder'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import { CircleIcon } from '@phosphor-icons/react/dist/csr/Circle'
import { Button, type ButtonProps } from './buttons'
import { Avatar, Badge, type BadgeTone } from './content'
import { cx } from './shared'

export function ProjectTableHeader() {
  return (
    <thead>
      <tr>
        <th scope="col">Name</th>
        <th scope="col">Health</th>
        <th scope="col">Priority</th>
        <th scope="col">Lead</th>
        <th scope="col">Progress</th>
      </tr>
    </thead>
  )
}
export function ProjectTableRow({
  name,
  health = 'No updates',
  priority = 'No priority',
  lead,
  progress = 0,
}: {
  name: string
  health?: string
  priority?: string
  lead?: string
  progress?: number
}) {
  return (
    <tr>
      <td>
        <span className="flex items-center gap-space-12 font-medium">
          <ProjectIcon />
          {name}
        </span>
      </td>
      <td>
        <ProjectHealth>{health}</ProjectHealth>
      </td>
      <td>{priority}</td>
      <td>{lead ? <Avatar name={lead} size={24} /> : 'Unassigned'}</td>
      <td>
        <ProjectProgress value={progress} />
      </td>
    </tr>
  )
}
export function ProjectTable({ children, label }: { children: ReactNode; label: string }) {
  return (
    <div className="w-full overflow-x-auto" tabIndex={0} role="region" aria-label={label}>
      <table
        className={`
  w-full min-w-[640px] border-collapse text-left text-[13px] [&_th]:h-[32px] [&_th]:text-[12px]
  [&_th]:font-normal [&_th]:text-muted [&_td]:h-[48px] [&_th]:border-b-[0.5px] [&_th]:border-divider
  [&_th]:px-space-12 [&_th]:py-space-8 [&_td]:border-b-[0.5px] [&_td]:border-divider [&_td]:px-space-12
  [&_td]:py-space-8
`}
      >
        {children}
      </table>
    </div>
  )
}
export function ProjectIcon() {
  return (
    <span
      className={`
  inline-flex size-[36px] items-center justify-center rounded-[6px] bg-warning text-background
`}
    >
      <FolderIcon size={18} aria-hidden="true" />
    </span>
  )
}
export function ProjectHealth({ children, tone = 'neutral' }: { children: ReactNode; tone?: BadgeTone }) {
  return (
    <Badge tone={tone}>
      <CircleIcon size={12} aria-hidden="true" />
      {children}
    </Badge>
  )
}
export function ProjectProgress({ value }: { value: number }) {
  const percent = Number.isFinite(value) ? Math.min(100, Math.max(0, value)) : 0
  return (
    <span
      className={`
  inline-flex items-center gap-space-8 text-[12px] text-muted [&>progress]:h-[4px] [&>progress]:w-[32px]
  [&>progress]:accent-primary
`}
    >
      <progress max={100} value={percent} aria-label="Project completion" />
      {percent}%
    </span>
  )
}
export function ResourceAction({ children = 'Add document or link…', ...props }: ButtonProps) {
  return (
    <Button variant="ghost" size="content" {...props}>
      <PlusIcon size={14} aria-hidden="true" />
      {children}
    </Button>
  )
}
export function MilestoneAddAction({ children = 'Milestone', ...props }: ButtonProps) {
  return <ResourceAction {...props}>{children}</ResourceAction>
}
export function ActivityEvent({
  actor,
  children,
  dateTime,
  timeLabel,
}: {
  actor: string
  children: ReactNode
  dateTime: string
  timeLabel: string
}) {
  return (
    <p
      className={`
  flex flex-wrap items-center gap-space-4 text-[12px] text-muted [&>strong]:font-medium
  [&>strong]:text-text
`}
    >
      <strong>{actor}</strong>
      <span>{children}</span>
      <span aria-hidden="true">·</span>
      <time dateTime={dateTime}>{timeLabel}</time>
    </p>
  )
}
export function PropertyChip({ icon, children, className, ...props }: ButtonProps & { icon?: ReactNode }) {
  return (
    <Button variant="ghost" size="content" {...props} className={cx('text-[13px]', className)}>
      {icon}
      {children}
    </Button>
  )
}
export function PropertyRow({ label, children, className, ...props }: HTMLAttributes<HTMLDivElement> & { label: ReactNode }) {
  return (
    <div {...props} className={cx('grid min-h-[28px] grid-cols-[minmax(80px,1fr)_2fr] items-center gap-space-12', className)}>
      <span className="text-[12px] leading-[18px] text-muted">{label}</span>
      <div>{children}</div>
    </div>
  )
}
