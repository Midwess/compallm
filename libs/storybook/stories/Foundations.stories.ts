import type { Meta, StoryObj } from '@storybook/svelte-vite'
import { Foundations } from '../src'
const meta = {
  title: 'Foundations/Infrastructure',
  component: Foundations,
  args: { title: 'Svelte foundations' },
} satisfies Meta<typeof Foundations>
export default meta
export const Default: StoryObj<typeof meta> = {}
