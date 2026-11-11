import { animated, LoadingMotion } from './motion'
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
  destructive: 'bg-error text-background shadow-action enabled:hover:brightness-110',
  link: 'bg-transparent p-0 text-focus shadow-none enabled:hover:brightness-110 !px-0',
}
const sizes = {
  compact: 'h-8',
  standard: 'h-10',
  authentication: 'h-12 w-88 text-[15px]',
  content: 'h-7',
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
    <animated.button
      {...props}
      type={type}
      disabled={disabled || loading}
      aria-busy={loading || undefined}
      className={cx(
        'inline-flex max-w-full shrink-0 cursor-pointer items-center justify-center gap-1 rounded-full',
        'border-[0.5px] border-transparent px-1.5 py-0 text-xsm leading-5 font-medium no-underline',
        'focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus',
        'disabled:cursor-not-allowed disabled:opacity-50 disabled:shadow-none disabled:border-transparent [&_svg]:shrink-0',
        sizes[size],
        variants[variant],
        size === 'compact' && variant === 'neutral' && 'h-6.5',
        className,
      )}
    >
      {loading ? (
        <LoadingMotion>
          <CircleNotchIcon size={14} aria-hidden="true" />
        </LoadingMotion>
      ) : null}
      {children}
    </animated.button>
  )
}
export function ContentButton({ className, ...props }: ButtonProps) {
  return <Button size="content" {...props} className={cx('min-w-41.5', className)} />
}

export interface TextLinkProps extends AnchorHTMLAttributes<HTMLAnchorElement> {
  tone?: 'secondary' | 'muted'
  disabled?: boolean
}
export function TextLink({ tone = 'secondary', disabled = false, className, href, ...props }: TextLinkProps) {
  return (
    <animated.a
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
    <p {...props} className={cx('flex flex-wrap items-center gap-1 text-xsm text-muted', className)}>
      <span>{children}</span>
      {action}
    </p>
  )
}
export function ResendPrompt({ remaining = 0, onResend }: { remaining?: number; onResend?: () => void }) {
  return (
    <p className="flex flex-wrap items-center gap-1 text-[12px] text-muted">
      <span>Didn’t receive a code?</span>
      <Button variant="link" disabled={remaining > 0} onClick={onResend}>
        {remaining > 0 ? `Resend in ${remaining}s` : 'Resend code'}
      </Button>
    </p>
  )
}

export function IconButton({ className, type = 'button', ...props }: ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <animated.button
      {...props}
      type={type}
      className={cx(
        'inline-flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-full border border-border/60',
        'bg-inset text-muted hover:border-input-border hover:text-text focus-visible:outline-1',
        `
focus-visible:outline-offset-2 focus-visible:outline-muted disabled:cursor-not-allowed disabled:opacity-40
[&_svg]:size-3.5
`,
        className,
      )}
    />
  )
}
