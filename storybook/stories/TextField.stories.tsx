import type { Meta, StoryObj } from '@storybook/react-vite'
import { TextField } from '../../portal/src/components/ui/TextField'

const meta = {
  title: 'Components/Text field',
  component: TextField,
  args: { label: 'Notebook name', placeholder: 'Research notes', hint: 'Choose a name you will recognize.' },
  decorators: [(Story) => <div className="design-system" style={{ maxWidth: 480, padding: 24 }}><Story /></div>],
} satisfies Meta<typeof TextField>
export default meta
type Story = StoryObj<typeof meta>
export const Default: Story = {}
export const Invalid: Story = { args: { error: 'Enter a notebook name.', hint: undefined, required: true } }
export const Disabled: Story = { args: { disabled: true, defaultValue: 'Research notes' } }
