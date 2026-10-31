import type { CSSProperties, HTMLAttributes, ReactNode } from 'react'
import { FileTextIcon } from '@phosphor-icons/react/dist/csr/FileText'
import { cx } from './shared'

export function SectionHeading({
  children,
  description,
  action,
}: {
  children: ReactNode
  description?: ReactNode
  action?: ReactNode
}) {
  return (
    <div
      className={`
  flex items-center justify-between gap-space-12 [&_h2]:text-[15px] [&_h2]:leading-[24px]
  [&_h2]:font-medium [&_p]:text-[13px]
`}
    >
      <div>
        <h2>{children}</h2>
        {description ? <p className="text-muted">{description}</p> : null}
      </div>
      {action}
    </div>
  )
}
export function SettingsRow({
  label,
  description,
  children,
}: {
  label: ReactNode
  description?: ReactNode
  children: ReactNode
}) {
  return (
    <div
      className={`
  flex min-h-[65px] items-center gap-space-12 bg-inset p-space-16 max-[480px]:flex-wrap
  [&>div:first-child]:min-w-0 [&>div:first-child]:flex-1 [&_strong]:text-[13px] [&_strong]:font-medium
`}
    >
      <div>
        <strong>{label}</strong>
        {description ? <p className="text-[12px] leading-[18px] text-muted">{description}</p> : null}
      </div>
      {children}
    </div>
  )
}
export function Avatar({ name, src, size = 40 }: { name: string; src?: string; size?: number }) {
  const initials = name
    .trim()
    .split(/\s+/)
    .slice(0, 2)
    .map((word) => word[0])
    .join('')
    .toUpperCase()
  return (
    <span
      className={`
  inline-flex flex-none items-center justify-center overflow-hidden rounded-full bg-elevated text-[13px]
  size-(--avatar-size) font-medium [&>img]:size-full [&>img]:object-cover
`}
      style={{ '--avatar-size': `${size}px` } as CSSProperties}
      role={src ? undefined : 'img'}
      aria-label={src ? undefined : name}
    >
      {src ? <img src={src} alt={name} /> : initials}
    </span>
  )
}
export type BadgeTone = 'neutral' | 'success' | 'warning' | 'error'
export function Badge({ tone = 'neutral', className, ...props }: HTMLAttributes<HTMLSpanElement> & { tone?: BadgeTone }) {
  return (
    <span
      {...props}
      className={cx(
        `
  inline-flex min-h-[24px] items-center justify-center gap-space-8 rounded-[6px] bg-elevated px-space-8
  py-0 text-[12px] font-medium data-[tone=success]:text-success data-[tone=warning]:text-warning
  data-[tone=error]:text-error
`,
        className,
      )}
      data-tone={tone}
    />
  )
}
export function CodePreview({ code, language }: { code: string; language?: string }) {
  return (
    <figure
      className={`
  overflow-hidden rounded-[6px] bg-elevated [&>figcaption]:border-b-[0.5px] [&>figcaption]:border-divider
  [&>figcaption]:p-space-12 [&>figcaption]:text-[12px] [&>figcaption]:text-muted [&>pre]:m-0
  [&>pre]:overflow-x-auto [&>pre]:p-space-16 [&>pre]:text-[12px] [&>pre]:leading-[20px]
`}
    >
      {language ? <figcaption>{language}</figcaption> : null}
      <pre>
        <code>{code}</code>
      </pre>
    </figure>
  )
}
export function DiffLine({
  change = 'context',
  lineNumber,
  children,
}: {
  change?: 'context' | 'added' | 'removed'
  lineNumber?: number
  children: ReactNode
}) {
  return (
    <div
      className={`
  flex min-h-[20px] gap-space-8 overflow-x-auto text-[12px] whitespace-pre
  data-[change=added]:bg-success/12 data-[change=removed]:bg-error/12
`}
      data-change={change}
    >
      <span className="min-w-[32px] text-right text-muted" aria-label={lineNumber ? `Line ${lineNumber}` : undefined}>
        {lineNumber}
      </span>
      <span aria-label={change === 'context' ? undefined : change}>
        {change === 'added' ? '+' : change === 'removed' ? '−' : ' '}
      </span>
      <code>{children}</code>
    </div>
  )
}
export function MemberRow({
  name,
  email,
  role,
  children,
}: {
  name: string
  email?: string
  role: ReactNode
  children?: ReactNode
}) {
  return (
    <div
      className={`
  flex min-h-[64px] flex-wrap items-center gap-space-12 bg-inset p-space-16 max-[480px]:gap-space-8
  [&_strong]:text-[13px] [&_strong]:font-medium
`}
    >
      <Avatar name={name} />
      <div className="min-w-0 flex-1">
        <strong>{name}</strong>
        {email ? <p className="text-[12px] leading-[18px] text-muted">{email}</p> : null}
      </div>
      <Badge>{role}</Badge>
      {children}
    </div>
  )
}
export function EditTimestamp({ dateTime, children }: { dateTime: string; children: ReactNode }) {
  return (
    <span className="text-[12px] leading-[18px] text-muted">
      Last edited <time dateTime={dateTime}>{children}</time>
    </span>
  )
}
export function ChangedFile({ path, added = 0, removed = 0 }: { path: string; added?: number; removed?: number }) {
  return (
    <span
      className={`
  inline-flex items-center gap-space-8 rounded-[6px] bg-inset px-space-8 py-space-4 text-[11px]
  [&>code]:wrap-anywhere [&>[data-tone=success]]:text-success [&>[data-tone=error]]:text-error
`}
    >
      <FileTextIcon size={14} aria-hidden="true" />
      <code>{path}</code>
      <span data-tone="success">+{added}</span>
      <span data-tone="error">−{removed}</span>
    </span>
  )
}
export function DocumentIcon({ size = 24 }: { size?: number }) {
  return <FileTextIcon size={size} aria-hidden="true" />
}
