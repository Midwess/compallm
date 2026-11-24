<script lang="ts">
  import * as Fields from '../../src/components/fields'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Inline summary" wide>
    <Fields.InlineSummaryEditor aria-label="Summary" placeholder="Add a short summary…"></Fields.InlineSummaryEditor>
  </Sample>
</Preview>
