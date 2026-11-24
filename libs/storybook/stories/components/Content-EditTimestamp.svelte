<script lang="ts">
  import * as Content from '../../src/components/content'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Last edited"><Content.EditTimestamp dateTime="2026-10-31T09:00:00Z">just now</Content.EditTimestamp></Sample>
</Preview>
