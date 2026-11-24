import type { Meta, StoryObj } from '@storybook/svelte-vite'
import FormDemo from './FormDemo.svelte'
import ValidationExample from './ValidationExample.svelte'
const meta = { title: 'Components/Form', component: FormDemo, parameters: { layout: 'padded' } } satisfies Meta<typeof FormDemo>
export default meta
type Story = StoryObj<typeof meta>
export const ButtonOnly: Story = {}
export const CommitOnBlur: Story = { args: { mode: 'blur' } }
export const CommitOnChange: Story = { args: { mode: 'change' } }
export const Debounced: Story = { args: { mode: 'debounce' } }
export const Rejected: Story = { args: { mode: 'blur', reject: true } }
export const ValidationFeedback = { render: () => ({ Component: ValidationExample, props: {} }) }
export const ValidationAtRightEdge = { render: () => ({ Component: ValidationExample, props: { edge: true } }) }
