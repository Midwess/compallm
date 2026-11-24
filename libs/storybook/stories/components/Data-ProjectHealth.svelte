<script lang="ts">
  import * as Data from '../../src/components/data'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
</script>

<Preview>
  <Sample label="Health status">
    <Row>
      <Data.ProjectHealth>No updates</Data.ProjectHealth><Data.ProjectHealth tone="success">
        On track
      </Data.ProjectHealth><Data.ProjectHealth tone="warning">At risk</Data.ProjectHealth>
    </Row>
  </Sample>
</Preview>
