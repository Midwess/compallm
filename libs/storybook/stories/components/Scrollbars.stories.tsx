import type { Meta, StoryObj } from '@storybook/react-vite'
import { SettingsPageTitle } from '../../src/components/settings'
import { MultilineField } from '../../src/components/fields'

const meta = { title: 'Components/Scrollbars', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>

export const Universal: Story = {
  render: () => (
    <div className="grid max-w-208 gap-6 p-6">
      <SettingsPageTitle>Native scrollbars</SettingsPageTitle>
      <p className="text-[13px] text-muted">The shared stylesheet styles every scroll container, including Safari.</p>
      <div className="grid gap-4 sm:grid-cols-2">
        <div
          tabIndex={0}
          role="region"
          aria-label="Vertical scrolling example"
          className="h-48 overflow-auto rounded-lg bg-inset p-4"
        >
          {Array.from({ length: 18 }, (_, i) => (
            <p key={i} className="py-2 text-[13px]">
              Scrollable item {i + 1}
            </p>
          ))}
        </div>
        <div
          tabIndex={0}
          role="region"
          aria-label="Both directions scrolling example"
          className="h-48 overflow-auto rounded-lg bg-inset p-4"
        >
          <div className="h-96 w-160 text-[13px]">Horizontal and vertical scrolling share the same transparent track.</div>
        </div>
      </div>
      <MultilineField
        aria-label="Scrollable native field"
        className="h-32 resize-none text-[13px]"
        defaultValue={Array.from({ length: 16 }, (_, i) => `Native textarea line ${i + 1}`).join('\n')}
      />
    </div>
  ),
}
