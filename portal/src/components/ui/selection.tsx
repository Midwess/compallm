import { useEffect, useRef, type InputHTMLAttributes, type ReactNode } from 'react'
import { cx } from './shared'

export function Switch({ className, ...props }: Omit<InputHTMLAttributes<HTMLInputElement>, 'type' | 'role'>) {
  return <input {...props} type="checkbox" role="switch" className={cx('base-switch', className)} />
}
export interface CheckboxProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'type'> { indeterminate?: boolean }
export function Checkbox({ indeterminate = false, className, ...props }: CheckboxProps) {
  const input = useRef<HTMLInputElement>(null)
  useEffect(() => { if (input.current) input.current.indeterminate = indeterminate }, [indeterminate])
  return <input {...props} ref={input} type="checkbox" className={cx('base-checkbox', className)} />
}
export function Radio({ className, ...props }: Omit<InputHTMLAttributes<HTMLInputElement>, 'type'>) {
  return <input {...props} type="radio" className={cx('base-radio', className)} />
}
export function ChoiceCard({ children, description, className, ...props }: Omit<InputHTMLAttributes<HTMLInputElement>, 'type' | 'title'> & { children: ReactNode; description?: ReactNode }) {
  return <label className={cx('base-choice-card', className)}><Radio {...props} /><span><strong>{children}</strong>{description ? <span className="base-helper">{description}</span> : null}</span></label>
}
export function CheckboxOption({ children, className, ...props }: CheckboxProps & { children: ReactNode }) {
  return <label className={cx('base-checkbox-option', className)}><Checkbox {...props} /><span>{children}</span></label>
}
