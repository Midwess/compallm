import type { Meta, StoryObj } from '@storybook/react-vite'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import * as Feedback from '../../src/components/feedback'
import { Button } from '../../src/components/buttons'
import { CheckboxOption } from '../../src/components/selection'
import { Preview, Sample } from './Preview'

const meta = { title: 'Components/Feedback & overlays', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
export const HelperText: Story = {
  render: () => (
    <Preview>
      <Sample label="Supporting text">
        <Feedback.HelperText>Supporting information for this field.</Feedback.HelperText>
      </Sample>
    </Preview>
  ),
}
export const InlineError: Story = {
  render: () => (
    <Preview>
      <Sample label="Inline feedback">
        <Feedback.InlineError>This value needs attention. Check it and try again.</Feedback.InlineError>
      </Sample>
    </Preview>
  ),
}
export const Tooltip: Story = {
  render: () => (
    <Preview>
      <Sample label="Hover or keyboard focus">
        <Feedback.Tooltip content="Add provider">
          <Button aria-label="Add provider">
            <PlusIcon size={14} aria-hidden="true" />
          </Button>
        </Feedback.Tooltip>
      </Sample>
    </Preview>
  ),
}
export const KeyboardKey: Story = {
  render: () => (
    <Preview>
      <Sample label="Keyboard hint">
        <Feedback.KeyboardKey>⌥</Feedback.KeyboardKey>
        <Feedback.KeyboardKey>⌘ K</Feedback.KeyboardKey>
      </Sample>
    </Preview>
  ),
}
export const EmptyState: Story = {
  render: () => (
    <Preview>
      <Sample label="Empty state" wide>
        <Feedback.EmptyState title="No items yet">New items will appear here when they are available.</Feedback.EmptyState>
      </Sample>
    </Preview>
  ),
}
export const Callout: Story = {
  render: () => (
    <Preview>
      <Sample label="Contextual feedback" wide>
        <Feedback.Callout title="Access is disabled" action={<Button variant="link">Configure →</Button>}>
          A workspace admin can configure access.
        </Feedback.Callout>
      </Sample>
    </Preview>
  ),
}
export const Popover: Story = {
  render: () => (
    <Preview>
      <Sample label="Local display controls">
        <Feedback.Popover label="Display options">
          <h2>Display options</h2>
          <CheckboxOption defaultChecked>Show descriptions</CheckboxOption>
          <CheckboxOption>Show archived items</CheckboxOption>
          <Feedback.HelperText>Changes apply to this sample only.</Feedback.HelperText>
        </Feedback.Popover>
      </Sample>
    </Preview>
  ),
}
export const CodeError: Story = {
  render: () => (
    <Preview>
      <Sample label="Code feedback">
        <Feedback.CodeError />
      </Sample>
    </Preview>
  ),
}
export const EmailRegistrationError: Story = {
  render: () => (
    <Preview>
      <Sample label="Registration feedback">
        <Feedback.EmailRegistrationError />
      </Sample>
    </Preview>
  ),
}
export const DropInsertionIndicator: Story = {
  render: () => (
    <Preview>
      <Sample label="Insertion position · 2 px">
        <Feedback.DropInsertionIndicator />
      </Sample>
    </Preview>
  ),
}
