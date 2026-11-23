import type { Meta, StoryObj } from '@storybook/svelte-vite'
import { Foundations } from '../src'
import type { ScaffoldCheck$result } from '@compallm/graphql-svelte'
const fixture = { __typename: 'Query' } satisfies ScaffoldCheck$result
const meta = {
  title: 'Foundations/Infrastructure',
  component: Foundations,
  args: { title: `Svelte foundations (${fixture.__typename})` },
} satisfies Meta<typeof Foundations>
export default meta
export const Default: StoryObj<typeof meta> = {}
