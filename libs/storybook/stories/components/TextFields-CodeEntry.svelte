<script lang="ts">
  import * as Fields from '../../src/components/fields'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Six digits · paste and autofill">
    <Fields.CodeEntry aria-label="Verification code"></Fields.CodeEntry>
  </Sample><Sample label="Filled"><Fields.CodeEntry aria-label="Filled code" value="123456"></Fields.CodeEntry></Sample><Sample
    label="Error"
  >
    <Fields.CodeEntry aria-label="Invalid code" aria-invalid={true} value="654321"></Fields.CodeEntry>
  </Sample><Sample label="Disabled"><Fields.CodeEntry aria-label="Disabled code" disabled></Fields.CodeEntry></Sample>
</Preview>
