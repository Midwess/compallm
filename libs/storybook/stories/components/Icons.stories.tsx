import type { Meta, StoryObj } from '@storybook/react-vite'
import { MagnifyingGlassIcon } from '@phosphor-icons/react/dist/csr/MagnifyingGlass'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import { HardDrivesIcon } from '@phosphor-icons/react/dist/csr/HardDrives'
import { ChatCircleIcon } from '@phosphor-icons/react/dist/csr/ChatCircle'
import { ProviderIcon } from '../../src/components/provider-icons'
import {
  CapabilityIcon,
  CapabilityIndicator,
  capabilityLabels,
  type CapabilityKind,
} from '../../src/components/capability-icon'
import { Preview, Row, Sample } from './Preview'

const meta = { title: 'Components/Icons', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
export const Phosphor: Story = {
  render: () => (
    <Preview>
      {([12, 14, 16, 20] as const).map((size) => (
        <Sample key={size} label={`Phosphor Regular · ${size} px`}>
          <Row>
            <MagnifyingGlassIcon size={size} aria-label="Search" />
            <CaretDownIcon size={size} aria-label="Expand" />
            <PlusIcon size={size} aria-label="Add" />
            <HardDrivesIcon size={size} aria-label="Providers" />
            <ChatCircleIcon size={size} aria-label="Chat" />
          </Row>
        </Sample>
      ))}
    </Preview>
  ),
}

export const Providers: Story = {
  render: () => (
    <Preview>
      {(['openai', 'anthropic', 'gemini', 'deepseek', 'groq', 'mistral'] as const).map((provider) => (
        <Sample key={provider} label={provider}>
          <Row>
            <ProviderIcon provider={provider} className="size-4" />
            <ProviderIcon provider={provider} className="size-6" />
            <ProviderIcon provider={provider} className="size-8" />
          </Row>
        </Sample>
      ))}
    </Preview>
  ),
}

export const Capabilities: Story = {
  render: () => (
    <Preview>
      <Sample label="Capability SVGs · consistent Phosphor Regular artwork" wide>
        <div className="grid grid-cols-1 gap-4 p-3 sm:grid-cols-3">
          {(Object.keys(capabilityLabels) as CapabilityKind[]).map((capability) => (
            <div key={capability} className="flex items-center gap-3">
              <CapabilityIcon capability={capability} className="size-6" />
              <span className="text-xsm">{capabilityLabels[capability]}</span>
            </div>
          ))}
        </div>
      </Sample>
      <Sample label="Model-column size · focus or hover for labels" wide>
        <Row>
          {(Object.keys(capabilityLabels) as CapabilityKind[]).map((capability) => (
            <CapabilityIndicator key={capability} capability={capability} />
          ))}
        </Row>
      </Sample>
    </Preview>
  ),
}
