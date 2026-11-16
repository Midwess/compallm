import type { Meta, StoryObj } from '@storybook/react-vite'
import { OrganizationCatalogPage } from '../../../src/pages/organization'
import { OrganizationDemo } from './organization-demo'

const meta = {
  title: 'Pages/Organization/Catalog',
  component: OrganizationCatalogPage,
  parameters: { layout: 'fullscreen' },
  args: { kind: 'models' },
} satisfies Meta<typeof OrganizationCatalogPage>
export default meta
type Story = StoryObj<typeof meta>
export const Models: Story = {
  render: () => (
    <OrganizationDemo>
      <OrganizationCatalogPage kind="models" />
    </OrganizationDemo>
  ),
}
export const Providers: Story = {
  render: () => (
    <OrganizationDemo>
      <OrganizationCatalogPage kind="providers" />
    </OrganizationDemo>
  ),
}
export const EmptyModels: Story = {
  render: () => (
    <OrganizationDemo empty>
      <OrganizationCatalogPage kind="models" />
    </OrganizationDemo>
  ),
}
export const EmptyProviders: Story = {
  render: () => (
    <OrganizationDemo empty>
      <OrganizationCatalogPage kind="providers" />
    </OrganizationDemo>
  ),
}
