<script lang="ts">
  import { Button } from '../../components/buttons'

  import { SettingsContent, Heading, Section, Rows, Row, Empty } from './shared'
  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController())
  let keys = $derived(readStore(__settings.store, (s) => s.apiKeys))
  let organization = $derived(readStore(__settings.store, (s) => s.organization.name, ['organization.name']))
</script>

<SettingsContent>
  <Heading description={`Manage API access for your membership in ${organization}.`}>API keys</Heading><Section
    title="Personal API keys"
    description="Keys are scoped to your current organization membership."
  >
    {#snippet action()}{#if keys.length}<Button onclick={() => c.pages.dialogs.openDialog('create-key')}>
          Create API key
        </Button>{:else}{/if}{/snippet}{#if keys.length}<Rows>
        {#each keys as key, i (key.id)}<Row label={key.name} description={key.prefix} last={i === keys.length - 1}>
            <Button variant="ghost" onclick={() => c.pages.dialogs.openDialog('revoke-key', key.id)}>Revoke</Button>
          </Row>{/each}
      </Rows>{:else}<Empty title="No API keys yet">
        {#snippet action()}<Button onclick={() => c.pages.dialogs.openDialog('create-key')}>
            Create API key
          </Button>{/snippet}Create a key to connect your tools to Compallm.
      </Empty>{/if}
  </Section>
  <p class="mt-4 px-4 text-xsm leading-5.5 text-muted-foreground">
    Keep API keys private. Revoke a key when it is no longer needed.
  </p>
</SettingsContent>
