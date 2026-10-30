import { useId, type InputHTMLAttributes, type LabelHTMLAttributes, type ReactNode } from 'react'
import { TextField } from './fields'
import { HelperText, InlineError } from './feedback'
import { cx } from './shared'

export function FormIntroduction({ title, children }: { title: ReactNode; children?: ReactNode }) { return <header className="base-form-intro"><h1>{title}</h1><div className="base-muted">{children}</div></header> }
export function FormStepMarker({ children }: { children: ReactNode }) { return <span className="base-step-marker">{children}</span> }
export function FormStepHeading({ step, children }: { step: number; children: ReactNode }) { return <h2 className="base-step-heading"><FormStepMarker>{step}</FormStepMarker>{children}</h2> }
export function FormStepConnector() { return <span className="base-step-connector" aria-hidden="true" /> }
export function FormFieldLabel({ className, ...props }: LabelHTMLAttributes<HTMLLabelElement>) { return <label {...props} className={cx('base-field-label', className)} /> }
export interface LabeledFieldProps extends InputHTMLAttributes<HTMLInputElement> { label: string; hint?: string; error?: string }
export function LabeledField({ label, hint, error, id, className, ...props }: LabeledFieldProps) {
  const generated = useId()
  const inputId = id ?? generated
  const describedBy = [props['aria-describedby'], hint ? `${inputId}-hint` : '', error ? `${inputId}-error` : ''].filter(Boolean).join(' ') || undefined
  return <div className={cx('base-labeled-field', className)}><FormFieldLabel htmlFor={inputId}>{label}</FormFieldLabel><TextField {...props} id={inputId} aria-describedby={describedBy} aria-invalid={error ? true : props['aria-invalid']} />{hint ? <HelperText id={`${inputId}-hint`}>{hint}</HelperText> : null}{error ? <InlineError id={`${inputId}-error`}>{error}</InlineError> : null}</div>
}
