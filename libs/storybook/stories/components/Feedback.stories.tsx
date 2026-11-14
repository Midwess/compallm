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
        <Feedback.Tooltip.Control content="Add provider">
          <Button aria-label="Add provider">
            <PlusIcon size={14} aria-hidden="true" />
          </Button>
        </Feedback.Tooltip.Control>
      </Sample>
    </Preview>
  ),
}
export const DisabledTooltip: Story = {
  render: () => (
    <Preview>
      <Sample label="Disabled control in a clipped container">
        <div className="overflow-hidden rounded-md border border-border p-2">
          <Feedback.DisabledTooltip disabled reason="The Owner role cannot be edited or deleted.">
            <Button disabled>Delete role</Button>
          </Feedback.DisabledTooltip>
        </div>
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
        <Feedback.Popover.Control label="Display options">
          <h2>Display options</h2>
          <CheckboxOption defaultChecked>Show descriptions</CheckboxOption>
          <CheckboxOption>Show archived items</CheckboxOption>
          <Feedback.HelperText>Changes apply to this sample only.</Feedback.HelperText>
        </Feedback.Popover.Control>
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

export const FieldHelp: Story = {
  render: () => (
    <Preview>
      <Sample label="Contextual field help" wide>
        <span className="inline-flex items-center gap-1 text-sm font-medium">
          Thinking efforts
          <Feedback.FieldHelp label="About thinking efforts">
            Map Low → Max to the thinking-effort values your model accepts (for example, low, medium, high). Higher effort can
            use more time and tokens. Fill levels in order; leave unused levels empty.
          </Feedback.FieldHelp>
        </span>
      </Sample>
    </Preview>
  ),
}

export const ComposedOverlays: Story = {
  render: () => (
    <Preview>
      <Feedback.Tooltip.Root>
        <Feedback.Tooltip.Trigger asChild>
          <Button>Hover or focus</Button>
        </Feedback.Tooltip.Trigger>
        <Feedback.Tooltip.Content>Composed tooltip</Feedback.Tooltip.Content>
      </Feedback.Tooltip.Root>
      <Feedback.Popover.Root>
        <Feedback.Popover.Trigger asChild>
          <Button>Composed popover</Button>
        </Feedback.Popover.Trigger>
        <Feedback.Popover.Content>
          <Feedback.HelperText>Outside clicks and Escape dismiss this popover.</Feedback.HelperText>
        </Feedback.Popover.Content>
      </Feedback.Popover.Root>
    </Preview>
  ),
}
