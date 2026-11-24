import DocumentBodyEditorPreview from './TextFields-DocumentBodyEditor.svelte'
import DocumentTitleEditorPreview from './TextFields-DocumentTitleEditor.svelte'
import DescriptionEditorPreview from './TextFields-DescriptionEditor.svelte'
import InlineSummaryEditorPreview from './TextFields-InlineSummaryEditor.svelte'
import InlineTitleEditorPreview from './TextFields-InlineTitleEditor.svelte'
import GuidanceEditorPreview from './TextFields-GuidanceEditor.svelte'
import CodeEntryPreview from './TextFields-CodeEntry.svelte'
import CodeFieldPreview from './TextFields-CodeField.svelte'
import EmailFieldPreview from './TextFields-EmailField.svelte'
import SearchFieldPreview from './TextFields-SearchField.svelte'
import MultilineFieldPreview from './TextFields-MultilineField.svelte'
import PrefixedTextFieldPreview from './TextFields-PrefixedTextField.svelte'
import UrlPreviewPreview from './TextFields-UrlPreview.svelte'
import TextFieldPreview from './TextFields-TextField.svelte'
import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ComponentGallery from './InvitationRecipientsDemo.svelte'
const meta = {
  title: 'Components/Text fields',
  component: ComponentGallery,
  parameters: { layout: 'fullscreen' },
} satisfies Meta<typeof ComponentGallery>
export default meta
type Story = StoryObj<typeof meta>
export const TextField = { render: () => ({ Component: TextFieldPreview, props: {} }) }
export const UrlPreview = { render: () => ({ Component: UrlPreviewPreview, props: {} }) }
export const PrefixedTextField = { render: () => ({ Component: PrefixedTextFieldPreview, props: {} }) }
export const MultilineField = { render: () => ({ Component: MultilineFieldPreview, props: {} }) }
export const SearchField = { render: () => ({ Component: SearchFieldPreview, props: {} }) }
export const EmailField = { render: () => ({ Component: EmailFieldPreview, props: {} }) }
export const CodeField = { render: () => ({ Component: CodeFieldPreview, props: {} }) }
export const CodeEntry = { render: () => ({ Component: CodeEntryPreview, props: {} }) }
export const GuidanceEditor = { render: () => ({ Component: GuidanceEditorPreview, props: {} }) }
export const InlineTitleEditor = { render: () => ({ Component: InlineTitleEditorPreview, props: {} }) }
export const InlineSummaryEditor = { render: () => ({ Component: InlineSummaryEditorPreview, props: {} }) }
export const DescriptionEditor = { render: () => ({ Component: DescriptionEditorPreview, props: {} }) }
export const DocumentTitleEditor = { render: () => ({ Component: DocumentTitleEditorPreview, props: {} }) }
export const DocumentBodyEditor = { render: () => ({ Component: DocumentBodyEditorPreview, props: {} }) }
export const InvitationRecipients: Story = {}

export const InvitationRecipientsMixedOrder: Story = {
  args: { initial: 'alex@example.com, incomplete; sam@example.com\nsecond-invalid | taylor@example.com' },
}
