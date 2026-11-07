import type { Meta, StoryObj } from '@storybook/react-vite'
import { SettingsDivider, SettingsPageTitle, SettingsSectionHeader } from '../../src/components/settings'
import { SettingsSectionDescription, SettingsSectionTitle } from '../../src/components/settings'
import { SettingsRow } from '../../src/components/content'
import { SectionSurface } from '../../src/components/surfaces'
import { TextField } from '../../src/components/fields'
import { Switch } from '../../src/components/selection'
import { Preview } from './Preview'
import { penpotFoundationUrl } from './foundation-mapping'

const source = penpotFoundationUrl('5c696641-0e0d-80da-8008-e74e57be1fd7')

const meta = {
  title: 'Components/Settings composition',
  parameters: {
    layout: 'fullscreen',
    docs: {
      description: {
        component:
          `[Penpot · Settings composition](${source}). ` +
          'Reusable primitives only; consumers own settings state and actions.',
      },
    },
  },
} satisfies Meta
export default meta
type Story = StoryObj<typeof meta>

export const PageTitle: Story = {
  render: () => (
    <Preview>
      <div className="w-full max-w-160 px-4">
        <SettingsPageTitle>Settings page title</SettingsPageTitle>
      </div>
    </Preview>
  ),
}

export const SectionHeader: Story = {
  render: () => (
    <Preview>
      <div className="grid w-full max-w-160 gap-12 px-4">
        <SettingsSectionHeader>
          <SettingsSectionTitle>Section heading</SettingsSectionTitle>
          <SettingsSectionDescription>Helpful context for this section.</SettingsSectionDescription>
        </SettingsSectionHeader>
        <SettingsSectionHeader>
          <SettingsSectionTitle>Heading without a description</SettingsSectionTitle>
        </SettingsSectionHeader>
      </div>
    </Preview>
  ),
}

export const Divider: Story = {
  render: () => (
    <Preview>
      <div className="w-full max-w-160 rounded-[10px] bg-inset p-4">
        <SettingsDivider />
      </div>
    </Preview>
  ),
}

export const Composition: Story = {
  render: () => (
    <Preview>
      <div className="grid w-full max-w-160 gap-10">
        <div className="px-4">
          <SettingsPageTitle>Settings primitives</SettingsPageTitle>
        </div>
        <div className="grid gap-4">
          <SettingsSectionHeader className="px-4">
            <SettingsSectionTitle>Section heading</SettingsSectionTitle>
            <SettingsSectionDescription>
              Compose existing fields and controls inside the shared surface.
            </SettingsSectionDescription>
          </SettingsSectionHeader>
          <SectionSurface className="p-0">
            <SettingsRow label={<label htmlFor="foundation-name">Name</label>} className="gap-4 bg-transparent p-4">
              <TextField id="foundation-name" defaultValue="Example value" className="w-56 max-w-full" />
            </SettingsRow>
            <SettingsDivider className="mx-4" />
            <SettingsRow
              label={<label htmlFor="foundation-enabled">Enabled</label>}
              description="A reusable switch with a caller-owned value."
              className="gap-4 bg-transparent p-4"
            >
              <Switch id="foundation-enabled" defaultChecked />
            </SettingsRow>
          </SectionSurface>
        </div>
      </div>
    </Preview>
  ),
}
