import { useFormField } from './form'
import { useContext, type ReactNode } from 'react'
import { Combobox } from '@base-ui/react/combobox'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { Button } from './buttons'
import { Searchbox } from './fields'
import { FloatingMotion } from './motion'
import { FloatingContainerContext } from '../context'

export interface ChoicePickerOption {
  id: string
  label: string
  description?: string
  color?: string
  disabled?: boolean
}

export interface ChoicePickerProps {
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

export function ChoicePicker({
  name,
  label,
  searchLabel,
  placeholder,
  emptyMessage,
  emptyAction,
  options,
  value: suppliedValue,
  onValueChange: suppliedChange,
  disabled,
}: ChoicePickerProps) {
  const field = useFormField<string[]>(name)
  const value = field ? (field.value ?? []) : (suppliedValue ?? [])
  const onValueChange = (value: string[]) => {
    if (field) field.change(value)
    else suppliedChange?.(value)
  }
  const container = useContext(FloatingContainerContext)
  const selected = options.filter((option) => value.includes(option.id))
  return (
    <Combobox.Root
      items={options}
      multiple
      onOpenChange={(open) => {
        if (!open) field?.blur()
      }}
      value={selected}
      disabled={disabled}
      itemToStringLabel={(option: ChoicePickerOption) => option.label}
      isItemEqualToValue={(left: ChoicePickerOption, right: ChoicePickerOption) => left.id === right.id}
      onValueChange={(next: ChoicePickerOption[]) => {
        const known = new Set(options.map((option) => option.id))
        onValueChange([...value.filter((id) => !known.has(id)), ...next.map((option) => option.id)])
      }}
    >
      <Combobox.Trigger
        aria-invalid={field?.invalid || undefined}
        aria-describedby={field ? `${name}-error` : undefined}
        render={<Button variant="neutral" className="h-6 rounded-full px-2 text-xs" />}
      >
        {label}
      </Combobox.Trigger>
      <Combobox.Portal container={container}>
        <Combobox.Positioner
          sideOffset={6}
          align="end"
          positionMethod="fixed"
          collisionPadding={8}
          className="z-50 outline-none"
        >
          <Combobox.Popup
            render={<FloatingMotion />}
            className="flex max-h-(--available-height) w-2xs max-w-[calc(100vw-1rem)] flex-col overflow-hidden
rounded-[10px] border border-border bg-menu font-sans text-text shadow-menu"
          >
            <div className="p-2 pb-0">
              <Searchbox>
                <Combobox.Input render={<Searchbox.Input />} aria-label={searchLabel} placeholder={placeholder} />
              </Searchbox>
            </div>
            <Combobox.Empty>
              <div className="px-3 py-4 text-center text-xs text-muted">
                <div>{emptyMessage}</div>
                {emptyAction}
              </div>
            </Combobox.Empty>
            <Combobox.List className="min-h-0 max-h-64 flex-1 overflow-y-auto overscroll-contain p-1">
              {(option: ChoicePickerOption) => (
                <Combobox.Item
                  key={option.id}
                  value={option}
                  disabled={option.disabled}
                  className="flex min-h-9 cursor-pointer items-center gap-2 rounded-md px-2 py-1.5 text-left text-xsm
outline-none data-highlighted:bg-elevated data-disabled:cursor-not-allowed data-disabled:opacity-50"
                >
                  {option.color ? (
                    <span className="size-1.5 shrink-0 rounded-full" style={{ backgroundColor: option.color }} />
                  ) : null}
                  <span className="min-w-0 flex-1">
                    <span className="block truncate">{option.label}</span>
                    {option.description ? (
                      <span className="block truncate text-xs text-muted">{option.description}</span>
                    ) : null}
                  </span>
                  <Combobox.ItemIndicator className="shrink-0">
                    <CheckIcon size={14} aria-hidden="true" />
                  </Combobox.ItemIndicator>
                </Combobox.Item>
              )}
            </Combobox.List>
          </Combobox.Popup>
        </Combobox.Positioner>
      </Combobox.Portal>
    </Combobox.Root>
  )
}
