import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { Form, createForm, useFormValue, type CommitMode } from '../../src/components/form'
import { TextField } from '../../src/components/fields'
import { Switch } from '../../src/components/selection'
import { Button } from '../../src/components/buttons'

function Demo({ mode = 'button', reject = false }: { mode?: CommitMode; reject?: boolean }) {
  const [saved, setSaved] = useState({ name: 'Research', enabled: true })
  const [form] = useState(() =>
    createForm(saved, {
      mode,
      submit: async (_values, patch, signal) => {
        await new Promise((resolve) => setTimeout(resolve, 800))
        if (signal.aborted) return false
        if (reject) throw new Error('Mock rejection')
        setSaved((current) => ({ ...current, ...patch }))
        return patch
      },
    }),
  )
  const name = useFormValue<string>(form, 'name')
  return (
    <div className="grid max-w-lg gap-4 p-6">
      <Form model={form} className="grid gap-4">
        <label className="grid gap-2 text-sm">
          Name
          <TextField name="name" />
        </label>
        <label className="flex items-center gap-2 text-sm">
          <Switch name="enabled" />
          Enabled
        </label>
        <div className="flex gap-3">
          <Form.Submit>Apply</Form.Submit>
          <Form.Reset>Reset</Form.Reset>
          <Form.Status />
        </div>
        <Button type="button" onClick={() => void form.retry()}>
          Retry latest draft
        </Button>
      </Form>
      <p className="text-xs text-muted">
        Draft: {name}. Confirmed: {saved.name}, {saved.enabled ? 'enabled' : 'disabled'}.
      </p>
      <p className="text-xs text-muted">
        Button mode suppresses blur and Enter. Other modes commit only at their configured trigger.
      </p>
      <TextField aria-label="Standalone input" placeholder="Outside the form" />
    </div>
  )
}
const meta = { title: 'Components/Form', component: Demo, parameters: { layout: 'padded' } } satisfies Meta<typeof Demo>
export default meta
type Story = StoryObj<typeof meta>
export const ButtonOnly: Story = {}
export const CommitOnBlur: Story = { args: { mode: 'blur' } }
export const CommitOnChange: Story = { args: { mode: 'change' } }
export const Debounced: Story = { args: { mode: 'debounce' } }
export const Rejected: Story = { args: { mode: 'blur', reject: true } }
