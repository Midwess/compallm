import type { Meta, StoryObj } from '@storybook/react-vite'
import * as Fields from '../../src/components/fields'
import { Preview, Sample } from './Preview'

const meta = { title: 'Components/Text fields', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>

export const TextField: Story = {
  render: () => (
    <Preview>
      <Sample label="Empty">
        <Fields.TextField aria-label="Empty text" placeholder="Enter a value…" />
      </Sample>
      <Sample label="Filled">
        <Fields.TextField aria-label="Filled text" defaultValue="Application name" />
      </Sample>
      <Sample label="Error">
        <Fields.TextField
          aria-label="Invalid text"
          aria-invalid="true"
          defaultValue="Invalid value"
          aria-describedby="text-error"
        />
        <p id="text-error" className="text-[13px] leading-[20px] text-error">
          Check this value.
        </p>
      </Sample>
      <Sample label="Disabled">
        <Fields.TextField aria-label="Disabled text" disabled placeholder="Enter a value…" />
      </Sample>
    </Preview>
  ),
}
export const MultilineField: Story = {
  render: () => (
    <Preview>
      <Sample label="Multiline">
        <Fields.MultilineField aria-label="Description" placeholder="Write a description…" />
      </Sample>
      <Sample label="Filled">
        <Fields.MultilineField aria-label="Filled description" defaultValue="A shared workspace for your team." />
      </Sample>
      <Sample label="Disabled">
        <Fields.MultilineField aria-label="Disabled description" disabled />
      </Sample>
    </Preview>
  ),
}
export const SearchField: Story = {
  render: () => (
    <Preview>
      <Sample label="Search">
        <Fields.SearchField aria-label="Search" placeholder="Search…" />
      </Sample>
      <Sample label="Disabled">
        <Fields.SearchField aria-label="Disabled search" disabled placeholder="Search…" />
      </Sample>
    </Preview>
  ),
}
export const EmailField: Story = {
  render: () => (
    <Preview>
      <Sample label="Authentication · 48 px">
        <Fields.EmailField aria-label="Email address" placeholder="you@example.com" />
      </Sample>
      <Sample label="Filled">
        <Fields.EmailField aria-label="Filled email" defaultValue="alex@example.com" />
      </Sample>
      <Sample label="Disabled">
        <Fields.EmailField aria-label="Disabled email" disabled />
      </Sample>
    </Preview>
  ),
}
export const CodeField: Story = {
  render: () => (
    <Preview>
      <Sample label="Code digit · 52 × 56">
        <Fields.CodeField aria-label="Code digit" />
      </Sample>
      <Sample label="Filled">
        <Fields.CodeField aria-label="Filled digit" defaultValue="4" />
      </Sample>
    </Preview>
  ),
}
export const CodeEntry: Story = {
  render: () => (
    <Preview>
      <Sample label="Six digits · paste and autofill">
        <Fields.CodeEntry aria-label="Verification code" />
      </Sample>
      <Sample label="Filled">
        <Fields.CodeEntry aria-label="Filled code" defaultValue="123456" />
      </Sample>
      <Sample label="Error">
        <Fields.CodeEntry aria-label="Invalid code" aria-invalid={true} defaultValue="654321" />
      </Sample>
      <Sample label="Disabled">
        <Fields.CodeEntry aria-label="Disabled code" disabled />
      </Sample>
    </Preview>
  ),
}
export const GuidanceEditor: Story = {
  render: () => (
    <Preview>
      <Sample label="Guidance" wide>
        <Fields.GuidanceEditor aria-label="Personal guidance" placeholder="Enter personal guidance (optional)…" />
      </Sample>
    </Preview>
  ),
}
export const InlineTitleEditor: Story = {
  render: () => (
    <Preview>
      <Sample label="Inline title" wide>
        <Fields.InlineTitleEditor aria-label="Title" defaultValue="Compallm" />
      </Sample>
    </Preview>
  ),
}
export const InlineSummaryEditor: Story = {
  render: () => (
    <Preview>
      <Sample label="Inline summary" wide>
        <Fields.InlineSummaryEditor aria-label="Summary" placeholder="Add a short summary…" />
      </Sample>
    </Preview>
  ),
}
export const DescriptionEditor: Story = {
  render: () => (
    <Preview>
      <Sample label="Description" wide>
        <Fields.DescriptionEditor aria-label="Description" placeholder="Write a description…" />
      </Sample>
    </Preview>
  ),
}
export const DocumentTitleEditor: Story = {
  render: () => (
    <Preview>
      <Sample label="Document title" wide>
        <Fields.DocumentTitleEditor aria-label="Document title" defaultValue="Untitled template" />
      </Sample>
    </Preview>
  ),
}
export const DocumentBodyEditor: Story = {
  render: () => (
    <Preview>
      <Sample label="Document body" wide>
        <Fields.DocumentBodyEditor aria-label="Document body" placeholder="Start writing your document…" />
      </Sample>
    </Preview>
  ),
}
