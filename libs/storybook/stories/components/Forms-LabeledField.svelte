<script lang="ts">
  import * as Forms from '../../src/components/forms'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Empty">
    <Forms.LabeledField label="Application name" hint="Explain what the field expects." placeholder="Enter a value…"
    ></Forms.LabeledField>
  </Sample><Sample label="Filled">
    <Forms.LabeledField label="Filled application name" hint="Supporting information." value="Application name"
    ></Forms.LabeledField>
  </Sample><Sample label="Error">
    <Forms.LabeledField label="Invalid application name" hint="Use a recognizable name." value="" error="A name is required."
    ></Forms.LabeledField>
  </Sample>
</Preview>
