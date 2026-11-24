<script lang="ts">
  import * as Fields from '../../src/components/fields'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Document title" wide>
    <Fields.DocumentTitleEditor aria-label="Document title" value="Untitled template"></Fields.DocumentTitleEditor>
  </Sample>
</Preview>
