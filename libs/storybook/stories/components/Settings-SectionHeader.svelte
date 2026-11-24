<script lang="ts">
  import { SettingsSectionHeader } from '../../src/components/settings'
  import { SettingsSectionDescription, SettingsSectionTitle } from '../../src/components/settings'

  import Preview from './Preview.svelte'
</script>

<Preview>
  <div class="grid w-full max-w-160 gap-12 px-4">
    <SettingsSectionHeader>
      <SettingsSectionTitle>Section heading</SettingsSectionTitle><SettingsSectionDescription>
        Helpful context for this section.
      </SettingsSectionDescription>
    </SettingsSectionHeader><SettingsSectionHeader>
      <SettingsSectionTitle>Heading without a description</SettingsSectionTitle>
    </SettingsSectionHeader>
  </div>
</Preview>
