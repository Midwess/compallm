<script lang="ts">
  import * as Fields from '../../src/components/fields'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Description" wide>
    <Fields.DescriptionEditor aria-label="Description" placeholder="Write a description…"></Fields.DescriptionEditor>
  </Sample>
</Preview>
