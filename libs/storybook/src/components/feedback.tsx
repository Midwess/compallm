import { cloneElement, useEffect, useId, useRef, useState, type HTMLAttributes, type ReactElement, type ReactNode } from 'react'
import { cx, popupMotion } from './shared'
import { RichMessage } from './rich-message'
import { Button } from './buttons'
import { QuestionIcon } from '@phosphor-icons/react/dist/csr/Question'
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
        <RichMessage message={message} />
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
export function FieldHelp({ label, children }: { label: string; children: ReactNode }) {
  const id = useId()
  const trigger = useRef<HTMLButtonElement>(null)
  const tooltip = useRef<HTMLSpanElement>(null)
  useEffect(() => {
    const dismiss = (event: KeyboardEvent) => {
      if (event.key !== 'Escape' || !tooltip.current?.matches(':popover-open')) return
      event.preventDefault()
      event.stopPropagation()
      tooltip.current.hidePopover()
    }
    document.addEventListener('keydown', dismiss, true)
    return () => document.removeEventListener('keydown', dismiss, true)
  }, [])
  function show() {
    const element = tooltip.current
    const button = trigger.current
    if (!element || !button || element.matches(':popover-open')) return
    element.showPopover()
    const bounds = button.getBoundingClientRect()
    const width = element.offsetWidth
    const height = element.offsetHeight
    element.style.left = `${Math.max(8, Math.min(bounds.left, window.innerWidth - width - 8))}px`
    const top = bounds.bottom + height <= window.innerHeight - 8 ? bounds.bottom : Math.max(8, bounds.top - height)
    element.style.top = `${top}px`
  }
  function hide() {
    tooltip.current?.hidePopover()
  }
  return (
    <span
      className="inline-flex align-middle"
      onPointerEnter={show}
      onPointerLeave={() => {
        if (document.activeElement !== trigger.current) hide()
      }}
      onBlur={hide}
    >
      <button
        ref={trigger}
        type="button"
        aria-label={label}
        aria-describedby={id}
        className={`
inline-flex size-5 shrink-0 cursor-help items-center justify-center rounded text-muted hover:text-text
focus-visible:text-text focus-visible:outline-1 focus-visible:outline-muted
`}
        onFocus={show}
        onClick={show}
      >
        <QuestionIcon className="size-4" aria-hidden="true" />
      </button>
      <span
        ref={tooltip}
        id={id}
        role="tooltip"
        popover="manual"
        className={`${popupMotion}
fixed m-0 w-72 max-w-[calc(100vw-1rem)] overflow-visible border-0 bg-transparent pt-1 text-left
text-xs leading-4 font-normal text-text
`}
      >
        <span className="block rounded-lg border border-border bg-elevated p-3">{children}</span>
      </span>
    </span>
  )
}
export function EmptyState({ title, children, action }: { title: ReactNode; children?: ReactNode; action?: ReactNode }) {
  return (
    <div
      className={`
  flex min-h-32 flex-col items-center gap-1 bg-surface p-3 text-center [&>h3]:text-[16px]
  [&>h3]:font-medium
`}
    >
      <h3>{title}</h3>
      <div className="text-[13px] leading-5 text-muted">{children}</div>
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
      <div className="text-[12px] leading-4.5 text-muted">{children}</div>
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
        className={`${popupMotion}
  w-12 max-w-[calc(100vw-32px)] rounded-xl border-[0.5px] border-input-border bg-menu p-2
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
  return <div aria-hidden="true" {...props} className={cx('h-0.5 w-full rounded-[1px] bg-focus', className)} />
}

export function FeedbackSlot({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return <div {...props} className={cx('h-fit py-1 overflow-y-auto', className)} />
}
