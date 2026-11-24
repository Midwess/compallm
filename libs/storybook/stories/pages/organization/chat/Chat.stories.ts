import type { Meta, StoryObj } from '@storybook/svelte-vite'
import Preview from './Chat.stories-Demo.svelte'
const meta = { title: 'Pages/organization/chat', parameters: { layout: 'fullscreen' }, component: Preview } satisfies Meta<
  typeof Preview
>
export default meta
type Story = StoryObj<typeof meta>
export const NewChat: Story = { args: {} }
export const Creating: Story = { args: { creating: true } }
export const NoModels: Story = { args: { noModels: true } }
