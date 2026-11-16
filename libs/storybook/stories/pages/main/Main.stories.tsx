import type { Meta, StoryObj } from '@storybook/react-vite'
import { MainPage } from '../../../src/pages/main'
import { PageHeader, Title, Functions } from '../../../src/components/page-header'
import { Button } from '../../../src/components/buttons'
import { SidebarDemo } from '../../components/sidebar-demo'

const meta = { title: 'Pages/main', component: MainPage, parameters: { layout: 'fullscreen' } } satisfies Meta<typeof MainPage>
export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = { args: { sidebar: <SidebarDemo /> } }
export const CollapsedSections: Story = { args: { sidebar: <SidebarDemo defaultOpen={false} /> } }
export const WithContent: Story = {
  args: {
    sidebar: <SidebarDemo initialSelection="#members" />,
    children: (
      <>
        <PageHeader>
          <Title>Workspace content</Title>
          <Functions>
            <Button variant="ghost">Add item</Button>
          </Functions>
        </PageHeader>
        <section className="px-3 py-6">
          <p className="text-xs text-muted-foreground">Feature pages supply this content through children.</p>
        </section>
      </>
    ),
  },
}
export const ScrollableContent: Story = {
  args: {
    sidebar: <SidebarDemo />,
    children: (
      <>
        <PageHeader>
          <Title>Scrollable content</Title>
        </PageHeader>
        <section className="px-3 py-6">
          <div className="grid gap-4">
            {Array.from({ length: 60 }, (_, index) => (
              <p key={index} className="text-xs">
                Example item {index + 1}
              </p>
            ))}
          </div>
        </section>
      </>
    ),
  },
}
