import CompositionPreview from './Settings-Composition.svelte'
import DividerPreview from './Settings-Divider.svelte'
import SectionHeaderPreview from './Settings-SectionHeader.svelte'
import PageTitlePreview from './Settings-PageTitle.svelte'
import type { Meta } from '@storybook/svelte-vite'
const meta = {
  title: 'Components/Settings composition',
  parameters: { layout: 'fullscreen' },
} satisfies Meta
export default meta

export const PageTitle = { render: () => ({ Component: PageTitlePreview, props: {} }) }
export const SectionHeader = { render: () => ({ Component: SectionHeaderPreview, props: {} }) }
export const Divider = { render: () => ({ Component: DividerPreview, props: {} }) }
export const Composition = { render: () => ({ Component: CompositionPreview, props: {} }) }
