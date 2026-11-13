import type { Meta, StoryObj } from '@storybook/react-vite'
import { FormPractice } from './FormPractice'

const meta = {
  title: 'Pages/Form practice',
  component: FormPractice,
  parameters: { layout: 'fullscreen' },
} satisfies Meta<typeof FormPractice>
export default meta
type Story = StoryObj<typeof meta>
export const Playground: Story = {}
