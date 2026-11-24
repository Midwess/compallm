<script lang="ts">
  import { Button } from '../../components/buttons'

  import { SettingsContent, Heading, Section, Rows, Row, Empty } from './shared'
  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'
  import { SessionSettings } from './account'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController())
  let apps = $derived(readStore(__settings.store, (s) => s.applications))
</script>

<SettingsContent>
  <Heading description="Manage your sessions and authorized applications.">Access</Heading>
  <div class="grid gap-12">
    <SessionSettings></SessionSettings><Section
      title="Authorized applications"
      description="Review applications that can access your account."
    >
      {#if apps.length}<Rows>
          {#each apps as app, i (app.id)}<Row
              label={app.name}
              description={`Granted access: ${app.scope}`}
              last={i === apps.length - 1}
            >
              <Button variant="ghost" onclick={() => c.pages.dialogs.openDialog('application', app.id)}>Review</Button>
            </Row>{/each}
        </Rows>{:else}<Empty title="No authorized applications">Applications you authorize will appear here.</Empty>{/if}
    </Section>
  </div>
</SettingsContent>
