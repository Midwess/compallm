<script lang="ts">
  import { MainPage } from '../../../src/pages/main'
  import { PageHeader, Title, Functions } from '../../../src/components/page-header'
  import { Button } from '../../../src/components/buttons'
  import SidebarDemo from '../../components/SidebarDemo.svelte'
</script>

<MainPage>
  {#snippet sidebar()}<SidebarDemo scenario="Selected"></SidebarDemo>{/snippet}<PageHeader>
    <Title>Workspace content</Title><Functions><Button variant="ghost">Add item</Button></Functions>
  </PageHeader>
  <section class="px-3 py-6">
    <p class="text-xs text-muted-foreground">Feature pages supply this content through children.</p>
  </section>
</MainPage>
