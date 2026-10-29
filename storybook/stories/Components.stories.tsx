import type { Meta, StoryObj } from '@storybook/react-vite'
import { Button } from '../../portal/src/components/ui/Button'
import { TextField } from '../../portal/src/components/ui/TextField'
import { Panel } from '../../portal/src/components/ui/Panel'
import { EmptyState } from '../../portal/src/components/ui/EmptyState'

const meta = {
  title: 'Components/Button',
  component: Button,
  args: { children: 'Create notebook' },
  argTypes: { variant: { control: 'select', options: ['primary', 'secondary', 'ghost'] } },
  decorators: [(Story) => <div className="design-system" style={{ padding: 24 }}><Story /></div>],
} satisfies Meta<typeof Button>
export default meta
type Story = StoryObj<typeof meta>
export const Primary: Story = {}
export const Secondary: Story = { args: { variant: 'secondary' } }
export const Ghost: Story = { args: { variant: 'ghost' } }
export const Disabled: Story = { args: { disabled: true } }
export const Working: Story = { args: { children: 'Creating…', disabled: true, 'aria-busy': true } }

export const Composition: Story = {
  render: () => (
    <Panel className="ui-stack" style={{ maxWidth: 520 }}>
      <TextField label="Notebook name" placeholder="What are you learning?" hint="A short name makes it easier to find later." />
      <TextField label="Email address" type="email" defaultValue="invalid" error="Enter a valid email address." />
      <Button>Create notebook</Button>
      <EmptyState title="A place for your ideas" description="Add your first source when you're ready." />
    </Panel>
  ),
}
