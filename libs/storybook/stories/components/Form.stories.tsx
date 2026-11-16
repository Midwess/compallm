import { RoleUpdateInput } from '@compallm/contract/validation'
import { useState, useEffect } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { Form, createForm, useFormValue, contractResult, type CommitMode } from '../../src/components/form'
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
      <Form.Root model={form} className="grid gap-4">
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
      </Form.Root>
      <p className="text-xs text-muted-foreground">
        Draft: {name}. Confirmed: {saved.name}, {saved.enabled ? 'enabled' : 'disabled'}.
      </p>
      <p className="text-xs text-muted-foreground">
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

function ValidationExample({ edge = false }: { edge?: boolean }) {
  const [form] = useState(() =>
    createForm(
      { name: '' },
      {
        mode: 'button',
        command: true,
        parse: (values) => contractResult(RoleUpdateInput.safeParse(values), values),
        submit: (_values, patch) => patch,
      },
    ),
  )
  useEffect(() => {
    void form.submit()
  }, [form])
  return (
    <div className={edge ? 'flex min-h-48 justify-end p-2' : 'min-h-48 p-6'}>
      <Form.Root model={form} className="grid w-72 content-start gap-3">
        <label className="grid gap-2 text-sm">
          Role name
          <TextField name="name" />
        </label>
        <Form.Error name="name" />
        <p className="text-xs text-muted-foreground">
          The message opens for five seconds. Hover or focus the warning to reopen it.
        </p>
        <Form.Submit>Validate</Form.Submit>
      </Form.Root>
    </div>
  )
}
export const ValidationFeedback: Story = { render: () => <ValidationExample /> }
export const ValidationAtRightEdge: Story = { render: () => <ValidationExample edge /> }
