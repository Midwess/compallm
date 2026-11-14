import type { CSSProperties } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { Table } from '../../src/components/table'
import { Button } from '../../src/components/buttons'
import { Checkbox, Switch } from '../../src/components/selection'
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
  ['Background', 'bg-background', '--background'],
  ['Surface', 'bg-surface', '--card'],
  ['Inset', 'bg-inset', '--inset'],
  ['Composer', 'bg-composer', '--composer'],
  ['Elevated', 'bg-elevated', '--elevated'],
  ['Menu', 'bg-menu', '--popover'],
  ['Divider', 'bg-divider', '--divider'],
  ['Border', 'bg-border', '--border'],
  ['Input border', 'bg-input', '--input'],
  ['Text', 'bg-foreground', '--foreground'],
  ['Muted', 'bg-muted-foreground', '--muted-foreground'],
  ['Disabled', 'bg-disabled', '--disabled'],
  ['Primary', 'bg-primary', '--primary'],
  ['Primary hover', 'bg-primary-hover', '--primary-hover'],
  ['Focus', 'bg-ring', '--ring'],
  ['On primary', 'bg-primary-foreground', '--primary-foreground'],
  ['Success', 'bg-success', '--success'],
  ['Warning', 'bg-warning', '--warning'],
  ['Error', 'bg-destructive', '--destructive'],
  ['Shadow', 'bg-shadow', '--shadow-color'],
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
      <Table.Root>
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
      </Table.Root>
    </div>
  ),
}

export const Colors: Story = {
  render: () => (
    <div className="grid gap-6 p-6">
      <SettingsPageTitle>Compallm colors</SettingsPageTitle>
      <div className="grid grid-cols-[repeat(auto-fit,minmax(160px,1fr))] gap-4">
        {colors.map(([name, background, token]) => (
          <figure key={name} className="grid gap-2">
            <div className={`${background} h-16 rounded-lg border-(length:--border-hairline) border-input`} />
            <figcaption className="text-xsm">
              {name} <span className="block text-xs text-muted-foreground">{token}</span>
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
            '--shadow-color': 'oklch(0.18 0.04 250)',
            '--ui-font-scale': '1.25',
          } as CSSProperties,
        },
      ].map(({ name, style }) => (
        <section key={name} style={style} className="grid gap-4 rounded-lg bg-background p-6 text-foreground">
          <h2 className="text-ui-20 font-medium">{name}</h2>
          <p className="text-xsm">Semantic colors and type scale apply at runtime.</p>
          <span className="w-fit rounded bg-primary px-3 py-2 text-xs text-primary-foreground">Primary action</span>
          <Button>Control shadow</Button>
        </section>
      ))}
    </div>
  ),
}

export const InteractionColors: Story = {
  render: () => (
    <div className="grid gap-6 p-6 sm:grid-cols-2">
      <section className="grid gap-3 rounded-xl bg-sidebar p-4">
        <h2 className="text-sm font-medium">Sidebar</h2>
        <span className="rounded-lg px-3 py-2 text-xsm text-muted-foreground">Default destination</span>
        <span className="rounded-lg bg-hover px-3 py-2 text-xsm">Hovered destination</span>
        <span className="rounded-lg bg-selection px-3 py-2 text-xsm text-selection-foreground">Selected destination</span>
      </section>
      <section className="overflow-hidden rounded-xl border border-border bg-surface">
        <h2 className="border-b border-divider bg-toolbar px-4 py-3 text-sm font-medium">Workspace toolbar</h2>
        <div className="grid gap-3 p-4 text-xsm">
          <p>Content surface</p>
          <span className="rounded-lg bg-inset px-3 py-2 text-muted-foreground">Inset surface</span>
          <span className="rounded-lg bg-elevated px-3 py-2">Elevated control</span>
          <span className="w-fit rounded-lg bg-primary px-3 py-2 text-primary-foreground">Primary action</span>
        </div>
      </section>
    </div>
  ),
}

export const ThemeBalance: Story = {
  render: () => (
    <div className="grid gap-6 p-6">
      <div className="grid gap-2">
        <SettingsPageTitle>Theme balance</SettingsPageTitle>
        <HelperText>Switch the palette in the toolbar to compare surfaces, text, borders, and shadows.</HelperText>
      </div>
      <div className="grid gap-6 lg:grid-cols-3">
        <section className="grid content-start gap-3 rounded-xl border border-divider bg-sidebar p-4">
          <h2 className="text-sm font-medium">Navigation</h2>
          <span className="rounded-lg px-3 py-2 text-xsm text-muted-foreground">Default destination</span>
          <span className="rounded-lg bg-hover px-3 py-2 text-xsm">Hovered destination</span>
          <span className="rounded-lg bg-selection px-3 py-2 text-xsm text-selection-foreground">Selected destination</span>
          <span className="rounded-lg bg-tab-selection px-3 py-2 text-xsm">Selected tab</span>
        </section>
        <section className="overflow-hidden rounded-xl border border-border bg-surface">
          <h2 className="border-b border-divider bg-toolbar px-4 py-3 text-sm font-medium">Workspace</h2>
          <div className="grid gap-4 p-4 text-xsm">
            <p>Primary text on the content surface</p>
            <p className="text-muted-foreground">Supporting text stays readable without dominating the page.</p>
            <label className="grid gap-2">
              Input border and focus
              <input
                placeholder="Type a value…"
                className="h-8 min-w-0 rounded-lg border border-input bg-inset px-3 text-foreground
placeholder:text-muted-foreground focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring"
              />
            </label>
            <div className="flex flex-wrap gap-3">
              <Button variant="primary">Primary action</Button>
              <Button>Neutral action</Button>
              <Button variant="destructive">Delete</Button>
              <Button disabled>Disabled</Button>
            </div>
            <div className="flex flex-wrap items-center gap-4">
              <label className="inline-flex items-center gap-2">
                <Checkbox defaultChecked />
                Selected option
              </label>
              <label className="inline-flex items-center gap-2">
                <Switch defaultChecked />
                Enabled
              </label>
            </div>
            <div className="flex flex-wrap gap-3">
              <span className="text-success">Success</span>
              <span className="text-warning">Warning</span>
              <span className="text-destructive">Error</span>
              <Button variant="link">Link</Button>
            </div>
            <p className="rounded-lg bg-composer p-3">Composer surface</p>
            <p className="w-fit rounded-full bg-error-background px-3 py-1 text-xs text-error-foreground shadow-floating">
              Field validation message
            </p>
          </div>
        </section>
        <section className="grid content-start gap-6 rounded-xl bg-surface p-6">
          <h2 className="text-sm font-medium">Elevation</h2>
          <div className="grid gap-2 rounded-control border border-border bg-menu p-2 text-xsm shadow-menu">
            <span className="px-3 py-2 text-muted-foreground">Menu item</span>
            <span className="rounded-lg bg-menu-hover px-3 py-2">Hovered item</span>
            <span className="rounded-lg bg-menu-selection px-3 py-2 text-selection-foreground">Selected item</span>
          </div>
          <div className="grid gap-2 rounded-xl border border-border bg-elevated p-4 text-xsm shadow-floating">
            <p>Floating notification</p>
            <p className="text-muted-foreground">Shadow color and strength come from the active palette.</p>
          </div>
          <div className="max-h-20 overflow-auto rounded-lg bg-inset p-3 text-xsm text-muted-foreground">
            {Array.from({ length: 8 }, (_, index) => (
              <p key={index}>Themed scrollbar · row {index + 1}</p>
            ))}
          </div>
        </section>
      </div>
    </div>
  ),
}
