<script lang="ts">
  import { LoadingRegion } from '../../components/loading'

  import { usePermission } from '../../components/permission'
  import { FormError } from '../../components/form'
  import { Button } from '../../components/buttons'
  import { TextField, UrlPreview } from '../../components/fields'
  import { SettingsContent, Heading, Section, Rows, Row, Empty, AvatarUploadControl } from './shared'
  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'

  import { browserOrigin } from './organization'

  const __settings = getSettingsController()
  let origin = $derived(browserOrigin())
  const c = untrack(() => getSettingsController())
  let org = $derived(readStore(__settings.store, (s) => s.organization, ['organization']))
  let savedName = $derived(readStore(__settings.store, (s) => s.organizationSavedName))
  let status = $derived(readStore(__settings.store, (s) => s.organizationProfileStatus))
  let canUpdateProfile = $derived(readStore(__settings.store, (s) => s.organizationCanUpdateProfile))
  let busy = $derived(readStore(__settings.store, (s) => s.organizationProfileSaving))
  let path = $derived(savedName || org.name ? `/${encodeURIComponent(savedName || org.name)}` : '')
  let organizationUrl = $derived(path && origin ? new URL(path, origin).href : path)
  const permission = usePermission(() => ({ actions: ['organization.settings.update'] }))
  let canUpdate = $derived(canUpdateProfile && permission.allowed)
  let ready = $derived(
    readStore(__settings.store, (s) => s.organizationProfileLoaded ?? s.organizationProfileStatus === 'ready'),
  )
</script>

{#if status === 'denied' || status === 'missing' || (status === 'error' && !ready)}<SettingsContent>
    <Heading>Organization</Heading><Empty
      title={status === 'missing'
        ? 'Organization not found'
        : status === 'denied'
          ? 'Organization access denied'
          : 'Couldn’t load organization'}
    >
      {#snippet action()}{#if status === 'error' && c.pages.organization.reload}<Button onclick={c.pages.organization.reload}>
            Retry
          </Button>{:else}{undefined}{/if}{/snippet}Organization profile is unavailable.
    </Empty>
  </SettingsContent>{:else}<SettingsContent>
    <Heading>Organization</Heading>{#if !canUpdate && ready && !permission.pending}<p
        class="mb-4 px-4 text-xsm text-muted-foreground"
      >
        You do not have permission to update this organization.
      </p>{:else}{/if}<LoadingRegion {ready} pending={status === 'loading'} label="Loading organization" class="grid gap-12">
      <Section title="Organization details" description="Your shared workspace identity.">
        <Rows>
          <Row label="Logo" description="Recommended size is 256×256px">
            <AvatarUploadControl
              name={org.name}
              saved={org.logo}
              field="organizationUpload"
              label="Upload organization logo"
              disabled={!canUpdate || busy}
              disabledReason={!canUpdate
                ? 'You do not have permission to update this organization.'
                : 'Saving your organization.'}
            ></AvatarUploadControl>
          </Row><Row>
            {#snippet label()}<label for="organization-name">Name</label>{/snippet}<TextField
              id="organization-name"
              class="w-70 max-w-full"
              name="organization.name"
              readonly={!canUpdate}
            ></TextField><FormError name="organization.name"></FormError>
          </Row><Row label="URL" last>
            <UrlPreview class="w-70 max-w-full" url={organizationUrl} onCopy={c.pages.dialogs.copy}></UrlPreview>
          </Row>
        </Rows>
      </Section><Section title="Danger zone" description="Manage deletion of this organization.">
        <Rows>
          <Row label="Delete organization" description="Manage deletion of this organization." last>
            <Button variant="ghost" class="text-destructive" onclick={() => c.pages.dialogs.openDialog('delete-organization')}>
              Delete organization
            </Button>
          </Row>
        </Rows>
      </Section>
      <div class="px-4"><Button variant="ghost" onclick={() => c.navigate('audit')}>Audit log →</Button></div>
    </LoadingRegion>
  </SettingsContent>{/if}
