import type { HTMLAttributes, ReactNode } from 'react'
import { FolderIcon } from '@phosphor-icons/react/dist/csr/Folder'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import { CircleIcon } from '@phosphor-icons/react/dist/csr/Circle'
import { Button, type ButtonProps } from './buttons'
import { Avatar, Badge, type BadgeTone } from './content'
import { cx } from './shared'

export function ProjectTableHeader() { return <thead><tr><th scope="col">Name</th><th scope="col">Health</th><th scope="col">Priority</th><th scope="col">Lead</th><th scope="col">Progress</th></tr></thead> }
export function ProjectTableRow({ name, health = 'No updates', priority = 'No priority', lead, progress = 0 }: { name: string; health?: string; priority?: string; lead?: string; progress?: number }) {
  return <tr><td><span className="base-project-name"><ProjectIcon />{name}</span></td><td><ProjectHealth>{health}</ProjectHealth></td><td>{priority}</td><td>{lead ? <Avatar name={lead} size={24} /> : 'Unassigned'}</td><td><ProjectProgress value={progress} /></td></tr>
}
export function ProjectTable({ children, label }: { children: ReactNode; label: string }) { return <div className="base-table-scroll" tabIndex={0} role="region" aria-label={label}><table className="base-project-table">{children}</table></div> }
export function ProjectIcon() { return <span className="base-project-icon"><FolderIcon size={18} aria-hidden="true" /></span> }
export function ProjectHealth({ children, tone = 'neutral' }: { children: ReactNode; tone?: BadgeTone }) { return <Badge tone={tone}><CircleIcon size={12} aria-hidden="true" />{children}</Badge> }
export function ProjectProgress({ value }: { value: number }) {
  const percent = Number.isFinite(value) ? Math.min(100, Math.max(0, value)) : 0
  return <span className="base-project-progress"><progress max={100} value={percent} aria-label="Project completion" />{percent}%</span>
}
export function ResourceAction({ children = 'Add document or link…', ...props }: ButtonProps) { return <Button variant="ghost" size="content" {...props}><PlusIcon size={14} aria-hidden="true" />{children}</Button> }
export function MilestoneAddAction({ children = 'Milestone', ...props }: ButtonProps) { return <ResourceAction {...props}>{children}</ResourceAction> }
export function ActivityEvent({ actor, children, dateTime, timeLabel }: { actor: string; children: ReactNode; dateTime: string; timeLabel: string }) { return <p className="base-activity-event"><strong>{actor}</strong><span>{children}</span><span aria-hidden="true">·</span><time dateTime={dateTime}>{timeLabel}</time></p> }
export function PropertyChip({ icon, children, className, ...props }: ButtonProps & { icon?: ReactNode }) { return <Button variant="ghost" size="content" {...props} className={cx('base-property-chip', className)}>{icon}{children}</Button> }
export function PropertyRow({ label, children, className, ...props }: HTMLAttributes<HTMLDivElement> & { label: ReactNode }) { return <div {...props} className={cx('base-property-row', className)}><span className="base-helper">{label}</span><div>{children}</div></div> }
