import type { Meta, StoryObj } from '@storybook/svelte-vite'
import Preview from './organization-demo-OrganizationDemo.svelte'
const meta = { title: 'Pages/organization', parameters: { layout: 'fullscreen' }, component: Preview } satisfies Meta<
  typeof Preview
>
export default meta
type Story = StoryObj<typeof meta>
export const Default: Story = { args: {} }
export const Empty: Story = { args: { empty: true } }
export const ReorderedSidebar: Story = { args: { sidebarOrder: ['PLUGINS', 'NOTEBOOKS', 'MODELS', 'PROVIDERS'] } }
export const CategoriesAndItems: Story = {
  args: {
    sidebarLayout: [
      { category: 'ORGANIZATION', items: ['PLUGINS', 'MEMBERS'] },
      { category: 'WORKSPACE', items: ['NOTEBOOKS', 'MODELS', 'PROVIDERS'] },
      { category: 'CONVERSATIONS', items: ['CONVERSATIONS'] },
      { category: 'ROUTERS', items: ['API_KEYS', 'STATS'] },
    ],
  },
}
export const EmptyCategory: Story = {
  args: {
    sidebarLayout: [
      { category: 'WORKSPACE', items: ['PROVIDERS', 'MODELS', 'NOTEBOOKS', 'PLUGINS', 'STATS', 'API_KEYS', 'MEMBERS'] },
      { category: 'ROUTERS', items: [] },
      { category: 'ORGANIZATION', items: [] },
      { category: 'CONVERSATIONS', items: ['CONVERSATIONS'] },
    ],
  },
}
export const SidebarLoading: Story = { args: { sidebarLoading: true } }
