import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ComponentGallery from './Collections.svelte'
const meta = {
  title: 'Components/Collections',
  component: ComponentGallery,
  parameters: { layout: 'fullscreen' },
} satisfies Meta<typeof ComponentGallery>
export default meta
type Story = StoryObj<typeof meta>
export const EditableTable: Story = { args: { scenario: 'EditableTable' } }
export const Categories: Story = { args: { scenario: 'Categories' } }
export const VariableHeights: Story = { args: { scenario: 'VariableHeights' } }
export const WindowList: Story = { args: { scenario: 'WindowList' } }
