<script lang="ts">
  import { SettingsDivider, SettingsPageTitle, SettingsSectionHeader } from '../../src/components/settings'
  import { SettingsSectionDescription, SettingsSectionTitle } from '../../src/components/settings'
  import { SettingsRow } from '../../src/components/content'
  import { SectionSurface } from '../../src/components/surfaces'
  import { TextField } from '../../src/components/fields'
  import { Switch } from '../../src/components/selection'
  import Preview from './Preview.svelte'
</script>

<Preview>
  <div class="grid w-full max-w-160 gap-10">
    <div class="px-4"><SettingsPageTitle>Settings primitives</SettingsPageTitle></div>
    <div class="grid gap-4">
      <SettingsSectionHeader class="px-4">
        <SettingsSectionTitle>Section heading</SettingsSectionTitle><SettingsSectionDescription>
          Compose existing fields and controls inside the shared surface.
        </SettingsSectionDescription>
      </SettingsSectionHeader><SectionSurface class="p-0">
        <SettingsRow class="gap-4 bg-transparent p-4">
          {#snippet label()}<label for="foundation-name">Name</label>{/snippet}<TextField
            id="foundation-name"
            value="Example value"
            class="w-56 max-w-full"
          ></TextField>
        </SettingsRow><SettingsDivider class="mx-4"></SettingsDivider><SettingsRow
          description="A reusable switch with a caller-owned value."
          class="gap-4 bg-transparent p-4"
        >
          {#snippet label()}<label for="foundation-enabled">Enabled</label>{/snippet}<Switch
            id="foundation-enabled"
            defaultChecked
          ></Switch>
        </SettingsRow>
      </SectionSurface>
    </div>
  </div>
</Preview>
