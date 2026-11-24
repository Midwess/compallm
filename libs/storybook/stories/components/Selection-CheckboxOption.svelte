<script lang="ts">
  import * as Selection from '../../src/components/selection'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Checkbox labels">
    <Selection.CheckboxOption defaultChecked>Low</Selection.CheckboxOption><Selection.CheckboxOption>
      Medium
    </Selection.CheckboxOption><Selection.CheckboxOption disabled>Unavailable</Selection.CheckboxOption>
  </Sample>
</Preview>
