import { animated } from './motion'
import { otpLength, otpPattern, otpCharacters } from '@compallm/contract/auth'
import { useEffect, useRef, useState, type ComponentPropsWithRef, type TextareaHTMLAttributes } from 'react'
import { MagnifyingGlassIcon } from '@phosphor-icons/react/dist/csr/MagnifyingGlass'
import { cx, composeRefs } from './shared'
import { useFormField, useNativeFormField } from './form'

export type TextFieldProps = ComponentPropsWithRef<'input'> & { variant?: 'default' | 'constant' }
export function TextField({ variant = 'default', className, onChange, onBlur, onKeyDown, value, ...props }: TextFieldProps) {
  const constant = variant === 'constant'
  const field = useNativeFormField(constant ? undefined : props.name)
  return (
    <animated.input
      {...props}
      {...field?.registration}
      disabled={constant || props.disabled}
      readOnly={constant || props.readOnly}
      tabIndex={constant ? -1 : props.tabIndex}
      data-variant={variant}
      ref={composeRefs(field?.registration.ref, props.ref)}
      value={field ? undefined : value}
      defaultValue={field ? field.defaultValue : props.defaultValue}
      aria-describedby={props['aria-describedby'] ?? (field ? `${props.name}-error` : undefined)}
      aria-invalid={props['aria-invalid']}
      onChange={(event) => {
        if (field) void field.registration.onChange(event)
        else onChange?.(event)
      }}
      onBlur={(event) => {
        if (field) void field.registration.onBlur(event)
        onBlur?.(event)
      }}
      onKeyDown={(event) => {
        onKeyDown?.(event)
        if (event.defaultPrevented || event.nativeEvent.isComposing || !field) return
        if (event.key === 'Enter') {
          event.preventDefault()
          field.enter()
        }
        if (event.key === 'Escape') {
          event.preventDefault()
          field.escape()
        }
      }}
      className={cx(
        `block h-8 w-full min-w-0 rounded-lg border-(length:--border-hairline) border-input bg-inset px-1.5
py-1 text-left text-sm leading-5.5 text-foreground placeholder:text-muted-foreground placeholder:opacity-100
focus:border-muted-foreground/60 focus:outline-none aria-invalid:border-destructive disabled:cursor-not-allowed
disabled:opacity-50`,
        className,
        constant &&
          `pointer-events-none cursor-default bg-elevated caret-transparent select-none
disabled:cursor-default disabled:opacity-100`,
      )}
    />
  )
}
export function MultilineField({ className, value, onChange, onBlur, onKeyDown, ...props }: ComponentPropsWithRef<'textarea'>) {
  const field = useNativeFormField(props.name)
  return (
    <animated.textarea
      {...props}
      {...field?.registration}
      ref={composeRefs(field?.registration.ref, props.ref)}
      value={field ? undefined : value}
      defaultValue={field ? field.defaultValue : props.defaultValue}
      aria-describedby={props['aria-describedby'] ?? (field ? `${props.name}-error` : undefined)}
      aria-invalid={props['aria-invalid']}
      onChange={(event) => {
        if (field) void field.registration.onChange(event)
        else onChange?.(event)
      }}
      onBlur={(event) => {
        if (field) void field.registration.onBlur(event)
        onBlur?.(event)
      }}
      onKeyDown={(event) => {
        onKeyDown?.(event)
        if (event.defaultPrevented || event.nativeEvent.isComposing || !field) return
        if (event.key === 'Enter' && (event.metaKey || event.ctrlKey)) {
          event.preventDefault()
          field.enter()
        }
        if (event.key === 'Escape') {
          event.preventDefault()
          field.escape()
        }
      }}
      className={cx(
        `block w-full min-w-0 rounded-lg border-(length:--border-hairline) border-input bg-inset px-1.5
py-1 text-left text-sm leading-5.5 text-foreground placeholder:text-muted-foreground placeholder:opacity-100
focus:border-muted-foreground/60 focus:outline-none aria-invalid:border-destructive disabled:cursor-not-allowed
disabled:opacity-50
h-auto min-h-16 resize-y`,
        className,
      )}
    />
  )
}

function SearchboxRoot({ className, children, ...props }: import('react').HTMLAttributes<HTMLDivElement>) {
  return (
    <div {...props} className={cx('relative min-w-0', className)}>
      <MagnifyingGlassIcon
        aria-hidden="true"
        className="pointer-events-none absolute top-1/2 left-1.5 size-3.5 -translate-y-1/2 text-muted-foreground"
      />
      {children}
    </div>
  )
}
function SearchboxInput(props: TextFieldProps) {
  return <TextField type="search" {...props} className={cx('h-8 py-1 pr-1.5 pl-6 text-xsm leading-5', props.className)} />
}
export const Searchbox = Object.assign(SearchboxRoot, { Input: SearchboxInput })
export function SearchField({ className, ...props }: TextFieldProps) {
  return (
    <Searchbox className={className}>
      <Searchbox.Input {...props} />
    </Searchbox>
  )
}

export function EmailField({ className, ...props }: TextFieldProps) {
  return <TextField type="email" autoComplete="email" {...props} className={cx('h-12 p-1.5 text-ui-15', className)} />
}

