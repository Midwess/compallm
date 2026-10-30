import type { AnchorHTMLAttributes, ButtonHTMLAttributes, ReactNode } from 'react'
import { CircleNotchIcon } from '@phosphor-icons/react/dist/csr/CircleNotch'
import { cx } from './shared'

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'neutral' | 'ghost' | 'destructive' | 'link'
  size?: 'compact' | 'standard' | 'authentication' | 'content'
  loading?: boolean
}

export function Button({ variant = 'neutral', size = 'compact', loading = false, disabled, type = 'button', children, className, ...props }: ButtonProps) {
  return <button {...props} type={type} disabled={disabled || loading} aria-busy={loading || undefined}
    className={cx('base-button', `base-button--${variant}`, `base-button--${size}`, className)}>
    {loading ? <CircleNotchIcon className="base-spinner" size={14} aria-hidden="true" /> : null}{children}
  </button>
}
export function ContentButton({ className, ...props }: ButtonProps) {
  return <Button size="content" {...props} className={cx('base-content-button', className)} />
}

export interface TextLinkProps extends AnchorHTMLAttributes<HTMLAnchorElement> { tone?: 'secondary' | 'muted'; disabled?: boolean }
export function TextLink({ tone = 'secondary', disabled = false, className, href, ...props }: TextLinkProps) {
  return <a {...props} href={disabled ? undefined : href} aria-disabled={disabled || undefined} tabIndex={disabled ? -1 : props.tabIndex}
    className={cx('base-link', `base-link--${tone}`, className)} onClick={event => {
      if (disabled) event.preventDefault()
      else props.onClick?.(event)
    }} />
}
export function AccountPrompt({ children, action }: { children: ReactNode; action: ReactNode }) {
  return <p className="base-account-prompt"><span>{children}</span>{action}</p>
}
export function ResendPrompt({ remaining = 0, onResend }: { remaining?: number; onResend?: () => void }) {
  return <p className="base-resend-prompt"><span>Didn’t receive a code?</span><Button variant="link" disabled={remaining > 0} onClick={onResend}>{remaining > 0 ? `Resend in ${remaining}s` : 'Resend code'}</Button></p>
}
