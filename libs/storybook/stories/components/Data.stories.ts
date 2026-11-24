import PropertyRowPreview from './Data-PropertyRow.svelte'
import PropertyChipPreview from './Data-PropertyChip.svelte'
import ActivityEventPreview from './Data-ActivityEvent.svelte'
import MilestoneAddActionPreview from './Data-MilestoneAddAction.svelte'
import ResourceActionPreview from './Data-ResourceAction.svelte'
import ProjectProgressPreview from './Data-ProjectProgress.svelte'
import ProjectHealthPreview from './Data-ProjectHealth.svelte'
import ProjectIconPreview from './Data-ProjectIcon.svelte'
import ProjectTableRowPreview from './Data-ProjectTableRow.svelte'
import ProjectTableHeaderPreview from './Data-ProjectTableHeader.svelte'
import type { Meta } from '@storybook/svelte-vite'
const meta = {
  title: 'Components/Data primitives',
  parameters: { layout: 'fullscreen' },
} satisfies Meta
export default meta

export const ProjectTableHeader = { render: () => ({ Component: ProjectTableHeaderPreview, props: {} }) }
export const ProjectTableRow = { render: () => ({ Component: ProjectTableRowPreview, props: {} }) }
export const ProjectIcon = { render: () => ({ Component: ProjectIconPreview, props: {} }) }
export const ProjectHealth = { render: () => ({ Component: ProjectHealthPreview, props: {} }) }
export const ProjectProgress = { render: () => ({ Component: ProjectProgressPreview, props: {} }) }
export const ResourceAction = { render: () => ({ Component: ResourceActionPreview, props: {} }) }
export const MilestoneAddAction = { render: () => ({ Component: MilestoneAddActionPreview, props: {} }) }
export const ActivityEvent = { render: () => ({ Component: ActivityEventPreview, props: {} }) }
export const PropertyChip = { render: () => ({ Component: PropertyChipPreview, props: {} }) }
export const PropertyRow = { render: () => ({ Component: PropertyRowPreview, props: {} }) }
