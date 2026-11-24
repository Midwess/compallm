<script lang="ts">
  import { Button } from '../../../../components/buttons'

  import { SettingsContent, Heading, Section, Rows, Row } from '../../shared'
  import { untrack } from 'svelte'

  import { getSettingsController } from '../../../../context'
  import { PreferenceControl, PreferenceRows } from './page'
  import { general, appearance, themes } from './page'

  const c = untrack(() => getSettingsController())
</script>

<SettingsContent>
  <Heading>General</Heading>
  <div class="grid gap-12">
    <Section title="General" description="Choose how Compallm opens.">
      <PreferenceRows items={general}></PreferenceRows>
    </Section><Section title="Interface" description="Make the workspace comfortable for you.">
      <Rows>
        <Row label="App sidebar" description="Choose which items appear in your sidebar.">
          <Button variant="ghost" onclick={() => c.pages.dialogs.openDialog('customize-sidebar')}>Customize</Button>
        </Row>{#each appearance as item, i (i)}<PreferenceControl {item} last={i === appearance.length - 1}
          ></PreferenceControl>{/each}
      </Rows>
    </Section><Section title="Interface theme" description="Follow your system or choose a theme.">
      <PreferenceRows items={themes}></PreferenceRows>
    </Section>
  </div>
</SettingsContent>
