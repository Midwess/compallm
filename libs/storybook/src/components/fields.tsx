import { otpLength, otpPattern, otpCharacters } from '@compallm/contract/auth'
import { useEffect, useRef, useState, type ComponentPropsWithRef, type TextareaHTMLAttributes } from 'react'
import { MagnifyingGlassIcon } from '@phosphor-icons/react/dist/csr/MagnifyingGlass'
import { cx } from './shared'

export type TextFieldProps = ComponentPropsWithRef<'input'>

export function TextField({ className, ...props }: TextFieldProps) {
  return (
    <input
      {...props}
      className={cx(
        `
  block h-8 w-full min-w-0 rounded-lg border-[0.5px] border-input-border bg-inset px-1.5
  py-0.5 text-sm leading-5 text-text placeholder:text-muted placeholder:opacity-100
  focus:border-muted/60 focus:outline-none aria-invalid:border-error
  disabled:cursor-not-allowed disabled:opacity-50
`,
        className,
      )}
    />
  )
}

export function MultilineField({ className, ...props }: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return (
    <textarea
      {...props}
      className={cx(
        `
  block h-8 w-full min-w-0 rounded-lg border-[0.5px] border-input-border bg-inset px-1.5
  py-0.5 text-sm leading-5 text-text placeholder:text-muted placeholder:opacity-100
  focus:border-muted/60 focus:outline-none aria-invalid:border-error
  disabled:cursor-not-allowed disabled:opacity-50 h-auto min-h-16 resize-y
`,
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
        className="pointer-events-none absolute top-1/2 left-2.5 size-3.5 -translate-y-1/2 text-muted"
      />
      {children}
    </div>
  )
}
function SearchboxInput(props: TextFieldProps) {
  return (
    <TextField
      type="search"
      {...props}
      className={cx(
        'h-8 rounded-lg border-transparent bg-elevated py-1 pr-3 pl-8 text-xs leading-5 focus:outline-input-border',
        props.className,
      )}
    />
  )
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
  return <TextField type="email" autoComplete="email" {...props} className={cx('h-[48px] p-1.5 text-[15px]', className)} />
}

export function CodeField({ className, ...props }: TextFieldProps) {
  return (
    <TextField
      inputMode="numeric"
      pattern={otpPattern}
      maxLength={1}
      {...props}
      className={cx('h-[56px] w-[52px] p-0 text-center text-[20px]', className)}
    />
  )
}

export interface CodeEntryProps {
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
  value,
  defaultValue = '',
  onValueChange,
  disabled,
  autoFocus,
  'aria-label': label,
  ...a11y
}: CodeEntryProps) {
  const [local, setLocal] = useState(defaultValue)
  const inputs = useRef<Array<HTMLInputElement | null>>([])
  const current = (value ?? local).padEnd(otpLength, ' ').slice(0, otpLength)
  useEffect(() => {
    if (autoFocus && !disabled) (inputs.current.find((input) => input?.value === '') ?? inputs.current[0])?.focus()
  }, [autoFocus, disabled])
  function update(next: string[], focus: number) {
    const nextCode = next.join('').trimEnd()
    setLocal(nextCode)
    onValueChange?.(nextCode)
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
  flex w-[352px] max-w-full gap-1 [&>input]:min-w-0 [&>input]:max-w-[52px] [&>input]:flex-1
`}
      data-ui="code-entry"
      role="group"
      aria-label={label}
    >
      {Array.from({ length: otpLength }, (_, index) => (
        <CodeField
          key={index}
          {...a11y}
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
  return <MultilineField {...props} className={cx('min-h-[180px] text-[13px]', className)} />
}
export function InlineTitleEditor({ className, ...props }: TextFieldProps) {
  return (
    <TextField
      {...props}
      className={cx('border-transparent bg-transparent p-0 h-[32px] text-[24px] leading-[32px] font-medium', className)}
    />
  )
}
export function InlineSummaryEditor({ className, ...props }: TextFieldProps) {
  return <TextField {...props} className={cx('border-transparent bg-transparent p-0', className)} />
}
export function DescriptionEditor({ className, ...props }: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return (
    <MultilineField {...props} className={cx('border-transparent bg-transparent p-0 min-h-[110px] text-[13px]', className)} />
  )
}
export function DocumentTitleEditor(props: TextFieldProps) {
  return <InlineTitleEditor {...props} />
}
export function DocumentBodyEditor({ className, ...props }: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return (
    <MultilineField
      {...props}
      className={cx('border-transparent bg-transparent p-0 min-h-[96px] text-[15px] leading-[24px]', className)}
    />
  )
}
