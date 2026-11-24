<script lang="ts">
  import * as Fields from '../../src/components/fields'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Code digit · 52 × 56"><Fields.CodeField aria-label="Code digit"></Fields.CodeField></Sample><Sample
    label="Filled"
  >
    <Fields.CodeField aria-label="Filled digit" value="4"></Fields.CodeField>
  </Sample>
</Preview>
