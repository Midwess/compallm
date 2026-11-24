<script lang="ts">
  import * as Data from '../../src/components/data'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
</script>

<Preview>
  <Sample label="Progress">
    <Row>
      <Data.ProjectProgress value={0}></Data.ProjectProgress><Data.ProjectProgress value={35}
      ></Data.ProjectProgress><Data.ProjectProgress value={100}></Data.ProjectProgress>
    </Row>
  </Sample>
</Preview>
