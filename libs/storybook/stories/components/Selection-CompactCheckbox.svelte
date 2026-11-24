<script lang="ts">
  import * as Selection from '../../src/components/selection'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
</script>

<Preview>
  <Sample label="Compact · unchecked, checked, mixed, disabled">
    <Row>
      <Selection.Checkbox size="compact" aria-label="Unchecked compact"></Selection.Checkbox><Selection.Checkbox
        size="compact"
        aria-label="Checked compact"
        defaultChecked
      ></Selection.Checkbox><Selection.Checkbox size="compact" aria-label="Mixed compact" indeterminate
      ></Selection.Checkbox><Selection.Checkbox size="compact" aria-label="Disabled compact" disabled></Selection.Checkbox>
    </Row>
  </Sample>
</Preview>
