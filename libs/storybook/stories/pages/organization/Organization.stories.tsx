import type { Meta, StoryObj } from '@storybook/react-vite'
import { OrganizationDemo } from './organization-demo'

const meta = { title: 'Pages/organization', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
export const Default: Story = { render: () => <OrganizationDemo /> }
export const Empty: Story = { render: () => <OrganizationDemo empty /> }
