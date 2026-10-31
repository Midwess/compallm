import type { Meta, StoryObj } from '@storybook/react-vite'
import { MainPage } from '../../../src/pages/main'
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
      <section className="p-8 pt-4">
        <h1 className="text-2xl font-semibold">Workspace content</h1>
        <p className="mt-4 text-xs text-muted">Feature pages supply this content through children.</p>
      </section>
    ),
  },
}
export const ScrollableContent: Story = {
  args: {
    sidebar: <SidebarDemo />,
    children: (
      <section className="p-8 pt-4">
        <h1 className="text-2xl font-semibold">Scrollable content</h1>
        <div className="mt-4 grid gap-4">
          {Array.from({ length: 60 }, (_, index) => (
            <p key={index} className="text-xs">
              Example item {index + 1}
            </p>
          ))}
        </div>
      </section>
    ),
  },
}
