<script lang="ts">
  import { preferenceOptions } from './values'

  import { SettingsContent, Heading, Section } from '../../shared'

  import { PreferenceRows } from './page'
</script>

<SettingsContent>
  <Heading>Messaging</Heading><Section title="Messages" description="Choose how you write and send messages.">
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
    ></PreferenceRows>
  </Section>
</SettingsContent>
