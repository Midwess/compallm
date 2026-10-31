import type { Meta, StoryObj } from '@storybook/react-vite'
import { SectionSurface as Surface } from '../../src/components/surfaces'
import { Preview, Sample } from './Preview'

const meta = { title: 'Components/Section surfaces', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
export const SectionSurface: Story = {
  render: () => (
    <Preview>
      <Sample label="Neutral section · 10 px corners" wide>
        <Surface>
          <p>Composable section content</p>
          <p className="text-[12px] leading-[18px] text-muted">Shared neutral background and token-based padding.</p>
        </Surface>
      </Sample>
    </Preview>
  ),
}
