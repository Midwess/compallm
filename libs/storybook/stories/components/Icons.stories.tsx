import type { Meta, StoryObj } from '@storybook/react-vite'
import { MagnifyingGlassIcon } from '@phosphor-icons/react/dist/csr/MagnifyingGlass'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import { HardDrivesIcon } from '@phosphor-icons/react/dist/csr/HardDrives'
import { ChatCircleIcon } from '@phosphor-icons/react/dist/csr/ChatCircle'
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
