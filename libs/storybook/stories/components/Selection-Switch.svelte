<script lang="ts">
  import * as Selection from '../../src/components/selection'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
</script>

<Preview>
  <Sample label="Off, on, disabled">
    <Row>
      <Selection.Switch aria-label="Off switch"></Selection.Switch><Selection.Switch aria-label="On switch" defaultChecked
      ></Selection.Switch><Selection.Switch aria-label="Disabled switch" disabled></Selection.Switch>
    </Row>
  </Sample>
</Preview>
