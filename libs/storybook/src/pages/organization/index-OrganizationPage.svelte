<script lang="ts">
  import { MainPage } from '../main'

  import type { Content } from '../../components/shared'

  import { OrganizationSidebar } from './index'

  import ContentValue from '../../components/Content.svelte'

  let {
    children,
  }: {
    children?: Content
  } = $props()
</script>

<MainPage>{#snippet sidebar()}<OrganizationSidebar></OrganizationSidebar>{/snippet}<ContentValue value={children} /></MainPage>
