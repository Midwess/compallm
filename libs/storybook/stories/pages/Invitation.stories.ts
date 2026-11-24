import type { Meta, StoryObj } from '@storybook/svelte-vite'
import Preview from './Invitation.stories-Demo.svelte'
const meta = { title: 'Pages/Invitation', parameters: { layout: 'fullscreen' }, component: Preview } satisfies Meta<
  typeof Preview
>
export default meta
type Story = StoryObj<typeof meta>
export const Continue: Story = { args: {} }
export const VerifyEmail: Story = { args: { step: 'code' } }
export const Unavailable: Story = { args: { unavailable: true } }
export const Loading: Story = { args: { loading: true } }
export const IncorrectCode: Story = { args: { step: 'code', error: 'Incorrect verification code' } }
export const CheckingCode: Story = { args: { step: 'code', busy: true } }
export const Confirmed: Story = { args: { step: 'confirmed' } }
