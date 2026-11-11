import type { ReactNode } from 'react'
import { Combobox } from '@base-ui/react/combobox'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { Button } from './buttons'
import { Searchbox } from './fields'
import { overlay } from './motion'

export interface ChoicePickerOption {
  id: string
  label: string
  description?: string
  color?: string
  disabled?: boolean
}

export interface ChoicePickerProps {
  label: string
  searchLabel: string
  placeholder: string
  emptyMessage: string
  emptyAction?: ReactNode
  options: ChoicePickerOption[]
  value: string[]
  onValueChange(value: string[]): void
  disabled?: boolean
}

export function ChoicePicker({
  label,
  searchLabel,
  placeholder,
  emptyMessage,
  emptyAction,
  options,
  value,
  onValueChange,
  disabled,
}: ChoicePickerProps) {
  const selected = options.filter((option) => value.includes(option.id))
  return (
    <Combobox.Root
      items={options}
      multiple
      value={selected}
      disabled={disabled}
      itemToStringLabel={(option: ChoicePickerOption) => option.label}
      isItemEqualToValue={(left: ChoicePickerOption, right: ChoicePickerOption) => left.id === right.id}
      onValueChange={(next: ChoicePickerOption[]) => {
        const known = new Set(options.map((option) => option.id))
        onValueChange([...value.filter((id) => !known.has(id)), ...next.map((option) => option.id)])
      }}
    >
      <Combobox.Trigger render={<Button variant="neutral" className="h-6 rounded-full px-2 text-xs" />}>
        {label}
      </Combobox.Trigger>
      <Combobox.Portal>
        <Combobox.Positioner sideOffset={6} align="end" collisionPadding={8} className="z-50 outline-none">
          <Combobox.Popup
            render={<overlay.div />}
            className="flex max-h-[var(--available-height)] w-72 max-w-[calc(100vw-1rem)] flex-col overflow-hidden
rounded-lg border border-border bg-menu font-sans text-text shadow-menu"
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
                  className="flex min-h-9 cursor-pointer items-center gap-2 rounded px-2 py-1.5 text-left text-[13px]
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
