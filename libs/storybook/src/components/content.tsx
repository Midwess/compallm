import type { CSSProperties, HTMLAttributes, ReactNode } from 'react'
import { FileTextIcon } from '@phosphor-icons/react/dist/csr/FileText'
import { cx } from './shared'

export function SectionHeading({
  children,
  description,
  action,
  className,
  ...props
}: {
  description?: ReactNode
  action?: ReactNode
} & HTMLAttributes<HTMLDivElement>) {
  return (
    <div {...props} className={cx('flex min-w-0 items-center justify-between gap-1.5', className)}>
      <div className="min-w-0">
        <h2 className="text-[15px] leading-[23px] font-medium">{children}</h2>
        {description ? <p className="mt-0.5 text-[13px] leading-5 text-muted">{description}</p> : null}
      </div>
      {action}
    </div>
  )
}
export function SettingsRow({
  label,
  description,
  children,
  className,
  ...props
}: {
  label: ReactNode
  description?: ReactNode
  children: ReactNode
} & HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      {...props}
      className={cx(
        `
  flex min-h-16.5 items-center gap-1.5 bg-inset p-2 max-[480px]:flex-wrap
  [&>div:first-child]:min-w-0 [&>div:first-child]:flex-1 [&_strong]:text-[13px] [&_strong]:font-medium
`,
        className,
      )}
    >
      <div>
        <strong className="block leading-4">{label}</strong>
        {description ? <p className="mt-0.5 text-[12px] leading-[15px] text-muted">{description}</p> : null}
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
  inline-flex min-h-6 items-center justify-center gap-1 rounded-[6px] bg-elevated px-1
  py-0 text-[12px] font-medium data-[tone=success]:text-success data-[tone=warning]:text-warning
  data-[tone=error]:text-error
`,
        className,
      )}
      data-tone={tone}
    />
  )
}
export { CodePreview } from './code-block'
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
  flex min-h-5 gap-1 overflow-x-auto text-[12px] whitespace-pre
  data-[change=added]:bg-success/12 data-[change=removed]:bg-error/12
`}
      data-change={change}
    >
      <span className="min-w-8 text-right text-muted" aria-label={lineNumber ? `Line ${lineNumber}` : undefined}>
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
  flex min-h-16 flex-wrap items-center gap-1.5 bg-inset p-2 max-[480px]:gap-1
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
  inline-flex items-center gap-1 rounded-[6px] bg-inset px-1 py-0.5 text-[11px]
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
