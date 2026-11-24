<script lang="ts">
  import { GeneralSettings, MessagingSettings, CodeSettings } from './preferences/user/page'

  import { ProfileSettings, AccessSettings, SecuritySettings, ApiKeySettings } from './account'
  import { OrganizationSettings, OrganizationOnboarding } from './organization'
  import { MembersSettings } from './members'
  import { GroupEditorSettings } from './group'
  import { RoleEditorSettings } from './role'
  import { UsageLimitSettings, MemberLimitSettings, UsageExplorerSettings, AuditSettings } from './usage'

  import { Empty, SettingsContent, SettingsDetailLoading } from './shared'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'

  const __settings = getSettingsController()
  let page = $derived(readStore(__settings.store, (s) => s.location.page))
  let status = $derived(readStore(__settings.store, (s) => s.status))
</script>

{#if status === 'loading'}<SettingsDetailLoading title="Settings" sections={['Settings']}
  ></SettingsDetailLoading>{:else}{#if status !== 'ready'}<SettingsContent>
      <Empty title="You do not have access">Ask an authorized administrator for access.</Empty>
    </SettingsContent>{:else}{#if page === 'general'}<GeneralSettings
      ></GeneralSettings>{:else if page === 'messaging'}<MessagingSettings
      ></MessagingSettings>{:else if page === 'code'}<CodeSettings></CodeSettings>{:else if page === 'profile'}<ProfileSettings
      ></ProfileSettings>{:else if page === 'access'}<AccessSettings
      ></AccessSettings>{:else if page === 'security'}<SecuritySettings
      ></SecuritySettings>{:else if page === 'api-keys'}<ApiKeySettings
      ></ApiKeySettings>{:else if page === 'organization'}<OrganizationSettings
      ></OrganizationSettings>{:else if page === 'organization-onboarding'}<OrganizationOnboarding
      ></OrganizationOnboarding>{:else if page === 'members'}<MembersSettings
      ></MembersSettings>{:else if page === 'role' || page === 'new-role'}<RoleEditorSettings
      ></RoleEditorSettings>{:else if page === 'group' || page === 'new-group'}<GroupEditorSettings
      ></GroupEditorSettings>{:else if page === 'usage-limits'}<UsageLimitSettings
      ></UsageLimitSettings>{:else if page === 'member-limits'}<MemberLimitSettings
      ></MemberLimitSettings>{:else if page === 'usage-explorer'}<UsageExplorerSettings
      ></UsageExplorerSettings>{:else if page === 'audit'}<AuditSettings></AuditSettings>{/if}{/if}{/if}
