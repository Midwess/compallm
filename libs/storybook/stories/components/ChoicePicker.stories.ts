import type { Meta, StoryObj } from '@storybook/svelte-vite'
import Demo from './ChoicePickerDemo.svelte'
const meta = { title: 'Components/Choice picker', component: Demo } satisfies Meta<typeof Demo>
export default meta
type Story = StoryObj<typeof meta>
export const Default: Story = {}
export const Empty: Story = { args: { empty: true } }
export const Disabled: Story = { args: { disabled: true } }
export const Composition: Story = { args: { composed: true } }
