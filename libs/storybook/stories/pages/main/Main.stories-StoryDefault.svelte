<script lang="ts">
  import { MainPage } from '../../../src/pages/main'

  import SidebarDemo from '../../components/SidebarDemo.svelte'
</script>

<MainPage>{#snippet sidebar()}<SidebarDemo></SidebarDemo>{/snippet}</MainPage>
