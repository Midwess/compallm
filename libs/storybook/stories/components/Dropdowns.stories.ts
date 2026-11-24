import SelectOptionPreview from './Dropdowns-SelectOption.svelte'
import MenuGroupHeadingPreview from './Dropdowns-MenuGroupHeading.svelte'
import DiagramDropdownPreview from './Dropdowns-DiagramDropdown.svelte'
import DiagramMenuPreview from './Dropdowns-DiagramMenu.svelte'
import ThemeMenuPreview from './Dropdowns-ThemeMenu.svelte'
import SelectPreview from './Dropdowns-Select.svelte'
import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ComponentGallery from './DropdownDemo.svelte'
const meta = {
  title: 'Components/Dropdowns',
  component: ComponentGallery,
  parameters: { layout: 'fullscreen' },
} satisfies Meta<typeof ComponentGallery>
export default meta
type Story = StoryObj<typeof meta>
export const SelectComposition: Story = { args: { scenario: 'SelectComposition' } }
export const SelectInDialog: Story = { args: { scenario: 'SelectInDialog' } }
export const Select = { render: () => ({ Component: SelectPreview, props: {} }) }
export const Dropdown: Story = { args: { scenario: 'Dropdown' } }
export const ThemeMenu = { render: () => ({ Component: ThemeMenuPreview, props: {} }) }
export const DiagramMenu = { render: () => ({ Component: DiagramMenuPreview, props: {} }) }
export const DiagramDropdown = { render: () => ({ Component: DiagramDropdownPreview, props: {} }) }
export const MenuGroupHeading = { render: () => ({ Component: MenuGroupHeadingPreview, props: {} }) }
export const SelectOption = { render: () => ({ Component: SelectOptionPreview, props: {} }) }
export const RepeatedSelection: Story = { args: { scenario: 'RepeatedSelection' } }
