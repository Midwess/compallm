import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import * as Menus from '../../src/components/dropdowns'
import { Preview, Sample } from './Preview'

const options = [
  { value: 'typescript', label: 'TypeScript' },
  { value: 'rust', label: 'Rust' },
  { value: 'python', label: 'Python' },
  { value: 'unavailable', label: 'Unavailable', disabled: true },
]
const meta = { title: 'Components/Dropdowns', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
function ControlledDropdown() {
  const [value, setValue] = useState('typescript')
  return (
    <>
      <Menus.Dropdown label="Language" options={options} value={value} onValueChange={setValue} />
      <output aria-label="Selected language">{value}</output>
    </>
  )
}
export const Select: Story = {
  render: () => (
    <Preview>
      <Sample label="Custom select">
        <Menus.Select aria-label="Language" options={options} />
      </Sample>
      <Sample label="Disabled">
        <Menus.Select aria-label="Disabled language" options={options} disabled />
      </Sample>
    </Preview>
  ),
}
export const Dropdown: Story = {
  render: () => (
    <Preview>
      <Sample label="Keyboard and pointer selection">
        <ControlledDropdown />
      </Sample>
      <Sample label="Expanded">
        <Menus.Dropdown label="Expanded language" options={options} defaultOpen />
      </Sample>
      <Sample label="Disabled">
        <Menus.Dropdown label="Disabled language" options={options} disabled />
      </Sample>
    </Preview>
  ),
}
export const ThemeMenu: Story = {
  render: () => (
    <Preview>
      <Sample label="Seven theme choices">
        <Menus.ThemeMenu />
      </Sample>
    </Preview>
  ),
}
export const DiagramMenu: Story = {
  render: () => (
    <Preview>
      <Sample label="Four diagram styles">
        <Menus.DiagramMenu />
      </Sample>
    </Preview>
  ),
}
export const DiagramDropdown: Story = {
  render: () => (
    <Preview>
      <Sample label="Diagram style">
        <Menus.DiagramDropdown defaultValue="Hairline" />
      </Sample>
    </Preview>
  ),
}
export const MenuGroupHeading: Story = {
  render: () => (
    <Preview>
      <Sample label="Menu group">
        <Menus.MenuGroupHeading>Integrations</Menus.MenuGroupHeading>
      </Sample>
    </Preview>
  ),
}
export const SelectOption: Story = {
  render: () => (
    <Preview>
      <Sample label="Default and selected">
        <Menus.SelectOption>Default option</Menus.SelectOption>
        <Menus.SelectOption selected>Selected option</Menus.SelectOption>
      </Sample>
    </Preview>
  ),
}
