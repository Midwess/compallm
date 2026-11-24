<script lang="ts">
  import { PermissionRequest } from '../../../components/permission'

  import { PageHeader, Title } from '../../../components/page-header'

  import { ChatInput } from './index'
</script>

<PermissionRequest actions={['model.read']}>
  {#snippet children(allowed)}{#if allowed}<section aria-label="Create chat" class="relative flex h-full min-h-0 flex-col">
        <PageHeader class="absolute inset-x-0 top-0"><Title>New chat</Title></PageHeader>
        <div class="flex min-h-0 flex-1 items-center justify-center overflow-auto px-3 py-6"><ChatInput></ChatInput></div>
      </section>{:else}<p role="status">You do not have access to this page.</p>{/if}{/snippet}
</PermissionRequest>
