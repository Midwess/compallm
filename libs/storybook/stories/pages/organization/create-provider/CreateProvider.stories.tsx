import { useEffect, useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { CreateProviderControllerContext } from '../../../../src/context'
import { CreateProviderPage } from '../../../../src/pages/organization/create-provider'
import { OrganizationDemo } from '../organization-demo'
import { DemoCreateProviderController, type Scenario } from './demo-controller'

function Demo({ scenario = 'choose' }: { scenario?: Scenario }) {
  const [intent, setIntent] = useState('')
  const [controller] = useState(() => new DemoCreateProviderController(scenario, setIntent))
  useEffect(() => controller.stop, [controller])
  return (
    <OrganizationDemo>
      <CreateProviderControllerContext value={controller}>
        <CreateProviderPage />
      </CreateProviderControllerContext>
      <output className="sr-only" aria-label="Provider controller intent">
        {intent}
      </output>
    </OrganizationDemo>
  )
}
const meta = { title: 'Pages/organization/create provider', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
export const ChooseProvider: Story = { render: () => <Demo /> }
export const ConfigurePreset: Story = { render: () => <Demo scenario="preset" /> }
export const CustomProvider: Story = { render: () => <Demo scenario="custom" /> }
export const AddModel: Story = { render: () => <Demo scenario="dialog" /> }
export const CheckPassed: Story = { render: () => <Demo scenario="passed" /> }
export const CheckFailed: Story = { render: () => <Demo scenario="failed" /> }
export const Testing: Story = { render: () => <Demo scenario="testing" /> }
export const Submitting: Story = { render: () => <Demo scenario="submitting" /> }
export const NoResults: Story = { render: () => <Demo scenario="no-results" /> }

export const SavedProvider: Story = { render: () => <Demo scenario="saved" /> }
export const SavedCustomProvider: Story = { render: () => <Demo scenario="saved-custom" /> }
export const UnsavedChanges: Story = { render: () => <Demo scenario="changed" /> }
export const CloneProvider: Story = { render: () => <Demo scenario="clone" /> }

export const ConfigureHeaders: Story = { render: () => <Demo scenario="headers" /> }
export const OpenAIResponse: Story = { render: () => <Demo scenario="response-dialog" /> }
export const Anthropic: Story = { render: () => <Demo scenario="anthropic-dialog" /> }
export const EnabledFilter: Story = { render: () => <Demo scenario="filtered" /> }

export const CapabilityFilter: Story = { render: () => <Demo scenario="capabilities" /> }
export const LongModelName: Story = { render: () => <Demo scenario="long-model-name" /> }
