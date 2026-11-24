<script lang="ts">
  import { optionalContext } from '../../components/form-context.svelte'
  import { PasskeySettings } from '../passkey'

  import { Button } from '../../components/buttons'

  import { SettingsContent, Heading, Section, Rows, Row, Empty } from './shared'
  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController, getPasskeyController } from '../../context'
  import { SessionSettings } from './account'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController())
  const passkeys = untrack(() => optionalContext(getPasskeyController))
  let previewPasskeys = $derived(readStore(__settings.store, (state) => state.passkeys))
  let applications = $derived(readStore(__settings.store, (state) => state.applications))
  let keys = $derived(readStore(__settings.store, (state) => state.apiKeys))
</script>

<SettingsContent>
  <Heading>Security &amp; access</Heading>
  <div class="grid gap-12">
    <SessionSettings></SessionSettings>{#if passkeys}<PasskeySettings></PasskeySettings>{:else}<Section
        title="Passkeys"
        description="Passkeys are a secure way to sign in to your Compallm account."
      >
        {#snippet action()}{#if previewPasskeys.length}<Button
              variant="ghost"
              onclick={() => c.pages.dialogs.openDialog('add-passkey')}
            >
              New passkey
            </Button>{:else}{undefined}{/if}{/snippet}{#if previewPasskeys.length}<Rows>
            {#each previewPasskeys as key, index (key.id)}<Row label={key.name} last={index === previewPasskeys.length - 1}>
                <Button variant="ghost" onclick={() => c.pages.dialogs.openDialog('remove-passkey', key.id)}>Revoke</Button>
              </Row>{/each}
          </Rows>{:else}<Empty title="No passkeys registered">
            {#snippet action()}<Button onclick={() => c.pages.dialogs.openDialog('add-passkey')}>
                New passkey
              </Button>{/snippet}Add a passkey for a simpler sign-in experience.
          </Empty>{/if}
      </Section>{/if}<Section title="Personal API keys" description="Use API keys to connect your tools to Compallm.">
      <Rows>
        <Row
          label={keys.length ? `${keys.length} API keys` : 'No API keys created'}
          description="Review your personal API access."
          last
        >
          <Button variant="ghost" onclick={() => c.navigate('api-keys')}>Manage API keys</Button>
        </Row>
      </Rows>
    </Section><Section title="Authorized applications" description="Applications you’ve approved">
      {#if applications.length}<Rows>
          {#each applications as app, index (app.id)}<Row
              label={app.name}
              description={`Granted access: ${app.scope}`}
              last={index === applications.length - 1}
            >
              <Button variant="ghost" onclick={() => c.pages.dialogs.openDialog('application', app.id)}>Review</Button>
            </Row>{/each}
        </Rows>{:else}<Empty title="No authorized applications">Applications you authorize will appear here.</Empty>{/if}
    </Section>
  </div>
</SettingsContent>
