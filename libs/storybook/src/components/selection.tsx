import { animated, useSelectionMotion } from './motion'
import { useEffect, useRef, type InputHTMLAttributes, type ReactNode } from 'react'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { MinusIcon } from '@phosphor-icons/react/dist/csr/Minus'
import { cx } from './shared'

// The 3px inset centers the thumb and standard checkbox track.
// eslint-disable-next-line design/spacing -- Preserve the current switch alignment.
const switchIndicatorPosition = 'top-2.25 left-2.25'
// eslint-disable-next-line design/spacing -- Preserve the standard checkbox track size.
const checkboxTrackInset = 'inset-0.75'

export function Switch({ className, ...props }: Omit<InputHTMLAttributes<HTMLInputElement>, 'type' | 'role'>) {
  const input = useRef<HTMLInputElement>(null)
  useSelectionMotion(input, 'switch')
  return (
    <span className="relative inline-flex h-8 w-10.5 flex-none has-disabled:opacity-50">
      <animated.input
        {...props}
        ref={input}
        type="checkbox"
        role="switch"
        className={cx(
          'peer absolute inset-0 size-full cursor-pointer appearance-none rounded-lg',
          'focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus disabled:cursor-not-allowed',
          className,
        )}
      />
      <span
        data-selection-track
        aria-hidden="true"
        className="pointer-events-none absolute top-1.5 left-1.5 h-5 w-7.5 rounded-full
border-[0.5px] border-input-border bg-disabled"
      />
      <span
        data-selection-indicator
        aria-hidden="true"
        className={cx('pointer-events-none absolute size-3.5 rounded-full bg-on-primary', switchIndicatorPosition)}
      />
    </span>
  )
}
export interface CheckboxProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'type' | 'size'> {
  indeterminate?: boolean
  size?: 'standard' | 'compact'
}
export function Checkbox({ indeterminate = false, size = 'standard', className, ...props }: CheckboxProps) {
  const input = useRef<HTMLInputElement>(null)
  useSelectionMotion(input, 'checkbox', indeterminate)
  useEffect(() => {
    if (input.current) input.current.indeterminate = indeterminate
  }, [indeterminate])
  return (
    <span
      className={cx(
        'relative inline-flex shrink-0 items-center justify-center has-disabled:cursor-not-allowed has-disabled:opacity-40',
        size === 'compact' ? 'size-5' : 'size-6',
      )}
    >
      <animated.input
        {...props}
        ref={input}
        type="checkbox"
        className={cx(
          `peer absolute inset-0 size-full cursor-pointer appearance-none rounded bg-transparent
    focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus disabled:cursor-not-allowed`,
          className,
        )}
      />
      <span
        data-selection-track
        aria-hidden="true"
        className={cx(
          'pointer-events-none absolute rounded-sm border border-input-border',
          size === 'compact' ? 'inset-1' : checkboxTrackInset,
        )}
      />
      <span data-selection-indicator aria-hidden="true" className="pointer-events-none relative inline-flex">
        {indeterminate ? (
          <MinusIcon
            aria-hidden="true"
            className={cx('pointer-events-none relative text-on-primary', size === 'compact' ? 'size-2.5' : 'size-3.5')}
          />
        ) : (
          <CheckIcon
            aria-hidden="true"
            className={cx('pointer-events-none relative text-on-primary', size === 'compact' ? 'size-2.5' : 'size-3.5')}
          />
        )}
      </span>
    </span>
  )
}

export function Radio({ className, ...props }: Omit<InputHTMLAttributes<HTMLInputElement>, 'type'>) {
  const input = useRef<HTMLInputElement>(null)
  useSelectionMotion(input, 'radio')
  return (
    <span className="relative inline-flex size-4 shrink-0 items-center justify-center has-disabled:opacity-50">
      <animated.input
        {...props}
        ref={input}
        type="radio"
        className={cx(
          'absolute inset-0 size-full cursor-pointer appearance-none rounded-full',
          'focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus disabled:cursor-not-allowed',
          className,
        )}
      />
      <span
        data-selection-track
        aria-hidden="true"
        className="pointer-events-none absolute inset-0 rounded-full border border-input-border"
      />
      <span
        data-selection-indicator
        aria-hidden="true"
        className="pointer-events-none relative size-1.5 rounded-full bg-on-primary"
      />
    </span>
  )
}
export function ChoiceCard({
  children,
  description,
  className,
  ...props
}: Omit<InputHTMLAttributes<HTMLInputElement>, 'type' | 'title'> & { children: ReactNode; description?: ReactNode }) {
  return (
    <animated.label
      className={cx(
        `
  flex min-h-15.5 w-75 max-w-full cursor-pointer items-center gap-1.5 rounded-[8px] border
  border-border bg-inset px-1.5 py-1.5 has-checked:border-primary
  has-checked:bg-[color-mix(in_srgb,var(--color-primary)_14%,var(--color-inset))]
  has-disabled:cursor-not-allowed has-disabled:opacity-50 [&_strong]:text-[13px] [&_strong]:font-medium
  [&>span>span]:block
`,
        className,
      )}
    >
      <Radio {...props} />
      <span>
        <strong>{children}</strong>
        {description ? <span className="text-[12px] leading-[18px] text-muted">{description}</span> : null}
      </span>
    </animated.label>
  )
}
export function CheckboxOption({ children, className, ...props }: CheckboxProps & { children: ReactNode }) {
  return (
    <animated.label className={cx('inline-flex min-h-6 cursor-pointer items-center gap-0.5 text-[13px]', className)}>
      <Checkbox {...props} />
      <span>{children}</span>
    </animated.label>
  )
}
