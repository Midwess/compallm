import CompactCheckboxPreview from './Selection-CompactCheckbox.svelte'
import CheckboxOptionPreview from './Selection-CheckboxOption.svelte'
import ChoiceCardPreview from './Selection-ChoiceCard.svelte'
import RadioPreview from './Selection-Radio.svelte'
import CheckboxPreview from './Selection-Checkbox.svelte'
import SwitchPreview from './Selection-Switch.svelte'
import type { Meta, StoryObj } from '@storybook/svelte-vite'
import ComponentGallery from './SelectionDemo.svelte'
const meta = {
  title: 'Components/Selection controls',
  component: ComponentGallery,
  parameters: { layout: 'fullscreen' },
} satisfies Meta<typeof ComponentGallery>
export default meta
type Story = StoryObj<typeof meta>
export const Switch = { render: () => ({ Component: SwitchPreview, props: {} }) }
export const Checkbox = { render: () => ({ Component: CheckboxPreview, props: {} }) }
export const Radio = { render: () => ({ Component: RadioPreview, props: {} }) }
export const ChoiceCard = { render: () => ({ Component: ChoiceCardPreview, props: {} }) }
export const CheckboxOption = { render: () => ({ Component: CheckboxOptionPreview, props: {} }) }
export const CompactCheckbox = { render: () => ({ Component: CompactCheckboxPreview, props: {} }) }
export const TransitionsAndReset: Story = { args: { scenario: 'TransitionsAndReset' } }
export const RestoredInStrictMode: Story = { args: { scenario: 'RestoredInStrictMode' } }
