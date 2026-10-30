import type { HTMLAttributes, ReactNode } from 'react'
import { FileTextIcon } from '@phosphor-icons/react/dist/csr/FileText'
import { cx } from './shared'

export function SectionHeading({ children, description, action }: { children: ReactNode; description?: ReactNode; action?: ReactNode }) {
  return <div className="base-section-heading"><div><h2>{children}</h2>{description ? <p className="base-muted">{description}</p> : null}</div>{action}</div>
}
export function SettingsRow({ label, description, children }: { label: ReactNode; description?: ReactNode; children: ReactNode }) {
  return <div className="base-settings-row"><div><strong>{label}</strong>{description ? <p className="base-helper">{description}</p> : null}</div>{children}</div>
}
export function Avatar({ name, src, size = 40 }: { name: string; src?: string; size?: number }) {
  const initials = name.trim().split(/\s+/).slice(0, 2).map(word => word[0]).join('').toUpperCase()
  return <span className="base-avatar" style={{ width: size, height: size }} role={src ? undefined : 'img'} aria-label={src ? undefined : name}>{src ? <img src={src} alt={name} /> : initials}</span>
}
export type BadgeTone = 'neutral' | 'success' | 'warning' | 'error'
export function Badge({ tone = 'neutral', className, ...props }: HTMLAttributes<HTMLSpanElement> & { tone?: BadgeTone }) { return <span {...props} className={cx('base-badge', className)} data-tone={tone} /> }
export function CodePreview({ code, language }: { code: string; language?: string }) { return <figure className="base-code-preview">{language ? <figcaption>{language}</figcaption> : null}<pre><code>{code}</code></pre></figure> }
export function DiffLine({ change = 'context', lineNumber, children }: { change?: 'context' | 'added' | 'removed'; lineNumber?: number; children: ReactNode }) {
  return <div className="base-diff-line" data-change={change}><span className="base-diff-number" aria-label={lineNumber ? `Line ${lineNumber}` : undefined}>{lineNumber}</span><span aria-label={change === 'context' ? undefined : change}>{change === 'added' ? '+' : change === 'removed' ? '−' : ' '}</span><code>{children}</code></div>
}
export function MemberRow({ name, email, role, children }: { name: string; email?: string; role: ReactNode; children?: ReactNode }) {
  return <div className="base-member-row"><Avatar name={name} /><div className="base-member-name"><strong>{name}</strong>{email ? <p className="base-helper">{email}</p> : null}</div><Badge>{role}</Badge>{children}</div>
}
export function EditTimestamp({ dateTime, children }: { dateTime: string; children: ReactNode }) { return <span className="base-helper">Last edited <time dateTime={dateTime}>{children}</time></span> }
export function ChangedFile({ path, added = 0, removed = 0 }: { path: string; added?: number; removed?: number }) { return <span className="base-changed-file"><FileTextIcon size={14} aria-hidden="true" /><code>{path}</code><span data-tone="success">+{added}</span><span data-tone="error">−{removed}</span></span> }
export function DocumentIcon({ size = 24 }: { size?: number }) { return <FileTextIcon size={size} aria-hidden="true" /> }
