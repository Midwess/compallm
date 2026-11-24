import CapabilitiesPreview from './Icons-Capabilities.svelte'
import ProvidersPreview from './Icons-Providers.svelte'
import PhosphorPreview from './Icons-Phosphor.svelte'
import type { Meta } from '@storybook/svelte-vite'
const meta = { title: 'Components/Icons', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta

export const Phosphor = { render: () => ({ Component: PhosphorPreview, props: {} }) }
export const Providers = { render: () => ({ Component: ProvidersPreview, props: {} }) }
export const Capabilities = { render: () => ({ Component: CapabilitiesPreview, props: {} }) }
