<script lang="ts">
  import { NavigationTab } from '../../src'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
  let selected = $state('Overview')
</script>

<Preview>
  <Sample label="View selection">
    <Row>
      {#each ['Overview', 'Activity'] as label (label)}<NavigationTab
          selected={selected === label}
          onclick={() => (selected = label)}
        >
          {label}
        </NavigationTab>{/each}
    </Row>
  </Sample>
</Preview>
