import { cloneElement, useId, useState, type HTMLAttributes, type ReactElement, type ReactNode } from 'react'
import { cx } from './shared'
import { Button } from './buttons'
import { InfoIcon } from '@phosphor-icons/react/dist/csr/Info'
import { WarningCircleIcon } from '@phosphor-icons/react/dist/csr/WarningCircle'
import { WarningIcon } from '@phosphor-icons/react/dist/csr/Warning'
import { XIcon } from '@phosphor-icons/react/dist/csr/X'

export interface ToastProps extends Omit<HTMLAttributes<HTMLDivElement>, 'children'> {
  message: string
  tone?: 'info' | 'error' | 'warning'
  onDismiss: () => void
  dismissLabel?: string
}

const toastTones = {
  info: { Icon: InfoIcon, color: 'text-focus', label: 'Info' },
  error: { Icon: WarningCircleIcon, color: 'text-error', label: 'Error' },
  warning: { Icon: WarningIcon, color: 'text-warning', label: 'Warning' },
}

export function Toast({
  message,
  tone = 'info',
  onDismiss,
  dismissLabel = 'Dismiss notification',
  className,
  ...props
}: ToastProps) {
  const { Icon, color, label } = toastTones[tone]
  return (
    <div
      {...props}
      data-ui="toast"
      data-tone={tone}
      className={cx(
        'flex min-h-[72px] w-[360px] max-w-full items-center gap-2 rounded-[10px]',
        'border-[0.5px] border-border bg-elevated p-4 font-sans text-[13px] leading-5 text-text',
        className,
      )}
    >
      <Icon size={16} className={cx('shrink-0', color)} aria-hidden="true" />
      <p className="min-w-0 flex-1 wrap-anywhere" role={tone === 'error' ? 'alert' : 'status'} aria-atomic="true">
        <span className="sr-only">{label}: </span>
        {message}
      </p>
      <button
        type="button"
        onClick={onDismiss}
        aria-label={dismissLabel}
        className={cx(
          'inline-flex size-6 shrink-0 cursor-pointer items-center justify-center rounded text-muted',
          'hover:text-text focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus',
        )}
      >
        <XIcon size={12} aria-hidden="true" />
      </button>
    </div>
  )
}

export function HelperText({ className, ...props }: HTMLAttributes<HTMLParagraphElement>) {
  return <p {...props} className={cx('text-[12px] leading-[18px] text-muted', className)} />
}
export function InlineError({ className, ...props }: HTMLAttributes<HTMLParagraphElement>) {
  return <p role="alert" {...props} className={cx('text-[13px] leading-[20px] text-error', className)} />
}
export function CodeError(props: HTMLAttributes<HTMLParagraphElement>) {
  return <InlineError {...props}>{props.children ?? 'Incorrect code. Try again.'}</InlineError>
}
export function EmailRegistrationError(props: HTMLAttributes<HTMLParagraphElement>) {
  return <InlineError {...props}>{props.children ?? 'Email already registered. Sign in instead.'}</InlineError>
}
export function KeyboardKey({ children }: { children: ReactNode }) {
  return (
    <kbd
      className={`
  inline-flex h-[28px] min-w-[28px] items-center justify-center rounded-[6px] bg-elevated p-0.5
  font-sans text-[14px]
`}
    >
      {children}
    </kbd>
  )
}
export function Tooltip({
  children,
  content,
}: {
  children: ReactElement<{ 'aria-describedby'?: string }>
  content: ReactNode
}) {
  const id = useId()
  const [dismissed, setDismissed] = useState(false)
  return (
    <span
      className="group/tooltip relative inline-flex"
      onPointerEnter={() => setDismissed(false)}
      onFocus={() => setDismissed(false)}
      onKeyDown={(event) => {
        if (event.key === 'Escape') setDismissed(true)
      }}
    >
      {cloneElement(children, { 'aria-describedby': [children.props['aria-describedby'], id].filter(Boolean).join(' ') })}
      <span
        id={id}
        role="tooltip"
        hidden={dismissed}
        className={`
  invisible absolute bottom-[calc(100%+--spacing(1))] left-1/2 z-30 -translate-x-1/2
  rounded-[6px] bg-menu p-1 text-[12px] whitespace-nowrap text-text group-hover/tooltip:visible
  group-focus-within/tooltip:visible
`}
      >
        {content}
      </span>
    </span>
  )
}
export function EmptyState({ title, children, action }: { title: ReactNode; children?: ReactNode; action?: ReactNode }) {
  return (
    <div
      className={`
  flex min-h-[128px] flex-col items-center gap-1 bg-surface p-3 text-center [&>h3]:text-[16px]
  [&>h3]:font-medium
`}
    >
      <h3>{title}</h3>
      <div className="text-[12px] leading-[18px] text-muted">{children}</div>
      {action}
    </div>
  )
}
export function Callout({
  title,
  children,
  action,
  tone = 'neutral',
}: {
  title: ReactNode
  children?: ReactNode
  action?: ReactNode
  tone?: 'neutral' | 'success' | 'warning' | 'error'
}) {
  return (
    <aside
      className={`
  flex flex-col gap-1 rounded-[10px] bg-elevated p-2 data-[tone=success]:text-success
  data-[tone=warning]:text-warning data-[tone=error]:text-error [&>strong]:text-[15px]
  [&>strong]:font-medium
`}
      data-tone={tone}
    >
      <strong>{title}</strong>
      <div className="text-[12px] leading-[18px] text-muted">{children}</div>
      {action}
    </aside>
  )
}
export function Popover({ label, children }: { label: string; children: ReactNode }) {
  const id = useId()
  return (
    <span className="inline">
      <Button popoverTarget={id}>{label}</Button>
      <div
        id={id}
        popover="auto"
        className={`
  w-[360px] max-w-[calc(100vw-32px)] rounded-[12px] border-[0.5px] border-input-border bg-menu p-2
  text-text [&:popover-open]:absolute [&:popover-open]:m-1 [&:popover-open]:[position-area:bottom_span-right]
  [&:popover-open]:[position-try-fallbacks:flip-block,flip-inline] [&>*+*]:mt-1.5
`}
      >
        {children}
      </div>
    </span>
  )
}
export function DropInsertionIndicator({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return <div aria-hidden="true" {...props} className={cx('h-[2px] w-full rounded-[1px] bg-focus', className)} />
}
