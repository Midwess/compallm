import ThemeBalancePreview from './Foundations-ThemeBalance.svelte'
import RuntimeTokensPreview from './Foundations-RuntimeTokens.svelte'
import ColorsPreview from './Foundations-Colors.svelte'
import MappingPreview from './Foundations-Mapping.svelte'
import InteractionColorsPreview from './Foundations-InteractionColors.svelte'
import SpacingPreview from './Foundations-Spacing.svelte'
import TypographyPreview from './Foundations-Typography.svelte'
import type { Meta } from '@storybook/svelte-vite'
const meta = {
  title: 'Components/Foundations',
  parameters: { layout: 'fullscreen' },
} satisfies Meta
export default meta

export const Mapping = { render: () => ({ Component: MappingPreview, props: {} }) }
export const Colors = { render: () => ({ Component: ColorsPreview, props: {} }) }
export const Typography = { render: () => ({ Component: TypographyPreview, props: {} }) }
export const Spacing = { render: () => ({ Component: SpacingPreview, props: {} }) }
export const RuntimeTokens = { render: () => ({ Component: RuntimeTokensPreview, props: {} }) }
export const InteractionColors = { render: () => ({ Component: InteractionColorsPreview, props: {} }) }
export const ThemeBalance = { render: () => ({ Component: ThemeBalancePreview, props: {} }) }
