import { useId } from 'react'
import { Button } from '../../components/buttons'
import { Select } from '../../components/dropdowns'
import { Switch } from '../../components/selection'
import { CodePreview } from '../../components/content'
import { Popup } from '../../components/popup'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { SettingsRow } from '../../components/content'
import { SettingsContent, Heading, Section, Rows, Row } from './shared'
import { useSettings, useSettingsController, useCodeAppearance } from './controller'

type Preference = { key: string; label: string; description: string; options?: string[] }
const general: Preference[] = [
  {
    key: 'home',
    label: 'Default home view',
    description: 'The first page you see when you open Compallm.',
    options: ['Chat', 'Models', 'Providers'],
  },
  {
    key: 'displayNames',
    label: 'Display names',
    description: 'Choose how member names appear.',
    options: ['Full name', 'Username'],
  },
  {
    key: 'firstDay',
    label: 'First day of the week',
    description: 'Used in calendars and reporting periods.',
    options: ['Monday', 'Sunday', 'Saturday'],
  },
]
const appearance: Preference[] = [
  {
    key: 'fontSize',
    label: 'Font size',
    description: 'Adjust the size of interface text.',
    options: ['Small', 'Default', 'Large'],
  },
  { key: 'underlineLinks', label: 'Underline links', description: 'Always underline links in message content.' },
  { key: 'reduceMotion', label: 'Reduce motion', description: 'Keep animated images and emoji still.' },
]
const themes: Preference[] = [
  {
    key: 'theme',
    label: 'Appearance',
    description: 'Choose your preferred color scheme.',
    options: ['System preference', 'Light', 'Dark'],
  },
  {
    key: 'lightTheme',
    label: 'Light theme',
    description: 'Used when your system has a light appearance.',
    options: ['Light', 'Pure Light'],
  },
  {
    key: 'darkTheme',
    label: 'Dark theme',
    description: 'Used when your system has a dark appearance.',
    options: ['Dark', 'Classic Dark'],
  },
]

function PreferenceControl({ item, last = false }: { item: Preference; last?: boolean }) {
  const id = useId(),
    c = useSettingsController(),
    value = useSettings((s) => s.preferences[item.key])
  return (
    <Row
      label={item.options ? item.label : <label htmlFor={id}>{item.label}</label>}
      description={item.description}
      last={last}
    >
      {item.options ? (
        <Select
          aria-label={item.label}
          value={String(value)}
          className="w-44"
          options={item.options.map((label) => ({ label, value: label }))}
          onValueChange={(v) => c.setPreference(item.key, v)}
        />
      ) : (
        <Switch id={id} checked={Boolean(value)} onChange={(e) => c.setPreference(item.key, e.target.checked)} />
      )}
    </Row>
  )
}
function PreferenceRows({ items }: { items: Preference[] }) {
  return (
    <Rows>
      {items.map((item, i) => (
        <PreferenceControl key={item.key} item={item} last={i === items.length - 1} />
      ))}
    </Rows>
  )
}
export function GeneralSettings() {
  return (
    <SettingsContent>
      <Heading>General</Heading>
      <Section title="General" description="Choose how Compallm opens and displays information.">
        <PreferenceRows items={general} />
      </Section>
    </SettingsContent>
  )
}
export function AppearanceSettings() {
  const c = useSettingsController()
  return (
    <SettingsContent>
      <Heading>Appearance</Heading>
      <div className="grid gap-12">
        <Section title="Interface" description="Make the workspace comfortable for you.">
          <Rows>
            <Row label="App sidebar" description="Choose which items appear in your sidebar.">
              <Button variant="ghost" onClick={() => c.openDialog('customize-sidebar')}>
                Customize
              </Button>
            </Row>
            {appearance.map((item, i) => (
              <PreferenceControl key={item.key} item={item} last={i === appearance.length - 1} />
            ))}
          </Rows>
        </Section>
        <Section title="Interface theme" description="Follow your system or choose a theme.">
          <PreferenceRows items={themes} />
        </Section>
        <CodeAppearanceSection />
      </div>
    </SettingsContent>
  )
}
export function MessagingSettings() {
  return (
    <SettingsContent>
      <Heading>Messaging</Heading>
      <Section title="Messages" description="Choose how you write and send messages.">
        <PreferenceRows
          items={[
            { key: 'emoticons', label: 'Convert emoticons to emoji', description: 'Convert text such as :) into emoji.' },
            {
              key: 'sendShortcut',
              label: 'Send messages on…',
              description: 'Choose the shortcut used to send messages.',
              options: ['⌘ + Enter', 'Enter'],
            },
          ]}
        />
      </Section>
    </SettingsContent>
  )
}
function CodeAppearanceSection() {
  const c = useSettingsController(),
    fontSize = useSettings((s) => s.preferences.codeFontSize),
    font = useSettings((s) => s.preferences.codeFont),
    language = useSettings((s) => String(s.preferences.codeLanguage ?? 'typescript')),
    examples = useSettings((s) => s.codeExamples)
  const example = examples[language] ?? { code: '' }
  const appearance = useCodeAppearance()
  return (
    <Rows>
      <PreferenceControl
        item={{
          key: 'codeTheme',
          label: 'Code theme',
          description: 'Matches your interface theme unless overridden.',
          options: ['Linear Dark', 'Linear Light', 'System preference'],
        }}
      />
      <SettingsRow label="Font" className="min-h-15 bg-transparent px-4 py-3">
        <Popup>
          <Popup.Trigger aria-label="Code font" className="h-8 w-auto gap-2 px-0 text-xs text-muted">
            {String(fontSize)}, Regular, {String(font)}
            <CaretDownIcon aria-hidden="true" className="size-3" />
          </Popup.Trigger>
          <Popup.Content aria-label="Code font settings">
            <Popup.Submenu label="Font size">
              {['12px', '13px', '14px', '16px'].map((size) => (
                <Popup.Item key={size} onClick={() => c.setPreference('codeFontSize', size)}>
                  {size}
                </Popup.Item>
              ))}
            </Popup.Submenu>
            <Popup.Submenu label="Font">
              {['Default', 'Monospace'].map((value) => (
                <Popup.Item key={value} onClick={() => c.setPreference('codeFont', value)}>
                  {value}
                </Popup.Item>
              ))}
            </Popup.Submenu>
          </Popup.Content>
        </Popup>
      </SettingsRow>
      <div className="px-2 pb-2">
        <CodePreview
          {...appearance}
          code={example.code}
          lineChanges={example.lineChanges}
          language={language}
          onLanguageChange={(value) => c.setPreference('codeLanguage', value)}
        />
      </div>
    </Rows>
  )
}

export function CodeSettings() {
  return (
    <SettingsContent>
      <Heading>Code appearance</Heading>
      <CodeAppearanceSection />
    </SettingsContent>
  )
}
