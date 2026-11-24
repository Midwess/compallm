import ComposedOverlaysPreview from './Feedback-ComposedOverlays.svelte'
import FieldHelpPreview from './Feedback-FieldHelp.svelte'
import DropInsertionIndicatorPreview from './Feedback-DropInsertionIndicator.svelte'
import EmailRegistrationErrorPreview from './Feedback-EmailRegistrationError.svelte'
import CodeErrorPreview from './Feedback-CodeError.svelte'
import PopoverPreview from './Feedback-Popover.svelte'
import CalloutPreview from './Feedback-Callout.svelte'
import EmptyStatePreview from './Feedback-EmptyState.svelte'
import KeyboardKeyPreview from './Feedback-KeyboardKey.svelte'
import DisabledTooltipPreview from './Feedback-DisabledTooltip.svelte'
import TooltipPreview from './Feedback-Tooltip.svelte'
import InlineErrorPreview from './Feedback-InlineError.svelte'
import HelperTextPreview from './Feedback-HelperText.svelte'
import type { Meta } from '@storybook/svelte-vite'
const meta = {
  title: 'Components/Feedback & overlays',
  parameters: { layout: 'fullscreen' },
} satisfies Meta
export default meta

export const HelperText = { render: () => ({ Component: HelperTextPreview, props: {} }) }
export const InlineError = { render: () => ({ Component: InlineErrorPreview, props: {} }) }
export const Tooltip = { render: () => ({ Component: TooltipPreview, props: {} }) }
export const DisabledTooltip = { render: () => ({ Component: DisabledTooltipPreview, props: {} }) }
export const KeyboardKey = { render: () => ({ Component: KeyboardKeyPreview, props: {} }) }
export const EmptyState = { render: () => ({ Component: EmptyStatePreview, props: {} }) }
export const Callout = { render: () => ({ Component: CalloutPreview, props: {} }) }
export const Popover = { render: () => ({ Component: PopoverPreview, props: {} }) }
export const CodeError = { render: () => ({ Component: CodeErrorPreview, props: {} }) }
export const EmailRegistrationError = { render: () => ({ Component: EmailRegistrationErrorPreview, props: {} }) }
export const DropInsertionIndicator = { render: () => ({ Component: DropInsertionIndicatorPreview, props: {} }) }
export const FieldHelp = { render: () => ({ Component: FieldHelpPreview, props: {} }) }
export const ComposedOverlays = { render: () => ({ Component: ComposedOverlaysPreview, props: {} }) }
