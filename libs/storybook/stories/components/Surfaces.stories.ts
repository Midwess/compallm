import SectionSurfacePreview from './Surfaces-SectionSurface.svelte'
import type { Meta } from '@storybook/svelte-vite'
const meta = {
  title: 'Components/Section surfaces',
  parameters: { layout: 'fullscreen' },
} satisfies Meta
export default meta

export const SectionSurface = { render: () => ({ Component: SectionSurfacePreview, props: {} }) }
