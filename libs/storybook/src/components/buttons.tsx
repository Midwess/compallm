import type { AnchorHTMLAttributes, ButtonHTMLAttributes, HTMLAttributes, ReactNode } from 'react'
import { CircleNotchIcon } from '@phosphor-icons/react/dist/csr/CircleNotch'
import { cx } from './shared'

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'neutral' | 'ghost' | 'destructive' | 'link'
  size?: 'compact' | 'standard' | 'authentication' | 'content'
  loading?: boolean
}

const variants = {
  primary: 'bg-primary text-on-primary shadow-action enabled:hover:bg-primary-hover',
  neutral: 'bg-elevated text-text shadow-control enabled:hover:brightness-110',
  ghost: 'bg-transparent text-text shadow-none enabled:hover:brightness-110',
  destructive: 'bg-elevated text-error shadow-control enabled:hover:brightness-110',
  link: 'bg-transparent p-0 text-focus shadow-none enabled:hover:brightness-110',
}
const sizes = {
  compact: 'h-[32px]',
  standard: 'h-[40px]',
  authentication: 'h-[48px] w-[352px] text-[15px]',
  content: 'h-[28px]',
}

export function Button({
  variant = 'neutral',
  size = 'compact',
  loading = false,
  disabled,
  type = 'button',
  children,
  className,
  ...props
}: ButtonProps) {
  return (
    <button
      {...props}
      type={type}
      disabled={disabled || loading}
      aria-busy={loading || undefined}
      className={cx(
        'inline-flex max-w-full shrink-0 cursor-pointer items-center justify-center gap-space-8 rounded-full',
        'border-[0.5px] border-transparent px-space-12 py-0 text-[13px] leading-[20px] font-medium no-underline',
        'focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus',
        'disabled:cursor-not-allowed disabled:opacity-50 [&_svg]:shrink-0',
        sizes[size],
        variants[variant],
        size === 'compact' && variant === 'neutral' && 'h-[26px]',
        className,
      )}
    >
      {loading ? <CircleNotchIcon className="animate-spin motion-reduce:animate-none" size={14} aria-hidden="true" /> : null}
      {children}
    </button>
  )
}
export function ContentButton({ className, ...props }: ButtonProps) {
  return <Button size="content" {...props} className={cx('min-w-[166px]', className)} />
}

export interface TextLinkProps extends AnchorHTMLAttributes<HTMLAnchorElement> {
  tone?: 'secondary' | 'muted'
  disabled?: boolean
}
export function TextLink({ tone = 'secondary', disabled = false, className, href, ...props }: TextLinkProps) {
  return (
    <a
      {...props}
      href={disabled ? undefined : href}
      aria-disabled={disabled || undefined}
      tabIndex={disabled ? -1 : props.tabIndex}
      className={cx(
        'text-[14px] underline-offset-[3px] hover:underline focus-visible:outline-2',
        'focus-visible:outline-offset-2 focus-visible:outline-focus',
        'aria-disabled:cursor-not-allowed aria-disabled:text-disabled',
        tone === 'muted' ? 'text-muted' : 'text-focus',
        className,
      )}
      onClick={(event) => {
        if (disabled) event.preventDefault()
        else props.onClick?.(event)
      }}
    />
  )
}
export function AccountPrompt({
  children,
  action,
  className,
  ...props
}: HTMLAttributes<HTMLParagraphElement> & { action: ReactNode }) {
  return (
    <p {...props} className={cx('flex flex-wrap items-center gap-space-8 text-[13px] text-muted', className)}>
      <span>{children}</span>
      {action}
    </p>
  )
}
export function ResendPrompt({ remaining = 0, onResend }: { remaining?: number; onResend?: () => void }) {
  return (
    <p className="flex flex-wrap items-center gap-space-8 text-[12px] text-muted">
      <span>Didn’t receive a code?</span>
      <Button variant="link" disabled={remaining > 0} onClick={onResend}>
        {remaining > 0 ? `Resend in ${remaining}s` : 'Resend code'}
      </Button>
    </p>
  )
}
