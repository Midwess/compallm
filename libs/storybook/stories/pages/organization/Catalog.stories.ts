import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ModelsPreview from './Catalog.stories-StoryModels.svelte'
import ProvidersPreview from './Catalog.stories-StoryProviders.svelte'
import EmptyModelsPreview from './Catalog.stories-StoryEmptyModels.svelte'
import EmptyProvidersPreview from './Catalog.stories-StoryEmptyProviders.svelte'
const meta = {
  component: ModelsPreview,
  title: 'Pages/Organization/Catalog',
  parameters: { layout: 'fullscreen' },
} satisfies Meta<typeof ModelsPreview>
export default meta
type Story = StoryObj<typeof meta>
export const Models: Story = { render: () => ({ Component: ModelsPreview }) }
export const Providers: Story = { render: () => ({ Component: ProvidersPreview }) }
export const EmptyModels: Story = { render: () => ({ Component: EmptyModelsPreview }) }
export const EmptyProviders: Story = { render: () => ({ Component: EmptyProvidersPreview }) }
