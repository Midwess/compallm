import ResponsiveColumnsPreview from './Primitives-ResponsiveColumns.svelte'
import RichMessageContentPreview from './Primitives-RichMessageContent.svelte'
import ControlsCompositionPreview from './Primitives-ControlsComposition.svelte'
import TableCompositionPreview from './Primitives-TableComposition.svelte'
import PopupCompositionPreview from './Primitives-PopupComposition.svelte'
import Preview from './Preview.svelte'
import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ComponentGallery from './PrimitiveDemo.svelte'
const meta = {
  title: 'Components/Composition primitives',
  component: ComponentGallery,
  parameters: { layout: 'padded' },
  decorators: [() => ({ Component: Preview, props: {} })],
} satisfies Meta<typeof ComponentGallery>
export default meta
type Story = StoryObj<typeof meta>
export const DialogComposition: Story = { args: { scenario: 'DialogComposition' } }
export const AlertComposition: Story = { args: { scenario: 'AlertComposition' } }
export const PopupComposition = { render: () => ({ Component: PopupCompositionPreview, props: {} }) }
export const TableComposition = { render: () => ({ Component: TableCompositionPreview, props: {} }) }
export const CategorizedTable: Story = { args: { scenario: 'CategorizedTable' } }
export const ControlsComposition = { render: () => ({ Component: ControlsCompositionPreview, props: {} }) }
export const RichMessageContent = { render: () => ({ Component: RichMessageContentPreview, props: {} }) }
export const FilterSubmenus: Story = { args: { scenario: 'FilterSubmenus' } }
export const FilterAtContainerEdge: Story = { args: { scenario: 'FilterAtContainerEdge' } }
export const ChildComposition: Story = { args: { scenario: 'ChildComposition' } }
export const ResponsiveColumns = { render: () => ({ Component: ResponsiveColumnsPreview, props: {} }) }
export const DialogRerender: Story = { args: { scenario: 'DialogRerender' } }
