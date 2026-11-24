import CircularIconButtonPreview from './Buttons-CircularIconButton.svelte'
import ResendPromptPreview from './Buttons-ResendPrompt.svelte'
import AccountPromptPreview from './Buttons-AccountPrompt.svelte'
import TextLinkPreview from './Buttons-TextLink.svelte'
import ContentButtonPreview from './Buttons-ContentButton.svelte'
import ButtonPreview from './Buttons-Button.svelte'
import type { Meta } from '@storybook/svelte-vite'
const meta = {
  title: 'Components/Buttons & links',
  parameters: { layout: 'fullscreen' },
} satisfies Meta
export default meta

export const Button = { render: () => ({ Component: ButtonPreview, props: {} }) }
export const ContentButton = { render: () => ({ Component: ContentButtonPreview, props: {} }) }
export const TextLink = { render: () => ({ Component: TextLinkPreview, props: {} }) }
export const AccountPrompt = { render: () => ({ Component: AccountPromptPreview, props: {} }) }
export const ResendPrompt = { render: () => ({ Component: ResendPromptPreview, props: {} }) }
export const CircularIconButton = { render: () => ({ Component: CircularIconButtonPreview, props: {} }) }
