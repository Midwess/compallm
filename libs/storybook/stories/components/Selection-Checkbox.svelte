<script lang="ts">
  import * as Selection from '../../src/components/selection'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
</script>

<Preview>
  <Sample label="Unchecked, checked, mixed, disabled">
    <Row>
      <Selection.Checkbox aria-label="Unchecked"></Selection.Checkbox><Selection.Checkbox aria-label="Checked" defaultChecked
      ></Selection.Checkbox><Selection.Checkbox aria-label="Mixed" indeterminate></Selection.Checkbox><Selection.Checkbox
        aria-label="Disabled"
        disabled
      ></Selection.Checkbox>
    </Row>
  </Sample>
</Preview>
