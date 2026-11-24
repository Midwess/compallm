import type { Meta, StoryObj } from '@storybook/svelte-vite'
import Preview from './CreateProvider.stories-Demo.svelte'
const meta = {
  title: 'Pages/organization/create provider',
  parameters: { layout: 'fullscreen' },
  component: Preview,
} satisfies Meta<typeof Preview>
export default meta
type Story = StoryObj<typeof meta>
export const ChooseProvider: Story = { args: {} }
export const ConfigurePreset: Story = { args: { scenario: 'preset' } }
export const CustomProvider: Story = { args: { scenario: 'custom' } }
export const AddModel: Story = { args: { scenario: 'model-add' } }
export const ModelSettings: Story = { args: { scenario: 'model-settings' } }
export const ModelLimitError: Story = { args: { scenario: 'model-limit-error' } }
export const ModelSettingsEmpty: Story = { args: { scenario: 'model-empty' } }
export const ModelPricingDefaults: Story = { args: { scenario: 'model-pricing-defaults' } }
export const CheckPassed: Story = { args: { scenario: 'passed' } }
export const CheckFailed: Story = { args: { scenario: 'failed' } }
export const Testing: Story = { args: { scenario: 'testing' } }
export const Submitting: Story = { args: { scenario: 'submitting' } }
export const NoResults: Story = { args: { scenario: 'no-results' } }
export const SavedProvider: Story = { args: { scenario: 'saved' } }
export const SavedCustomProvider: Story = { args: { scenario: 'saved-custom' } }
export const UnsavedChanges: Story = { args: { scenario: 'changed' } }
export const CloneProvider: Story = { args: { scenario: 'clone' } }
export const ConfigureHeaders: Story = { args: { scenario: 'headers' } }
export const OpenAIResponse: Story = { args: { scenario: 'model-response' } }
export const Anthropic: Story = { args: { scenario: 'model-anthropic' } }
export const EnabledFilter: Story = { args: { scenario: 'filtered' } }
export const CapabilityFilter: Story = { args: { scenario: 'capabilities' } }
export const LongModelName: Story = { args: { scenario: 'long-model-name' } }
export const MissingEndpoint: Story = { args: { scenario: 'missing-endpoint' } }
