import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ComponentGallery from './LoadingDemo.svelte'
const meta = { title: 'Components/Loading', component: ComponentGallery, parameters: { layout: 'fullscreen' } } satisfies Meta<
  typeof ComponentGallery
>
export default meta
type Story = StoryObj<typeof meta>
export const FirstLoad: Story = { args: { scenario: 'FirstLoad' } }
export const Loaded: Story = { args: { scenario: 'Loaded' } }
export const Refreshing: Story = { args: { scenario: 'Refreshing' } }
export const RecoverableError: Story = { args: { scenario: 'RecoverableError' } }
export const Empty: Story = { args: { scenario: 'Empty' } }
