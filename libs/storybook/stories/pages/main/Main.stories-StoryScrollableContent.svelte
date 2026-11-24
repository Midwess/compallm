<script lang="ts">
  import { MainPage } from '../../../src/pages/main'
  import { PageHeader, Title } from '../../../src/components/page-header'

  import SidebarDemo from '../../components/SidebarDemo.svelte'
</script>

<MainPage>
  {#snippet sidebar()}<SidebarDemo></SidebarDemo>{/snippet}<PageHeader>
    <Title>Scrollable content</Title>
  </PageHeader>
  <section class="px-3 py-6">
    <div class="grid gap-4">
      {#each Array.from({ length: 60 }) as _, index (index)}<p class="text-xs">Example item {index + 1}</p>{/each}
    </div>
  </section>
</MainPage>
