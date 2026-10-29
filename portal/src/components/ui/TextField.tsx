import { useId, type InputHTMLAttributes } from 'react'

export interface TextFieldProps extends InputHTMLAttributes<HTMLInputElement> {
  label: string
  hint?: string
  error?: string
}

/** Labeled input with associated hint and validation message. */
export function TextField({ label, hint, error, id, className = '', ...props }: TextFieldProps) {
  const generatedId = useId()
  const inputId = id ?? generatedId
  const descriptionId = `${inputId}-description`
  const describedBy = [props['aria-describedby'], hint || error ? descriptionId : undefined].filter(Boolean).join(' ') || undefined

  return (
    <div className={`ui-field ${className}`}>
      <label htmlFor={inputId}>{label}</label>
      <input {...props} id={inputId} aria-invalid={error ? true : props['aria-invalid']} aria-describedby={describedBy} />
      {hint || error ? <p id={descriptionId} className={error ? 'ui-field-error' : 'ui-muted'}>{error ?? hint}</p> : null}
    </div>
  )
}
