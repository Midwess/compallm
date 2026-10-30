import type { Meta, StoryObj } from '@storybook/react-vite'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import * as Buttons from '../../../portal/src/components/ui/buttons'
import { Preview, Row, Sample } from './Preview'

const meta = { title: 'Components/Buttons & links', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
export const Button: Story = { render: () => <Preview><Sample label="Compact · 32 px"><Row>{(['primary', 'neutral', 'ghost', 'destructive', 'link'] as const).map(variant => <Buttons.Button key={variant} variant={variant}>{variant}</Buttons.Button>)}</Row></Sample><Sample label="States"><Row><Buttons.Button variant="primary" loading>Continue</Buttons.Button><Buttons.Button disabled>Disabled</Buttons.Button><Buttons.Button aria-label="Add"><PlusIcon size={14} /></Buttons.Button></Row></Sample><Sample label="Standard · 40 px"><Buttons.Button size="standard">Continue</Buttons.Button></Sample><Sample label="Authentication · 48 px"><Buttons.Button size="authentication" variant="primary">Continue with email</Buttons.Button></Sample></Preview> }
export const ContentButton: Story = { render: () => <Preview><Sample label="Content · 166 × 28"><Buttons.ContentButton>Use custom configuration</Buttons.ContentButton><Buttons.ContentButton disabled>Disabled</Buttons.ContentButton></Sample></Preview> }
export const TextLink: Story = { render: () => <Preview><Sample label="Link tones"><Buttons.TextLink href="#different-email">Use a different email</Buttons.TextLink><Buttons.TextLink tone="muted" href="#help">Learn more</Buttons.TextLink><Buttons.TextLink disabled href="#disabled">Unavailable</Buttons.TextLink></Sample></Preview> }
export const AccountPrompt: Story = { render: () => <Preview><Sample label="Signup"><Buttons.AccountPrompt action={<Buttons.TextLink href="#signin">Sign in</Buttons.TextLink>}>Already have an account?</Buttons.AccountPrompt></Sample><Sample label="Signin"><Buttons.AccountPrompt action={<Buttons.TextLink href="#signup">Sign up</Buttons.TextLink>}>New to Compallm?</Buttons.AccountPrompt></Sample></Preview> }
export const ResendPrompt: Story = { render: () => <Preview><Sample label="Ready"><Buttons.ResendPrompt /></Sample><Sample label="Cooldown · caller supplies remaining time"><Buttons.ResendPrompt remaining={28} /></Sample></Preview> }
