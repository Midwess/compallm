import type { Meta, StoryObj } from '@storybook/react-vite'
import { OrganizationDemo } from './organization-demo'

const meta = { title: 'Pages/organization', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
export const Default: Story = { render: () => <OrganizationDemo /> }
export const Empty: Story = { render: () => <OrganizationDemo empty /> }

export const ReorderedSidebar: Story = {
  render: () => <OrganizationDemo sidebarOrder={['PLUGINS', 'NOTEBOOKS', 'MODELS', 'PROVIDERS']} />,
}

export const CategoriesAndItems: Story = {
  render: () => (
    <OrganizationDemo
      sidebarLayout={[
        { category: 'ORGANIZATION', items: ['PLUGINS', 'MEMBERS'] },
        { category: 'WORKSPACE', items: ['NOTEBOOKS', 'MODELS', 'PROVIDERS'] },
        { category: 'CONVERSATIONS', items: ['CONVERSATIONS'] },
        { category: 'ROUTERS', items: ['API_KEYS', 'STATS'] },
      ]}
    />
  ),
}

export const EmptyCategory: Story = {
  render: () => (
    <OrganizationDemo
      sidebarLayout={[
        { category: 'WORKSPACE', items: ['PROVIDERS', 'MODELS', 'NOTEBOOKS', 'PLUGINS', 'STATS', 'API_KEYS', 'MEMBERS'] },
        { category: 'ROUTERS', items: [] },
        { category: 'ORGANIZATION', items: [] },
        { category: 'CONVERSATIONS', items: ['CONVERSATIONS'] },
      ]}
    />
  ),
}
