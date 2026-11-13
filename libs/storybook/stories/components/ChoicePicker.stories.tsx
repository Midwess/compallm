import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
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
      <ChoicePicker
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
