<script lang="ts">
  import * as Data from '../../src/components/data'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Activity">
    <Data.ActivityEvent actor="Alex Morgan" dateTime="2026-10-27" timeLabel="Oct 27">created the project</Data.ActivityEvent>
  </Sample>
</Preview>
