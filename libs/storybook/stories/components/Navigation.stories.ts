import DragHandlePreview from './Navigation-DragHandle.svelte'
import ToolbarActionPreview from './Navigation-ToolbarAction.svelte'
import AgentDockPreview from './Navigation-AgentDock.svelte'
import ViewPillPreview from './Navigation-ViewPill.svelte'
import BackBreadcrumbPreview from './Navigation-BackBreadcrumb.svelte'
import BreadcrumbTrailPreview from './Navigation-BreadcrumbTrail.svelte'
import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ComponentGallery from './NavigationTabDemo.svelte'
const meta = {
  title: 'Components/Navigation controls',
  component: ComponentGallery,
  parameters: { layout: 'fullscreen' },
} satisfies Meta<typeof ComponentGallery>
export default meta
type Story = StoryObj<typeof meta>
export const BreadcrumbTrail = { render: () => ({ Component: BreadcrumbTrailPreview, props: {} }) }
export const BackBreadcrumb = { render: () => ({ Component: BackBreadcrumbPreview, props: {} }) }
export const ViewPill = { render: () => ({ Component: ViewPillPreview, props: {} }) }
export const AgentDock = { render: () => ({ Component: AgentDockPreview, props: {} }) }
export const NavigationTab: Story = {}
export const ToolbarAction = { render: () => ({ Component: ToolbarActionPreview, props: {} }) }
export const DragHandle = { render: () => ({ Component: DragHandlePreview, props: {} }) }
