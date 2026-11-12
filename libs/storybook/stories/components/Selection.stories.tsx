import type { Meta, StoryObj } from '@storybook/react-vite'
import { StrictMode, useState } from 'react'
import { Button } from '../../src/components/buttons'
import * as Selection from '../../src/components/selection'
import { Preview, Row, Sample } from './Preview'

const meta = { title: 'Components/Selection controls', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
export const Switch: Story = {
  render: () => (
    <Preview>
      <Sample label="Off, on, disabled">
        <Row>
          <Selection.Switch aria-label="Off switch" />
          <Selection.Switch aria-label="On switch" defaultChecked />
          <Selection.Switch aria-label="Disabled switch" disabled />
        </Row>
      </Sample>
    </Preview>
  ),
}
export const Checkbox: Story = {
  render: () => (
    <Preview>
      <Sample label="Unchecked, checked, mixed, disabled">
        <Row>
          <Selection.Checkbox aria-label="Unchecked" />
          <Selection.Checkbox aria-label="Checked" defaultChecked />
          <Selection.Checkbox aria-label="Mixed" indeterminate />
          <Selection.Checkbox aria-label="Disabled" disabled />
        </Row>
      </Sample>
    </Preview>
  ),
}
export const Radio: Story = {
  render: () => (
    <Preview>
      <Sample label="Radio group">
        <fieldset className="flex max-w-full flex-wrap items-center gap-2">
          <legend className="text-[12px] leading-4.5 text-muted">Default response</legend>
          <label className="inline-flex min-h-6 cursor-pointer items-center gap-0.5 text-xsm">
            <Selection.Radio name="response" value="concise" defaultChecked />
            Concise
          </label>
          <label className="inline-flex min-h-6 cursor-pointer items-center gap-0.5 text-xsm">
            <Selection.Radio name="response" value="detailed" />
            Detailed
          </label>
          <Selection.Radio name="response" aria-label="Unavailable" disabled />
        </fieldset>
      </Sample>
    </Preview>
  ),
}
export const ChoiceCard: Story = {
  render: () => (
    <Preview>
      <Sample label="Provider choices">
        <Selection.ChoiceCard name="provider" value="first" description="Provider models" defaultChecked>
          Provider one
        </Selection.ChoiceCard>
        <Selection.ChoiceCard name="provider" value="second" description="Provider models">
          Provider two
        </Selection.ChoiceCard>
      </Sample>
    </Preview>
  ),
}
export const CheckboxOption: Story = {
  render: () => (
    <Preview>
      <Sample label="Checkbox labels">
        <Selection.CheckboxOption defaultChecked>Low</Selection.CheckboxOption>
        <Selection.CheckboxOption>Medium</Selection.CheckboxOption>
        <Selection.CheckboxOption disabled>Unavailable</Selection.CheckboxOption>
      </Sample>
    </Preview>
  ),
}

export const CompactCheckbox: Story = {
  render: () => (
    <Preview>
      <Sample label="Compact · unchecked, checked, mixed, disabled">
        <Row>
          <Selection.Checkbox size="compact" aria-label="Unchecked compact" />
          <Selection.Checkbox size="compact" aria-label="Checked compact" defaultChecked />
          <Selection.Checkbox size="compact" aria-label="Mixed compact" indeterminate />
          <Selection.Checkbox size="compact" aria-label="Disabled compact" disabled />
        </Row>
      </Sample>
    </Preview>
  ),
}

function SelectionTransitions() {
  const [checked, setChecked] = useState(false)
  return (
    <Preview>
      <Sample label="Controlled and native selection transitions" wide>
        <form className="grid gap-4">
          <label className="flex items-center gap-2 text-xsm">
            <Selection.Switch checked={checked} onChange={(event) => setChecked(event.currentTarget.checked)} />
            Controlled switch
          </label>
          <Button onClick={() => setChecked((value) => !value)}>Toggle controlled value</Button>
          <Selection.CheckboxOption defaultChecked>Resettable checkbox</Selection.CheckboxOption>
          <Row>
            <label className="flex items-center gap-2 text-xsm">
              <Selection.Radio name="motion-choice" defaultChecked /> First
            </label>
            <label className="flex items-center gap-2 text-xsm">
              <Selection.Radio name="motion-choice" /> Second
            </label>
          </Row>
          <Button type="reset">Reset native selections</Button>
        </form>
      </Sample>
    </Preview>
  )
}

export const TransitionsAndReset: Story = { render: () => <SelectionTransitions /> }

function RestoredSelections() {
  const [checked, setChecked] = useState(true)
  return (
    <Preview>
      <Sample label="Loaded selection state in Strict Mode" wide>
        <label className="flex items-center gap-2 text-xsm">
          <Selection.Switch checked={checked} onChange={(event) => setChecked(event.currentTarget.checked)} />
          Saved permission: {checked ? 'enabled' : 'disabled'}
        </label>
        <Row>
          <Selection.Checkbox aria-label="Saved checked value" defaultChecked />
          <Selection.Checkbox aria-label="Saved mixed value" indeterminate />
          <Selection.Radio aria-label="Saved radio value" defaultChecked />
        </Row>
        <Button onClick={() => setChecked(true)}>Restore enabled permission</Button>
      </Sample>
    </Preview>
  )
}

export const RestoredInStrictMode: Story = {
  render: () => (
    <StrictMode>
      <RestoredSelections />
    </StrictMode>
  ),
}
