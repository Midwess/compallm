import { useFormField } from './form'
import { createContext, useContext, type ComponentPropsWithRef, type ReactNode } from 'react'
import { Combobox } from '@base-ui/react/combobox'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { Button } from './buttons'
import { Searchbox } from './fields'
import { FloatingMotion } from './motion'
import { FloatingContainerContext } from '../context'
import { cx } from './shared'
import { Slot, useControllableState, type OpenStateProps, type CompositionProps } from './composition'

export interface ChoicePickerOption {
  id: string
  label: string
  description?: string
  color?: string
  disabled?: boolean
}

export interface ChoicePickerProps extends OpenStateProps {
  defaultValue?: string[]
  name?: string
  label: string
  searchLabel: string
  placeholder: string
  emptyMessage: string
  emptyAction?: ReactNode
  options: ChoicePickerOption[]
  value?: string[]
  onValueChange?(value: string[]): void
  disabled?: boolean
}

export interface ChoicePickerRootProps extends OpenStateProps {
  children: ReactNode
  options: ChoicePickerOption[]
  name?: string
  value?: string[]
  defaultValue?: string[]
  onValueChange?(value: string[]): void
  disabled?: boolean
}
const PickerContext = createContext<{ name?: string; invalid?: boolean } | null>(null)
function usePicker() {
  const context = useContext(PickerContext)
  if (!context) throw new Error('ChoicePicker parts require ChoicePicker.Root.')
  return context
}
function Root({
  children,
  options,
  name,
  value: suppliedValue,
  defaultValue = [],
  onValueChange: suppliedChange,
  disabled,
  open,
  defaultOpen,
  onOpenChange,
}: ChoicePickerRootProps) {
  const field = useFormField<string[]>(name)
  const [value, change] = useControllableState(suppliedValue, defaultValue, suppliedChange)
  const ids = field ? (field.value ?? []) : value
  const selected = options.filter((option) => ids.includes(option.id))
  return (
    <PickerContext value={{ name, invalid: field?.invalid }}>
      <Combobox.Root
        items={options}
        multiple
        value={selected}
        disabled={disabled}
        open={open}
        defaultOpen={defaultOpen}
        onOpenChange={(next) => {
          if (!next) field?.blur()
          onOpenChange?.(next)
        }}
        itemToStringLabel={(option: ChoicePickerOption) => option.label}
        isItemEqualToValue={(left: ChoicePickerOption, right: ChoicePickerOption) => left.id === right.id}
        onValueChange={(next: ChoicePickerOption[]) => {
          const known = new Set(options.map((option) => option.id))
          const result = [...ids.filter((id) => !known.has(id)), ...next.map((option) => option.id)]
          if (field) field.change(result)
          else change(result)
        }}
      >
        {children}
      </Combobox.Root>
    </PickerContext>
  )
}
function Trigger({ asChild = false, children, ...props }: ComponentPropsWithRef<'button'> & CompositionProps) {
  const { name, invalid } = usePicker()
  return (
    <Combobox.Trigger
      {...props}
      data-form-field={name}
      aria-invalid={invalid || undefined}
      aria-describedby={name ? `${name}-error` : undefined}
      render={(partProps, state) =>
        asChild ? (
          <Slot {...partProps} data-state={state.open ? 'open' : 'closed'}>
            {children}
          </Slot>
        ) : (
          <Button
            {...partProps}
            data-state={state.open ? 'open' : 'closed'}
            variant="neutral"
            className="h-6 rounded-full px-2 text-xs"
          />
        )
      }
    >
      {asChild ? undefined : children}
    </Combobox.Trigger>
  )
}
function Content({ className, ...props }: ComponentPropsWithRef<'div'>) {
  const container = useContext(FloatingContainerContext)
  return (
    <Combobox.Portal container={container}>
      <Combobox.Positioner sideOffset={6} align="end" positionMethod="fixed" collisionPadding={8} className="z-50 outline-none">
        <Combobox.Popup
          {...props}
          render={<FloatingMotion />}
          className={cx(
            `flex max-h-(--available-height) w-2xs max-w-[calc(100vw-1rem)] flex-col overflow-hidden
rounded-control border border-border bg-menu font-sans text-foreground shadow-menu`,
            className,
          )}
        />
      </Combobox.Positioner>
    </Combobox.Portal>
  )
}
function Input(props: ComponentPropsWithRef<'input'>) {
  return <Combobox.Input {...props} render={<Searchbox.Input />} />
}
function Empty({ children, ...props }: ComponentPropsWithRef<'div'>) {
  return (
    <Combobox.Empty {...props}>
      <div className="px-3 py-4 text-center text-xs text-muted-foreground">{children}</div>
    </Combobox.Empty>
  )
}
function List({
  className,
  ...props
}: Omit<ComponentPropsWithRef<'div'>, 'children'> & {
  children: ReactNode | ((option: ChoicePickerOption) => ReactNode)
}) {
  return <Combobox.List {...props} className={cx('min-h-0 max-h-64 flex-1 overflow-y-auto overscroll-none p-1', className)} />
}
function Item({
  className,
  children,
  value,
  ...props
}: Omit<ComponentPropsWithRef<'div'>, 'children'> & {
  value: ChoicePickerOption
  disabled?: boolean
  children?: ReactNode
}) {
  return (
    <Combobox.Item
      {...props}
      value={value}
      disabled={props.disabled ?? value.disabled}
      className={cx(
        `flex min-h-9 cursor-pointer items-center gap-2 rounded-md px-2 py-1.5 text-left text-xsm
outline-none data-highlighted:bg-hover data-disabled:cursor-not-allowed data-disabled:opacity-50`,
        className,
      )}
    >
      {children ?? (
        <>
          {value.color ? <span className="size-1.5 shrink-0 rounded-full" style={{ backgroundColor: value.color }} /> : null}
          <span className="min-w-0 flex-1">
            <span className="block truncate">{value.label}</span>
            {value.description ? (
              <span className="block truncate text-xs text-muted-foreground">{value.description}</span>
            ) : null}
          </span>
          <Combobox.ItemIndicator className="shrink-0">
            <CheckIcon size={14} aria-hidden="true" />
          </Combobox.ItemIndicator>
        </>
      )}
    </Combobox.Item>
  )
}
function Control({ label, searchLabel, placeholder, emptyMessage, emptyAction, ...props }: ChoicePickerProps) {
  return (
    <Root {...props}>
      <Trigger>{label}</Trigger>
      <Content>
        <div className="p-2 pb-0">
          <Searchbox.Root>
            <Input aria-label={searchLabel} placeholder={placeholder} />
          </Searchbox.Root>
        </div>
        <Empty>
          <div>{emptyMessage}</div>
          {emptyAction}
        </Empty>
        <List>{(option) => <Item key={option.id} value={option} />}</List>
      </Content>
    </Root>
  )
}
export const ChoicePicker = { Root, Trigger, Content, Input, Empty, List, Item, Control }
