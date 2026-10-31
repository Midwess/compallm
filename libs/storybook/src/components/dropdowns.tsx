import {
  useEffect,
  useId,
  useLayoutEffect,
  useRef,
  useState,
  type ButtonHTMLAttributes,
  type KeyboardEvent,
  type ReactNode,
  type CSSProperties,
} from 'react'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { cx } from './shared'

export interface SelectItem {
  value: string
  label: string
  disabled?: boolean
  icon?: ReactNode
}
export interface SelectProps extends Omit<DropdownProps, 'label'> {
  'aria-label': string
}
export function Select({ 'aria-label': label, ...props }: SelectProps) {
  return <Dropdown {...props} label={label} />
}

export interface DropdownProps {
  options: SelectItem[]
  value?: string
  defaultValue?: string
  onValueChange?: (value: string) => void
  label: string
  disabled?: boolean
  defaultOpen?: boolean
  className?: string
}
export function Dropdown({
  options,
  value,
  defaultValue,
  onValueChange,
  label,
  disabled,
  defaultOpen = false,
  className,
}: DropdownProps) {
  const [local, setLocal] = useState(defaultValue ?? options.find((item) => !item.disabled)?.value ?? '')
  const [open, setOpen] = useState(defaultOpen)
  const [active, setActive] = useState(0)
  const selected = value ?? local
  const root = useRef<HTMLDivElement>(null)
  const trigger = useRef<HTMLButtonElement>(null)
  const list = useRef<HTMLDivElement>(null)
  const id = useId()
  const anchor = `--dropdown-${id.replace(/[^a-zA-Z0-9-]/g, '')}`
  const enabled = options.map((item, index) => (item.disabled ? -1 : index)).filter((index) => index >= 0)
  function close() {
    setOpen(false)
    trigger.current?.focus()
  }
  useLayoutEffect(() => {
    if (!open || disabled) return
    list.current?.showPopover()
    list.current?.focus({ preventScroll: true })
  }, [open, disabled])
  useEffect(() => {
    if (!open) return
    function dismiss(event: PointerEvent) {
      if (event.target instanceof Node && !root.current?.contains(event.target)) setOpen(false)
    }
    document.addEventListener('pointerdown', dismiss)
    return () => document.removeEventListener('pointerdown', dismiss)
  }, [open])
  function select(index: number) {
    const item = options[index]
    if (!item || item.disabled) return
    setLocal(item.value)
    onValueChange?.(item.value)
    close()
  }
  function keydown(event: KeyboardEvent<HTMLDivElement>) {
    const step = event.key === 'ArrowDown' ? 1 : event.key === 'ArrowUp' ? -1 : 0
    if (step && enabled.length) {
      event.preventDefault()
      const position = enabled.indexOf(active)
      setActive(enabled[(position + step + enabled.length) % enabled.length])
    } else if (event.key === 'Home' || event.key === 'End') {
      event.preventDefault()
      setActive(enabled[event.key === 'Home' ? 0 : enabled.length - 1] ?? 0)
    } else if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault()
      select(active)
    } else if (event.key === 'Escape') {
      event.preventDefault()
      close()
    } else if (event.key === 'Tab') {
      trigger.current?.focus()
      setOpen(false)
    } else if (event.key.length === 1) {
      const match = options.findIndex((item) => !item.disabled && item.label.toLowerCase().startsWith(event.key.toLowerCase()))
      if (match >= 0) setActive(match)
    }
  }
  return (
    <div ref={root} data-ui="dropdown" className={cx('relative inline-block w-[240px] max-w-full', className)}>
      <button
        ref={trigger}
        style={{ anchorName: anchor } as CSSProperties}
        type="button"
        className={`
  block h-[30px] w-full min-w-0 rounded-[8px] border-[0.5px] border-input-border bg-elevated px-space-10
  py-0 text-[13px] leading-[20px] text-text focus:outline-1 focus:outline-offset-1 focus:outline-focus
  disabled:cursor-not-allowed disabled:opacity-50 flex cursor-pointer items-center justify-between
  gap-space-8 border-transparent shadow-control
`}
        aria-label={label}
        aria-haspopup="listbox"
        aria-expanded={open}
        aria-controls={id}
        disabled={disabled}
        onClick={() => {
          setActive(
            Math.max(
              0,
              options.findIndex((item) => item.value === selected && !item.disabled),
            ),
          )
          setOpen(!open)
        }}
        onKeyDown={(event) => {
          if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
            event.preventDefault()
            setOpen(true)
            setActive(enabled[0] ?? 0)
          }
        }}
      >
        <span>{options.find((item) => item.value === selected)?.label ?? label}</span>
        <CaretDownIcon size={10} aria-hidden="true" />
      </button>
      {open && !disabled ? (
        <div
          ref={list}
          popover="manual"
          style={{ positionAnchor: anchor } as CSSProperties}
          id={id}
          role="listbox"
          aria-label={label}
          tabIndex={-1}
          className={`
  fixed inset-auto top-[anchor(bottom)] left-[anchor(left)] z-20 my-space-6 mx-0
  max-h-[min(320px,calc(100dvh-16px))] w-[anchor-size(width)] max-w-[calc(100vw-16px)] overflow-y-auto
  rounded-[8px] border-[0.5px] border-border bg-elevated p-space-8 text-text shadow-menu
  [position-try-fallbacks:flip-block,flip-inline]
`}
          aria-activedescendant={`${id}-${active}`}
          onKeyDown={keydown}
        >
          {options.map((item, index) => (
            <div
              key={item.value}
              id={`${id}-${index}`}
              role="option"
              aria-selected={item.value === selected}
              aria-disabled={item.disabled || undefined}
              className={`
  flex min-h-[32px] w-full cursor-pointer items-center gap-space-8 rounded-[4px] px-space-12 py-space-4
  text-[13px] data-active:bg-border aria-disabled:cursor-not-allowed aria-disabled:opacity-50
  [&>span]:flex-1
`}
              data-active={index === active || undefined}
              onPointerMove={() => {
                if (!item.disabled) setActive(index)
              }}
              onClick={() => select(index)}
            >
              {item.icon}
              <span>{item.label}</span>
              {item.value === selected ? <CheckIcon size={14} aria-hidden="true" /> : null}
            </div>
          ))}
        </div>
      ) : null}
    </div>
  )
}

