import type { Meta, StoryObj } from '@storybook/react-vite'
import * as Forms from '../../src/components/forms'
import { Preview, Sample } from './Preview'

const meta = { title: 'Components/Form primitives', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
export const FormIntroduction: Story = {
  render: () => (
    <Preview>
      <Sample label="Form introduction" wide>
        <Forms.FormIntroduction title="Add a connection">Connect a service to your workspace.</Forms.FormIntroduction>
      </Sample>
    </Preview>
  ),
}
export const FormStepMarker: Story = {
  render: () => (
    <Preview>
      <Sample label="Step marker · 20 px">
        <Forms.FormStepMarker>1</Forms.FormStepMarker>
      </Sample>
    </Preview>
  ),
}
export const FormStepHeading: Story = {
  render: () => (
    <Preview>
      <Sample label="Step heading">
        <Forms.FormStepHeading step={1}>Section title</Forms.FormStepHeading>
      </Sample>
    </Preview>
  ),
}
export const FormStepConnector: Story = {
  render: () => (
    <Preview>
      <Sample label="Step connector · geometry">
        <Forms.FormStepConnector />
      </Sample>
    </Preview>
  ),
}
export const FormFieldLabel: Story = {
  render: () => (
    <Preview>
      <Sample label="Persistent label">
        <Forms.FormFieldLabel htmlFor="label-sample">Field label</Forms.FormFieldLabel>
        <input
          className={`
  block h-[32px] w-full min-w-0 rounded-[8px] border-[0.5px] border-input-border bg-inset px-space-12
  py-space-6 text-[14px] leading-[20px] text-text placeholder:text-muted placeholder:opacity-100
  focus:outline-1 focus:outline-offset-1 focus:outline-focus aria-invalid:border-error
  disabled:cursor-not-allowed disabled:opacity-50
`}
          id="label-sample"
        />
      </Sample>
    </Preview>
  ),
}
export const LabeledField: Story = {
  render: () => (
    <Preview>
      <Sample label="Empty">
        <Forms.LabeledField label="Application name" hint="Explain what the field expects." placeholder="Enter a value…" />
      </Sample>
      <Sample label="Filled">
        <Forms.LabeledField label="Filled application name" hint="Supporting information." defaultValue="Application name" />
      </Sample>
      <Sample label="Error">
        <Forms.LabeledField
          label="Invalid application name"
          hint="Use a recognizable name."
          defaultValue=""
          error="A name is required."
        />
      </Sample>
    </Preview>
  ),
}
