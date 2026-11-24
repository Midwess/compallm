import type { Meta, StoryObj } from '@storybook/svelte-vite'
import FormPractice from './FormPractice.svelte'
const meta = { title: 'Pages/Form practice', component: FormPractice, parameters: { layout: 'fullscreen' } } satisfies Meta<
  typeof FormPractice
>
export default meta
export const Playground: StoryObj<typeof meta> = {}
