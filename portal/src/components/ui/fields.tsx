import { useState, type InputHTMLAttributes, type TextareaHTMLAttributes } from 'react'
import { MagnifyingGlassIcon } from '@phosphor-icons/react/dist/csr/MagnifyingGlass'
import { cx } from './shared'

export type TextFieldProps = InputHTMLAttributes<HTMLInputElement>

export function TextField({ className, ...props }: TextFieldProps) {
  return <input {...props} className={cx('base-field', className)} />
}

export function MultilineField({ className, ...props }: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return <textarea {...props} className={cx('base-field base-textarea', className)} />
}

export function SearchField({ className, ...props }: TextFieldProps) {
  return <span className={cx('base-search', className)}><MagnifyingGlassIcon size={14} aria-hidden="true" /><TextField type="search" {...props} /></span>
}

export function EmailField({ className, ...props }: TextFieldProps) {
  return <TextField type="email" autoComplete="email" {...props} className={cx('base-email', className)} />
}

export function CodeField({ className, ...props }: TextFieldProps) {
  return <TextField inputMode="numeric" pattern="[0-9]*" maxLength={1} {...props} className={cx('base-code-digit', className)} />
}

export interface CodeEntryProps {
  value?: string
  defaultValue?: string
  onValueChange?: (value: string) => void
  disabled?: boolean
  'aria-invalid'?: boolean
  'aria-describedby'?: string
  'aria-label': string
}

export function CodeEntry({ value, defaultValue = '', onValueChange, disabled, 'aria-label': label, ...a11y }: CodeEntryProps) {
  const [local, setLocal] = useState(defaultValue)
  const current = value ?? local
  return <span className="base-code-entry">
    <input {...a11y} type="text" inputMode="numeric" autoComplete="one-time-code" maxLength={6}
      aria-label={label} value={current} disabled={disabled} onChange={event => {
        const digits = event.target.value.replace(/\D/g, '').slice(0, 6)
        setLocal(digits)
        onValueChange?.(digits)
      }} />
    {Array.from({ length: 6 }, (_, index) => <span key={index} aria-hidden="true" className="base-field base-code-digit" data-invalid={a11y['aria-invalid'] || undefined}>{current[index] ?? ''}</span>)}
  </span>
}

export function GuidanceEditor({ className, ...props }: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return <MultilineField {...props} className={cx('base-guidance', className)} />
}
export function InlineTitleEditor({ className, ...props }: TextFieldProps) {
  return <TextField {...props} className={cx('base-inline-editor base-title-editor', className)} />
}
export function InlineSummaryEditor({ className, ...props }: TextFieldProps) {
  return <TextField {...props} className={cx('base-inline-editor', className)} />
}
export function DescriptionEditor({ className, ...props }: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return <MultilineField {...props} className={cx('base-inline-editor base-description-editor', className)} />
}
export function DocumentTitleEditor(props: TextFieldProps) { return <InlineTitleEditor {...props} /> }
export function DocumentBodyEditor({ className, ...props }: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return <MultilineField {...props} className={cx('base-inline-editor base-document-editor', className)} />
}
