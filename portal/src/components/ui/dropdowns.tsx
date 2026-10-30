import { useEffect, useId, useRef, useState, type ButtonHTMLAttributes, type KeyboardEvent, type ReactNode, type SelectHTMLAttributes } from 'react'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { cx } from './shared'

export interface SelectItem { value: string; label: string; disabled?: boolean; icon?: ReactNode }
export interface SelectProps extends SelectHTMLAttributes<HTMLSelectElement> { options: SelectItem[] }
export function Select({ options, className, ...props }: SelectProps) {
  return <span className={cx('base-select-wrap', className)}><select {...props} className="base-select">{options.map(item => <option key={item.value} value={item.value} disabled={item.disabled}>{item.label}</option>)}</select><CaretDownIcon size={10} aria-hidden="true" /></span>
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
export function Dropdown({ options, value, defaultValue, onValueChange, label, disabled, defaultOpen = false, className }: DropdownProps) {
  const [local, setLocal] = useState(defaultValue ?? options.find(item => !item.disabled)?.value ?? '')
  const [open, setOpen] = useState(defaultOpen)
  const [active, setActive] = useState(0)
  const selected = value ?? local
  const root = useRef<HTMLDivElement>(null)
  const trigger = useRef<HTMLButtonElement>(null)
  const list = useRef<HTMLDivElement>(null)
  const id = useId()
  const enabled = options.map((item, index) => item.disabled ? -1 : index).filter(index => index >= 0)
  function close() { setOpen(false); trigger.current?.focus() }
  useEffect(() => { if (open) list.current?.focus() }, [open])
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
    setLocal(item.value); onValueChange?.(item.value); close()
  }
  function keydown(event: KeyboardEvent<HTMLDivElement>) {
    const step = event.key === 'ArrowDown' ? 1 : event.key === 'ArrowUp' ? -1 : 0
    if (step && enabled.length) {
      event.preventDefault()
      const position = enabled.indexOf(active)
      setActive(enabled[(position + step + enabled.length) % enabled.length])
    } else if (event.key === 'Home' || event.key === 'End') {
      event.preventDefault(); setActive(enabled[event.key === 'Home' ? 0 : enabled.length - 1] ?? 0)
    } else if (event.key === 'Enter' || event.key === ' ') { event.preventDefault(); select(active) }
    else if (event.key === 'Escape') { event.preventDefault(); close() }
    else if (event.key === 'Tab') setOpen(false)
    else if (event.key.length === 1) {
      const match = options.findIndex(item => !item.disabled && item.label.toLowerCase().startsWith(event.key.toLowerCase()))
      if (match >= 0) setActive(match)
    }
  }
  return <div ref={root} className={cx('base-dropdown', className)}>
    <button ref={trigger} type="button" className="base-select base-dropdown-trigger" aria-label={label} aria-haspopup="listbox" aria-expanded={open} aria-controls={id} disabled={disabled}
      onClick={() => { setActive(Math.max(0, options.findIndex(item => item.value === selected && !item.disabled))); setOpen(!open) }}
      onKeyDown={event => { if (event.key === 'ArrowDown' || event.key === 'ArrowUp') { event.preventDefault(); setOpen(true); setActive(enabled[0] ?? 0) } }}>
      <span>{options.find(item => item.value === selected)?.label ?? label}</span><CaretDownIcon size={10} aria-hidden="true" />
    </button>
    {open && !disabled ? <div ref={list} id={id} role="listbox" aria-label={label} tabIndex={-1} className="base-menu" aria-activedescendant={`${id}-${active}`} onKeyDown={keydown}>
      {options.map((item, index) => <div key={item.value} id={`${id}-${index}`} role="option" aria-selected={item.value === selected} aria-disabled={item.disabled || undefined}
        className="base-menu-option" data-active={index === active || undefined} onPointerMove={() => { if (!item.disabled) setActive(index) }} onClick={() => select(index)}>
        {item.icon}<span>{item.label}</span>{item.value === selected ? <CheckIcon size={14} aria-hidden="true" /> : null}
      </div>)}
    </div> : null}
  </div>
}

const themes: SelectItem[] = ['System preference', 'Light', 'Pure Light', 'Dark', 'Magic Blue', 'Classic Dark', 'Custom'].map(label => ({ value: label, label }))
const diagrams: SelectItem[] = ['Default', 'TUI', 'Hairline', 'Hand-drawn'].map(label => ({ value: label, label }))
export function ThemeMenu(props: Omit<DropdownProps, 'options' | 'label'>) { return <Dropdown {...props} label="Theme" options={themes} defaultValue={props.defaultValue ?? 'Dark'} defaultOpen={props.defaultOpen ?? true} /> }
export function DiagramMenu(props: Omit<DropdownProps, 'options' | 'label'>) { return <Dropdown {...props} label="Diagram style" options={diagrams} defaultOpen={props.defaultOpen ?? true} /> }
export function DiagramDropdown(props: Omit<DropdownProps, 'options' | 'label'>) { return <Dropdown {...props} label="Diagram style" options={diagrams} /> }
export function MenuGroupHeading({ children }: { children: ReactNode }) { return <div className="base-menu-heading">{children}</div> }
export function SelectOption({ selected = false, children, className, ...props }: ButtonHTMLAttributes<HTMLButtonElement> & { selected?: boolean }) {
  return <button type="button" {...props} aria-pressed={selected} className={cx('base-select-option', className)}>{children}{selected ? <CheckIcon size={14} aria-hidden="true" /> : null}</button>
}
