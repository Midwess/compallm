import type { CSSProperties } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { Table } from '../../src/components/table'
import { FormFieldLabel } from '../../src/components/forms'
import { HelperText } from '../../src/components/feedback'
import { SettingsPageTitle, SettingsSectionTitle } from '../../src/components/settings'
import { foundationFamilies, penpotFoundationUrl } from './foundation-mapping'

const meta = {
  title: 'Components/Foundations',
  parameters: { layout: 'fullscreen' },
} satisfies Meta
export default meta
type Story = StoryObj<typeof meta>

const linkClassName = `text-ring underline-offset-4 hover:underline
focus-visible:outline-2 focus-visible:outline-ring`

const colors = [
  ['Background', 'bg-background', '#09090A'],
  ['Surface', 'bg-surface', '#121213'],
  ['Inset', 'bg-inset', '#1A1A1B'],
  ['Composer', 'bg-composer', '#1D1E1F'],
  ['Elevated', 'bg-elevated', '#232325'],
  ['Menu', 'bg-menu', '#28292B'],
  ['Divider', 'bg-divider', '#212224'],
  ['Border', 'bg-border', '#303135'],
  ['Input border', 'bg-input', '#48494D'],
  ['Text', 'bg-foreground', '#E4E5E7'],
  ['Muted', 'bg-muted-foreground', '#98999C'],
  ['Disabled', 'bg-disabled', '#626365'],
  ['Primary', 'bg-primary', '#5E6AD2'],
  ['Primary hover', 'bg-primary-hover', '#6D78D5'],
  ['Focus', 'bg-ring', '#8FA6FF'],
  ['On primary', 'bg-primary-foreground', '#FFFFFF'],
  ['Success', 'bg-success', '#41BA56'],
  ['Warning', 'bg-warning', '#F2994A'],
  ['Error', 'bg-destructive', '#EB5757'],
] as const

export const Mapping: Story = {
  render: () => (
    <div className="grid gap-6 p-6">
      <div className="grid gap-2">
        <SettingsPageTitle>Design foundations</SettingsPageTitle>
        <p className="text-xsm leading-5 text-muted-foreground">
          Penpot foundation families map to shared components and their existing Storybook previews.
        </p>
      </div>
      <Table>
        <Table.Content className="min-w-3xl text-xsm" aria-label="Penpot to Storybook foundation mapping">
          <Table.Header>
            <Table.Row>
              <Table.Head className="w-3xs">Penpot family</Table.Head>
              <Table.Head>Library source</Table.Head>
              <Table.Head className="w-32">Preview</Table.Head>
            </Table.Row>
          </Table.Header>
          <Table.Body>
            {foundationFamilies.map((family) => (
              <Table.Row key={family.board}>
                <Table.Cell>
                  <a href={penpotFoundationUrl(family.board)} target="_blank" rel="noreferrer" className={linkClassName}>
                    {family.name}
                  </a>
                </Table.Cell>
                <Table.Cell className="wrap-break-word text-muted-foreground">{family.source}</Table.Cell>
                <Table.Cell>
                  <a href={`/?path=/story/${family.story}`} target="_top" className={linkClassName}>
                    Open story
                  </a>
                </Table.Cell>
              </Table.Row>
            ))}
          </Table.Body>
        </Table.Content>
      </Table>
    </div>
  ),
}

export const Colors: Story = {
  render: () => (
    <div className="grid gap-6 p-6">
      <SettingsPageTitle>Compallm colors</SettingsPageTitle>
      <div className="grid grid-cols-[repeat(auto-fit,minmax(160px,1fr))] gap-4">
        {colors.map(([name, background, hex]) => (
          <figure key={name} className="grid gap-2">
            <div className={`${background} h-16 rounded-lg border-(length:--border-hairline) border-input`} />
            <figcaption className="text-xsm">
              {name} <span className="block text-xs text-muted-foreground">{hex}</span>
            </figcaption>
          </figure>
        ))}
      </div>
    </div>
  ),
}

export const Typography: Story = {
  render: () => (
    <div className="grid max-w-3xl gap-8 p-6">
      <SettingsPageTitle>Page title · 24 / 32 · Medium</SettingsPageTitle>
      <SettingsSectionTitle>Section title · 15 / 23 · Medium</SettingsSectionTitle>
      <FormFieldLabel>Field label · 14 / 21 · Medium</FormFieldLabel>
      <p className="text-xsm leading-5">Body text · 13 / 20 · Regular</p>
      <HelperText>Helper text · 12 / 18 · Regular</HelperText>
    </div>
  ),
}

const spaces = [
  ['2', 'w-0.5'],
  ['4', 'w-1'],
  ['6', 'w-1.5'],
  ['8', 'w-2'],
  ['10', 'w-2.5'],
  ['12', 'w-3'],
  ['16', 'w-4'],
  ['24', 'w-6'],
  ['32', 'w-8'],
  ['48', 'w-12'],
  ['64', 'w-16'],
] as const

export const Spacing: Story = {
  render: () => (
    <div className="grid max-w-xl gap-4 p-6">
      <SettingsPageTitle>Resolved spacing</SettingsPageTitle>
      <HelperText>Use half steps on Tailwind’s default scale, starting at 2 px, and preserve justified exceptions.</HelperText>
      {spaces.map(([pixels, width]) => (
        <div key={pixels} className="flex items-center gap-4 text-xs">
          <span className="w-12 text-muted-foreground">{pixels} px</span>
          <span className={`${width} h-4 shrink-0 rounded-sm bg-primary`} aria-hidden="true" />
        </div>
      ))}
    </div>
  ),
}

export const RuntimeTokens: Story = {
  render: () => (
    <div className="grid gap-6 p-6 sm:grid-cols-2">
      {[
        { name: 'Default', style: {} },
        {
          name: 'Scoped color and larger text',
          style: {
            '--background': 'oklch(0.32 0.04 250)',
            '--foreground': 'oklch(0.96 0.01 250)',
            '--primary': 'oklch(0.72 0.15 160)',
            '--ui-font-scale': '1.25',
          } as CSSProperties,
        },
      ].map(({ name, style }) => (
        <section key={name} style={style} className="grid gap-4 rounded-lg bg-background p-6 text-foreground">
          <h2 className="text-ui-20 font-medium">{name}</h2>
          <p className="text-xsm">Semantic colors and type scale apply at runtime.</p>
          <span className="w-fit rounded bg-primary px-3 py-2 text-xs text-primary-foreground">Primary action</span>
        </section>
      ))}
    </div>
  ),
}
