import type { Meta, StoryObj } from '@storybook/svelte-vite'
import Demo from './ProviderControlsDemo.svelte'
const meta = { title: 'Components/Provider controls', component: Demo, parameters: { layout: 'fullscreen' } } satisfies Meta<
  typeof Demo
>
export default meta
type Story = StoryObj<typeof meta>
export const Headers: Story = { args: { scenario: 'Headers' } }
export const EmptyHeaders: Story = { args: { scenario: 'EmptyHeaders' } }
export const Efforts: Story = { args: { scenario: 'Efforts' } }
export const Avatar: Story = { args: { scenario: 'Avatar' } }
export const CompactEfforts: Story = { args: { scenario: 'CompactEfforts' } }
export const TokenCapacityRuler: Story = { args: { scenario: 'TokenCapacityRuler' } }
