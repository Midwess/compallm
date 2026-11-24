import DocumentIconPreview from './Content-DocumentIcon.svelte'
import ChangedFilePreview from './Content-ChangedFile.svelte'
import EditTimestampPreview from './Content-EditTimestamp.svelte'
import MemberRowPreview from './Content-MemberRow.svelte'
import DiffLinePreview from './Content-DiffLine.svelte'
import CodePreviewPreview from './Content-CodePreview.svelte'
import BadgePreview from './Content-Badge.svelte'
import AvatarPreview from './Content-Avatar.svelte'
import SettingsRowPreview from './Content-SettingsRow.svelte'
import SectionHeadingPreview from './Content-SectionHeading.svelte'
import type { Meta } from '@storybook/svelte-vite'
const meta = {
  title: 'Components/Content & code',
  parameters: { layout: 'fullscreen' },
} satisfies Meta
export default meta

export const SectionHeading = { render: () => ({ Component: SectionHeadingPreview, props: {} }) }
export const SettingsRow = { render: () => ({ Component: SettingsRowPreview, props: {} }) }
export const Avatar = { render: () => ({ Component: AvatarPreview, props: {} }) }
export const Badge = { render: () => ({ Component: BadgePreview, props: {} }) }
export const CodePreview = { render: () => ({ Component: CodePreviewPreview, props: {} }) }
export const DiffLine = { render: () => ({ Component: DiffLinePreview, props: {} }) }
export const MemberRow = { render: () => ({ Component: MemberRowPreview, props: {} }) }
export const EditTimestamp = { render: () => ({ Component: EditTimestampPreview, props: {} }) }
export const ChangedFile = { render: () => ({ Component: ChangedFilePreview, props: {} }) }
export const DocumentIcon = { render: () => ({ Component: DocumentIconPreview, props: {} }) }
