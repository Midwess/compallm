<script lang="ts">
  import { SettingsPageTitle } from '../../components/settings'

  import { cx } from '../../components/shared'

  import type { Content } from '../../components/shared'

  import ContentValue from '../../components/Content.svelte'
  import { getSettingsController } from '../../context'
  import type { SettingsDestination } from './types'

  const controller = getSettingsController()
  const descriptions: Partial<Record<SettingsDestination, string>> = {
    general: 'Choose how your workspace looks and opens.',
    messaging: 'Choose how you write and send messages.',
    code: 'Customize how code appears in your workspace.',
    profile: 'Manage your profile and personal details.',
    access: 'Manage your sessions and authorized applications.',
    security: 'Manage sign-in methods and account access.',
    'api-keys': 'Manage API access for your organization membership.',
    organization: 'Manage your organization’s name and identity.',
    'organization-onboarding': 'Customize the welcome experience for new members.',
    members: 'Manage workspace members and invitations.',
    role: 'Choose what members with this role can access.',
    'new-role': 'Define permissions for a new role.',
    group: 'Organize members and assign shared roles.',
    'new-group': 'Create a group for members with shared roles.',
    'usage-limits': 'Review workspace spending limits and reset schedules.',
    'member-limits': 'Review individual spending limits and usage.',
    'usage-explorer': 'Explore model usage and provider costs.',
    audit: 'Review security and permission changes.',
  }

  let {
    children,
    description,
    action,
    class: className,
  }: {
    children: Content
    description?: Content
    action?: Content
    class?: string
  } = $props()
  let summary = $derived(description ?? descriptions[controller.store.getState().location.page])
</script>

<div class={cx('mb-10 px-4', className)}>
  <div class="flex flex-wrap items-center justify-between gap-4">
    <SettingsPageTitle><ContentValue value={children} /></SettingsPageTitle><ContentValue value={action} />
  </div>
  {#if summary}<p class="mt-2 text-xsm leading-5.5 text-muted-foreground">
      <ContentValue value={summary} />
    </p>{:else}{/if}
</div>
