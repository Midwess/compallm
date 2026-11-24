<script lang="ts">
  import * as Fields from '../../src/components/fields'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Inline title" wide>
    <Fields.InlineTitleEditor aria-label="Title" value="Compallm"></Fields.InlineTitleEditor>
  </Sample>
</Preview>
