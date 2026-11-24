import PlainFallbackPreview from './CodeBlock-PlainFallback.svelte'
import LightPreview from './CodeBlock-Light.svelte'
import ReferencePreview from './CodeBlock-Reference.svelte'
import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ComponentGallery from './CedarEditorDemo.svelte'
const meta = {
  title: 'Components/Code block',
  component: ComponentGallery,
  parameters: { layout: 'padded' },
} satisfies Meta<typeof ComponentGallery>
export default meta
type Story = StoryObj<typeof meta>
export const Reference = { render: () => ({ Component: ReferencePreview, props: {} }) }
export const EditableCedar: Story = { args: { scenario: 'EditableCedar' } }
export const ShortCedar: Story = { args: { scenario: 'ShortCedar' } }
export const LongCedar: Story = { args: { scenario: 'LongCedar' } }
export const Light = { render: () => ({ Component: LightPreview, props: {} }) }
export const PlainFallback = { render: () => ({ Component: PlainFallbackPreview, props: {} }) }
