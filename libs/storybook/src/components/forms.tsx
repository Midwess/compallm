import { useId, type InputHTMLAttributes, type LabelHTMLAttributes, type ReactNode } from 'react'
import { TextField } from './fields'
import { HelperText, InlineError } from './feedback'
import { cx } from './shared'

export function FormIntroduction({ title, children }: { title: ReactNode; children?: ReactNode }) {
  return (
    <header
      className={`
  [&>h1]:text-[24px] [&>h1]:leading-[32px] [&>h1]:font-medium [&>div]:mt-1 [&>div]:text-[13px]
`}
    >
      <h1>{title}</h1>
      <div className="text-muted">{children}</div>
    </header>
  )
}
export function FormStepMarker({ children }: { children: ReactNode }) {
  return (
    <span
      className={`
  inline-flex size-[20px] items-center justify-center rounded-full border-[0.5px] border-input-border
  bg-elevated text-[12px] font-normal
`}
    >
      {children}
    </span>
  )
}
export function FormStepHeading({ step, children }: { step: number; children: ReactNode }) {
  return (
    <h2 className="flex items-center gap-1.5 text-[16px] leading-[24px] font-medium">
      <FormStepMarker>{step}</FormStepMarker>
      {children}
    </h2>
  )
}
export function FormStepConnector() {
  return <span className="block h-[144px] w-[2px] bg-border" aria-hidden="true" />
}
export function FormFieldLabel({ className, ...props }: LabelHTMLAttributes<HTMLLabelElement>) {
  return <label {...props} className={cx('block text-[14px] leading-[21px] font-medium', className)} />
}
export interface LabeledFieldProps extends InputHTMLAttributes<HTMLInputElement> {
  label: string
  hint?: string
  error?: string
}
export function LabeledField({ label, hint, error, id, className, ...props }: LabeledFieldProps) {
  const generated = useId()
  const inputId = id ?? generated
  const describedBy =
    [props['aria-describedby'], hint ? `${inputId}-hint` : '', error ? `${inputId}-error` : ''].filter(Boolean).join(' ') ||
    undefined
  return (
    <div className={cx('grid gap-0.5', className)}>
      <FormFieldLabel htmlFor={inputId}>{label}</FormFieldLabel>
      <TextField {...props} id={inputId} aria-describedby={describedBy} aria-invalid={error ? true : props['aria-invalid']} />
      {hint ? <HelperText id={`${inputId}-hint`}>{hint}</HelperText> : null}
      {error ? <InlineError id={`${inputId}-error`}>{error}</InlineError> : null}
    </div>
  )
}