const themes: SelectItem[] = ['System preference', 'Light', 'Pure Light', 'Dark', 'Magic Blue', 'Classic Dark', 'Custom'].map(
  (label) => ({ value: label, label }),
)
const diagrams: SelectItem[] = ['Default', 'TUI', 'Hairline', 'Hand-drawn'].map((label) => ({ value: label, label }))
export function ThemeMenu(props: Omit<DropdownProps, 'options' | 'label'>) {
  return (
    <Dropdown
      {...props}
      label="Theme"
      options={themes}
      defaultValue={props.defaultValue ?? 'Dark'}
      defaultOpen={props.defaultOpen ?? true}
    />
  )
}
export function DiagramMenu(props: Omit<DropdownProps, 'options' | 'label'>) {
  return <Dropdown {...props} label="Diagram style" options={diagrams} defaultOpen={props.defaultOpen ?? true} />
}
export function DiagramDropdown(props: Omit<DropdownProps, 'options' | 'label'>) {
  return <Dropdown {...props} label="Diagram style" options={diagrams} />
}
export function MenuGroupHeading({ children }: { children: ReactNode }) {
  return <div className="p-space-12 text-[12px] font-medium text-muted">{children}</div>
}
export function SelectOption({
  selected = false,
  children,
  className,
  ...props
}: ButtonHTMLAttributes<HTMLButtonElement> & { selected?: boolean }) {
  return (
    <button
      type="button"
      {...props}
      aria-pressed={selected}
      className={cx(
        `
  flex min-h-[32px] w-full cursor-pointer items-center justify-between gap-space-8 rounded-[4px] border-0
  bg-transparent px-space-12 py-space-4 text-left text-[13px] text-inherit hover:bg-border
  aria-pressed:bg-border focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus
`,
        className,
      )}
    >
      {children}
      {selected ? <CheckIcon size={14} aria-hidden="true" /> : null}
    </button>
  )
}
