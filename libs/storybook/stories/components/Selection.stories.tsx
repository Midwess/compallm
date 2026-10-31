import type { Meta, StoryObj } from '@storybook/react-vite'
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
          <legend className="text-[12px] leading-[18px] text-muted">Default response</legend>
          <label className="inline-flex min-h-[24px] cursor-pointer items-center gap-0.5 text-[13px]">
            <Selection.Radio name="response" value="concise" defaultChecked />
            Concise
          </label>
          <label className="inline-flex min-h-[24px] cursor-pointer items-center gap-0.5 text-[13px]">
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
