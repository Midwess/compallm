<script lang="ts">
  import { Rows } from '../../shared'

  import { PreferenceControl } from './page'
  import type { Preference } from './page'

  let {
    items,
  }: {
    items: Preference[]
  } = $props()
</script>

<Rows>
  {#each items as item, i (i)}<PreferenceControl {item} last={i === items.length - 1}></PreferenceControl>{/each}
</Rows>