export function CodeField({ className, ...props }: TextFieldProps) {
  return (
    <TextField
      inputMode="numeric"
      pattern={otpPattern}
      maxLength={1}
      {...props}
      className={cx('h-14 w-13 p-0 text-center text-ui-20', className)}
    />
  )
}

export interface CodeEntryProps {
  name?: string
  value?: string
  defaultValue?: string
  onValueChange?: (value: string) => void
  disabled?: boolean
  autoFocus?: boolean
  'aria-invalid'?: boolean
  'aria-describedby'?: string
  'aria-label': string
}

export function CodeEntry({
  name,
  value,
  defaultValue = '',
  onValueChange,
  disabled,
  autoFocus,
  'aria-label': label,
  ...a11y
}: CodeEntryProps) {
  const field = useFormField<string>(name)
  const [local, setLocal] = useState(defaultValue)
  const inputs = useRef<Array<HTMLInputElement | null>>([])
  const current = (field ? (field.value ?? '') : (value ?? local)).padEnd(otpLength, ' ').slice(0, otpLength)
  useEffect(() => {
    if (autoFocus && !disabled) (inputs.current.find((input) => input?.value === '') ?? inputs.current[0])?.focus()
  }, [autoFocus, disabled])
  function update(next: string[], focus: number) {
    const nextCode = next.join('').trimEnd()
    if (field) field.change(nextCode)
    else {
      setLocal(nextCode)
      onValueChange?.(nextCode)
    }
    inputs.current[Math.max(0, Math.min(otpLength - 1, focus))]?.focus()
  }
  function insert(text: string, index: number) {
    const digits = otpCharacters(text)
    if (!digits) return
    const start = digits.length === otpLength ? 0 : index
    const next = current.split('')
    for (let offset = 0; offset < digits.length && start + offset < otpLength; offset++) next[start + offset] = digits[offset]
    update(next, start + digits.length)
  }
  return (
    <div
      className={`
  flex w-88 max-w-full gap-1 [&>input]:min-w-0 [&>input]:max-w-13 [&>input]:flex-1
`}
      onBlur={(event) => {
        if (!event.currentTarget.contains(event.relatedTarget)) field?.blur()
      }}
      data-ui="code-entry"
      data-form-field={name}
      role="group"
      aria-label={label}
    >
      {Array.from({ length: otpLength }, (_, index) => (
        <CodeField
          key={index}
          {...a11y}
          aria-invalid={field?.invalid || a11y['aria-invalid']}
          aria-describedby={a11y['aria-describedby'] ?? (field ? `${name}-error` : undefined)}
          ref={(element) => {
            inputs.current[index] = element
          }}
          aria-label={`Digit ${index + 1} of ${otpLength}`}
          value={current[index].trim()}
          maxLength={otpLength}
          disabled={disabled}
          autoFocus={autoFocus && index === 0}
          autoComplete={index === 0 ? 'one-time-code' : 'off'}
          onFocus={(event) => event.currentTarget.select()}
          onChange={(event) => {
            if (event.target.value) insert(event.target.value, index)
            else {
              const next = current.split('')
              next[index] = ' '
              update(next, index)
            }
          }}
          onPaste={(event) => {
            event.preventDefault()
            insert(event.clipboardData.getData('text'), index)
          }}
          onKeyDown={(event) => {
            if (event.key === 'ArrowLeft' || event.key === 'ArrowRight') {
              event.preventDefault()
              inputs.current[index + (event.key === 'ArrowLeft' ? -1 : 1)]?.focus()
            } else if (event.key === 'Backspace' && current[index] === ' ' && index > 0) {
              event.preventDefault()
              const next = current.split('')
              next[index - 1] = ' '
              update(next, index - 1)
            }
          }}
        />
      ))}
    </div>
  )
}

export function GuidanceEditor({ className, ...props }: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return <MultilineField {...props} className={cx('min-h-45 text-xsm', className)} />
}
export function InlineTitleEditor({ className, ...props }: TextFieldProps) {
  return (
    <TextField
      {...props}
      className={cx('border-transparent bg-transparent p-0 h-8 text-ui-24 leading-8 font-medium', className)}
    />
  )
}
export function InlineSummaryEditor({ className, ...props }: TextFieldProps) {
  return <TextField {...props} className={cx('border-transparent bg-transparent p-0', className)} />
}
export function DescriptionEditor({ className, ...props }: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return <MultilineField {...props} className={cx('border-transparent bg-transparent p-0 min-h-27.5 text-xsm', className)} />
}
export function DocumentTitleEditor(props: TextFieldProps) {
  return <InlineTitleEditor {...props} />
}
export function DocumentBodyEditor({ className, ...props }: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return (
    <MultilineField
      {...props}
      className={cx('border-transparent bg-transparent p-0 min-h-24 text-ui-15 leading-6', className)}
    />
  )
}

export function FileField({ name, onChange, ...props }: Omit<TextFieldProps, 'type' | 'value' | 'defaultValue'>) {
  const field = useFormField<File | null>(name)
  return (
    <animated.input
      {...props}
      name={name}
      type="file"
      onChange={(event) => {
        if (field) field.change(event.currentTarget.files?.[0] ?? null)
        else onChange?.(event)
        event.currentTarget.value = ''
      }}
    />
  )
}
