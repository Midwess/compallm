import type { Meta, StoryObj } from '@storybook/svelte-vite'
import DefaultPreview from './Main.stories-StoryDefault.svelte'
import CollapsedSectionsPreview from './Main.stories-StoryCollapsedSections.svelte'
import WithContentPreview from './Main.stories-StoryWithContent.svelte'
import ScrollableContentPreview from './Main.stories-StoryScrollableContent.svelte'
const meta = { component: DefaultPreview, title: 'Pages/main', parameters: { layout: 'fullscreen' } } satisfies Meta<
  typeof DefaultPreview
>
export default meta
type Story = StoryObj<typeof meta>
export const Default: Story = { render: () => ({ Component: DefaultPreview }) }
export const CollapsedSections: Story = { render: () => ({ Component: CollapsedSectionsPreview }) }
export const WithContent: Story = { render: () => ({ Component: WithContentPreview }) }
export const ScrollableContent: Story = { render: () => ({ Component: ScrollableContentPreview }) }
