import { useId, type HTMLAttributes, type ReactNode } from 'react'
import { cx } from './shared'

export function HelperText({ className, ...props }: HTMLAttributes<HTMLParagraphElement>) { return <p {...props} className={cx('base-helper', className)} /> }
export function InlineError({ className, ...props }: HTMLAttributes<HTMLParagraphElement>) { return <p role="alert" {...props} className={cx('base-error', className)} /> }
export function CodeError(props: HTMLAttributes<HTMLParagraphElement>) { return <InlineError {...props}>{props.children ?? 'Incorrect code. Try again.'}</InlineError> }
export function EmailRegistrationError(props: HTMLAttributes<HTMLParagraphElement>) { return <InlineError {...props}>{props.children ?? 'Email already registered. Sign in instead.'}</InlineError> }
export function KeyboardKey({ children }: { children: ReactNode }) { return <kbd className="base-key">{children}</kbd> }
export function Tooltip({ children, content }: { children: ReactNode; content: ReactNode }) {
  const id = useId()
  return <span className="base-tooltip-anchor" tabIndex={0} aria-describedby={id}>{children}<span id={id} role="tooltip" className="base-tooltip">{content}</span></span>
}
export function EmptyState({ title, children, action }: { title: ReactNode; children?: ReactNode; action?: ReactNode }) {
  return <div className="base-empty"><h3>{title}</h3><div className="base-helper">{children}</div>{action}</div>
}
export function Callout({ title, children, action, tone = 'neutral' }: { title: ReactNode; children?: ReactNode; action?: ReactNode; tone?: 'neutral' | 'success' | 'warning' | 'error' }) {
  return <aside className="base-callout" data-tone={tone}><strong>{title}</strong><div className="base-helper">{children}</div>{action}</aside>
}
export function Popover({ label, children }: { label: string; children: ReactNode }) {
  const id = useId()
  return <span className="base-popover-anchor"><button className="base-button base-button--neutral base-button--compact" type="button" popoverTarget={id}>{label}</button><div id={id} popover="auto" className="base-popover">{children}</div></span>
}
export function DropInsertionIndicator({ className, ...props }: HTMLAttributes<HTMLDivElement>) { return <div aria-hidden="true" {...props} className={cx('base-drop-indicator', className)} /> }
