<script lang="ts">
  import { OrganizationOnboardingEditor } from './onboarding/page'

  import { SettingsContent, Heading, Empty } from './shared'
  import { untrack } from 'svelte'

  import { getSettingsController } from '../../context'

  const controller = untrack(() => getSettingsController().pages.onboarding)
</script>

{#if !controller}<SettingsContent>
    <Heading>Onboarding</Heading><Empty title="Onboarding unavailable"></Empty>
  </SettingsContent>{:else}<OrganizationOnboardingEditor></OrganizationOnboardingEditor>{/if}
