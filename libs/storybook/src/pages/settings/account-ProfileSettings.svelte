<script lang="ts">
  import { LoadingRegion } from '../../components/loading'

  import { FormError } from '../../components/form'
  import { Button } from '../../components/buttons'
  import { TextField } from '../../components/fields'
  import { SettingsContent, Heading, Section, Rows, Row, Empty, AvatarUploadControl } from './shared'
  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController())
  let profile = $derived(readStore(__settings.store, (s) => s.profile, ['profile']))
  let busy = $derived(readStore(__settings.store, (s) => s.profileSaving))
  let status = $derived(readStore(__settings.store, (s) => s.profileStatus))
  let ready = $derived(readStore(__settings.store, (s) => s.profileLoaded ?? s.profileStatus === 'ready'))
</script>

{#if status === 'denied' || status === 'missing' || (status === 'error' && !ready)}<SettingsContent>
    <Heading>Profile</Heading><Empty
      title={status === 'denied'
        ? 'Profile access denied'
        : status === 'missing'
          ? 'Profile not found'
          : 'Couldn’t load profile'}
    >
      {#snippet action()}{#if status === 'error' && c.pages.profile.reload}<Button onclick={c.pages.profile.reload}>
            Retry
          </Button>{:else}{undefined}{/if}{/snippet}Your profile could not be loaded.
    </Empty>
  </SettingsContent>{:else}<SettingsContent>
    <Heading>Profile</Heading><LoadingRegion {ready} pending={status === 'loading'} label="Loading profile" class="grid gap-12">
      <Rows>
        <Row label="Profile picture" description="Choose a photo for your account">
          <AvatarUploadControl
            name={profile.name || profile.username}
            saved={profile.avatar}
            field="profileUpload"
            label="Upload profile photo"
            disabled={busy}
            disabledReason="Saving your profile."
          ></AvatarUploadControl>
        </Row><Row description="Your name as it appears in Compallm">
          {#snippet label()}<label for="settings-full-name">Full name</label>{/snippet}<TextField
            id="settings-full-name"
            class="w-70 max-w-full"
            name="profile.name"
          ></TextField><FormError name="profile.name"></FormError>
        </Row><Row description="Your unique username">
          {#snippet label()}<label for="settings-username">Username</label>{/snippet}<TextField
            id="settings-username"
            class="w-70 max-w-full"
            name="profile.username"
          ></TextField><FormError name="profile.username"></FormError>
        </Row><Row description="Email changes are not available yet." last>
          {#snippet label()}<label for="settings-email">Email</label>{/snippet}<TextField
            id="settings-email"
            type="email"
            variant="constant"
            class="w-70 max-w-full"
            value={profile.email}
            placeholder="Not available"
          ></TextField>
        </Row>
      </Rows><Section title="Danger zone" description="Manage your personal account.">
        <Rows>
          <Row label="Delete account" description="Manage the deletion of your Compallm account." last>
            <Button variant="ghost" class="text-destructive" onclick={() => c.pages.dialogs.openDialog('delete-account')}>
              Delete account
            </Button>
          </Row>
        </Rows>
      </Section>
    </LoadingRegion>
  </SettingsContent>{/if}
