import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { Button } from '../../src/components/buttons'
import { Searchbox } from '../../src/components/fields'
import { ChoicePicker } from '../../src/components/choice-picker'

const options = [
  { id: 'operator', label: 'Model operator', description: 'Manage and use models.', color: '#8b9cda' },
  { id: 'viewer', label: 'Usage viewer', description: 'View usage reports.', color: '#7abda3' },
  { id: 'disabled', label: 'Unavailable role', disabled: true },
]

function Example({ empty = false, disabled = false }: { empty?: boolean; disabled?: boolean }) {
  const [value, setValue] = useState<string[]>([])
  return (
    <div className="flex min-h-72 flex-col items-start gap-4 bg-background p-6 text-foreground">
      <ChoicePicker.Control
        label="Add role"
        searchLabel="Search roles"
        placeholder="Search roles…"
        emptyMessage="No roles found."
        options={empty ? [] : options}
        value={value}
        onValueChange={setValue}
        disabled={disabled}
      />
      <p className="text-xs text-muted-foreground">{value.length} selected</p>
    </div>
  )
}

const meta = { title: 'Components/Choice picker', component: Example } satisfies Meta<typeof Example>
export default meta
type Story = StoryObj<typeof meta>
export const Default: Story = {}
export const Empty: Story = { args: { empty: true } }
export const Disabled: Story = { args: { disabled: true } }

function ComposedPicker() {
  const [value, setValue] = useState<string[]>(['viewer'])
  const [open, setOpen] = useState(false)
  return (
    <div className="flex min-h-72 flex-col items-start gap-4 bg-background p-6 text-foreground">
      <ChoicePicker.Root options={options} value={value} onValueChange={setValue} open={open} onOpenChange={setOpen}>
        <ChoicePicker.Trigger asChild>
          <Button>Add roles</Button>
        </ChoicePicker.Trigger>
        <ChoicePicker.Content>
          <div className="p-2 pb-0">
            <Searchbox.Root>
              <ChoicePicker.Input aria-label="Search roles" placeholder="Search roles…" />
            </Searchbox.Root>
          </div>
          <ChoicePicker.Empty>No roles found.</ChoicePicker.Empty>
          <ChoicePicker.List>{(option) => <ChoicePicker.Item key={option.id} value={option} />}</ChoicePicker.List>
        </ChoicePicker.Content>
      </ChoicePicker.Root>
      <output>{value.join(', ') || 'No roles selected'}</output>
      <Button onClick={() => setValue([])}>Reset selection</Button>
    </div>
  )
}
export const Composition: Story = { render: () => <ComposedPicker /> }
