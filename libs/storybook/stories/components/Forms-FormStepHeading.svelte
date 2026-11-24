<script lang="ts">
  import * as Forms from '../../src/components/forms'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview><Sample label="Step heading"><Forms.FormStepHeading step={1}>Section title</Forms.FormStepHeading></Sample></Preview>
