import { preferenceOptions, preferenceLabel } from './values'
import { FormError } from '../../../../components/form'
import { useId } from 'react'
import { Button } from '../../../../components/buttons'
import { Select } from '../../../../components/dropdowns'
import { Switch } from '../../../../components/selection'
import { CodePreview } from '../../../../components/content'
import { Popup } from '../../../../components/popup'
import { CaretDownIcon } from '@phosphor-icons/react/dist/csr/CaretDown'
import { SettingsRow } from '../../../../components/content'
import { SettingsContent, Heading, Section, Rows, Row } from '../../shared'
import { useSettings, useSettingsController } from '../../controller'
import { useUserPreference, useUserPreferenceController, useCodeAppearance } from './controller'

type Preference = { key: string; label: string; description: string; options?: { value: string; label: string }[] }
const general: Preference[] = [
  {
    key: 'home',
    label: 'Default home view',
    description: 'The first page you see when you open Compallm.',
    options: preferenceOptions.home,
  },
  {
    key: 'displayNames',
    label: 'Display names',
    description: 'Choose how member names appear. Applying this preference is not available yet.',
    options: preferenceOptions.displayNames,
  },
  {
    key: 'firstDay',
    label: 'First day of the week',
    description: 'Used in calendars and reporting periods. Applying this preference is not available yet.',
    options: preferenceOptions.firstDay,
  },
]
const appearance: Preference[] = [
  {
    key: 'fontSize',
    label: 'Font size',
    description: 'Adjust the size of interface text.',
    options: preferenceOptions.fontSize,
  },
  { key: 'pointerCursors', label: 'Use pointer cursors', description: 'Use a pointer cursor on interactive elements.' },
  { key: 'underlineLinks', label: 'Underline links', description: 'Always underline links in message content.' },
  { key: 'reduceMotion', label: 'Reduce motion', description: 'Keep animated images and emoji still.' },
]
const themes: Preference[] = [
  {
    key: 'theme',
    label: 'Appearance',
    description: 'Choose your preferred color scheme.',
    options: preferenceOptions.theme,
  },
  {
    key: 'lightTheme',
    label: 'Light theme',
    description: 'Used when your system has a light appearance.',
    options: preferenceOptions.lightTheme,
  },
  {
    key: 'darkTheme',
    label: 'Dark theme',
    description: 'Used when your system has a dark appearance.',
    options: preferenceOptions.darkTheme,
  },
]

function PreferenceControl({ item, last = false }: { item: Preference; last?: boolean }) {
  const id = useId()
  return (
    <Row
      label={item.options ? item.label : <label htmlFor={id}>{item.label}</label>}
      description={item.description}
      last={last}
    >
      {item.options ? (
        <Select aria-label={item.label} name={`draft.${item.key}`} className="w-44" options={item.options} />
      ) : (
        <Switch id={id} name={`draft.${item.key}`} />
      )}
      <FormError name={`draft.${item.key}`} />
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
  const c = useSettingsController()
  return (
    <SettingsContent>
      <Heading>General</Heading>
      <div className="grid gap-12">
        <Section title="General" description="Choose how Compallm opens.">
          <PreferenceRows items={general} />
        </Section>
        <Section title="Interface" description="Make the workspace comfortable for you.">
          <Rows>
            <Row label="App sidebar" description="Choose which items appear in your sidebar.">
              <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('customize-sidebar')}>
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
              options: preferenceOptions.sendShortcut,
            },
          ]}
        />
      </Section>
    </SettingsContent>
  )
}
function CodeAppearanceSection() {
  const c = useUserPreferenceController(),
    fontSize = useUserPreference((s) => s.draft.codeFontSize, ['draft.codeFontSize']),
    font = useUserPreference((s) => s.draft.codeFont, ['draft.codeFont']),
    language = useUserPreference((s) => s.draft.codeLanguage, ['draft.codeLanguage']),
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
          options: preferenceOptions.codeTheme,
        }}
      />
      <SettingsRow label="Font" className="min-h-15 bg-transparent px-4 py-3">
        <Popup>
          <Popup.Trigger aria-label="Code font" className="h-8 w-auto gap-2 px-0 text-xs text-muted-foreground">
            {preferenceLabel(fontSize)}, Regular, {preferenceLabel(font)}
            <CaretDownIcon aria-hidden="true" className="size-3" />
          </Popup.Trigger>
          <Popup.Content aria-label="Code font settings">
            <Popup.Submenu label="Font size">
              {preferenceOptions.codeFontSize.map(({ value, label }) => (
                <Popup.Item key={value} onClick={() => c.store.form.change('draft.codeFontSize', value)}>
                  {label}
                </Popup.Item>
              ))}
            </Popup.Submenu>
            <Popup.Submenu label="Font">
              {preferenceOptions.codeFont.map(({ value, label }) => (
                <Popup.Item key={value} onClick={() => c.store.form.change('draft.' + 'codeFont', value)}>
                  {label}
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
          onLanguageChange={(value) => c.store.form.change('draft.' + 'codeLanguage', value)}
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
