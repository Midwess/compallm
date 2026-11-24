<script lang="ts">
  import { PermissionRequest } from '../../components/permission'

  import { SidebarLoading } from '../../components/sidebar'

  import type { SettingsDestination } from './types'

  import type { Content } from '../../components/shared'

  import ContentValue from '../../components/Content.svelte'

  let {
    page,
    children: content,
  }: {
    page: SettingsDestination
    children: Content
  } = $props()
</script>

{#if page === 'organization-onboarding'}<PermissionRequest actions={['organization.onboarding.read']}>
    {#snippet pending()}<SidebarLoading rows={1}></SidebarLoading>{/snippet}{#snippet children(
      allowed,
    )}{#if allowed}<ContentValue value={content} />{:else}{/if}{/snippet}
  </PermissionRequest>{:else}{#if page === 'organization'}<PermissionRequest actions={['organization.settings.read']}>
      {#snippet pending()}<SidebarLoading rows={1}></SidebarLoading>{/snippet}{#snippet children(
        allowed,
      )}{#if allowed}<ContentValue value={content} />{:else}{/if}{/snippet}
    </PermissionRequest>{:else}{#if page === 'members'}<PermissionRequest actions={['member.read']}>
        {#snippet pending()}<SidebarLoading rows={1}></SidebarLoading>{/snippet}{#snippet children(
          allowed,
        )}{#if allowed}<ContentValue value={content} />{:else}{/if}{/snippet}
      </PermissionRequest>{:else}<ContentValue value={content} />{/if}{/if}{/if}
