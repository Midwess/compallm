import { useEffect, useRef, type InputHTMLAttributes, type ReactNode } from 'react'
import { cx } from './shared'

export function Switch({ className, ...props }: Omit<InputHTMLAttributes<HTMLInputElement>, 'type' | 'role'>) {
  return (
    <input
      {...props}
      type="checkbox"
      role="switch"
      className={cx(
        `
  relative h-[20px] w-[30px] flex-none cursor-pointer appearance-none rounded-full border-[0.5px]
  border-input-border bg-elevated after:absolute after:top-[2px] after:left-[2px] after:size-[14px]
  after:rounded-full after:bg-on-primary after:content-[''] checked:border-primary checked:bg-primary
  checked:after:left-[12px] focus-visible:outline-2 focus-visible:outline-offset-2
  focus-visible:outline-focus disabled:cursor-not-allowed disabled:opacity-50
`,
        className,
      )}
    />
  )
}
export interface CheckboxProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'type'> {
  indeterminate?: boolean
}
export function Checkbox({ indeterminate = false, className, ...props }: CheckboxProps) {
  const input = useRef<HTMLInputElement>(null)
  useEffect(() => {
    if (input.current) input.current.indeterminate = indeterminate
  }, [indeterminate])
  return (
    <input
      {...props}
      ref={input}
      type="checkbox"
      className={cx(
        `
  size-[16px] flex-none cursor-pointer accent-primary focus-visible:outline-2
  focus-visible:outline-offset-2 focus-visible:outline-focus disabled:cursor-not-allowed
  disabled:opacity-50
`,
        className,
      )}
    />
  )
}
export function Radio({ className, ...props }: Omit<InputHTMLAttributes<HTMLInputElement>, 'type'>) {
  return (
    <input
      {...props}
      type="radio"
      className={cx(
        `
  size-[16px] flex-none cursor-pointer rounded-full accent-primary focus-visible:outline-2
  focus-visible:outline-offset-2 focus-visible:outline-focus disabled:cursor-not-allowed
  disabled:opacity-50
`,
        className,
      )}
    />
  )
}
export function ChoiceCard({
  children,
  description,
  className,
  ...props
}: Omit<InputHTMLAttributes<HTMLInputElement>, 'type' | 'title'> & { children: ReactNode; description?: ReactNode }) {
  return (
    <label
      className={cx(
        `
  flex min-h-[62px] w-[300px] max-w-full cursor-pointer items-center gap-1.5 rounded-[8px] border
  border-border bg-inset px-1.5 py-1.375 has-checked:border-primary
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
    </label>
  )
}
export function CheckboxOption({ children, className, ...props }: CheckboxProps & { children: ReactNode }) {
  return (
    <label className={cx('inline-flex min-h-[24px] cursor-pointer items-center gap-0.5 text-[13px]', className)}>
      <Checkbox {...props} />
      <span>{children}</span>
    </label>
  )
}
