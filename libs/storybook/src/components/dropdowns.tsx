import { useContext, type ReactNode } from 'react'
import { Select as BaseSelect } from '@base-ui/react/select'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { animated, FloatingMotion } from './motion'
import { useFormField, useFormInvalid } from './form'
import { cx } from './shared'
import { FloatingContainerContext } from '../context'

export interface SelectItem {
  value: string
  label: string
  disabled?: boolean
  icon?: ReactNode
}
export interface SelectProps extends Omit<DropdownProps, 'label'> {
  'aria-label': string
}

function SelectRoot(props: Omit<BaseSelect.Root.Props<string, false>, 'modal' | 'multiple'>) {
  const field = useFormField<string>(props.name)
  return (
    <BaseSelect.Root
      {...props}
      value={field ? field.value : props.value}
      onValueChange={(value, details) => {
        if (field && value !== null) field.change(value)
        else props.onValueChange?.(value, details)
      }}
      onOpenChange={(open, details) => {
        if (!open) field?.blur()
        props.onOpenChange?.(open, details)
      }}
      modal={false}
    />
  )
}

function SelectTrigger({
  className,
  children,
  ...props
}: Omit<BaseSelect.Trigger.Props, 'className'> & { className?: string }) {
  return (
    <BaseSelect.Trigger
      {...props}
      render={<animated.button type="button" />}
      className={cx(
        `flex h-7.5 w-full min-w-0 cursor-pointer items-center justify-between gap-1 rounded-xl
border-(length:--border-hairline)
        border-transparent bg-elevated px-1.5 py-0 text-xsm leading-5 text-foreground shadow-control
        focus-visible:outline-1 focus-visible:outline-offset-1 focus-visible:outline-ring
        disabled:cursor-not-allowed disabled:opacity-50 disabled:shadow-none`,
        className,
      )}
    >
      {children}
      <BaseSelect.Icon>
        <CaretDownIcon size={10} aria-hidden="true" />
      </BaseSelect.Icon>
    </BaseSelect.Trigger>
  )
}

function SelectValue({ className, ...props }: Omit<BaseSelect.Value.Props, 'className'> & { className?: string }) {
  return <BaseSelect.Value {...props} className={cx('flex min-w-0 items-center gap-2 truncate', className)} />
}

function SelectContent({ className, children, ...props }: Omit<BaseSelect.Popup.Props, 'className'> & { className?: string }) {
  const container = useContext(FloatingContainerContext)
  return (
    <BaseSelect.Portal container={container}>
      <BaseSelect.Positioner
        alignItemWithTrigger={false}
        sideOffset={4}
        align="start"
        positionMethod="fixed"
        collisionPadding={8}
        className="z-50 outline-none"
      >
        <BaseSelect.Popup
          {...props}
          render={<FloatingMotion />}
          className={cx(
            `w-(--anchor-width) max-w-[calc(100vw-1rem)] origin-(--transform-origin) rounded-control
            border-(length:--border-hairline) border-border bg-elevated font-sans text-foreground shadow-menu outline-none`,
            className,
          )}
        >
          <BaseSelect.List
            className="max-h-[min(--spacing(80),var(--available-height))] overflow-y-auto
            overscroll-contain p-1"
          >
            {children}
          </BaseSelect.List>
        </BaseSelect.Popup>
      </BaseSelect.Positioner>
    </BaseSelect.Portal>
  )
}

function SelectEntry({ className, children, ...props }: Omit<BaseSelect.Item.Props, 'className'> & { className?: string }) {
  return (
    <BaseSelect.Item
      {...props}
      className={cx(
        `flex min-h-8 w-full cursor-pointer items-center gap-1 rounded-md px-1.5 py-0.5 text-xsm outline-none
        data-highlighted:bg-border data-disabled:cursor-not-allowed data-disabled:opacity-50`,
        className,
      )}
    >
      <BaseSelect.ItemText className="flex min-w-0 flex-1 items-center gap-1">{children}</BaseSelect.ItemText>
      <BaseSelect.ItemIndicator>
        <CheckIcon size={14} aria-hidden="true" />
      </BaseSelect.ItemIndicator>
    </BaseSelect.Item>
  )
}

function SelectControl({ 'aria-label': label, ...props }: SelectProps) {
  return <Dropdown {...props} label={label} />
}

export const Select = Object.assign(SelectControl, {
  Root: SelectRoot,
  Trigger: SelectTrigger,
  Value: SelectValue,
  Content: SelectContent,
  Item: SelectEntry,
})

export interface DropdownProps {
  name?: string
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
  name,
  options,
  value,
  defaultValue,
  onValueChange,
  label,
  disabled,
  defaultOpen = false,
  className,
}: DropdownProps) {
  const invalid = useFormInvalid(name)
  return (
    <div data-ui="dropdown" className={cx('relative inline-block w-60 max-w-full', className)}>
      <Select.Root
        name={name}
        items={options}
        value={value}
        defaultValue={defaultValue ?? options.find((item) => !item.disabled)?.value ?? ''}
        onValueChange={(next) => {
          if (next !== null) onValueChange?.(next)
        }}
        disabled={disabled}
        defaultOpen={defaultOpen}
      >
        <Select.Trigger
          aria-label={label}
          aria-invalid={invalid || undefined}
          aria-describedby={name ? `${name}-error` : undefined}
        >
          <Select.Value placeholder={label}>
            {(selected: string | null) => {
              const item = options.find((option) => option.value === selected)
              return (
                <>
                  {item?.icon}
                  <span className="truncate">{item?.label ?? label}</span>
                </>
              )
            }}
          </Select.Value>
        </Select.Trigger>
        <Select.Content aria-label={label}>
          {options.map((item) => (
            <Select.Item key={item.value} value={item.value} label={item.label} disabled={item.disabled}>
              {item.icon}
              {item.label}
            </Select.Item>
          ))}
        </Select.Content>
      </Select.Root>
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
  return <div className="p-1.5 text-ui-12 font-medium text-muted-foreground">{children}</div>
}
export const SelectOption = SelectEntry
