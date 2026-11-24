<script lang="ts">
  import { PageHeader, Title, Functions, Button, SearchField } from '../../src'
  let { scenario = 'Default' }: { scenario?: string } = $props()
  let action = $state('')
</script>

<div data-ui="component-preview" class="min-h-64 bg-surface font-sans text-foreground">
  <PageHeader>
    <Title>
      {scenario === 'LongTitle' ? 'A longer page title that truncates while keeping actions visible' : 'Members'}
    </Title><Functions><Button variant="ghost" onclick={() => (action = 'Invite member')}>Invite member</Button></Functions>
  </PageHeader>
  <div class="grid gap-6 px-4 py-6">
    <SearchField aria-label="Search members" placeholder="Search members…" class="max-w-70" />
    <p data-ui="content-start" class="text-xs text-muted-foreground">Content shares the header title’s left edge.</p>
    <output aria-label="Header action" class="text-xs">{action}</output>
  </div>
</div>
